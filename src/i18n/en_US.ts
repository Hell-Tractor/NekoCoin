export default {
    app_name: 'NekoCoin',
    welcome: 'Welcome back!',
    total_balance: 'Total Balance',
    this_month: 'This Month',
    income: 'Income',
    expense: 'Expense',
    WIP: 'Work in Progress...',
    loading: 'Loading...',
    actions: {
        save: 'Save',
        confirm: 'Confirm',
        cancel: 'Cancel',
        edit: 'Edit',
        delete: 'Delete',
        copy: 'Copy',
    },
    page: {
        home: 'Home',
        transactions: 'Transactions',
        reports: 'Reports',
        accounts: 'Accounts',
        tags: 'Tags',
        settings: 'Settings',
    },
    validation: {
        required: 'Required',
        isValidMoney: 'Not a valid amount.(Up to 2 decimal places)',
        maxLength: 'At most {max} characters',
        isValidSearchText: 'Only letters, numbers, and Chinese characters are allowed',
        min: 'At least {min}',
        max: 'At most {max}',
    },
    account: {
        update: 'Update Account',
        add: 'Add Account',
        select: 'Select Account',
        select_from: 'Transfer account from',
        select_to: 'Transfer account to',
        enter: {
            name: 'Enter Account Name',
            amount: 'Enter Amount',
            remark: 'Enter Remark',
        },
        no_account: 'No Account',
        no_account_tip: 'Click the button in the bottom right corner to add an account!',
        details: 'Account Details',
    },
    tag: {
        add: 'Add Tag',
        update: 'Update Tag',
        select: 'Select Tag',
        no_available_parent: 'No available parent tag',
        enter: {
            name: 'Enter Tag Name',
            remark: 'Enter Remark',
            parent_tag: 'Select Parent Tag(Optional)',
        },
        search: {
            hint: 'Search...',
            count: 'Found {count} tags',
        },
        type: {
            Income: 'Income',
            Expense: 'Expense',
            Transfer: 'Transfer',
        }
    },
    transaction: {
        add: 'Add Transaction',
        update: 'Update Transaction',
        datetime: 'Date & Time',
        remark: 'Remark',
        account: 'Account',
        has_split: 'Split Bill?',
        split: {
            title: 'Split Bill',
            count: 'Number of Splits(Including yourself)',
            your: 'Your Split',
            other: 'Others each: {each}, Total: {total}',
            select_wallet: 'Select Recieve Wallet',
            people: '{count} people',
        },
        enter: {
            remark: 'Enter Remark',
            amount: 'Enter Amount',
        },
        list: {
            title: 'Recent transactions',
        }
    },
    list: {
        summary: 'Found {0} records',
    },
    date: {
        today: 'Today',
        yesterday: 'Yesterday',
    },
    icon: {
        select: 'Select Icon',
        foodAndDrink: 'Food & Drink',
        dailyLife: 'Daily Life',
        shopping: 'Shopping',
        bank: 'Bank',
        transportation: 'Transportation',
        travel: 'Travel',
        education: 'Education',
        pets: 'Pets',
        hobbies: 'Hobbies',
        gifts: 'Gifts',
        health: 'Health',
        sports: 'Sports',
    },
    warning: {
        irrevertible: {
            title: 'Warning',
            content: 'This action is IRREVERTIBLE, confirm to proceed?',
        },
        cascade_and_irrevertible: {
            title: 'Warning',
            content: 'This action is IRREVERTIBLE and will also delete ALL RELATED data, confirm to proceed?',
        },
    }
}