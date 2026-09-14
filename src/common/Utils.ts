import i18n from "../i18n";
import { settings } from './Settings';
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

export const formatAmount = function(cents: number): string {
    const amount = cents / 100;
    const decimals = Math.max(0, Math.min(4, settings.decimal_places));
    return settings.thousands_separator
        ? amount.toLocaleString('en-US', { minimumFractionDigits: decimals, maximumFractionDigits: decimals })
        : amount.toFixed(decimals);
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