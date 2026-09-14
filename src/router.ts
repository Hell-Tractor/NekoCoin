import { createRouter, createWebHistory } from "vue-router";
import Home from "./pages/Home.vue";
import Tags from "./pages/Tags.vue";
import Transaction from "./pages/Transaction.vue";
import AddTransaction from "./pages/AddTransaction.vue";
import Main from "./pages/Main.vue";
import AddWallet from "./pages/AddWallet.vue";
import AddTag from "./pages/AddTag.vue";
import TagDetails from "./pages/TagDetails.vue";
import Wallets from "./pages/Wallets.vue";
import WalletDetails from "./pages/WalletDetails.vue";
import Report from "./pages/Report.vue";
import Settings from "./pages/Settings.vue";
import Onboarding from "./pages/Onboarding.vue";
import Activities from "./pages/Activities.vue";
import AddActivity from "./pages/AddActivity.vue";
import ActivityDetails from "./pages/ActivityDetails.vue";
import { load_settings, settings } from "./common/Settings";

const router = createRouter({
    history: createWebHistory(),
    routes:[
        { path: "/", redirect: "/main/home" },
        {
            path: "/main",
            component: Main,
            children: [
                { path: "home", component: Home },
                { path: "accounts", component: Wallets },
                { path: "tags", component: Tags },
                { path: "activities", component: Activities },
                { path: "transactions", component: Transaction },
                { path: "reports", component: Report },
            ],
        },
        { path: "/transaction/add", component: AddTransaction },
        { path: "/account/add", component: AddWallet },
        { path: "/tag/add", component: AddTag },
        { path: "/tag/:id", component: TagDetails, props: route => ({ id: Number(route.params.id) }) },
        { path: "/activity/add", component: AddActivity },
        { path: "/activity/:id", component: ActivityDetails, props: route => ({ id: Number(route.params.id) }) },
        { path: "/wallet/:id", component: WalletDetails, props: route => ({ id: Number(route.params.id) }) },
        { path: "/settings", component: Settings },
        { path: "/onboarding", component: Onboarding },
    ]
});

router.beforeEach(async (to) => {
    await load_settings();
    if (!settings.initialized && to.path !== '/onboarding') {
        return '/onboarding';
    }
    if (settings.initialized && to.path === '/onboarding') {
        return '/main/home';
    }
});

export default router;