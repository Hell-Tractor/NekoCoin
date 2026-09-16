<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { privacy_mode, toggle_privacy_mode } from '../../common/Settings';

const { t } = useI18n();
</script>

<template>
    <v-btn
        icon
        :aria-label="privacy_mode ? t('privacy.show_amounts') : t('privacy.hide_amounts')"
        @click="toggle_privacy_mode"
    >
        <v-icon>{{ privacy_mode ? 'mdi-eye-off-outline' : 'mdi-eye-outline' }}</v-icon>
    </v-btn>
</template>
