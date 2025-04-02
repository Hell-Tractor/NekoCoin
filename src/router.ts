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
                { path: "transactions", component: Transaction },
            ],
        },
        { path: "/transaction/add", component: AddTransaction },
        { path: "/account/add", component: AddWallet },
        { path: "/tag/add", component: AddTag },
        { path: "/tag/:id", component: TagDetails, props: route => ({ id: Number(route.params.id) }) },
        { path: "/wallet/:id", component: WalletDetails, props: route => ({ id: Number(route.params.id) }) },
    ]
});

export default router;