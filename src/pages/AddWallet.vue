<script setup lang="ts">
import BackTitleBar from './components/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import { onMounted, Ref, ref } from 'vue';
import { get_random_theme_color } from '../themes/palettes';
import { load_settings, settings } from '../common/Settings';
import Constants from '../common/Constants';
import { invoke } from '@tauri-apps/api/core';
import { show_error } from '../common/Notify';
import { Currency } from '../common/Money';
import IconSelector from './components/IconSelector.vue';
import { Wallet } from './Wallets.vue';
import { useRouter } from 'vue-router';
import ColorPalette from './components/ColorPalette.vue';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    init?: Wallet
}>();

const id: Ref<number | undefined> = ref(undefined);
const icon: Ref<string> = ref('mdi-credit-card');
const selected_color: Ref<string> = ref(get_random_theme_color(settings.theme));
const selected_currency: Ref<Currency> = ref(Constants.CURRENCIES[0]);
const form: Ref<boolean> = ref(false);
const wallet_name: Ref<string> = ref('');
const wallet_remark: Ref<string> = ref('');
const wallet_amount: Ref<number | null> = ref(null);
const page: Ref<string> = ref('main');

const addWallet = async function() {
    try {
        if (id.value) {
            await invoke('update_wallet', { vo: {
                id: id.value,
                name: wallet_name.value,
                remark: wallet_remark.value,
                balance: Math.round(wallet_amount.value! * 100),
                icon: icon.value,
                color: selected_color.value
            }})
        } else {
            await invoke('create_wallet', {
                name: wallet_name.value,
                remark: wallet_remark.value,
                balance: Math.round(wallet_amount.value! * 100),
                currencyCode: selected_currency.value.code,
                icon: icon.value,
                color: selected_color.value
            });
        }
        router.back();
    } catch (error) {
        show_error(error);
    }
}

const currencyItemProps = function(item: Currency) {
    return {
        'title': item.symbol,
        'subtitle': item.code
    };
}

onMounted(async () => {
    await load_settings();
    if (props.init) {
        id.value = props.init.id;
        wallet_name.value = props.init.name;
        wallet_remark.value = props.init.remark || '';
        selected_currency.value = Constants.CURRENCIES.find(c => c.code == props.init!.currency_code)!;
        icon.value = props.init.icon;
        selected_color.value = props.init.color;
        wallet_amount.value = props.init.balance / 100;
    } else {
        selected_color.value = get_random_theme_color(settings.theme);
    }
})
</script>

<template>
    <div v-if="page == 'main'">
        <BackTitleBar :title="t(id == undefined ? 'account.add' : 'account.update')" @back="router.back()"></BackTitleBar>
        <v-main class="main">
            <v-form class="fill-height form-page" v-model="form">
                <v-text-field v-model="wallet_name" :placeholder="t('account.enter.name')" variant="outlined" density="comfortable" :rules="[rules.required, rules.maxLength(Constants.MAX_WALLET_NAME_LENGTH)]"></v-text-field>
                <v-text-field v-model="wallet_remark" :placeholder="t('account.enter.remark')" variant="outlined" density="comfortable" :rules="[rules.maxLength(Constants.MAX_WALLET_REMARK_LENGTH)]"></v-text-field>
                <div class="d-flex">
                    <v-text-field v-model.number="wallet_amount" :placeholder="t('account.enter.amount')" variant="outlined" density="comfortable" :rules="[rules.required, rules.isValidMoney]"></v-text-field>
                    <v-select :disabled="id != undefined" max-width="80" :items="Constants.CURRENCIES" v-model="selected_currency" :item-props="currencyItemProps" return-object density="comfortable" variant="outlined"></v-select>
                </div>
                <v-btn :prepend-icon="icon" variant="text" @click="page = 'icon_selector'" block size="large" class="justify-start">{{ t('icon.select') }}</v-btn>
                <ColorPalette v-model="selected_color" />
                <v-color-picker elevation="0" width="100%" v-model="selected_color" mode="rgb" class="mt-2"></v-color-picker>
                <v-btn @click="addWallet" color="primary" class="form-save-btn" :disabled="!form">{{ t('actions.save') }}</v-btn>
            </v-form>
        </v-main>
    </div>
    <IconSelector v-else-if="page == 'icon_selector'" @confirm="(selected_icon: string) => icon = selected_icon" @back="page = 'main'"></IconSelector>
</template>