<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { show_error } from '../common/Notify';
import { useI18n } from 'vue-i18n';
import WalletCard from './components/WalletCard.vue';
import EmptyState from './components/EmptyState.vue';
import { useRouter } from 'vue-router';
const { t } = useI18n();
const router = useRouter();

export interface Wallet {
    id: number;
    name: string;
    remark?: string;
    balance: number;
    currency_code: string;
    icon: string;
    color: string;
};

const wallets: Ref<Wallet[]> = ref([]);

const retrieveWallets = async function() {
    try {
        wallets.value = await invoke('retrieve_wallets');
    } catch (error) {
        show_error(error);
    }
}

onMounted(() => {
    retrieveWallets();
});
</script>

<template>
    <WalletCard v-if="wallets.length > 0" v-for="wallet in wallets" :key="wallet.id" :wallet="wallet" @click="router.push({ path: `/wallet/${wallet.id}`})" />
    <EmptyState v-else :title="t('account.no_account')" :tip="t('account.no_account_tip')" />
</template>
