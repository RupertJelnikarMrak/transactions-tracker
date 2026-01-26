CREATE TABLE transactions (
    signature VARCHAR(88) PRIMARY KEY,
    slot BIGINT NOT NULL,
    block_time TIMESTAMPTZ NOT NULL,
    user_address VARCHAR(44) NOT NULL,
    token_mint VARCHAR(44) NOT NULL,
    amount DOUBLE PRECISION NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW()
);
