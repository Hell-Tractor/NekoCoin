export default {
    app_name: 'NekoCoin',
    welcome: 'Welcome back!',
    total_balance: 'Total Balance',
    this_month: 'This Month',
    income: 'Income',
    expense: 'Expense',
    WIP: 'Work in Progress...',
    select_icon: 'Select Icon',
    save: 'Save',
    confirm: 'Confirm',
    cancel: 'Cancel',
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
    },
    account: {
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
    },
    tag: {
        add: 'Add Tag',
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
            income: 'Income',
            expense: 'Expense',
            transfer: 'Transfer',
        }
    },
    transaction: {
        add: 'Add Transaction',
        datetime: 'Date & Time',
        enter: {
            remark: 'Enter Remark',
            amount: 'Enter Amount',
        },
        list: {
            title: 'Recent transactions',
        }
    },
    list: {
        no_more_data: 'No more data',
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
    }
}