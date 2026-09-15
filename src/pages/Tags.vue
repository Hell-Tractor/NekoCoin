<script setup lang="ts">
import { computed, onMounted, ref, Ref } from 'vue';
import Tag, { TagType, TagTypeNames, TagTypeToString, buildTagForest } from '../common/Tag';
import { useI18n } from 'vue-i18n';
import { invoke } from '@tauri-apps/api/core';
import { show_error } from '../common/Notify';
import { useRouter } from 'vue-router';
import EmptyState from './components/EmptyState.vue';
import TagTreeList from './components/TagTreeList.vue';
const { t } = useI18n();
const router = useRouter();

const selected_kind: Ref<TagType | 'all'> = ref('all');
const tags: Ref<Tag[]> = ref([]);
const collapsed = ref<Set<number>>(new Set());
const kind_filters = [{ type: 'all' as const, name: 'all' }, ...TagTypeNames];

const forest = computed(() => buildTagForest(tags.value));
const sections = computed(() => TagTypeNames
    .map(item => ({
        ...item,
        nodes: buildTagForest(tags.value.filter(tag => tag.type === item.name)),
    }))
    .filter(item => item.nodes.length > 0));

const retrieve_tags = async function() {
    try {
        const kind = selected_kind.value === 'all' ? undefined : TagTypeToString(selected_kind.value);
        tags.value = await invoke('retrieve_tags', { filter: '', kind });
        collapsed.value = new Set();
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

const toggle_node = function(id: number) {
    const next = new Set(collapsed.value);
    if (next.has(id)) {
        next.delete(id);
    } else {
        next.add(id);
    }
    collapsed.value = next;
}

onMounted(() => {
    retrieve_tags();
});
</script>
<template>
    <v-card id="card" rounded="xl">
        <v-chip-group mandatory v-model="selected_kind" @update:model-value="retrieve_tags">
            <v-chip v-for="item in kind_filters" :key="item.name" :value="item.type" variant="flat" color="secondary" size="small">
                {{ item.type === 'all' ? t('tag.filter.all') : t(`tag.type.${item.name}`) }}
            </v-chip>
        </v-chip-group>
        <EmptyState v-if="tags.length === 0" variant="flat" :title="t('tag.no_tag')" :tip="t('tag.no_tag_tip')" />
        <template v-else-if="selected_kind === 'all'">
            <div v-for="(section, index) in sections" :key="section.name" :class="{ 'mt-4': index > 0 }">
                <div class="list-section-title">{{ t(`tag.type.${section.name}`) }}</div>
                <TagTreeList :nodes="section.nodes" :collapsed="collapsed" :kind-icon="kind_icon" @open="id => router.push({ path: `/tag/${id}` })" @toggle="toggle_node" />
            </div>
        </template>
        <TagTreeList v-else :nodes="forest" :collapsed="collapsed" :kind-icon="kind_icon" @open="id => router.push({ path: `/tag/${id}` })" @toggle="toggle_node" />
    </v-card>
</template>
<style scoped>
#card {
    padding: 10px;
}

:deep(.tag-row) {
    width: 100%;
    padding-right: 4px;
    border: 0;
    background: transparent;
    color: inherit;
    text-align: left;
    cursor: pointer;
}

:deep(.tag-tree-toggle) {
    display: flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    margin-left: -6px;
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
    cursor: pointer;
}

:deep(.tag-tree-toggle-spacer) {
    flex: 0 0 22px;
}

:deep(.v-flipped) {
    transform: scaleY(-1);
}
</style>
