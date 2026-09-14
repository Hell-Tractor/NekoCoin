<script setup lang="ts">
import BackTitleBar from './components/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import { onMounted, Ref, ref } from 'vue';
import Tag, { TagType, TagTypeNames, TagTypeToString } from '../common/Tag';
import Constants from '../common/Constants';
import { get_random_theme_color } from '../themes/palettes';
import { load_settings, settings } from '../common/Settings';
import { invoke } from '@tauri-apps/api/core';
import { show_error } from '../common/Notify';
import IconSelector from './components/IconSelector.vue';
import { useRoute, useRouter } from 'vue-router';
import ColorPalette from './components/ColorPalette.vue';
const { t } = useI18n();
const router = useRouter();
const route = useRoute();

const props = defineProps<{
    init?: Tag
}>();

const id: Ref<number | undefined> = ref(undefined);
const form: Ref<boolean> = ref(false);
const tag_name: Ref<string> = ref('');
const tag_remark: Ref<string> = ref('');
const icon: Ref<string> = ref('mdi-tag');
const selected_color: Ref<string> = ref(get_random_theme_color(settings.theme));
const parent_tag: Ref<Tag | null> = ref(null);

const tag_search_text: Ref<string> = ref('');

const selected_tag_type: Ref<{ type: TagType, name: string }> = ref(TagTypeNames[0]);
const tags: Ref<Tag[]> = ref([]);
const page: Ref<string> = ref('main');

const collect_descendant_ids = function(root_id: number, all: Tag[]) {
    const ids = new Set<number>();
    const walk = (parent_id: number) => {
        for (const tag of all) {
            if (tag.parent_id === parent_id && !ids.has(tag.id)) {
                ids.add(tag.id);
                walk(tag.id);
            }
        }
    };
    walk(root_id);
    return ids;
};

const retrieve_tags = async function() {
    try {
        const kind = TagTypeToString(selected_tag_type.value.type);
        const result = await invoke('retrieve_tags', { filter: tag_search_text.value, kind }) as Tag[];
        if (id.value === undefined) {
            tags.value = result;
            return;
        }
        const all = tag_search_text.value.trim() === ''
            ? result
            : await invoke('retrieve_tags', { filter: '', kind }) as Tag[];
        const forbidden = collect_descendant_ids(id.value, all);
        forbidden.add(id.value);
        tags.value = result.filter(tag => !forbidden.has(tag.id));
    } catch (error) {
        show_error(error);
    }
}

const addTag = async function() {
    try {
        if (id.value === undefined) {
            let params = {
                name: tag_name.value,
                remark: tag_remark.value,
                color: selected_color.value,
                icon: icon.value,
                kind: TagTypeToString(selected_tag_type.value.type),
                parentId: parent_tag.value?.id
            };
            await invoke('create_tag', params);
        } else {
            let params = {
                id: id.value,
                name: tag_name.value,
                remark: tag_remark.value,
                color: selected_color.value,
                icon: icon.value,
                parentId: parent_tag.value?.id
            };
            await invoke('update_tag', { vo: params });
        }
        router.back();
    } catch (error) {
        show_error(error);
    }
}

onMounted(async () => {
    await load_settings();

    if (props.init) {
        id.value = props.init.id;
        tag_name.value = props.init.name;
        tag_remark.value = props.init.remark || '';
        icon.value = props.init.icon;
        selected_color.value = props.init.color;
        selected_tag_type.value = TagTypeNames.find(tag => tag.name == props.init!.type) || TagTypeNames[0];
        await retrieve_tags();
        parent_tag.value = tags.value.find(tag => tag.id == props.init!.parent_id) || null;
    } else {
        const kind_name = route.query.kind;
        if (typeof kind_name === 'string') {
            selected_tag_type.value = TagTypeNames.find(tag => tag.name === kind_name) || TagTypeNames[0];
        }
        selected_color.value = get_random_theme_color(settings.theme);
        await retrieve_tags();
    }
});
</script>
<template>
    <div v-if="page == 'main'">
        <BackTitleBar :title="t(id == undefined ? 'tag.add' : 'tag.update')" @back="router.back()"></BackTitleBar>
        <v-main class="main">
            <v-form class="fill-height form-page" v-model="form">
                <v-chip-group mandatory v-model="selected_tag_type" @update:model-value="parent_tag = null; retrieve_tags()" :disabled="id !== undefined">
                    <v-chip v-for="tag in TagTypeNames" :value="tag" :key="tag.type" variant="flat" color="secondary">{{ t(`tag.type.${tag.name}`) }}</v-chip>
                </v-chip-group>
                <v-text-field v-model="tag_name" :placeholder="t('tag.enter.name')" variant="outlined" density="comfortable" :rules="[rules.required, rules.maxLength(Constants.MAX_TAG_NAME_LENGTH)]"></v-text-field>
                <v-text-field v-model="tag_remark" :placeholder="t('tag.enter.remark')" variant="outlined" density="comfortable" :rules="[rules.maxLength(Constants.MAX_TAG_REMARK_LENGTH)]"></v-text-field>
                <v-select :no-data-text="t('tag.no_available_parent')" :placeholder="t('tag.enter.parent_tag')" :items="tags" variant="outlined" item-title="name" clearable v-model="parent_tag" return-object>
                    <template v-slot:prepend-item>
                        <v-text-field v-model="tag_search_text" :placeholder="t('tag.search.hint')" dense @update:model-value="retrieve_tags"></v-text-field>
                    </template>
                </v-select>
                <v-btn :prepend-icon="icon" size="large" variant="text" @click="page = 'select_icon'" block class="justify-start">{{ t('icon.select') }}</v-btn>
                <ColorPalette v-model="selected_color" />
                <v-color-picker elevation="0" width="100%" v-model="selected_color" mode="rgb" class="mt-2"></v-color-picker>
                <v-btn @click="addTag" color="primary" class="form-save-btn" :disabled="!form">{{ t('actions.save') }}</v-btn>
            </v-form>
        </v-main>
    </div>
    <IconSelector v-else-if="page == 'select_icon'" @back="page = 'main'" @confirm="selected_icon => icon = selected_icon"></IconSelector>
</template>