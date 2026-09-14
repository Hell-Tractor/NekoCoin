<script setup lang="ts">
import BackTitleBar from './components/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import { onBeforeUnmount, onMounted, Ref, ref, watch } from 'vue';
import { get_random_theme_color } from '../themes/palettes';
import { load_settings, settings } from '../common/Settings';
import Constants from '../common/Constants';
import { invoke } from '@tauri-apps/api/core';
import IconSelector from './components/IconSelector.vue';
import TagSelector from './components/TagSelector.vue';
import { onBeforeRouteLeave, useRouter } from 'vue-router';
import ColorPalette from './components/ColorPalette.vue';
import Tag, { TagType, TagTypeToString } from '../common/Tag';
import { Activity } from '../common/Activity';
import { activity_draft, clear_activity_draft } from '../common/ActivityDraft';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    init?: Activity
}>();

const id: Ref<number | undefined> = ref(undefined);
const icon: Ref<string> = ref('mdi-flag');
const selected_color: Ref<string> = ref(get_random_theme_color(settings.theme));
const form: Ref<boolean> = ref(false);
const activity_name: Ref<string> = ref('');
const activity_remark: Ref<string> = ref('');
const activity_open: Ref<boolean> = ref(true);
const selected_tag: Ref<Tag | undefined> = ref(undefined);
const tags: Ref<Tag[]> = ref([]);
const page: Ref<string> = ref('main');
let restoring_draft = false;

const retrieve_tags = async function() {
    try {
        tags.value = await invoke('retrieve_tags', { filter: '', kind: TagTypeToString(TagType.ACTIVITY) });
    } catch (error) {
        console.error(error);
    }
}

const save_draft = function() {
    if (props.init) {
        return;
    }
    activity_draft.active = true;
    activity_draft.name = activity_name.value;
    activity_draft.remark = activity_remark.value;
    activity_draft.color = selected_color.value;
    activity_draft.icon = icon.value;
    activity_draft.tag_id = selected_tag.value?.id;
    activity_draft.tag = selected_tag.value ? { ...selected_tag.value } : undefined;
};

const restore_draft = function() {
    activity_name.value = activity_draft.name;
    activity_remark.value = activity_draft.remark;
    selected_color.value = activity_draft.color || get_random_theme_color(settings.theme);
    icon.value = activity_draft.icon;
    selected_tag.value = tags.value.find(tag => tag.id === activity_draft.tag_id) ?? activity_draft.tag;
};

const save = async function() {
    try {
        if (id.value) {
            await invoke('update_activity', { vo: {
                id: id.value,
                name: activity_name.value,
                remark: activity_remark.value,
                color: selected_color.value,
                icon: icon.value,
                open: activity_open.value,
                tagId: selected_tag.value!.id,
            }});
        } else {
            await invoke('create_activity', {
                name: activity_name.value,
                remark: activity_remark.value,
                color: selected_color.value,
                icon: icon.value,
                tagId: selected_tag.value!.id,
            });
        }
        clear_activity_draft();
        router.back();
    } catch (error) {
        console.error(error);
    }
}

watch([
    activity_name,
    activity_remark,
    selected_color,
    icon,
    selected_tag,
], () => {
    if (!restoring_draft) {
        save_draft();
    }
});

onMounted(async () => {
    await load_settings();
    await retrieve_tags();
    if (props.init) {
        clear_activity_draft();
        id.value = props.init.id;
        activity_name.value = props.init.name;
        activity_remark.value = props.init.remark || '';
        icon.value = props.init.icon;
        selected_color.value = props.init.color;
        activity_open.value = props.init.open;
        selected_tag.value = tags.value.find(tag => tag.id == props.init!.tag_id);
    } else if (activity_draft.active) {
        restoring_draft = true;
        restore_draft();
        restoring_draft = false;
    } else {
        selected_color.value = get_random_theme_color(settings.theme);
    }
});

onBeforeUnmount(() => {
    save_draft();
});

onBeforeRouteLeave(() => {
    save_draft();
});
</script>

<template>
    <div v-if="page == 'main'">
        <BackTitleBar :title="t(id == undefined ? 'activity.add' : 'activity.update')" @back="router.back()"></BackTitleBar>
        <v-main class="main">
            <v-form class="fill-height form-page" v-model="form">
                <v-text-field v-model="activity_name" :placeholder="t('activity.enter.name')" variant="outlined" density="comfortable" :rules="[rules.required, rules.maxLength(Constants.MAX_ACTIVITY_NAME_LENGTH)]"></v-text-field>
                <v-text-field v-model="activity_remark" :placeholder="t('activity.enter.remark')" variant="outlined" density="comfortable" :rules="[rules.maxLength(Constants.MAX_ACTIVITY_REMARK_LENGTH)]"></v-text-field>
                <v-switch v-if="id" v-model="activity_open" :label="activity_open ? t('activity.open') : t('activity.closed')" color="primary" hide-details class="mb-2" />
                <TagSelector v-model="selected_tag" :tags="tags" :kind="TagType.ACTIVITY"></TagSelector>
                <v-btn :prepend-icon="icon" size="large" variant="text" @click="page = 'select_icon'" block class="justify-start">{{ t('icon.select') }}</v-btn>
                <ColorPalette v-model="selected_color" />
                <v-color-picker elevation="0" width="100%" v-model="selected_color" mode="rgb" class="mt-2"></v-color-picker>
                <v-btn @click="save" color="primary" class="form-save-btn" :disabled="!form || selected_tag == undefined">{{ t('actions.save') }}</v-btn>
            </v-form>
        </v-main>
    </div>
    <IconSelector v-else-if="page == 'select_icon'" @back="page = 'main'" @confirm="selected_icon => icon = selected_icon"></IconSelector>
</template>
