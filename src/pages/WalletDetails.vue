<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import WalletCard from './components/WalletCard.vue';
import { Wallet } from './Wallets.vue';
import { invoke } from '@tauri-apps/api/core';
import BackTitleBar from './components/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import SummaryBar from './components/SummaryBar.vue';
import { Money } from '../common/Money';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    id: number,
}>();

const wallet: Ref<Wallet | undefined> = ref(undefined);
const sum_balance: Ref<{ income: number, expense: number }> = ref({ income: 0, expense: 0 });

const get_wallet = async function() {
    try {
        wallet.value = await invoke('get_wallet_by_id', { id: props.id });
    } catch (error) {
        console.log(error);
    }
}

const get_sum_balance = async function() {
    try {
        sum_balance.value = await invoke('get_sum_balance_in_wallet', { walletId: props.id });
    } catch (error) {
        console.log(error);
    }
}

onMounted(() => {
    get_wallet();
    get_sum_balance();
});
</script>

<template>
    <BackTitleBar :title="t('account.details')" @back="router.back()"/>
    <v-main v-if="wallet" class="main">
        <WalletCard variant="flat" :wallet="wallet" />
        <SummaryBar variant="flat" rounded="lg" :current-expense="new Money(sum_balance.expense, { symbol: wallet.currency, code: '' })" :current-income="new Money(sum_balance.income, { symbol: wallet.currency, code: '' })"></SummaryBar>
    </v-main>
</template>