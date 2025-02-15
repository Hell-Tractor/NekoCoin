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
        welcome: 'Welcome back!',
        total_balance: 'Total Balance',
        this_month: 'This Month',
    },
    datetime: {
        short: datetime_short,
        long: datetime_long
    }
}