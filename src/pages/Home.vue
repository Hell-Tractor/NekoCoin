<script setup lang="ts">
import { ref } from 'vue';
import { Money } from '../common/Money';
import { useI18n } from 'vue-i18n';
import SummaryBar from '../common/SummaryBar.vue';
import Constants from '../common/Constants';
const { t } = useI18n();

const totalBalance = ref(new Money(12345678, Constants.CURRENCIES[0]));
const currentMonthIncome = ref(new Money(12345678, Constants.CURRENCIES[0]));
const currentMonthExpense = ref(new Money(12345678, Constants.CURRENCIES[0]));
const today = function() {
    return `${new Date().getFullYear()}-${new Date().getMonth() + 1}-${new Date().getDate()}`;
}
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
            <p class="text-h5 font-weight-black">{{ totalBalance }}</p>
        </v-card-text>
    </v-card>
    <SummaryBar :title="t('this_month')" :current-income="currentMonthIncome as Money" :current-expense="currentMonthExpense as Money"></SummaryBar>
</template>