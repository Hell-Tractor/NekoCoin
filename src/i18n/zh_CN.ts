import { SpecificDateTimeFormatOptions } from "@intlify/core-base";

const datetime_short: SpecificDateTimeFormatOptions = {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
}

const datetime_long: SpecificDateTimeFormatOptions = {
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
    weekday: 'long',
    hour: 'numeric',
    minute: 'numeric',
    second: 'numeric',
}

export default {
    message: {
        welcome: '欢迎回来！',
        total_balance: '总余额',
        this_month: '本月',
    },
    datetime: {
        short: datetime_short,
        long: datetime_long
    }
}