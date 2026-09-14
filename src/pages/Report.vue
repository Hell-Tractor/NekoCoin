<script setup lang="ts">
import { ref } from 'vue';
import { SummaryType, SummaryTypeList } from '../common/SummaryType';
import ColumnDiagram from './components/ColumnDiagram.vue';
import ExpenseTagRanking from './components/ExpenseTagRanking.vue';
import ExpenseActivityRanking from './components/ExpenseActivityRanking.vue';
import NetCashFlowDiagram from './components/NetCashFlowDiagram.vue';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const summary_type = ref<SummaryType>(SummaryType.Monthly);
</script>

<template>
    <ColumnDiagram class="mb-2" :summary-type="summary_type" variant="flat" rounded="xl">
        <template v-slot:title>
            <span>{{ t('report.overview') }}</span>
            <v-select class="grouping-select" variant="outlined" density="compact" :hide-details="true" :items="SummaryTypeList" :item-title="v => t(`report.summary_type.${v.toString()}`)" return-object v-model="summary_type"></v-select>
        </template>
    </ColumnDiagram>
    <NetCashFlowDiagram class="mb-2" variant="flat" rounded="xl" />
    <ExpenseTagRanking class="mb-2" />
    <ExpenseActivityRanking class="mb-2" mode="instance" />
    <ExpenseActivityRanking class="mb-2" mode="class" />
</template>

<style scoped>
.grouping-select {
    flex: 0 0 auto;
    width: fit-content;
    min-width: 92px;
    /* max-width: 112px; */
}
</style>