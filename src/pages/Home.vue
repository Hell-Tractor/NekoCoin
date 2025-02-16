<script setup lang="ts">
import { ref } from 'vue';
import { Money } from '../common/Money';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const totalBalance = ref(new Money(1234567.89, '￥'));
const currentMonthIncome = ref(new Money(1234567.89, '￥'));
const currentMonthExpense = ref(new Money(1234567.89, '￥'));
const today = function() {
    return `${new Date().getFullYear()}-${new Date().getMonth() + 1}-${new Date().getDate()}`;
}
</script>

<template>
    <div>{{ today() }}</div>
    <div>{{ t('welcome') }}</div>
    <v-card>
        <v-card-text>
            <div>{{ t('total_balance') }}</div>
            <div style="height: 10px;"></div>
            <p class="text-h5 font-weight-black">{{ totalBalance }}</p>
        </v-card-text>
    </v-card>
    <v-card variant="text">
        <v-card-text>
            <div>{{ t('this_month') }}</div>
            <v-row class="flex-nowrap">
                <v-col class="flex-grow-1">
                    <p>{{ t('income') }}<v-icon color="green">mdi-chart-line-variant</v-icon></p>
                    <p class="text-h8 font-weight-black">{{ currentMonthIncome }}</p>
                </v-col>
                <v-col class="flex-grow-1">
                    <p>{{ t('expense') }}<v-icon color="red" class="v-flipped">mdi-chart-line-variant</v-icon></p>
                    <p class="text-h8 font-weight-black">{{ currentMonthExpense }}</p>
                </v-col>
            </v-row>
        </v-card-text>
    </v-card>
</template>

<style scoped>
.v-flipped {
    transform: scaleY(-1);
}
</style>