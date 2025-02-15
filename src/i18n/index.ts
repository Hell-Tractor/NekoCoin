import { createI18n } from "vue-i18n";
import { en as en_vuetify, zhHans as zh_cn_vuetify } from "vuetify/locale";
import en_US from "./en_US.ts";
import zh_cn from "./zh_CN.ts";

const i18n = createI18n({
    legacy: false,
    locale: "zh-CN",
    fallbackLocale: 'en',
    messages: {
        'en-US': { ...en_vuetify, ...en_US.message },
        'zh-CN': { ...zh_cn_vuetify, ...zh_cn.message },
    },
    datetimeFormats: {
        'en-US': en_US.datetime,
        'zh-CN': zh_cn.datetime,
    }
});

export default i18n;