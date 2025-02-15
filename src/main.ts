import { createApp } from "vue";
import App from "./App.vue";

// i18n
import i18n from "./i18n";

// Vuetify
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import theme from "./themes";

const vuetify = createVuetify({
    components,
    directives,
    icons: {
        defaultSet: 'mdi'
    },
    theme
});

createApp(App)
    .use(vuetify)
    .use(i18n)
    .mount("#app");
