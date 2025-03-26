<script setup lang="ts">
import BackTitleBar from '../common/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import { onMounted, Ref, ref } from 'vue';
import Tag, { TagType, TagTypeNames, TagTypeToString } from '../common/Tag';
import Constants from '../common/Constants';
import { getRandomColor } from '../common/Utils';
import { invoke } from '@tauri-apps/api/core';
import IconSelector from '../common/IconSelector.vue';
const { t } = useI18n();

const emits = defineEmits<{
    back: []
}>();

const form: Ref<boolean> = ref(false);
const tag_name: Ref<string> = ref('');
const tag_remark: Ref<string> = ref('');
const icon: Ref<string> = ref('mdi-tag');
const selected_color: Ref<string> = ref(getRandomColor('rgb'));
const parent_tag: Ref<Tag | null> = ref(null);

const tag_search_text: Ref<string> = ref('');

const selected_tag_type: Ref<{ type: TagType, name: string }> = ref(TagTypeNames[0]);
const tags: Ref<Tag[]> = ref([]);
const page: Ref<string> = ref('main');

const retrieve_tags = async function() {
    try {
        tags.value = await invoke('retrieve_tags', { filter: tag_search_text.value, kind: TagTypeToString(selected_tag_type.value.type) });
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}

const addTag = async function() {
    try {
        await invoke('create_tag', {
            name: tag_name.value,
            remark: tag_remark.value,
            color: selected_color.value,
            icon: icon.value,
            kind: TagTypeToString(selected_tag_type.value.type),
            parentId: parent_tag.value?.id
        });
        emits('back');
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}

onMounted(() => {
    retrieve_tags();
});
</script>
<template>
    <div v-if="page == 'main'">
        <BackTitleBar :title="t('tag.add')" @back="emits('back')"></BackTitleBar>
        <v-form class="fill-height" v-model="form">
            <v-chip-group mandatory v-model="selected_tag_type" @update:model-value="parent_tag = null; retrieve_tags()">
                <v-chip v-for="tag in TagTypeNames" :value="tag" :key="tag.type" variant="flat" color="secondary">{{ t(`tag.type.${tag.name}`) }}</v-chip>
            </v-chip-group>
            <v-text-field v-model="tag_name" :placeholder="t('tag.enter.name')" variant="outlined" density="comfortable" :rules="[rules.required, rules.maxLength(Constants.MAX_TAG_NAME_LENGTH)]"></v-text-field>
            <v-text-field v-model="tag_remark" :placeholder="t('tag.enter.remark')" variant="outlined" density="comfortable" :rules="[rules.maxLength(Constants.MAX_TAG_REMARK_LENGTH)]"></v-text-field>
            <v-select :no-data-text="t('tag.no_available_parent')" :placeholder="t('tag.enter.parent_tag')" :items="tags" variant="outlined" item-title="name" clearable v-model="parent_tag" return-object>
                <template v-slot:prepend-item>
                    <v-text-field v-model="tag_search_text" :placeholder="t('tag.search.hint')" dense @update:model-value="retrieve_tags"></v-text-field>
                </template>
            </v-select>
            <v-btn :prepend-icon="icon" size="large" variant="text" @click="page = 'select_icon'" block class="justify-start">{{ t('select_icon') }}</v-btn>
            <v-color-picker elevation="0" width="100%" v-model="selected_color" mode="rgb" style="margin-top: 10px; margin-bottom: 60px;"></v-color-picker>
            <v-btn @click="addTag" color="primary" width="93%" style="position: fixed; bottom: 10px;" :disabled="!form">{{ t('save') }}</v-btn>
        </v-form>
    </div>
    <IconSelector v-else-if="page == 'select_icon'" @back="page = 'main'" @confirm="selected_icon => icon = selected_icon"></IconSelector>
</template>