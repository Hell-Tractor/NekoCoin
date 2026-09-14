<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useTheme } from 'vuetify';
import { useRouter } from 'vue-router';
import Constants from '../common/Constants';
import { rules } from '../common/Rules';
import { settings, save_settings } from '../common/Settings';

const { t, locale } = useI18n();
const theme = useTheme();
const router = useRouter();
const form_valid = ref(false);
const submitting = ref(false);

const language_items = [
    { value: 'zh-CN', title: '简体中文' },
    { value: 'en-US', title: 'English' },
];

const update_locale = function(value: 'zh-CN' | 'en-US') {
    locale.value = value;
};

const finish = async function() {
    if (!form_valid.value || submitting.value) {
        return;
    }
    submitting.value = true;
    try {
        settings.initialized = true;
        await save_settings();
        locale.value = settings.locale;
        theme.global.name.value = settings.theme;
        await router.replace('/main/home');
    } catch (error) {
        console.error(error);
        settings.initialized = false;
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
                <div class="onboarding-title">{{ t('onboarding.title') }}</div>
                <div class="onboarding-subtitle">{{ t('onboarding.subtitle') }}</div>
                <v-form v-model="form_valid" @submit.prevent="finish">
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
                        :loading="submitting"
                        :disabled="!form_valid"
                    >
                        {{ t('onboarding.start') }}
                    </v-btn>
                </v-form>
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
