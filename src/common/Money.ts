import { formatAmount } from './Utils';

export class Money {
    private _amount: number;
    private _currency: Currency;

    constructor(amount: number, currency: Currency) {
        this._amount = amount;
        this._currency = currency;
    }

    public getAmount(): number {
        return this._amount;
    }

    public toString() : string {
        return `${this._currency.code} ${formatAmount(this._amount)}`;
    }
}

export interface Currency {
    symbol: string;
    code: string;
}