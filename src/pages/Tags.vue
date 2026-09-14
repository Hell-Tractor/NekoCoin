<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import Tag, { TagType, TagTypeNames, TagTypeToString } from '../common/Tag';
import { useI18n } from 'vue-i18n';
import { invoke } from '@tauri-apps/api/core';
import { show_error } from '../common/Notify';
import { useRouter } from 'vue-router';
import { entity_accent_color, entity_avatar_style } from '../common/Utils';
import EmptyState from './components/EmptyState.vue';
const { t } = useI18n();
const router = useRouter();

const filter_key: Ref<string> = ref('');
const selected_kind: Ref<TagType | 'all'> = ref('all');
const tags: Ref<Tag[]> = ref([]);
const kind_filters = [{ type: 'all' as const, name: 'all' }, ...TagTypeNames];

const retrieve_tags = async function() {
    try {
        const kind = selected_kind.value === 'all' ? undefined : TagTypeToString(selected_kind.value);
        tags.value = await invoke('retrieve_tags', { filter: filter_key.value, kind });
    } catch (error) {
        show_error(error);
    }
}

const kind_icon = function(type: string) {
    if (type == 'Income') {
        return { icon: 'mdi-chart-line-variant', color: 'success', flipped: false };
    }
    if (type == 'Expense') {
        return { icon: 'mdi-chart-line-variant', color: 'error', flipped: true };
    }
    if (type == 'Activity') {
        return { icon: 'mdi-flag', color: 'warning', flipped: false };
    }
    return { icon: 'mdi-swap-horizontal', color: 'info', flipped: false };
}

onMounted(() => {
    retrieve_tags();
});
</script>
<template>
    <v-card id="card" rounded="xl">
        <v-text-field height="50px" clearable density="compact" :placeholder="t('tag.search.hint')" append-inner-icon="mdi-magnify" v-model="filter_key" variant="outlined" @update:model-value="retrieve_tags" hide-details></v-text-field>
        <v-chip-group mandatory v-model="selected_kind" @update:model-value="retrieve_tags" class="mt-2">
            <v-chip v-for="item in kind_filters" :key="item.name" :value="item.type" variant="flat" color="secondary" size="small">
                {{ item.type === 'all' ? t('tag.filter.all') : t(`tag.type.${item.name}`) }}
            </v-chip>
        </v-chip-group>
        <EmptyState v-if="tags.length === 0" variant="flat" :title="t('tag.no_tag')" :tip="t('tag.no_tag_tip')" />
        <button
            v-else
            v-for="item in tags"
            :key="item.id"
            class="entity-row tag-row"
            type="button"
            @click="router.push({ path: `/tag/${item.id}`})"
        >
            <div class="entity-avatar" :style="entity_avatar_style(item.color)">
                <v-icon :color="entity_accent_color(item.color)" size="22">{{ item.icon }}</v-icon>
            </div>
            <div class="entity-copy">
                <div class="entity-name">{{ item.name }}</div>
                <div v-if="item.remark" class="entity-meta">{{ item.remark }}</div>
            </div>
            <v-icon size="22" :color="kind_icon(item.type).color" :class="{ 'v-flipped': kind_icon(item.type).flipped }">{{ kind_icon(item.type).icon }}</v-icon>
        </button>
    </v-card>
</template>
<style scoped>
#card {
    padding: 10px;
}

.tag-row {
    width: 100%;
    padding-left: 4px;
    padding-right: 4px;
    border: 0;
    background: transparent;
    color: inherit;
    text-align: left;
    cursor: pointer;
}

.v-flipped {
    transform: scaleY(-1);
}
</style>
