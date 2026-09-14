<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { Activity } from '../../common/Activity';
import { color_with_alpha, format_net_cash_flow, net_cash_flow_color } from '../../common/Utils';

const { t } = useI18n();

const props = defineProps<{
    activity: Pick<Activity, 'name' | 'remark' | 'color' | 'icon' | 'open'>,
    net?: number,
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain",
    layout?: 'list' | 'hero',
}>();

const layout = computed(() => props.layout ?? 'list');
const card_style = computed(() => ({
    background: `linear-gradient(135deg, ${color_with_alpha(props.activity.color, 0.22)} 0%, transparent 58%)`,
}));
const avatar_style = computed(() => ({
    backgroundColor: color_with_alpha(props.activity.color, 0.22),
    color: props.activity.color,
}));
const net_style = computed(() => ({
    color: net_cash_flow_color(props.net ?? 0) || undefined,
}));
</script>

<template>
    <v-card :variant="variant" class="entity-card mb-2" rounded="xl" :style="card_style">
        <div v-if="layout === 'list'" class="entity-row">
            <div class="entity-avatar" :style="avatar_style">
                <v-icon :color="activity.color" size="22">{{ activity.icon }}</v-icon>
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
                    <v-icon :color="activity.color" size="28">{{ activity.icon }}</v-icon>
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

<style scoped>
.entity-card {
    overflow: hidden;
}

.entity-row {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
    padding: 14px 16px;
}

.entity-hero {
    padding: 4px 0 8px;
}

.entity-avatar {
    display: flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: center;
    width: 42px;
    height: 42px;
    border-radius: 14px;
}

.entity-avatar-lg {
    width: 52px;
    height: 52px;
    border-radius: 16px;
}

.entity-copy {
    min-width: 0;
    flex: 1 1 auto;
}

.entity-name {
    overflow: hidden;
    font-size: 1rem;
    font-weight: 600;
    line-height: 1.3;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.entity-meta {
    overflow: hidden;
    margin-top: 2px;
    color: rgba(var(--v-theme-on-surface), 0.58);
    font-size: 0.75rem;
    line-height: 1.3;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.entity-meta-wrap {
    white-space: normal;
}

.entity-aside {
    display: flex;
    flex: 0 0 auto;
    align-items: center;
}

.entity-extra {
    padding: 0 16px 8px;
}
</style>
