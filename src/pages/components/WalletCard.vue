<script setup lang="ts">
import { Wallet } from '../Wallets.vue';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const CARD_MIN_HEIGHT = 150;

const props = defineProps<{
    wallet: Wallet,
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain"
}>();
</script>

<template>
    <v-card :variant="variant" class="mb-2" rounded="xl" :min-height="CARD_MIN_HEIGHT" :color="props.wallet.color">
        <v-card-title>
            <v-row class="d-flex">
                <v-col class="flex-grow-0 pr-0"><v-icon>{{ props.wallet.icon }}</v-icon></v-col>
                <v-col>{{ props.wallet.name }}</v-col>
            </v-row>
        </v-card-title>
        <v-card-text class="pb-0">
            <v-row><v-col>{{ props.wallet.remark }}</v-col></v-row>
            <div class="position-absolute bottom-0 mb-4">
                <div>{{ t('total_balance') }}</div>
                <v-row><v-col class="text-h5">{{ props.wallet.currency_code + ' ' + (props.wallet.balance / 100).toFixed(2) }}</v-col></v-row>
            </div>
        </v-card-text>
    </v-card>
</template>