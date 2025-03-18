<script setup lang="ts">
import { onMounted, Ref, ref } from 'vue';
import { Money } from '../common/Money';
import { useI18n } from 'vue-i18n';
import SummaryBar from '../common/SummaryBar.vue';
import Constants from '../common/Constants';
import { invoke } from '@tauri-apps/api/core';
const { t } = useI18n();

const totalBalance: Ref<Money | undefined> = ref(undefined);
const currentMonthIncome = ref(new Money(12345678, Constants.CURRENCIES[0]));
const currentMonthExpense = ref(new Money(12345678, Constants.CURRENCIES[0]));
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

onMounted(() => {
    getTotalBalance();
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
    <v-card>
        <v-card-text>
            <div>{{ t('total_balance') }}</div>
            <div style="height: 10px;"></div>
            <p class="text-h5 font-weight-black">{{ totalBalance ?? "loading..." }}</p>
        </v-card-text>
    </v-card>
    <SummaryBar :title="t('this_month')" :current-income="currentMonthIncome as Money" :current-expense="currentMonthExpense as Money"></SummaryBar>
</template>