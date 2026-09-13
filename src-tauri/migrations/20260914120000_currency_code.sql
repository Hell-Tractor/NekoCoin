ALTER TABLE wallets RENAME COLUMN currency TO currency_code;

UPDATE wallets
SET currency_code = CASE currency_code
    WHEN '¥' THEN 'CNY'
    WHEN '$' THEN 'USD'
    WHEN '€' THEN 'EUR'
    WHEN '£' THEN 'GBP'
    WHEN '₩' THEN 'KRW'
    ELSE currency_code
END;