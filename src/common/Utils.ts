import i18n from "../i18n";
import { privacy_mode, settings } from './Settings';
import { is_theme_dark } from '../themes';
const t = i18n.global.t;

export const getRandomColor = function(type: 'rgb' | 'rgba') : string {
    const r = Math.floor(Math.random() * 256);
    const g = Math.floor(Math.random() * 256);
    const b = Math.floor(Math.random() * 256);
    if (type === 'rgb') {
        return `rgb(${r},${g},${b})`;
    } else {
        const a = Math.random().toFixed(1);
        return `rgba(${r},${g},${b},${a})`;
    }
}

export const formatDatetime = function(naive_date: Date) : string {
    // format: %Y-%m-%d %H:%M:%S
    const year = naive_date.getFullYear();
    const month = (naive_date.getMonth() + 1).toString().padStart(2, '0');
    const date = naive_date.getDate().toString().padStart(2, '0');
    const hour = naive_date.getHours().toString().padStart(2, '0');
    const minute = naive_date.getMinutes().toString().padStart(2, '0');
    const second = naive_date.getSeconds().toString().padStart(2, '0');
    return `${year}-${month}-${date} ${hour}:${minute}:${second}`;
}

export const formatDate = function(naive_date: Date) : string {
    // format: %Y-%m-%d
    const year = naive_date.getFullYear();
    const month = (naive_date.getMonth() + 1).toString().padStart(2, '0');
    const date = naive_date.getDate().toString().padStart(2, '0');
    return `${year}-${month}-${date}`;
}

export const formatDisplayDate = function(date: Date): string {
    const year = date.getFullYear().toString();
    const month = (date.getMonth() + 1).toString().padStart(2, '0');
    const day = date.getDate().toString().padStart(2, '0');
    if (settings.date_format === 'DD/MM/YYYY') {
        return `${day}/${month}/${year}`;
    }
    if (settings.date_format === 'MM/DD/YYYY') {
        return `${month}/${day}/${year}`;
    }
    return `${year}-${month}-${day}`;
}

export const HIDDEN_AMOUNT = '••••';

export const formatAmount = function(cents: number): string {
    const amount = cents / 100;
    const decimals = Math.max(0, Math.min(4, settings.decimal_places));
    return settings.thousands_separator
        ? amount.toLocaleString('en-US', { minimumFractionDigits: decimals, maximumFractionDigits: decimals })
        : amount.toFixed(decimals);
}

export const display_amount = function(cents: number, currency_code?: string, options?: { signed?: boolean }): string {
    if (privacy_mode.value) {
        return currency_code ? `${currency_code} ${HIDDEN_AMOUNT}` : HIDDEN_AMOUNT;
    }
    const prefix = options?.signed && cents > 0 ? '+' : '';
    const number = formatAmount(cents);
    return currency_code ? `${currency_code} ${prefix}${number}` : `${prefix}${number}`;
}

export const format_chart_number = function(value: number, digits = 0): string {
    if (privacy_mode.value) {
        return HIDDEN_AMOUNT;
    }
    return value.toFixed(digits);
}

export const format_chart_axis = function(value: number): string {
    if (privacy_mode.value) {
        return HIDDEN_AMOUNT;
    }
    const signs = ['', 'K', 'M', 'B', 'T'];
    if (value === 0) {
        return '0';
    }
    const index = Math.floor(Math.log10(Math.abs(value)) / 3);
    if (index < 0 || index > signs.length - 1) {
        return value.toString();
    }
    return `${(value / Math.pow(10, index * 3)).toFixed(0)}${signs[index]}`;
}

export const formatDatetimeRelative = function(date: Date, relative_date: Date) : string {
    // convert to: today, yesterday
    //             month, date
    //             year, month, date
    const oneDay = 24 * 60 * 60 * 1000;
    const diffDays = Math.floor((relative_date.getTime() - date.getTime()) / oneDay);

    if (diffDays === 0 && date.getDate() == relative_date.getDate()) {
        return t('date.today');
    } else if (diffDays <= 1) {
        return t('date.yesterday');
    } else {
        return formatDisplayDate(date);
    }
}

export const format_bytes = function(bytes: number): string {
    if (bytes < 1024) {
        return `${bytes} B`;
    }
    if (bytes < 1024 * 1024) {
        return `${(bytes / 1024).toFixed(1)} KB`;
    }
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export const formatTime = function(date: Date) : string {
    if (settings.time_format === '12hr') {
        return date.toLocaleTimeString(i18n.global.locale.value, {
            hour: 'numeric',
            minute: '2-digit',
            hour12: true,
        });
    }
    const hour = date.getHours().toString().padStart(2, '0');
    const minute = date.getMinutes().toString().padStart(2, '0');
    return `${hour}:${minute}`;
}

export const net_cash_flow_color = function(cents: number): string {
    if (cents > 0) {
        return 'rgb(var(--v-theme-success))';
    }
    if (cents < 0) {
        return 'rgb(var(--v-theme-error))';
    }
    return '';
}

export const flow_color_for_tag = function(type: string): string {
    if (type === 'Income') {
        return 'rgb(var(--v-theme-success))';
    }
    if (type === 'Expense') {
        return 'rgb(var(--v-theme-error))';
    }
    return 'rgb(var(--v-theme-info))';
}

interface Rgb {
    r: number;
    g: number;
    b: number;
}

export const is_dark_theme = function(): boolean {
    return is_theme_dark(settings.theme);
}

const parse_color_rgb = function(color: string): Rgb | null {
    const value = color.trim();
    if (value.startsWith('#')) {
        const hex = value.slice(1);
        const full = hex.length === 3 ? hex.split('').map(part => part + part).join('') : hex;
        if (full.length < 6) {
            return null;
        }
        const r = parseInt(full.slice(0, 2), 16);
        const g = parseInt(full.slice(2, 4), 16);
        const b = parseInt(full.slice(4, 6), 16);
        if ([r, g, b].some(channel => Number.isNaN(channel))) {
            return null;
        }
        return { r, g, b };
    }
    const channels = value.match(/\d+/g);
    if (channels && channels.length >= 3) {
        return { r: Number(channels[0]), g: Number(channels[1]), b: Number(channels[2]) };
    }
    return null;
}

const rgb_luminance = function({ r, g, b }: Rgb): number {
    const to_linear = function(channel: number) {
        const value = channel / 255;
        return value <= 0.03928 ? value / 12.92 : Math.pow((value + 0.055) / 1.055, 2.4);
    };
    return 0.2126 * to_linear(r) + 0.7152 * to_linear(g) + 0.0722 * to_linear(b);
}

const mix_rgb = function(from: Rgb, to: Rgb, amount: number): Rgb {
    return {
        r: Math.round(from.r + (to.r - from.r) * amount),
        g: Math.round(from.g + (to.g - from.g) * amount),
        b: Math.round(from.b + (to.b - from.b) * amount),
    };
}

const rgb_to_css = function({ r, g, b }: Rgb): string {
    return `rgb(${r}, ${g}, ${b})`;
}

export const color_with_alpha = function(color: string, alpha: number): string {
    const rgb = parse_color_rgb(color);
    if (!rgb) {
        return color;
    }
    return `rgba(${rgb.r}, ${rgb.g}, ${rgb.b}, ${alpha})`;
}

export const entity_accent_color = function(color: string): string {
    if (!is_dark_theme()) {
        return color;
    }
    const rgb = parse_color_rgb(color);
    if (!rgb) {
        return color;
    }
    const luminance = rgb_luminance(rgb);
    if (luminance >= 0.42) {
        return color;
    }
    const amount = Math.min(0.78, 0.4 + (0.42 - luminance) * 0.95);
    return rgb_to_css(mix_rgb(rgb, { r: 255, g: 255, b: 255 }, amount));
}

export const entity_tint = function(color: string): string {
    const accent = entity_accent_color(color);
    return color_with_alpha(accent, is_dark_theme() ? 0.28 : 0.18);
}

export const entity_ink_color = function(color: string): string {
    const rgb = parse_color_rgb(color);
    if (!rgb) {
        return color;
    }
    if (is_dark_theme()) {
        return entity_accent_color(color);
    }
    const luminance = rgb_luminance(rgb);
    if (luminance < 0.45) {
        return color;
    }
    const amount = Math.min(0.74, 0.38 + (luminance - 0.45) * 1.15);
    return rgb_to_css(mix_rgb(rgb, { r: 36, g: 30, b: 28 }, amount));
}

export const entity_chip_style = function(color: string, selected: boolean): Record<string, string> {
    const accent = entity_accent_color(color);
    if (is_dark_theme()) {
        if (selected) {
            return { borderWidth: '1px', borderColor: accent };
        }
        return { backgroundColor: entity_tint(color) };
    }
    const ink = entity_ink_color(color);
    const style: Record<string, string> = {
        backgroundColor: color_with_alpha(accent, selected ? 0.34 : 0.24),
        color: ink,
    };
    if (selected) {
        style.outline = `1px solid ${accent}`;
    }
    return style;
}

export const entity_avatar_style = function(color: string): Record<string, string> {
    const accent = entity_accent_color(color);
    const style: Record<string, string> = {
        backgroundColor: color_with_alpha(accent, is_dark_theme() ? 0.24 : 0.18),
    };
    if (is_dark_theme()) {
        style.boxShadow = `inset 0 0 0 1px ${color_with_alpha(accent, 0.55)}`;
    }
    return style;
}

export const entity_card_style = function(color: string): Record<string, string> {
    const accent = entity_accent_color(color);
    if (is_dark_theme()) {
        return {
            background: `linear-gradient(105deg, ${color_with_alpha(accent, 0.3)} 0%, ${color_with_alpha(accent, 0.1)} 40%, rgba(255, 255, 255, 0.03) 100%)`,
            boxShadow: `inset 3px 0 0 ${accent}`,
        };
    }
    return {
        background: `linear-gradient(135deg, ${color_with_alpha(color, 0.18)} 0%, transparent 58%)`,
    };
}

export const entity_card_background = function(color: string): string {
    return entity_card_style(color).background ?? '';
}