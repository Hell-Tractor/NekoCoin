<script setup lang="ts">
import { onMounted, Ref, ref } from 'vue';
import { Money } from '../common/Money';
import { useI18n } from 'vue-i18n';
import SummaryBar from './components/SummaryBar.vue';
import Constants from '../common/Constants';
import { invoke } from '@tauri-apps/api/core';
import { formatDate } from '../common/Utils';
import TransactionList from './components/TransactionList.vue';
const { t } = useI18n();

const totalBalance: Ref<Money | undefined> = ref(undefined);
const currentMonthIncome: Ref<Money | undefined> = ref(undefined);
const currentMonthExpense: Ref<Money | undefined> = ref(undefined);
const today = function() {
    return `${new Date().getFullYear()}-${new Date().getMonth() + 1}-${new Date().getDate()}`;
}
const getTotalBalance = async function() {
    try {
        const result: number = await invoke('get_sum_balance', { currency: Constants.CURRENCIES[0].symbol });
        totalBalance.value = new Money(result, Constants.CURRENCIES[0]);
    } catch (e) {
        console.error(e);
    }
}
const getMonthBalance = async function() {
    try {
        const beginDate = new Date(new Date().getFullYear(), new Date().getMonth(), 1);
        const endDate = new Date(new Date().getFullYear(), new Date().getMonth(), new Date().getDate());
        const result: { income: number, expense: number } = await invoke('get_sum_balance_with_type', { currency: Constants.CURRENCIES[0].symbol, begin: formatDate(beginDate), end: formatDate(endDate) });
        currentMonthIncome.value = new Money(result.income, Constants.CURRENCIES[0]);
        currentMonthExpense.value = new Money(result.expense, Constants.CURRENCIES[0]);
    } catch (e) {
        console.error(e);
    }
}
const refresh = async function() {
    try {
        await getTotalBalance();
        await getMonthBalance();
    } catch (e) {
        console.error(e);
    }
}

onMounted(() => {
    getTotalBalance();
    getMonthBalance();
});
</script>

<template>
    <v-row>
        <v-col class="flex-grow-0">
            <v-avatar icon="mdi-cat" size="large" />
        </v-col>
        <v-col>
            <div>{{ today() }}</div>
            <div>{{ t('welcome') }}</div>
        </v-col>
    </v-row>
    <v-card variant="flat">
        <v-card-text>
            <div>{{ t('total_balance') }}</div>
            <div style="height: 10px;"></div>
            <p class="text-h5 font-weight-black">{{ totalBalance ?? "loading..." }}</p>
        </v-card-text>
    </v-card>
    <SummaryBar v-if="!!currentMonthExpense && !! currentMonthIncome" :title="t('this_month')" :current-income="currentMonthIncome as Money" :current-expense="currentMonthExpense as Money"></SummaryBar>
    <TransactionList variant="flat" :title="t('transaction.list.title')" @deleted="_ => refresh()"></TransactionList>
</template>