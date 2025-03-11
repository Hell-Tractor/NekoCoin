<script setup lang="ts">
import BackTitleBar from '../common/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import { Ref, ref } from 'vue';
import { TagTypeNames } from '../common/Tag';
import Constants from '../common/Constants';
import { getRandomColor } from '../common/Utils';
const { t } = useI18n();

const emits = defineEmits<{
    back: []
}>();

const form: Ref<boolean> = ref(false);
const selected_tag_type: Ref<number> = ref(0);
const tag_name: Ref<string> = ref('');
const tag_remark: Ref<string> = ref('');
const icon: Ref<string> = ref('mdi-tag');
const showMdiSelector: Ref<boolean> = ref(false);
const selected_color: Ref<string> = ref(getRandomColor('rgb'));

const addTag = function() {
    // TODO
}
</script>
<template>
    <BackTitleBar :title="t('tag.add')" @back="emits('back')"></BackTitleBar>
    <v-form class="fill-height" v-model="form">
        <v-chip-group mandatory v-model="selected_tag_type">
            <v-chip v-for="tag in TagTypeNames" :key="tag" variant="flat" color="secondary">{{ t(`tag.type.${tag}`) }}</v-chip>
        </v-chip-group>
        <v-text-field v-model="tag_name" :placeholder="t('tag.enter.name')" variant="outlined" density="comfortable" :rules="[rules.required, rules.maxLength(Constants.MAX_TAG_NAME_LENGTH)]"></v-text-field>
        <v-text-field v-model="tag_remark" :placeholder="t('tag.enter.remark')" variant="outlined" density="comfortable" :rules="[rules.maxLength(Constants.MAX_TAG_REMARK_LENGTH)]"></v-text-field>
        <!-- TODO create icon select page -->
        <v-btn :prepend-icon="icon" variant="text" @click="showMdiSelector=true" width="100%" class="justify-start">{{ t('select_icon') }}</v-btn>
        <v-color-picker elevation="0" width="100%" v-model="selected_color" mode="rgb" style="margin-top: 10px; margin-bottom: 60px;"></v-color-picker>
        <v-btn @click="addTag" color="primary" width="93%" style="position: fixed; bottom: 10px;" :disabled="!form">{{ t('save') }}</v-btn>
    </v-form>
</template>