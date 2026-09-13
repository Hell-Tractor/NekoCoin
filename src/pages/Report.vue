<script setup lang="ts">
import { ref } from 'vue';
import { SummaryType, SummaryTypeList } from '../common/SummaryType';
import ColumnDiagram from './components/ColumnDiagram.vue';
import ExpenseTagRanking from './components/ExpenseTagRanking.vue';
import NetCashFlowDiagram from './components/NetCashFlowDiagram.vue';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const summary_type = ref<SummaryType>(SummaryType.Monthly);
</script>

<template>
    <ColumnDiagram class="mb-2" :summary-type="summary_type" variant="flat" rounded="xl">
        <template v-slot:title>
            <v-row class="d-flex align-center">
                <v-col>{{ t('report.overview') }}</v-col>
                <v-col class="flex-grow-0"><v-select width="100" variant="outlined" density="compact" :hide-details="true" :items="SummaryTypeList" :item-title="v => t(`report.summary_type.${v.toString()}`)" return-object v-model="summary_type"></v-select></v-col>
            </v-row>
        </template>
    </ColumnDiagram>
    <NetCashFlowDiagram class="mb-2" variant="flat" rounded="xl" />
    <ExpenseTagRanking />
</template>