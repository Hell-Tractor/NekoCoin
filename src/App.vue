<script setup lang="ts">
import { ref, Ref } from 'vue';
import Home from './pages/Home.vue';
import Wallet from './pages/Wallet.vue';
import Tags from './pages/Tags.vue';

const currentPage: Ref<string> = ref('首页');
const showDrawer: Ref<boolean> = ref(false);

const changePage = function(target_page: string) : void {
    currentPage.value = target_page;
    showDrawer.value = false;
};
</script>

<template>
    <v-app>
        <v-app-bar density="compact" color="primary">
            <v-app-bar-nav-icon @click="showDrawer = !showDrawer;"></v-app-bar-nav-icon>
            <v-toolbar-title>{{ currentPage }}</v-toolbar-title>

            <v-spacer></v-spacer>

            <v-btn icon="mdi-pencil-plus"></v-btn>
        </v-app-bar>

        <v-navigation-drawer v-model="showDrawer">
            <v-list>
                <v-list-item v-for="item in ['首页', '记账', '报表', '钱包', '标签', '设置']" :key="item" @click="changePage(item)">
                    <v-list-item-title>{{ item }}</v-list-item-title>
                </v-list-item>
            </v-list>
        </v-navigation-drawer>

        <v-main class="page">
            <Home v-if="currentPage === '首页'" />
            <Wallet v-else-if="currentPage === '钱包'" />
            <Tags v-else-if="currentPage === '标签'" />
            <span v-else>等等，你为什么会在这里？（WIP）</span>
        </v-main>
    </v-app>
</template>

<style scoped>
.page {
    margin: 20px;
}
</style>
