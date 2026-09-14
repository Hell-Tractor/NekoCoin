<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Tag, { TagType, TagTypeToString } from '../../common/Tag';
import { useRouter } from 'vue-router';
import { entity_accent_color, entity_tint } from '../../common/Utils';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    tags: Tag[],
    kind?: TagType,
}>();
const selected_tag = defineModel<Tag>();

const chip_style = function(color: string, selected: boolean) {
    const accent = entity_accent_color(color);
    if (selected) {
        return { borderWidth: '1px', borderColor: accent };
    }
    return { backgroundColor: entity_tint(color) };
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
                <v-chip v-for="tag in props.tags" :key="tag.id" :value="tag" :color="entity_accent_color(tag.color)" :style="chip_style(tag.color, !!selected_tag && selected_tag.id == tag.id)" label :prepend-icon="tag.icon">{{ tag.name }}</v-chip>
            </v-chip-group>
        </v-card-text>
    </v-card>
</template>
