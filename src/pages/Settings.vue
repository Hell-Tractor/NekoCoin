<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useTheme } from 'vuetify';
import Constants from '../common/Constants';
import { clear_logs, get_log_usage, reset_app, settings, save_settings } from '../common/Settings';
import { format_bytes } from '../common/Utils';
import { get_theme_color_palette } from '../themes/palettes';
import BackTitleBar from './components/BackTitleBar.vue';
import ConfirmSheet from './components/ConfirmSheet.vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const { t, locale } = useI18n();
const theme = useTheme();
const theme_items = computed(() => ['pinkPad', 'midnight', 'calico', 'neon'].map(value => ({
    value,
    title: t(`settings.themes.${value}`),
    palette: get_theme_color_palette(value).slice(0, 6),
})));
const avatar_items = [
    'mdi-cat',
    'mdi-dog',
    'mdi-owl',
    'mdi-panda',
    'mdi-rabbit',
    'mdi-robot',
];
const default_page_items = computed(() => [
    { value: 'home', title: t('page.home') },
    { value: 'transactions', title: t('page.transactions') },
    { value: 'reports', title: t('page.reports') },
    { value: 'accounts', title: t('page.accounts') },
]);
const date_format_items = computed(() => [
    { value: 'YYYY-MM-DD', title: '2026-09-14' },
    { value: 'DD/MM/YYYY', title: '14/09/2026' },
    { value: 'MM/DD/YYYY', title: '09/14/2026' },
]);
const time_format_items = computed(() => [
    { value: '24hr', title: '14:30' },
    { value: '12hr', title: '2:30 PM' },
]);
const language_items = [
    { value: 'zh-CN', title: '简体中文' },
    { value: 'en-US', title: 'English' },
];
const log_retention_items = computed(() => [
    { value: 0, title: t('settings.logs.retention_forever') },
    { value: 7, title: t('settings.logs.retention_days', { days: 7 }) },
    { value: 30, title: t('settings.logs.retention_days', { days: 30 }) },
    { value: 90, title: t('settings.logs.retention_days', { days: 90 }) },
    { value: 180, title: t('settings.logs.retention_days', { days: 180 }) },
]);
const log_usage_bytes = ref(0);
const log_usage_display = computed(() => format_bytes(log_usage_bytes.value));

const update_theme = function(value: string) {
    theme.global.name.value = value;
};

let save_timer: number | undefined;
const save = async function() {
    try {
        await save_settings();
    } catch (error) {
        console.error(error);
    }
};

const queue_save = function() {
    if (save_timer !== undefined) {
        window.clearTimeout(save_timer);
    }
    save_timer = window.setTimeout(() => {
        save();
    }, 500);
};

const update_locale = function(value: 'zh-CN' | 'en-US') {
    locale.value = value;
};

const refresh_log_usage = async function() {
    try {
        log_usage_bytes.value = await get_log_usage();
    } catch (error) {
        console.error(error);
    }
};

const show_clear_logs_confirm = ref(false);
const clearing_logs = ref(false);

const confirm_clear_logs = async function() {
    if (clearing_logs.value) {
        return;
    }
    clearing_logs.value = true;
    try {
        await clear_logs();
        await refresh_log_usage();
    } catch (error) {
        console.error(error);
    } finally {
        clearing_logs.value = false;
    }
};

const show_reset_confirm = ref(false);
const resetting = ref(false);

const confirm_reset = async function() {
    if (resetting.value) {
        return;
    }
    resetting.value = true;
    try {
        await reset_app();
    } catch (error) {
        console.error(error);
        resetting.value = false;
    }
};

watch(settings, queue_save, { deep: true });

onMounted(refresh_log_usage);
</script>

<template>
    <back-title-bar :title="t('settings.title')" @back="router.back()"/>
    <v-main class="main">
        <v-card rounded="xl">
            <!-- <v-card-title>{{ t('settings.title') }}</v-card-title> -->
            <v-card-text>
                <div class="settings-section-title">{{ t('settings.user_section') }}</div>
                <v-text-field
                    v-model="settings.user_name"
                    :label="t('settings.user_name')"
                    variant="outlined"
                    density="comfortable"
                />
                <v-select
                    v-model="settings.avatar"
                    :items="avatar_items"
                    :label="t('settings.avatar')"
                    variant="outlined"
                    density="comfortable"
                >
                    <template #selection="{ item }">
                        <v-icon class="mr-2">{{ item.raw }}</v-icon>{{ item.raw }}
                    </template>
                    <template #item="{ props: item_props, item }">
                        <v-list-item v-bind="item_props">
                            <template #prepend><v-icon>{{ item.raw }}</v-icon></template>
                        </v-list-item>
                    </template>
                </v-select>
                <v-divider class="section-divider" />
                <div class="settings-section-title">{{ t('settings.application_section') }}</div>
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
                <v-select
                    v-model="settings.default_page"
                    :items="default_page_items"
                    item-title="title"
                    item-value="value"
                    :label="t('settings.default_page')"
                    variant="outlined"
                    density="comfortable"
                />
                <v-select
                    v-model="settings.date_format"
                    :items="date_format_items"
                    item-title="title"
                    item-value="value"
                    :label="t('settings.date_format')"
                    variant="outlined"
                    density="comfortable"
                />
                <v-select
                    v-model="settings.time_format"
                    :items="time_format_items"
                    item-title="title"
                    item-value="value"
                    :label="t('settings.time_format')"
                    variant="outlined"
                    density="comfortable"
                />
                <v-select
                    v-model="settings.decimal_places"
                    :items="[0, 1, 2, 3, 4]"
                    :label="t('settings.decimal_places')"
                    variant="outlined"
                    density="comfortable"
                />
                <v-switch
                    v-model="settings.thousands_separator"
                    :label="t('settings.thousands_separator')"
                    color="primary"
                    hide-details
                    class="mb-4"
                />
                <v-select
                    v-model="settings.theme"
                    :items="theme_items"
                    item-title="title"
                    item-value="value"
                    :label="t('settings.theme')"
                    variant="outlined"
                    density="comfortable"
                    @update:model-value="update_theme"
                >
                    <template #selection="{ item }">
                        <span class="theme-name">{{ item.raw.title }}</span>
                        <span class="theme-palette">
                            <span v-for="color in item.raw.palette" :key="color" class="theme-swatch" :style="{ backgroundColor: color }"></span>
                        </span>
                    </template>
                    <template #item="{ props: item_props, item }">
                        <v-list-item v-bind="item_props">
                            <template #append>
                                <span class="theme-palette">
                                    <span v-for="color in item.raw.palette" :key="color" class="theme-swatch" :style="{ backgroundColor: color }"></span>
                                </span>
                            </template>
                        </v-list-item>
                    </template>
                </v-select>
            </v-card-text>
        </v-card>
        <v-card class="mt-4" rounded="xl">
            <v-card-text>
                <div class="settings-section-title">{{ t('settings.logs.section') }}</div>
                <v-text-field
                    :model-value="log_usage_display"
                    :label="t('settings.logs.usage')"
                    variant="outlined"
                    density="comfortable"
                    readonly
                />
                <v-select
                    v-model="settings.log_retention_days"
                    :items="log_retention_items"
                    item-title="title"
                    item-value="value"
                    :label="t('settings.logs.retention')"
                    variant="outlined"
                    density="comfortable"
                />
                <v-btn
                    variant="outlined"
                    rounded="xl"
                    :loading="clearing_logs"
                    @click="show_clear_logs_confirm = true"
                >
                    {{ t('settings.logs.clear') }}
                </v-btn>
            </v-card-text>
        </v-card>
        <v-card class="mt-4 danger-zone" rounded="xl" variant="outlined">
            <v-card-text>
                <div class="danger-zone-title">{{ t('settings.danger_zone.title') }}</div>
                <div class="danger-zone-description">{{ t('settings.danger_zone.description') }}</div>
                <v-btn
                    color="error"
                    variant="outlined"
                    rounded="xl"
                    :loading="resetting"
                    @click="show_reset_confirm = true"
                >
                    {{ t('settings.danger_zone.reset_app') }}
                </v-btn>
            </v-card-text>
        </v-card>
        <confirm-sheet
            v-model="show_clear_logs_confirm"
            :title="t('settings.logs.confirm_title')"
            :text="t('settings.logs.confirm_text')"
            @confirm="confirm_clear_logs"
        />
        <confirm-sheet
            v-model="show_reset_confirm"
            :title="t('settings.danger_zone.confirm_title')"
            :text="t('settings.danger_zone.confirm_text')"
            @confirm="confirm_reset"
        />
    </v-main>
</template>

<style scoped>
.settings-section-title {
    margin-bottom: 14px;
    color: rgb(var(--v-theme-primary));
    font-size: 0.95rem;
    font-weight: 700;
}

.section-divider {
    margin: 8px 0 20px;
}

.theme-swatch {
    display: inline-block;
    width: 14px;
    height: 14px;
    flex: 0 0 auto;
    border: 1px solid rgba(var(--v-theme-on-surface), 0.18);
    border-radius: 50%;
}

.theme-name {
    margin-right: 10px;
}

.theme-palette {
    display: inline-flex;
    align-items: center;
    gap: 4px;
}

.danger-zone {
    border-color: rgb(var(--v-theme-error)) !important;
}

.danger-zone-title {
    margin-bottom: 8px;
    color: rgb(var(--v-theme-error));
    font-size: 0.95rem;
    font-weight: 700;
}

.danger-zone-description {
    margin-bottom: 16px;
    color: rgba(var(--v-theme-on-surface), 0.7);
    font-size: 0.9rem;
}
</style>
