<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Money } from '../../common/Money';
const { t } = useI18n();

defineProps<{
    currentIncome: Money;
    currentExpense: Money;
    title?: string;
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain"
    rounded?: string | boolean;
}>();
</script>

<template>
    <v-card :variant="variant" :rounded="rounded">
        <v-card-text>
            <div style="margin-bottom: 5px;" v-if="!!title">{{ title }}</div>
            <v-row class="flex-nowrap">
                <v-col class="flex-grow-1">
                    <p>{{ t('income') }}<v-icon color="green">mdi-chart-line-variant</v-icon></p>
                    <p class="text-h8 font-weight-black">{{ currentIncome }}</p>
                </v-col>
                <v-col class="flex-grow-1">
                    <p>{{ t('expense') }}<v-icon color="red" class="v-flipped">mdi-chart-line-variant</v-icon></p>
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
</style>