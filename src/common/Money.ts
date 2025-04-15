export class Money {
    private _amount: number;
    private _currency: Currency;

    constructor(amount: number, currency_sign: Currency) {
        this._amount = amount;
        this._currency = currency_sign;
    }

    public toString() : string {
        // format the money amount to 2 decimal places with comma separated thousands
        return `${this._currency.symbol}${(this._amount / 100).toFixed(2).replace(/\d(?=(\d{3})+\.)/g, '$&,')}`;
    }
}

export interface Currency {
    symbol: string;
    code: string;
}