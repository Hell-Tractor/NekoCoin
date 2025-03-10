<script setup lang="ts">
import BackTitleBar from '../common/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import { Ref, ref } from 'vue';
import { getRandomColor } from '../common/Utils';
import Constants from '../common/Constants';
const { t } = useI18n();

const emits = defineEmits<{
    back: []
}>();

const icon: Ref<string> = ref('mdi-credit-card');
const showMdiSelector: Ref<boolean> = ref(false);
const selected_color: Ref<string> = ref(getRandomColor('rgb'));
const form: Ref<boolean> = ref(false);

const addWallet = function() {
    // TODO
    emits('back');
}
</script>

<template>
    <BackTitleBar :title="t('add_account')" @back="emits('back')"></BackTitleBar>
    <v-form class="fill-height" v-model="form">
        <v-text-field :placeholder="t('enter_account_name')" variant="outlined" density="comfortable" :rules="[rules.required, rules.maxLength(Constants.MAX_WALLET_NAME_LENGTH)]"></v-text-field>
        <v-text-field :placeholder="t('enter_remark')" variant="outlined" density="comfortable" :rules="[rules.maxLength(Constants.MAX_WALLET_REMARK_LENGTH)]"></v-text-field>
        <v-text-field :placeholder="t('enter_amount')" variant="outlined" density="comfortable" :rules="[rules.required, rules.isValidMoney]"></v-text-field>
        <v-btn :prepend-icon="icon" variant="text" @click="showMdiSelector=true" width="100%" class="justify-start">{{ t('select_icon') }}</v-btn>
        <v-color-picker elevation="0" width="100%" v-model="selected_color" mode="rgb" style="margin-top: 10px; margin-bottom: 60px;"></v-color-picker>
        <v-btn @click="addWallet" color="primary" width="93%" style="position: fixed; bottom: 10px;" :disabled="!form">{{ t('save') }}</v-btn>
    </v-form>
</template>