<!-- Copyright 2023 OpenObserve Inc.

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
  <div class="running-queries-page" v-if="isMetaOrg">
    <q-table
      data-test="running-queries-table"
      ref="qTable"
      :rows="rows"
      :columns="columns"
      :pagination="pagination as QTableProps['pagination']"
      row-key="trace_id"
      style="width: 100%"
      selection="multiple"
      v-model:selected="selectedRowsModel"
    >
      <template #no-data>
        <div v-if="!loadingState" class="text-center full-width full-height">
          <NoData />
        </div>
        <div v-else class="text-center full-width full-height q-mt-lg">
          <q-spinner-hourglass color="primary" size="lg" />
        </div>
      </template>
      <template #header-selection="scope">
        <q-checkbox v-model="scope.selected" size="xs" color="secondary" />
      </template>
      <template #body-selection="scope">
        <q-checkbox v-model="scope.selected" size="xs" color="secondary" />
      </template>
      <template #body-cell-actions="props">
        <q-td :props="props">
          <q-btn
            icon="list_alt"
            :title="t('queries.queryList')"
            class="q-ml-xs"
            padding="sm"
            unelevated
            size="sm"
            round
            flat
            @click="listSchema(props)"
            data-test="queryList-btn"
          />
          <q-btn
            :icon="outlinedCancel"
            :title="t('queries.cancelQuery')"
            class="q-ml-xs"
            padding="sm"
            unelevated
            size="sm"
            style="color: red"
            round
            flat
            @click="confirmDeleteAction(props)"
            data-test="cancelQuery-btn"
          />
        </q-td>
      </template>

      <template #bottom="scope">
        <q-btn
          data-test="qm-multiple-cancel-query-btn"
          class="text-bold"
          outline
          padding="sm lg"
          color="red"
          :disable="selectedRowsModel?.length === 0"
          @click="handleMultiQueryCancel"
          no-caps
          :label="t('queries.cancelQuery')"
        />
        <q-space />
        <div style="width: auto">
          <q-table-pagination
            data-test="query-stream-table-pagination"
            :scope="scope"
            :resultTotal="rows.length"
            :perPageOptions="perPageOptions"
            position="bottom"
            @update:changeRecordPerPage="changePagination"
            class="fit"
          />
        </div>
      </template>
    </q-table>
    <q-dialog
      v-model="showListSchemaDialog"
      position="right"
      full-height
      maximized
      data-test="list-schema-dialog"
    >
      <QueryList :schemaData="schemaData" />
    </q-dialog>
  </div>
</template>

<script lang="ts">
import useIsMetaOrg from "@/composables/useIsMetaOrg";
import { ref, type Ref, defineComponent, computed } from "vue";
import { type QTableProps, QTable } from "quasar";
import QTablePagination from "@/components/shared/grid/Pagination.vue";
import { useI18n } from "vue-i18n";
import { outlinedCancel } from "@quasar/extras/material-icons-outlined";
import NoData from "@/components/shared/grid/NoData.vue";
import { useStore } from "vuex";
import QueryList from "@/components/queries/QueryList.vue";

// TODO OK : Define types and interfaces for data properties.

export default defineComponent({
  name: "RunningQueriesList",
  components: { QueryList, QTablePagination, NoData },
  props: {
    rows: {
      type: Array,
      required: true,
    },
    selectedRows: {
      type: Array,
      required: false,
    },
  },
  emits: [
    "update:selectedRows",
    "delete:queries",
    "delete:query",
    "show:schema",
  ],
  setup(props, { emit }) {
    const store = useStore();
    const schemaData = ref({});
    const lastRefreshed = ref("");
    const { isMetaOrg } = useIsMetaOrg();
    const resultTotal = ref<number>(0);

    const loadingState = ref(false);

    const deleteDialog = ref({
      show: false,
      title: "Delete Running Query",
      message: "Are you sure you want to delete this running query?",
      data: null as any,
    });

    const qTable: Ref<InstanceType<typeof QTable> | null> = ref(null);
    const { t } = useI18n();
    const showListSchemaDialog = ref(false);

    const listSchema = (props: any) => {
      //pass whole props.row to schemaData
      emit("show:schema", props.row);
    };

    const perPageOptions: any = [
      { label: "5", value: 5 },
      { label: "10", value: 10 },
      { label: "20", value: 20 },
      { label: "50", value: 50 },
      { label: "100", value: 100 },
    ];
    const selectedPerPage = ref(20);
    const pagination: any = ref({
      rowsPerPage: 20,
    });
    const changePagination = (val: { label: string; value: any }) => {
      selectedPerPage.value = val.value;
      pagination.value.rowsPerPage = val.value;
      qTable.value?.setPagination(pagination.value);
    };

    const columns = ref<QTableProps["columns"]>([
      {
        name: "#",
        label: "#",
        field: "#",
        align: "left",
      },
      {
        name: "user_id",
        field: "user_id",
        label: t("user.email"),
        align: "left",
        sortable: true,
      },
      {
        name: "org_id",
        field: "org_id",
        label: t("organization.id"),
        align: "left",
        sortable: true,
      },
      {
        name: "search_type",
        field: "search_type",
        label: t("queries.searchType"),
        align: "left",
        sortable: true,
      },
      {
        name: "duration",
        label: t("queries.duration"),
        align: "left",
        sortable: true,
        field: "duration",
      },
      {
        name: "queryRange",
        label: t("queries.queryRange"),
        align: "left",
        sortable: true,
        field: "queryRange",
      },
      {
        name: "work_group",
        label: t("queries.queryType"),
        align: "left",
        sortable: true,
        field: "work_group",
      },
      {
        name: "status",
        field: "status",
        label: t("queries.status"),
        align: "left",
        sortable: true,
      },
      {
        name: "stream_type",
        field: "stream_type",
        label: t("alerts.streamType"),
        align: "left",
        sortable: true,
      },
      {
        name: "actions",
        field: "actions",
        label: t("common.actions"),
        align: "center",
      },
    ]);

    const selectedRowsModel = computed({
      get: () => props.selectedRows,
      set: (value) => {
        emit("update:selectedRows", value);
      },
    });

    const confirmDeleteAction = (props: any) => {
      emit("delete:query", props.row);
    };

    const handleMultiQueryCancel = () => {
      emit("delete:queries");
    };

    return {
      t,
      store,
      columns,
      confirmDeleteAction,
      deleteDialog,
      perPageOptions,
      listSchema,
      showListSchemaDialog,
      changePagination,
      outlinedCancel,
      schemaData,
      loadingState,
      lastRefreshed,
      isMetaOrg,
      resultTotal,
      selectedPerPage,
      qTable,
      selectedRowsModel,
      handleMultiQueryCancel,
      pagination,
    };
  },
});
</script>

<style lang="scss" scoped>
.query-management-tabs {
  :deep(.q-btn:before) {
    border: none !important;
  }
}

.label-container {
  display: flex;
  width: 100%;
  justify-content: flex-end;
}
</style>

<style lang="scss">
.running-queries-page {
  .search-input {
    width: 250px;
  }
}

.search-field-select {
  .q-field__control {
    padding-left: 12px;
    top: -1px;
    position: relative;
  }
}
</style>
