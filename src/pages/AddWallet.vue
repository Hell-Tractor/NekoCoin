<script setup lang="ts">
import BackTitleBar from '../common/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import { Ref, ref } from 'vue';
const { t } = useI18n();

const emits = defineEmits<{
    back: []
}>();

const icon: Ref<string> = ref('mdi-credit-card');
const showMdiSelector: Ref<boolean> = ref(false);
</script>

<template>
    <BackTitleBar :title="t('add_account')" @back="emits('back')"></BackTitleBar>
    <v-text-field :placeholder="t('enter_account_name')" variant="outlined" density="comfortable" :rules="[rules.required]"></v-text-field>
    <v-text-field :placeholder="t('enter_amount')" variant="outlined" density="comfortable" :rules="[rules.required, rules.isValidMoney]"></v-text-field>
    <v-btn :prepend-icon="icon" variant="text" @click="showMdiSelector=true" width="100%" class="justify-start">{{ t('select_icon') }}</v-btn>
</template>