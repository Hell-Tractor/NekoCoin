<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const CARD_MIN_HEIGHT = 100;
const CARD_MIN_WIDTH = 200;

export interface Wallet {
    id: number;
    name: string;
    remark?: string;
    balance: number;
    currency: string;
    icon: string;
    color: string;
};

const wallets: Ref<Wallet[]> = ref([]);

const retrieveWallets = async function() {
    try {
        wallets.value = await invoke('retrieve_wallets');
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}
const deleteWallet = async function(id: number) {
    try {
        await invoke('delete_wallet', { id });
        wallets.value = wallets.value.filter(wallet => wallet.id !== id);
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
};

onMounted(() => {
    retrieveWallets();
});
</script>

<template>
    <v-row v-if="wallets.length > 0">
        <v-col v-for="wallet in wallets">
            <v-card :min-width="CARD_MIN_WIDTH" :min-height="CARD_MIN_HEIGHT" class="fill-height d-flex flex-column" :color="wallet.color">
                <v-card-title>
                    <v-icon style="margin-right: 10px;">{{ wallet.icon }}</v-icon>
                    <span>{{ wallet.name }}</span>
                    <span style="float: right;">{{ (wallet.balance / 100).toFixed(2) + ' ' + wallet.currency }}</span>
                </v-card-title>
                <v-card-text>{{ wallet.remark }}</v-card-text>
            </v-card>
        </v-col>
    </v-row>
    <v-row v-else>
        <v-col>
            <v-card class="fill-height d-flex flex-column">
                <v-card-title>{{ t('account.no_account') }}</v-card-title>
                <v-card-text>{{ t('account.no_account_tip') }}</v-card-text>
            </v-card>
        </v-col>
    </v-row>
</template>