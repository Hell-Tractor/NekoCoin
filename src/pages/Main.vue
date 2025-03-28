<script setup lang="ts">
import { computed, ref, Ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';
const { t } = useI18n();
const router = useRouter();
const route = useRoute();

interface Page {
    name: string;
    indexInBottom?: number;
    icon?: string;
    nextPage?: string;
}

const allPages: Page[] = [
    { name: 'home', indexInBottom: 0, icon: 'mdi-home', nextPage: '/transaction/add' },
    { name: 'accounts', indexInBottom: 1, icon: 'mdi-credit-card', nextPage: '/account/add' },
    { name: 'tags', nextPage: '/tag/add' },
    { name: 'transactions' },
    { name: 'reports', indexInBottom: 2, icon: 'mdi-chart-multiple' },
    { name: 'settings' },
];
const bottomPages = computed(() => allPages.filter(page => page.indexInBottom != undefined).sort((a, b) => (a.indexInBottom as number) - (b.indexInBottom as number)));
const showDrawer: Ref<boolean> = ref(false);

const changePage = function(target_page: Page) : void {
    showDrawer.value = false;
    router.push({ path: `/main/${target_page.name}` });
};
const globalButtonClick = function() : void {
    let current_page_name = route.path.split('/')[2];
    let currentPage = allPages.find(page => page.name === current_page_name);
    if (currentPage?.nextPage) {
        router.push({ path: currentPage.nextPage });
    }
}
const hasNextAction = function() : boolean {
    let current_page_name = route.path.split('/')[2];
    let currentPage = allPages.find(page => page.name === current_page_name);
    return !!currentPage?.nextPage;
}

</script>

<template>
    <v-app-bar density="compact" color="primary">
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

    <v-main class="main">
        <router-view />
    </v-main>
    <v-btn v-if="hasNextAction()" color="secondary" @click="globalButtonClick" icon="mdi-paw" size="large" class="right-0 bottom-0" style="margin: 10px; margin-bottom: 65px; position: fixed;"></v-btn>

    <v-bottom-navigation grow mandatory bg-color="primary">
        <v-btn v-for="page in bottomPages" :key="page.indexInBottom as number" @click="changePage(page)">
            <v-icon>{{ page!.icon }}</v-icon>
            <span>{{ t(`page.${page.name}`) }}</span>
        </v-btn>
    </v-bottom-navigation>
</template>
