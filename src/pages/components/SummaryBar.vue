<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Money } from '../../common/Money';
const { t } = useI18n();

const props = defineProps<{
    currentIncome: Money;
    currentExpense: Money;
    currentNetCashFlow?: Money;
    title?: string;
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain"
    rounded?: string | boolean;
}>();

const net_cash_flow_icon = function() {
    const amount = props.currentNetCashFlow?.getAmount() ?? 0;
    return amount > 0 ? 'mdi-trending-up' : amount < 0 ? 'mdi-trending-down' : 'mdi-minus';
};

const net_cash_flow_color = function() {
    const amount = props.currentNetCashFlow?.getAmount() ?? 0;
    return amount > 0 ? 'success' : amount < 0 ? 'error' : 'grey';
};

const net_cash_flow_hint = function() {
    const amount = props.currentNetCashFlow?.getAmount() ?? 0;
    return amount > 0 ? t('net_cash_flow_status.increase') : amount < 0 ? t('net_cash_flow_status.decrease') : t('net_cash_flow_status.unchanged');
};
</script>

<template>
    <v-card :variant="variant" :rounded="rounded ?? 'xl'" class="mb-2">
        <v-card-text>
            <div v-if="!!title" class="summary-title">
                <span>{{ title }}</span>
                <v-divider v-if="currentNetCashFlow" vertical class="summary-divider" />
                <span v-if="currentNetCashFlow" class="net-cash-flow">
                    <v-tooltip :text="net_cash_flow_hint()" location="top">
                        <template #activator="{ props: tooltip_props }">
                            <v-icon v-bind="tooltip_props" :color="net_cash_flow_color()" :icon="net_cash_flow_icon()" size="small" />
                        </template>
                    </v-tooltip>
                    <span class="text-body-2">{{ currentNetCashFlow }}</span>
                </span>
            </div>
            <v-row class="flex-nowrap">
                <v-col class="flex-grow-1">
                    <p>{{ t('income') }}<v-icon color="success">mdi-chart-line-variant</v-icon></p>
                    <p class="text-h8 font-weight-black">{{ currentIncome }}</p>
                </v-col>
                <v-col class="flex-grow-1">
                    <p>{{ t('expense') }}<v-icon color="error" class="v-flipped">mdi-chart-line-variant</v-icon></p>
                    <p class="text-h8 font-weight-black">{{ currentExpense }}</p>
                </v-col>
            </v-row>
        </v-card-text>
    </v-card>
</template>

<style scoped>
.v-flipped {
    transform: scaleY(-1);
}

.summary-title {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 5px;
}

.summary-divider {
    height: 18px;
    opacity: 0.45;
}

.net-cash-flow {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 0.85rem;
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
}
</style>