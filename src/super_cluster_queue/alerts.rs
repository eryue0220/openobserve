// Copyright 2024 Zinc Labs Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use config::{meta::alerts::alert::Alert, utils::json};
use infra::{
    db::{connect_to_orm, ORM_CLIENT},
    errors::{Error, Result},
    table,
};
use o2_enterprise::enterprise::super_cluster::queue::{Message, MessageType};
use serde::{Deserialize, Serialize};
use svix_ksuid::Ksuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AlertMessage {
    Create {
        org_id: String,
        folder_id: String,
        alert: Alert,
    },
    Update {
        org_id: String,
        folder_id: Option<String>,
        alert: Alert,
    },
    Delete {
        org_id: String,
        alert_id: Ksuid,
    },
}

impl TryFrom<Message> for AlertMessage {
    type Error = Error;

    fn try_from(msg: Message) -> std::result::Result<Self, Self::Error> {
        let bytes = msg
            .value
            .ok_or(Error::Message("Message missing value".to_string()))?;
        let alert_msg: AlertMessage = json::from_slice(&bytes).inspect_err(|_| {
            log::error!("[SUPER_CLUSTER:DB] Failed to parse alert message");
        })?;
        Ok(alert_msg)
    }
}

pub(crate) async fn process(msg: Message) -> Result<()> {
    match msg.message_type {
        MessageType::AlertsTable => {
            let msg = msg.try_into()?;
            process_msg(msg).await?;
        }
        _ => {
            // Try to process the message as an old event for the meta table for backward
            // compatability. This logic can be removed after all logic reading and writing alerts
            // to the meta table is removed from the application.
            super::meta::process(msg).await?;
        }
    }

    Ok(())
}

pub(crate) async fn process_msg(msg: AlertMessage) -> Result<()> {
    let conn = ORM_CLIENT.get_or_init(connect_to_orm).await;
    match msg {
        AlertMessage::Create {
            org_id,
            folder_id,
            alert,
        } => {
            let alert = table::alerts::create(conn, &org_id, &folder_id, alert).await?;
            infra::cluster_coordinator::alerts::emit_put_event(&org_id, &alert).await?;
        }
        AlertMessage::Update {
            org_id,
            folder_id,
            alert,
        } => {
            let alert = table::alerts::update(conn, &org_id, folder_id.as_deref(), alert).await?;
            infra::cluster_coordinator::alerts::emit_put_event(&org_id, &alert).await?;
        }
        AlertMessage::Delete { org_id, alert_id } => {
            if let Some((_, alert)) = table::alerts::get_by_id(conn, &org_id, alert_id).await? {
                table::alerts::delete_by_id(conn, &org_id, alert_id).await?;
                infra::cluster_coordinator::alerts::emit_delete_event(
                    &org_id,
                    alert.stream_type,
                    &alert.stream_name,
                    &alert.name,
                )
                .await?;
            }
        }
    };
    Ok(())
}