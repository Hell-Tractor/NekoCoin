import { reactive, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface UserSettings {
    primary_currency_code: string;
    theme: string;
    user_name: string;
    avatar: string;
    locale: 'zh-CN' | 'en-US';
    default_page: string;
    date_format: 'YYYY-MM-DD' | 'DD/MM/YYYY' | 'MM/DD/YYYY';
    time_format: '12hr' | '24hr';
    decimal_places: number;
    thousands_separator: boolean;
    initialized: boolean;
    log_retention_days: number;
}

export const default_settings: UserSettings = {
    primary_currency_code: 'CNY',
    theme: 'pinkPad',
    user_name: '',
    avatar: 'mdi-cat',
    locale: 'zh-CN',
    default_page: 'home',
    date_format: 'YYYY-MM-DD',
    time_format: '24hr',
    decimal_places: 2,
    thousands_separator: true,
    initialized: false,
    log_retention_days: 30,
};

export const settings = reactive<UserSettings>({ ...default_settings });
export const settings_loaded = ref(false);
let loading_settings: Promise<void> | undefined;

export const load_settings = async function() {
    if (settings_loaded.value) {
        return;
    }
    if (loading_settings !== undefined) {
        return loading_settings;
    }
    loading_settings = invoke('get_settings')
        .then(loaded => {
            Object.assign(settings, default_settings, loaded as UserSettings);
        })
        .catch(error => console.error(error))
        .finally(() => {
            settings_loaded.value = true;
            loading_settings = undefined;
        });
    return loading_settings;
};

export const save_settings = async function() {
    await invoke('save_settings', { settings: { ...settings } });
};

export const reset_app = async function() {
    await invoke('reset_app');
};

export const get_log_usage = async function(): Promise<number> {
    return await invoke('get_log_usage') as number;
};

export const clear_logs = async function() {
    await invoke('clear_logs');
};
