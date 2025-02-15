export class Money {
    private _amount: number;
    private _currency: string;

    constructor(amount: number, currency_sign: string) {
        this._amount = amount;
        this._currency = currency_sign;
    }

    public toString() : string {
        // format the money amount to 2 decimal places with comma separated thousands
        return `${this._currency}${this._amount.toFixed(2).replace(/\d(?=(\d{3})+\.)/g, '$&,')}`;
    }
}