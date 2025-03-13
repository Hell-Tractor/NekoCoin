<script setup lang="ts">
import BackTitleBar from '../common/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import { Ref, ref } from 'vue';
import { getRandomColor } from '../common/Utils';
import Constants from '../common/Constants';
import { invoke } from '@tauri-apps/api/core';
import { Currency } from '../common/Money';
const { t } = useI18n();

const emits = defineEmits<{
    back: []
}>();

const icon: Ref<string> = ref('mdi-credit-card');
const showMdiSelector: Ref<boolean> = ref(false);
const selected_color: Ref<string> = ref(getRandomColor('rgb'));
const selected_currency: Ref<Currency> = ref(Constants.CURRENCIES[0]);
const form: Ref<boolean> = ref(false);
const wallet_name: Ref<string> = ref('');
const wallet_remark: Ref<string> = ref('');
const wallet_amount: Ref<number | null> = ref(null);

const addWallet = async function() {
    try {
        console.log('balance:', wallet_amount.value);
        await invoke('create_wallet', {
            name: wallet_name.value,
            remark: wallet_remark.value,
            balance: Math.round(wallet_amount.value! * 100),
            currency: selected_currency.value.symbol,
            icon: icon.value,
            color: selected_color.value
        });
        emits('back');
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}

const currencyItemProps = function(item: Currency) {
    return {
        'title': item.symbol,
        'subtitle': item.code
    };
}
</script>

<template>
    <BackTitleBar :title="t('account.add')" @back="emits('back')"></BackTitleBar>
    <v-form class="fill-height" v-model="form">
        <v-text-field v-model="wallet_name" :placeholder="t('account.enter.name')" variant="outlined" density="comfortable" :rules="[rules.required, rules.maxLength(Constants.MAX_WALLET_NAME_LENGTH)]"></v-text-field>
        <v-text-field v-model="wallet_remark" :placeholder="t('account.enter.remark')" variant="outlined" density="comfortable" :rules="[rules.maxLength(Constants.MAX_WALLET_REMARK_LENGTH)]"></v-text-field>
        <div class="d-flex">
            <v-text-field v-model.number="wallet_amount" :placeholder="t('account.enter.amount')" variant="outlined" density="comfortable" :rules="[rules.required, rules.isValidMoney]"></v-text-field>
            <v-select max-width="80" :items="Constants.CURRENCIES" v-model="selected_currency" :item-props="currencyItemProps" return-object density="comfortable" variant="outlined"></v-select>
        </div>
        <!-- TODO create icon select page -->
        <v-btn :prepend-icon="icon" variant="text" @click="showMdiSelector=true" width="100%" class="justify-start">{{ t('select_icon') }}</v-btn>
        <v-color-picker elevation="0" width="100%" v-model="selected_color" mode="rgb" style="margin-top: 10px; margin-bottom: 60px;"></v-color-picker>
        <v-btn @click="addWallet" color="primary" width="93%" style="position: fixed; bottom: 10px;" :disabled="!form">{{ t('save') }}</v-btn>
    </v-form>
</template>