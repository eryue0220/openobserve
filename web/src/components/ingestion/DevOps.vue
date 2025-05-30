<!-- Copyright 2025 OpenObserve Inc.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
-->

<template>
  <q-splitter
    v-model="splitterModel"
    unit="px"
    style="min-height: calc(100vh - 130px)"
  >
    <template v-slot:before>
      <q-input
        data-test="devops-list-search-input"
        v-model="tabsFilter"
        borderless
        filled
        dense
        class="no-border"
        :placeholder="t('common.search')"
      >
        <template #prepend>
          <q-icon name="search" class="cursor-pointer" />
        </template>
      </q-input>
      <q-tabs
        v-model="ingestTabType"
        indicator-color="transparent"
        inline-label
        vertical
        class="data-sources-database-tabs !tw-mt-3 item-left"
      >
        <template v-for="(tab, index) in filteredList" :key="tab.name">
          <q-route-tab
            :title="tab.name"
            :default="index === 0"
            :name="tab.name"
            :to="tab.to"
            :icon="tab.icon"
            :label="tab.label"
            content-class="tab_content"
          />
        </template>
      </q-tabs>
    </template>

    <template v-slot:after>
      <router-view
        :title="tabs"
        :currOrgIdentifier="currOrgIdentifier"
        :currUserEmail="currentUserEmail"
      >
      </router-view>
    </template>
  </q-splitter>
</template>

<script lang="ts">
// @ts-ignore
import { defineComponent, ref, onBeforeMount, onUpdated, computed } from "vue";
import { useI18n } from "vue-i18n";
import { useStore } from "vuex";
import { useRouter } from "vue-router";
import { copyToClipboard, useQuasar } from "quasar";
import config from "@/aws-exports";
import { getImageURL, verifyOrganizationStatus } from "@/utils/zincutils";

export default defineComponent({
  name: "DevOpsPage",
  props: {
    currOrgIdentifier: {
      type: String,
      default: "",
    },
  },
  setup() {
    const { t } = useI18n();
    const store = useStore();
    const q = useQuasar();
    const router: any = useRouter();
    const tabs = ref("");
    const currentOrgIdentifier: any = ref(
      store.state.selectedOrganization.identifier,
    );

    const tabsFilter = ref("");

    const ingestTabType = ref("jenkins");

    onBeforeMount(() => {
      if (router.currentRoute.value.name === "devops") {
        router.push({
          name: "jenkins",
          query: {
            org_identifier: store.state.selectedOrganization.identifier,
          },
        });
        return;
      }
    });

    onUpdated(() => {
      if (router.currentRoute.value.name === "devops") {
        router.push({
          name: "jenkins",
          query: {
            org_identifier: store.state.selectedOrganization.identifier,
          },
        });
        return;
      }
    });

    const devopsTabs = [
      {
        name: "jenkins",
        to: {
          name: "jenkins",
          query: {
            org_identifier: store.state.selectedOrganization.identifier,
          },
        },
        icon: "img:" + getImageURL("images/ingestion/jenkins.svg"),
        label: t("ingestion.jenkins"),
        contentClass: "tab_content",
      },
      {
        name: "ansible",
        to: {
          name: "ansible",
          query: {
            org_identifier: store.state.selectedOrganization.identifier,
          },
        },
        icon: "img:" + getImageURL("images/ingestion/ansible.svg"),
        label: t("ingestion.ansible"),
        contentClass: "tab_content",
      },
      {
        name: "terraform",
        to: {
          name: "terraform",
          query: {
            org_identifier: store.state.selectedOrganization.identifier,
          },
        },
        icon: "img:" + getImageURL("images/ingestion/terraform.svg"),
        label: t("ingestion.terraform"),
        contentClass: "tab_content",
      },
      {
        name: "github-actions",
        to: {
          name: "github-actions",
          query: {
            org_identifier: store.state.selectedOrganization.identifier,
          },
        },
        icon: "img:" + getImageURL("images/ingestion/github-actions.svg"),
        label: t("ingestion.gactions"),
        contentClass: "tab_content",
      }
    ];

    // create computed property to filter tabs
    const filteredList = computed(() => {
      return devopsTabs.filter((tab) => {
        return tab.label.toLowerCase().includes(tabsFilter.value.toLowerCase());
      });
    });

    return {
      t,
      store,
      router,
      config,
      splitterModel: ref(270),
      currentUserEmail: store.state.userInfo.email,
      currentOrgIdentifier,
      getImageURL,
      verifyOrganizationStatus,
      tabs,
      ingestTabType,
      tabsFilter,
      filteredList,
    };
  },
});
</script>

<style scoped lang="scss">
.data-sources-database-tabs {
  :deep(.q-tab) {
    min-height: 36px;
  }
}
.ingestionPage {
  padding: 1.5rem 0 0;
  .head {
    padding-bottom: 1rem;
  }
  .q-tabs {
    &--vertical {
      margin: 1.5rem 1rem 0 1rem;
      .q-tab {
        justify-content: flex-start;
        padding: 0 0.6rem 0 0.6rem;
        border-radius: 0.5rem;
        margin-bottom: 0.5rem;
        text-transform: capitalize;

        &__content.tab_content {
          .q-tab {
            &__icon + &__label {
              padding-left: 0.875rem;
              font-weight: 600;
            }
          }
        }
        &--active {
          color: black;
          background-color: $accent;
        }
      }
    }
  }
}
</style>
