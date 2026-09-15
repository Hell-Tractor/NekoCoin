<script setup lang="ts">
import { TagNode } from '../../common/Tag';
import { entity_accent_color, entity_avatar_style } from '../../common/Utils';

defineOptions({ name: 'TagTreeList' });

defineProps<{
    nodes: TagNode[];
    depth?: number;
    expanded: Set<number>;
    kindIcon: (type: string) => { icon: string; color: string; flipped: boolean };
}>();

const emit = defineEmits<{
    open: [id: number];
    toggle: [id: number];
}>();
</script>

<template>
    <template v-for="node in nodes" :key="node.id">
        <div
            class="entity-row tag-row"
            role="button"
            tabindex="0"
            :style="{ paddingLeft: `${4 + (depth ?? 0) * 16}px` }"
            @click="emit('open', node.id)"
            @keydown.enter="emit('open', node.id)"
        >
            <button
                v-if="node.children.length > 0"
                class="tag-tree-toggle"
                type="button"
                @click.stop="emit('toggle', node.id)"
            >
                <v-icon size="20">{{ expanded.has(node.id) ? 'mdi-chevron-down' : 'mdi-chevron-right' }}</v-icon>
            </button>
            <span v-else class="tag-tree-toggle-spacer" />
            <div class="entity-avatar" :style="entity_avatar_style(node.color)">
                <v-icon :color="entity_accent_color(node.color)" size="22">{{ node.icon }}</v-icon>
            </div>
            <div class="entity-copy">
                <div class="entity-name">{{ node.name }}</div>
                <div v-if="node.remark" class="entity-meta">{{ node.remark }}</div>
            </div>
            <v-icon size="22" :color="kindIcon(node.type).color" :class="{ 'v-flipped': kindIcon(node.type).flipped }">{{ kindIcon(node.type).icon }}</v-icon>
        </div>
        <TagTreeList
            v-if="node.children.length > 0 && expanded.has(node.id)"
            :nodes="node.children"
            :depth="(depth ?? 0) + 1"
            :expanded="expanded"
            :kind-icon="kindIcon"
            @open="emit('open', $event)"
            @toggle="emit('toggle', $event)"
        />
    </template>
</template>
