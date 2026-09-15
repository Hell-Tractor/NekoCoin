<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import Tag, { TagNode, TagType, TagTypeFromString, TagTypeToString, buildTagForest, findTagNode, tagPath, toTag } from '../../common/Tag';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { entity_accent_color, entity_chip_style, is_dark_theme } from '../../common/Utils';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    tags: Tag[],
    kind?: TagType,
}>();
const selected_tag = defineModel<Tag | undefined>();
const expanded = ref<TagNode | null>(null);
const frequent = ref<Tag[]>([]);

const kind_name = computed(() => {
    if (props.kind === undefined) {
        return undefined;
    }
    if (typeof props.kind === 'string') {
        try {
            return TagTypeToString(TagTypeFromString(props.kind));
        } catch {
            return TagTypeToString(Number(props.kind) as TagType);
        }
    }
    return TagTypeToString(props.kind);
});
const forest = computed(() => buildTagForest(props.tags));
const roots = computed(() => forest.value);
const available_ids = computed(() => new Set(props.tags.map(tag => tag.id)));
const frequent_visible = computed(() => frequent.value.filter(tag => available_ids.value.has(tag.id)));
const child_nodes = computed(() => expanded.value?.children ?? []);
const breadcrumb = computed(() => expanded.value ? tagPath(forest.value, expanded.value.id) : []);
const show_back = computed(() => breadcrumb.value.length > 1);
const branch_root_id = computed(() => breadcrumb.value[0]?.id);

const chip_style = function(color: string, selected: boolean) {
    return entity_chip_style(color, selected);
}

const chip_color = function(color: string) {
    return is_dark_theme() ? entity_accent_color(color) : undefined;
}

const chip_variant = function() {
    return is_dark_theme() ? 'tonal' : 'flat';
}

const is_selected = function(id: number) {
    return selected_tag.value?.id === id;
}

const node_by_id = function(id: number) {
    return findTagNode(forest.value, id);
}

const has_children = function(id: number) {
    return (node_by_id(id)?.children.length ?? 0) > 0;
}

const chevron_icon = function(id: number) {
    if (!has_children(id)) {
        return undefined;
    }
    return breadcrumb.value.some(item => item.id === id) ? 'mdi-chevron-up' : 'mdi-chevron-down';
}

const expand_for_selection = function(id: number) {
    const path = tagPath(forest.value, id);
    if (path.length === 0) {
        expanded.value = null;
        return;
    }
    const node = path[path.length - 1];
    expanded.value = node.children.length > 0 ? node : (path[path.length - 2] ?? null);
}

const on_chip_click = function(node: TagNode) {
    if (is_selected(node.id)) {
        selected_tag.value = undefined;
        expanded.value = null;
        return;
    }
    selected_tag.value = toTag(node);
    expand_for_selection(node.id);
}

const on_plain_click = function(tag: Tag) {
    const node = findTagNode(forest.value, tag.id);
    if (node) {
        on_chip_click(node);
        return;
    }
    if (is_selected(tag.id)) {
        selected_tag.value = undefined;
        expanded.value = null;
        return;
    }
    selected_tag.value = tag;
}

const go_up = function() {
    if (!expanded.value) {
        return;
    }
    const path = tagPath(forest.value, expanded.value.id);
    expanded.value = path.length > 1 ? path[path.length - 2] : null;
}

const open_add_tag = function() {
    const query = kind_name.value === undefined ? {} : { kind: kind_name.value };
    router.push({ path: '/tag/add', query });
}

const load_frequent = async function() {
    if (kind_name.value === undefined || props.tags.length === 0) {
        frequent.value = [];
        return;
    }
    try {
        const result = await invoke('frequent_tags', {
            kind: kind_name.value,
            limit: 5,
        }) as Tag[];
        frequent.value = result;
    } catch {
        frequent.value = [];
    }
}

watch(() => selected_tag.value?.id, (id, old_id) => {
    if (id == null) {
        if (old_id != null) {
            expanded.value = null;
        }
        return;
    }
    if (id === old_id && expanded.value) {
        return;
    }
    expand_for_selection(id);
});

watch(forest, () => {
    if (expanded.value) {
        expanded.value = findTagNode(forest.value, expanded.value.id) ?? null;
    }
    if (selected_tag.value?.id != null && !expanded.value) {
        expand_for_selection(selected_tag.value.id);
    }
});

watch(() => [kind_name.value, props.tags.map(tag => tag.id).join(',')], () => {
    load_frequent();
}, { immediate: true });
</script>

<template>
    <v-card variant="text" density="compact">
        <v-card-text>
            <v-row class="flex-nowarp">
                <v-col style="padding-left: 3px;">
                    <span>{{ t('tag.select') }}</span>
                </v-col>
                <v-col class="d-flex justify-end">
                    <v-btn icon="mdi-plus" size="medium" density="compact" variant="text" @click="open_add_tag"></v-btn>
                </v-col>
            </v-row>
            <div v-if="props.tags.length === 0" class="on-surface-lighten-1 text-body-2 mt-2 mb-2">{{ t('tag.empty_on_select') }}</div>
            <template v-else>
                <div v-if="frequent_visible.length > 0" class="tag-frequent">
                    <div class="tag-section-label">{{ t('tag.frequent') }}</div>
                    <div class="tag-chip-row">
                        <v-chip
                            v-for="tag in frequent_visible"
                            :key="`frequent-${tag.id}`"
                            size="small"
                            :variant="chip_variant()"
                            :color="chip_color(tag.color)"
                            :class="{ 'tag-chip-light': !is_dark_theme() }"
                            :style="chip_style(tag.color, is_selected(tag.id))"
                            label
                            :prepend-icon="tag.icon"
                            :append-icon="chevron_icon(tag.id)"
                            @click="on_plain_click(tag)"
                        >{{ tag.name }}</v-chip>
                    </div>
                </div>
                <div class="tag-all">
                    <div class="tag-section-label">{{ t('tag.all_tags') }}</div>
                    <div class="tag-chip-row">
                        <template v-for="tag in roots" :key="tag.id">
                            <v-chip
                                :variant="chip_variant()"
                                :color="chip_color(tag.color)"
                                :class="{ 'tag-chip-light': !is_dark_theme() }"
                                :style="chip_style(tag.color, is_selected(tag.id))"
                                label
                                :prepend-icon="tag.icon"
                                :append-icon="chevron_icon(tag.id)"
                                @click="on_chip_click(tag)"
                            >{{ tag.name }}</v-chip>
                            <div v-if="branch_root_id === tag.id && child_nodes.length > 0" class="tag-branch">
                                <button v-if="show_back" class="tag-back" type="button" @click="go_up">
                                    <v-icon size="16">mdi-chevron-left</v-icon>
                                    <span>{{ breadcrumb.map(item => item.name).join(' / ') }}</span>
                                    <span class="tag-back-hint">{{ t('tag.back_level') }}</span>
                                </button>
                                <div class="tag-chip-row tag-chip-row--nested">
                                    <v-chip
                                        v-for="child in child_nodes"
                                        :key="child.id"
                                        :variant="chip_variant()"
                                        :color="chip_color(child.color)"
                                        :class="{ 'tag-chip-light': !is_dark_theme() }"
                                        :style="chip_style(child.color, is_selected(child.id))"
                                        label
                                        :prepend-icon="child.icon"
                                        :append-icon="chevron_icon(child.id)"
                                        @click="on_chip_click(child)"
                                    >{{ child.name }}</v-chip>
                                </div>
                            </div>
                        </template>
                    </div>
                </div>
            </template>
        </v-card-text>
    </v-card>
</template>

<style scoped>
.tag-frequent {
    padding: 8px 10px 4px;
    margin: 0 -4px 10px;
    border-radius: 12px;
    background: rgba(var(--v-theme-on-surface), 0.05);
}

.tag-all {
    padding-top: 2px;
}

.tag-section-label {
    margin-bottom: 6px;
    color: rgba(var(--v-theme-on-surface), 0.58);
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.02em;
}

.tag-chip-row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
    margin: 0 0 4px;
}

.tag-chip-row :deep(.tag-chip-light .v-icon) {
    color: inherit;
    opacity: 1;
}

.tag-chip-row--nested {
    margin: 0;
}

.tag-branch {
    flex: 1 1 100%;
    padding: 6px 0 8px 10px;
    margin: 0 0 4px;
    border-left: 2px solid rgba(var(--v-theme-on-surface), 0.16);
}

.tag-back {
    display: flex;
    align-items: center;
    gap: 4px;
    margin: 0 0 8px;
    padding: 0;
    border: 0;
    background: transparent;
    color: rgba(var(--v-theme-on-surface), 0.72);
    font-size: 0.8125rem;
    cursor: pointer;
}

.tag-back-hint {
    margin-left: 4px;
    color: rgba(var(--v-theme-on-surface), 0.46);
}
</style>
