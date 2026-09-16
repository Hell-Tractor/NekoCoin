<script setup lang="ts">
import { onMounted, Ref, ref } from 'vue';
import { Money } from '../common/Money';
import { useI18n } from 'vue-i18n';
import AmountText from './components/AmountText.vue';
import SummaryBar from './components/SummaryBar.vue';
import Constants from '../common/Constants';
import { invoke } from '@tauri-apps/api/core';
import { show_error } from '../common/Notify';
import { formatDate, formatDisplayDate } from '../common/Utils';
import TransactionList from './components/TransactionList.vue';
import ActiveActivityList from './components/ActiveActivityList.vue';
import { load_settings, settings } from '../common/Settings';
const { t } = useI18n();

const totalBalance: Ref<Money | undefined> = ref(undefined);
const currentMonthIncome: Ref<Money | undefined> = ref(undefined);
const currentMonthExpense: Ref<Money | undefined> = ref(undefined);
const currentMonthNetCashFlow: Ref<Money | undefined> = ref(undefined);
const today = function() {
    return formatDisplayDate(new Date());
}
const getTotalBalance = async function() {
    try {
        const currency = Constants.CURRENCIES.find(item => item.code === settings.primary_currency_code) ?? Constants.CURRENCIES[0];
        const result: number = await invoke('get_sum_balance', { currencyCode: currency.code });
        totalBalance.value = new Money(result, currency);
    } catch (e) {
        show_error(e);
    }
}
const getMonthBalance = async function() {
    try {
        const beginDate = new Date(new Date().getFullYear(), new Date().getMonth(), 1);
        const endDate = new Date(new Date().getFullYear(), new Date().getMonth(), new Date().getDate());
        const currency = Constants.CURRENCIES.find(item => item.code === settings.primary_currency_code) ?? Constants.CURRENCIES[0];
        const result: { income: number, expense: number } = await invoke('get_sum_balance_with_type', { currencyCode: currency.code, begin: formatDate(beginDate), end: formatDate(endDate) });
        currentMonthIncome.value = new Money(result.income, currency);
        currentMonthExpense.value = new Money(result.expense, currency);
        currentMonthNetCashFlow.value = new Money(result.income - result.expense, currency);
    } catch (e) {
        show_error(e);
    }
}
const refresh = async function() {
    try {
        await getTotalBalance();
        await getMonthBalance();
    } catch (e) {
        show_error(e);
    }
}

onMounted(() => {
    load_settings().then(() => {
        getTotalBalance();
        getMonthBalance();
    });
});
</script>

<template>
    <v-card rounded="xl" class="mb-2">
        <v-card-text>
            <div class="home-hero-top">
                <v-avatar :icon="settings.avatar" size="40" />
                <div class="entity-copy">
                    <div class="entity-meta">{{ today() }}</div>
                    <div class="entity-name">{{ settings.user_name ? t('welcome_user', { username: settings.user_name }) : t('welcome') }}</div>
                </div>
            </div>
            <div class="entity-meta">{{ t('total_balance') }}</div>
            <div class="entity-hero-amount">
                <AmountText v-if="totalBalance" :money="totalBalance" />
                <span v-else>{{ t('loading') }}</span>
            </div>
        </v-card-text>
    </v-card>
    <SummaryBar
        v-if="!!currentMonthExpense && !!currentMonthIncome && !!currentMonthNetCashFlow"
        class="mb-2"
        variant="flat"
        rounded="xl"
        :title="t('this_month')"
        :current-income="currentMonthIncome as Money"
        :current-expense="currentMonthExpense as Money"
        :current-net-cash-flow="currentMonthNetCashFlow as Money"
    />
    <ActiveActivityList />
    <TransactionList :title="t('transaction.list.title')" @deleted="_ => refresh()"></TransactionList>
</template>
