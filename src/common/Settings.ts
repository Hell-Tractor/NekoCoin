import { reactive, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface UserSettings {
    primary_currency_code: string;
    theme: string;
    user_name: string;
    avatar: string;
}

export const default_settings: UserSettings = {
    primary_currency_code: 'CNY',
    theme: 'pinkPad',
    user_name: '',
    avatar: 'mdi-cat',
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
