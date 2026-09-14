<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { Activity } from '../../common/Activity';
import { entity_accent_color, entity_avatar_style, entity_card_style, format_net_cash_flow, net_cash_flow_color } from '../../common/Utils';

const { t } = useI18n();

const props = defineProps<{
    activity: Pick<Activity, 'name' | 'remark' | 'color' | 'icon' | 'open'>,
    net?: number,
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain",
    layout?: 'list' | 'hero',
}>();

const layout = computed(() => props.layout ?? 'list');
const card_style = computed(() => entity_card_style(props.activity.color));
const avatar_style = computed(() => entity_avatar_style(props.activity.color));
const accent_color = computed(() => entity_accent_color(props.activity.color));
const net_style = computed(() => ({
    color: net_cash_flow_color(props.net ?? 0) || undefined,
}));
</script>

<template>
    <v-card :variant="variant" class="entity-card mb-2" rounded="xl" :style="card_style">
        <div v-if="layout === 'list'" class="entity-row">
            <div class="entity-avatar" :style="avatar_style">
                <v-icon :color="accent_color" size="22">{{ activity.icon }}</v-icon>
            </div>
            <div class="entity-copy">
                <div class="entity-name">{{ activity.name }}</div>
                <div class="entity-meta" :style="net !== undefined ? net_style : undefined">
                    {{ net !== undefined ? format_net_cash_flow(net) : (activity.remark || '') }}
                </div>
            </div>
            <div class="entity-aside">
                <slot name="append">
                    <v-chip size="x-small" :color="activity.open ? 'primary' : undefined" variant="tonal">
                        {{ activity.open ? t('activity.open') : t('activity.closed') }}
                    </v-chip>
                </slot>
            </div>
        </div>
        <div v-else class="entity-hero">
            <div class="entity-row">
                <div class="entity-avatar entity-avatar-lg" :style="avatar_style">
                    <v-icon :color="accent_color" size="28">{{ activity.icon }}</v-icon>
                </div>
                <div class="entity-copy">
                    <div class="entity-name">{{ activity.name }}</div>
                    <div v-if="activity.remark" class="entity-meta entity-meta-wrap">{{ activity.remark }}</div>
                </div>
            </div>
            <div v-if="$slots.extra" class="entity-extra">
                <slot name="extra" />
            </div>
        </div>
    </v-card>
</template>
