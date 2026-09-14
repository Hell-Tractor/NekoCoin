<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useTheme } from 'vuetify';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import Constants from '../common/Constants';
import { rules } from '../common/Rules';
import { settings, save_settings } from '../common/Settings';
import { show_error } from '../common/Notify';
import { Currency } from '../common/Money';
import { get_random_theme_color } from '../themes/palettes';

const { t, locale } = useI18n();
const theme = useTheme();
const router = useRouter();
const step = ref<'profile' | 'wallet'>('profile');
const form_valid = ref(false);
const wallet_valid = ref(false);
const submitting = ref(false);
const wallet_name = ref('');
const wallet_amount = ref<number | null>(null);
const selected_currency = ref<Currency>(Constants.CURRENCIES[0]);

const language_items = [
    { value: 'zh-CN', title: '简体中文' },
    { value: 'en-US', title: 'English' },
];

const currencyItemProps = function(item: Currency) {
    return {
        title: item.symbol,
        subtitle: item.code,
    };
};

const update_locale = function(value: 'zh-CN' | 'en-US') {
    locale.value = value;
};

const go_wallet_step = function() {
    if (!form_valid.value) {
        return;
    }
    const currency = Constants.CURRENCIES.find(item => item.code === settings.primary_currency_code) ?? Constants.CURRENCIES[0];
    selected_currency.value = currency;
    if (!wallet_name.value) {
        wallet_name.value = t('onboarding.default_wallet_name');
    }
    step.value = 'wallet';
};

const finish = async function() {
    if (!form_valid.value || !wallet_valid.value || submitting.value) {
        return;
    }
    submitting.value = true;
    try {
        settings.initialized = true;
        await save_settings();
        locale.value = settings.locale;
        theme.global.name.value = settings.theme;
        await invoke('create_wallet', {
            name: wallet_name.value,
            remark: '',
            balance: Math.round((wallet_amount.value ?? 0) * 100),
            currencyCode: selected_currency.value.code,
            icon: 'mdi-wallet',
            color: get_random_theme_color(settings.theme),
        });
        await router.replace('/main/home');
    } catch (error) {
        show_error(error);
        settings.initialized = false;
        try {
            await save_settings();
        } catch (save_error) {
            show_error(save_error);
        }
    } finally {
        submitting.value = false;
    }
};
</script>

<template>
    <v-app-bar density="compact" color="primary" flat>
        <v-app-bar-title>{{ t('app_name') }}</v-app-bar-title>
    </v-app-bar>
    <v-main class="main">
        <v-card rounded="xl">
            <v-card-text>
                <template v-if="step === 'profile'">
                    <div class="onboarding-title">{{ t('onboarding.title') }}</div>
                    <div class="onboarding-subtitle">{{ t('onboarding.subtitle') }}</div>
                    <v-form v-model="form_valid" @submit.prevent="go_wallet_step">
                        <v-text-field
                            v-model="settings.user_name"
                            :label="t('settings.user_name')"
                            :rules="[rules.required, rules.maxLength(32)]"
                            variant="outlined"
                            density="comfortable"
                        />
                        <v-select
                            v-model="settings.locale"
                            :items="language_items"
                            item-title="title"
                            item-value="value"
                            :label="t('settings.language')"
                            variant="outlined"
                            density="comfortable"
                            @update:model-value="update_locale"
                        />
                        <v-select
                            v-model="settings.primary_currency_code"
                            :items="Constants.CURRENCIES"
                            item-title="code"
                            item-value="code"
                            :label="t('settings.primary_currency')"
                            variant="outlined"
                            density="comfortable"
                        />
                        <v-btn
                            type="submit"
                            color="primary"
                            block
                            rounded="xl"
                            size="large"
                            class="mt-2"
                            :disabled="!form_valid"
                        >
                            {{ t('onboarding.next') }}
                        </v-btn>
                    </v-form>
                </template>
                <template v-else>
                    <div class="onboarding-title">{{ t('onboarding.wallet_title') }}</div>
                    <div class="onboarding-subtitle">{{ t('onboarding.wallet_subtitle') }}</div>
                    <v-form v-model="wallet_valid" @submit.prevent="finish">
                        <v-text-field
                            v-model="wallet_name"
                            :label="t('account.enter.name')"
                            :rules="[rules.required, rules.maxLength(Constants.MAX_WALLET_NAME_LENGTH)]"
                            variant="outlined"
                            density="comfortable"
                        />
                        <div class="d-flex">
                            <v-text-field
                                v-model.number="wallet_amount"
                                :label="t('account.enter.amount')"
                                :rules="[rules.required, rules.isValidMoney]"
                                variant="outlined"
                                density="comfortable"
                            />
                            <v-select
                                max-width="80"
                                :items="Constants.CURRENCIES"
                                v-model="selected_currency"
                                :item-props="currencyItemProps"
                                return-object
                                density="comfortable"
                                variant="outlined"
                            />
                        </div>
                        <v-btn
                            type="submit"
                            color="primary"
                            block
                            rounded="xl"
                            size="large"
                            class="mt-2"
                            :loading="submitting"
                            :disabled="!wallet_valid"
                        >
                            {{ t('onboarding.start') }}
                        </v-btn>
                    </v-form>
                </template>
            </v-card-text>
        </v-card>
    </v-main>
</template>

<style scoped>
.onboarding-title {
    margin-bottom: 8px;
    color: rgb(var(--v-theme-primary));
    font-size: 1.25rem;
    font-weight: 700;
}

.onboarding-subtitle {
    margin-bottom: 20px;
    color: rgba(var(--v-theme-on-surface), 0.7);
    font-size: 0.95rem;
}
</style>
