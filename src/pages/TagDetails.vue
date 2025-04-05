<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import BackTitleBar from './components/BackTitleBar.vue';
import Tag, { TagType, TagTypeToString } from '../common/Tag';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import TransactionList from './components/TransactionList.vue';
import ConfirmSheet from './components/ConfirmSheet.vue';
import AddTag from './AddTag.vue';
import StackDiagram from './components/StackDiagram.vue';
import Constants from '../common/Constants';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    id: number,
}>();

const tag: Ref<Tag | undefined> = ref(undefined);
const show_confirm_sheet: Ref<boolean> = ref(false);

const get_tag = async function() {
    try {
        tag.value = await invoke('get_tag_by_id', { id: props.id });
    } catch (e) {
        console.error(e);
    }
}

const delete_tag = async function() {
    try {
        await invoke('delete_tag', { id: props.id });
        router.back();
    } catch (e) {
        console.error(e);
    }
}

const edit_tag = async function() {
    if (router.hasRoute("tag_edit")) {
        router.removeRoute("tag_edit");
    }
    router.addRoute({ path: "/tag/edit", name: "tag_edit", props: { init: tag.value }, component: AddTag })
    await router.push({ path: "/tag/edit" });
}

onMounted(async () => {
    await get_tag();
});
</script>

<template>
    <BackTitleBar :title="tag?.name ?? t('loading')" @back="router.back()">
        <template v-slot:append>
            <v-btn icon="mdi-pencil" @click="edit_tag"></v-btn>
            <v-btn icon="mdi-delete" @click="show_confirm_sheet=true"></v-btn>
        </template>
    </BackTitleBar>
    <v-main class="main">
        <StackDiagram v-if="tag && tag.type != TagTypeToString(TagType.TRANSFER)" class="mb-2" :item_id="{ type: 'tag', value: props.id }" :currency="Constants.CURRENCIES[0].symbol" variant="flat" rounded="lg" :kind="tag?.type == 'Expense' ? TagType.EXPENSE : TagType.INCOME"></StackDiagram>
        <TransactionList :filter="{ by: 'tag', id: props.id }" variant="flat"></TransactionList>
        <ConfirmSheet v-model="show_confirm_sheet" :title="t('warning.cascade_and_irrevertible.title')" :text="t('warning.cascade_and_irrevertible.content')" @confirm="delete_tag"></ConfirmSheet>
    </v-main>
</template>