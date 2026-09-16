<script setup lang="ts">
import { Ref, ref } from 'vue';
import BackTitleBar from './BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const emits = defineEmits<{
    back: []
    confirm: [string]
}>();

interface IconGroup {
    name: string;
    icons: string[];
}

const selected_icon: Ref<string | undefined> = ref(undefined);

const iconGroups: IconGroup[] = [
    {
        name: 'icon.foodAndDrink',
        icons: [
            'mdi-baguette',
            'mdi-beer',
            'mdi-cupcake',
            'mdi-candy',
            'mdi-coffee',
            'mdi-cookie',
            'mdi-apple',
            'mdi-food',
            'mdi-food-fork-drink',
            'mdi-fruit-watermelon',
            'mdi-hamburger',
            'mdi-ice-cream',
            'mdi-water',
            'mdi-tea',
            'mdi-glass-wine'
        ]
    }, {
        name: 'icon.dailyLife',
        icons: [
            'mdi-broom',
            'mdi-calendar',
            'mdi-clock',
            'mdi-home',
            'mdi-lightbulb',
            'mdi-lock',
            'mdi-map-marker',
            'mdi-phone',
            'mdi-shield',
            'mdi-sleep',
            'mdi-sofa',
            'mdi-wifi',
            'mdi-lightning-bolt',
            'mdi-paper-roll',
            'mdi-bathtub',
        ]
    }, {
        name: 'icon.shopping',
        icons: [
            'mdi-cart',
            'mdi-credit-card',
            'mdi-gift',
            'mdi-shopping',
            'mdi-store',
            'mdi-tag',
            'mdi-wallet',
            'mdi-tshirt-crew',
            'mdi-shoe-sneaker',
            'mdi-purse',
            'mdi-watch',
        ]
    }, {
        name: 'icon.bank',
        icons: [
            'mdi-bank',
            'mdi-cash',
            'mdi-currency-usd',
            'mdi-piggy-bank',
            'mdi-sack',
            'mdi-sack-percent'
        ]
    }, {
        name: 'icon.transportation',
        icons: [
            'mdi-bicycle',
            'mdi-bus',
            'mdi-car',
            'mdi-motorbike',
            'mdi-airplane',
            'mdi-train'
        ]
    }, {
        name: 'icon.travel',
        icons: [
            'mdi-beach',
            'mdi-campfire',
            'mdi-caravan',
            'mdi-hiking',
            'mdi-compass',
            'mdi-map',
            'mdi-airballoon',
        ]
    }, {
        name: 'icon.education',
        icons: [
            'mdi-book',
            'mdi-pen',
            'mdi-pencil',
            'mdi-library',
            'mdi-school'
        ]
    }, {
        name: 'icon.pets',
        icons: [
            'mdi-cat',
            'mdi-dog',
            'mdi-fish',
            'mdi-paw',
        ]
    }, {
        name: 'icon.hobbies',
        icons: [
            'mdi-artstation',
            'mdi-camera',
            'mdi-gamepad-variant',
            'mdi-guitar-electric',
            'mdi-headphones',
            'mdi-movie',
            'mdi-palette',
            'mdi-piano',
        ]
    }, {
        name: 'icon.gifts',
        icons: [
            'mdi-balloon',
            'mdi-cake',
            'mdi-gift',
            'mdi-heart',
            'mdi-flower'
        ]
    }, {
        name: "icon.health",
        icons: [
            'mdi-bandage',
            'mdi-blood-bag',
            'mdi-hospital-building',
            'mdi-medical-bag',
            'mdi-stethoscope'
        ]
    }, {
        name: "icon.sports",
        icons: [
            'mdi-baseball-bat',
            'mdi-basketball',
            'mdi-football',
            'mdi-golf',
            'mdi-hockey-sticks',
            'mdi-soccer',
            'mdi-table-tennis',
            'mdi-tennis-ball',
            'mdi-sail-boat',
            'mdi-ski',
        ]
    }
]
</script>

<template>
    <BackTitleBar :title="t('icon.select')" @back="emits('back')"></BackTitleBar>
    <v-main class="main form-page">
        <v-card v-for="group in iconGroups" :key="group.name" class="mx-2 my-2" variant="flat">
            <v-card-text>
                <div>{{ t(group.name) }}</div>
                <v-item-group class="d-flex flex-wrap" v-model="selected_icon" mandatory>
                    <v-item v-for="icon in group.icons" :key="icon" :value="icon" v-slot="{ isSelected, toggle }">
                        <v-btn variant="text" size="large" :icon="icon" @click="toggle" :active="isSelected"></v-btn>
                    </v-item>
                </v-item-group>
            </v-card-text>
        </v-card>
        <v-btn color="primary" class="form-save-btn" :disabled="!selected_icon" @click="emits('confirm', selected_icon!); emits('back');">{{ t('actions.save') }}</v-btn>
    </v-main>
</template>