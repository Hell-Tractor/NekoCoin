<script setup lang="ts">
import { computed, ref, Ref } from 'vue';
import Home from './pages/Home.vue';
import Wallet from './pages/Wallet.vue';
import Tags from './pages/Tags.vue';
import { useI18n } from 'vue-i18n';
import AddWallet from './pages/AddWallet.vue';
import AddTag from './pages/AddTag.vue';
import AddTransaction from './pages/AddTransaction.vue';
import Transaction from './pages/Transaction.vue';
const { t } = useI18n();

interface Page {
    name: string;
    indexInBottom?: number;
    icon?: string;
    nextPage?: string;
}

const allPages: Page[] = [
    { name: 'home', indexInBottom: 0, icon: 'mdi-home', nextPage: 'add_transaction' },
    { name: 'accounts', indexInBottom: 1, icon: 'mdi-credit-card', nextPage: 'add_account' },
    { name: 'tags' },
    { name: 'transactions' },
    { name: 'reports', indexInBottom: 2, icon: 'mdi-chart-multiple' },
    { name: 'settings' },
];
const bottomPages = computed(() => allPages.filter(page => page.indexInBottom != undefined).sort((a, b) => (a.indexInBottom as number) - (b.indexInBottom as number)));
const currentPage: Ref<Page> = ref(allPages[0]);
const showDrawer: Ref<boolean> = ref(false);

const changePage = function(target_page: Page) : void {
    currentPage.value = target_page;
    showDrawer.value = false;
};
const changePageByName = function(target_page_name: string) : void {
    currentPage.value = allPages.find(page => page.name === target_page_name) as Page;
    showDrawer.value = false;
}
const globalButtonClick = function() : void {
    if (currentPage.value.nextPage) {
        currentPage.value = { name: currentPage.value.nextPage } as Page;
    }
}
const showMenuBar = computed(() => allPages.map(page => page.name).includes(currentPage.value.name));
</script>

<template>
    <v-app>
        <v-app-bar density="compact" color="primary" v-if="showMenuBar">
            <v-app-bar-nav-icon @click="showDrawer = !showDrawer;"></v-app-bar-nav-icon>
            <v-toolbar-title>{{ t('app_name') }}</v-toolbar-title>
        </v-app-bar>

        <v-navigation-drawer v-model="showDrawer">
            <v-list>
                <v-list-item v-for="item in allPages" :key="item.name" @click="changePage(item)">
                    <v-list-item-title>{{ t(`page.${item.name}`) }}</v-list-item-title>
                </v-list-item>
            </v-list>
        </v-navigation-drawer>

        <v-main class="page">
            <Home v-if="currentPage.name === 'home'" />
            <Wallet v-else-if="currentPage.name === 'accounts'" />
            <Tags v-else-if="currentPage.name === 'tags'" @create-tag="currentPage = { name: 'add_tag' }" />
            <Transaction v-else-if="currentPage.name === 'transactions'" />
            <AddWallet v-else-if="currentPage.name === 'add_account'" @back="changePageByName('accounts')"/>
            <AddTag v-else-if="currentPage.name === 'add_tag'" @back="changePageByName('tags')"/>
            <AddTransaction v-else-if="currentPage.name === 'add_transaction'" @back="changePageByName('home')"/>
            <span v-else>{{ t("WIP") }}</span>
        </v-main>
        <v-btn v-if="!!currentPage.nextPage" color="secondary" @click="globalButtonClick" icon="mdi-paw" size="large" class="right-0 bottom-0" style="margin: 10px; margin-bottom: 65px; position: fixed;"></v-btn>

        <v-bottom-navigation grow mandatory bg-color="primary" v-if="showMenuBar">
            <v-btn v-for="page in bottomPages" :key="page.indexInBottom as number" @click="changePage(page)">
                <v-icon>{{ page!.icon }}</v-icon>
                <span>{{ t(`page.${page.name}`) }}</span>
            </v-btn>
        </v-bottom-navigation>
    </v-app>
</template>

<style scoped>
.page {
    margin: 10px;
}
</style>
