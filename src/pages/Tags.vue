<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import Tag, { TagType, TagTypeNames, TagTypeToString } from '../common/Tag';
import { useI18n } from 'vue-i18n';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
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
        console.error(error);
    }
}

const kind_icon = function(type: string) {
    if (type == 'Income') {
        return { icon: 'mdi-chart-line-variant', color: 'green', flipped: false };
    }
    if (type == 'Expense') {
        return { icon: 'mdi-chart-line-variant', color: 'red', flipped: true };
    }
    if (type == 'Activity') {
        return { icon: 'mdi-flag', color: 'orange', flipped: false };
    }
    return { icon: 'mdi-swap-horizontal', color: 'blue', flipped: false };
}

onMounted(() => {
    retrieve_tags();
});
</script>
<template>
    <v-card id="card">
        <v-text-field height="50px" clearable density="compact" :placeholder="t('tag.search.hint')" append-inner-icon="mdi-magnify" v-model="filter_key" variant="outlined" @update:model-value="retrieve_tags"></v-text-field>
        <v-chip-group mandatory v-model="selected_kind" @update:model-value="retrieve_tags">
            <v-chip v-for="item in kind_filters" :key="item.name" :value="item.type" variant="flat" color="secondary" size="small">
                {{ item.type === 'all' ? t('tag.filter.all') : t(`tag.type.${item.name}`) }}
            </v-chip>
        </v-chip-group>
        <v-card-text v-if="tags.length === 0" class="text-center on-surface-lighten-1">{{ t('report.no_data') }}</v-card-text>
        <template v-else>
            <v-card variant="text" block v-for="item in tags" :key="item.id" @click="router.push({ path: `/tag/${item.id}`})">
                <v-card-text style="padding: 12px;">
                    <v-row class="flex-nowrap align-center">
                        <v-col class="flex-grow-0">
                            <v-icon :color="item.color" size="x-large">{{ item.icon }}</v-icon>
                        </v-col>
                        <v-col>
                            <v-row><v-col class="text-body-1" style="padding: 0px;">{{ item.name }}</v-col></v-row>
                            <v-row v-if="item.remark"><v-col class="on-surface-lighten-1" style="padding: 2px 0px 0px 0px;">{{ item.remark }}</v-col></v-row>
                        </v-col>
                        <v-col class="flex-grow-0">
                            <v-icon size="large" :color="kind_icon(item.type).color" :class="{ 'v-flipped': kind_icon(item.type).flipped }">{{ kind_icon(item.type).icon }}</v-icon>
                        </v-col>
                    </v-row>
                </v-card-text>
            </v-card>
        </template>
    </v-card>
</template>
<style scoped>
#card {
    padding: 10px;
}

.v-flipped {
    transform: scaleY(-1);
}
</style>
