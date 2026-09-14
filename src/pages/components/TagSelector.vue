<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Tag, { TagType, TagTypeToString } from '../../common/Tag';
import { useRouter } from 'vue-router';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    tags: Tag[],
    kind?: TagType,
}>();
const selected_tag = defineModel<Tag>();

const clamp = function(value: number, min: number, max: number) {
    return Math.max(min, Math.min(max, value));
}

const offsetColor = function(color: string, offset: number, alpha: number) {
    var [r, g, b] = color.substring(4, color.length - 1).split(',');
    return `rgba(${clamp(parseInt(r) + offset, 0, 255)}, ${clamp(parseInt(g) + offset, 0, 255)}, ${clamp(parseInt(b) + offset, 0, 255)}, ${alpha})`;
}

const open_add_tag = function() {
    const query = props.kind === undefined ? {} : { kind: TagTypeToString(props.kind) };
    router.push({ path: '/tag/add', query });
}
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
            <v-chip-group v-else mandatory column v-model="selected_tag">
                <v-chip v-for="tag in props.tags" :key="tag.id" :value="tag" :color="tag.color" :style="(!selected_tag || selected_tag.id != tag.id) ? { backgroundColor: offsetColor(tag.color, 50, 0.7) } : { borderWidth: '1px', borderColor: offsetColor(tag.color, -50, 1) }" label :prepend-icon="tag.icon">{{ tag.name }}</v-chip>
            </v-chip-group>
        </v-card-text>
    </v-card>
</template>
