<script setup lang="ts">
import { computed, ref, Ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';
import { settings } from '../common/Settings';
const { t } = useI18n();
const router = useRouter();
const route = useRoute();

interface Page {
    name: string;
    path: string;
    indexInBottom?: number;
    icon?: string;
    nextPage?: string;
}

const allPages: Page[] = [
    { name: 'home', path: '/main/home', indexInBottom: 0, icon: 'mdi-home', nextPage: '/transaction/add' },
    { name: 'accounts', path: '/main/accounts', indexInBottom: 1, icon: 'mdi-credit-card', nextPage: '/account/add' },
    { name: 'tags', path: '/main/tags', icon: 'mdi-tag-multiple', nextPage: '/tag/add' },
    { name: 'transactions', path: '/main/transactions', icon: 'mdi-format-list-bulleted' },
    { name: 'reports', path: '/main/reports', indexInBottom: 2, icon: 'mdi-chart-multiple' },
    { name: 'settings', path: '/settings', icon: 'mdi-cog-outline' },
];
const bottomPages = computed(() => allPages.filter(page => page.indexInBottom != undefined).sort((a, b) => (a.indexInBottom as number) - (b.indexInBottom as number)));
const showDrawer: Ref<boolean> = ref(false);

const changePage = function(target_page: Page) : void {
    showDrawer.value = false;
    router.push({ path: target_page.path });
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

    <v-navigation-drawer v-model="showDrawer" width="288" class="app-drawer">
        <div class="drawer-profile">
            <v-avatar :icon="settings.avatar" color="secondary" size="46" />
            <div class="drawer-profile-copy">
                <div class="drawer-profile-name">{{ settings.user_name || t('app_name') }}</div>
                <div class="drawer-profile-caption">{{ t('app_name') }}</div>
            </div>
        </div>

        <v-divider />

        <v-list nav density="comfortable" class="drawer-list">
            <v-list-item
                v-for="item in allPages"
                :key="item.name"
                :active="route.path === item.path"
                color="primary"
                rounded="lg"
                class="drawer-item"
                @click="changePage(item)"
            >
                <template #prepend>
                    <v-icon :icon="item.icon" />
                </template>
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

<style scoped>
.app-drawer :deep(.v-navigation-drawer__content) {
    padding: 12px;
}

.drawer-profile {
    display: flex;
    align-items: center;
    gap: 12px;
    min-height: 64px;
    padding: 8px 10px 16px;
}

.drawer-profile-copy {
    min-width: 0;
}

.drawer-profile-name {
    overflow: hidden;
    font-size: 1rem;
    font-weight: 700;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.drawer-profile-caption {
    margin-top: 2px;
    color: rgba(var(--v-theme-on-surface), 0.58);
    font-size: 0.75rem;
}

.drawer-list {
    padding: 12px 0 0;
}

.drawer-item {
    margin-bottom: 4px;
}
</style>
