import { is_dark_theme } from './Utils';

const css_theme_rgb = function(token: string, fallback: string): string {
    if (typeof document === 'undefined') {
        return fallback;
    }
    const raw = getComputedStyle(document.documentElement).getPropertyValue(token).trim();
    return raw ? `rgb(${raw})` : fallback;
};

export const apex_chart_theme = function() {
    const dark = is_dark_theme();
    const fore = css_theme_rgb('--v-theme-on-surface', dark ? '#E7ECF4' : '#3A302E');
    const muted = dark ? 'rgba(231, 236, 244, 0.22)' : 'rgba(42, 42, 42, 0.16)';
    const success = css_theme_rgb('--v-theme-success', dark ? '#6FDC9E' : '#2E8A5F');
    const error = css_theme_rgb('--v-theme-error', dark ? '#FF8A82' : '#C53B32');
    const info = css_theme_rgb('--v-theme-info', dark ? '#8BB4FF' : '#3D6FDB');
    const warning = css_theme_rgb('--v-theme-warning', dark ? '#F2C14E' : '#C45E12');
    const primary = css_theme_rgb('--v-theme-primary', dark ? '#9DC4FF' : '#C45B78');
    const secondary = css_theme_rgb('--v-theme-secondary', dark ? '#E8B86D' : '#3F8A76');
    return {
        dark,
        success,
        error,
        info,
        warning,
        primary,
        secondary,
        income_expense_pairs: [
            [success, error],
            [info, warning],
            [primary, secondary],
        ] as [string, string][],
        line_colors: [success, info, warning, primary, secondary],
        chart: {
            foreColor: fore,
            background: 'transparent',
        },
        theme: {
            mode: dark ? 'dark' : 'light',
        },
        grid: {
            borderColor: muted,
            strokeDashArray: 3,
        },
        xaxis: {
            labels: {
                style: { colors: fore },
            },
            axisBorder: { color: muted },
            axisTicks: { color: muted },
        },
        yaxis: {
            labels: {
                style: { colors: fore },
            },
        },
        legend: {
            labels: { colors: fore },
        },
        tooltip: {
            theme: dark ? 'dark' : 'light',
        },
        markers: dark
            ? { strokeColors: css_theme_rgb('--v-theme-surface', '#1C2330') }
            : {},
    };
};
