-- Add migration script here
CREATE TABLE `swap_intervals` (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    average_slip DOUBLE NOT NULL,
    from_trade_average_slip DOUBLE NOT NULL,
    from_trade_count BIGINT UNSIGNED NOT NULL,
    from_trade_fees BIGINT UNSIGNED NOT NULL,
    from_trade_volume BIGINT UNSIGNED NOT NULL,
    from_trade_volume_usd BIGINT UNSIGNED NOT NULL,
    rune_price_usd DOUBLE NOT NULL,
    synth_mint_average_slip DOUBLE NOT NULL,
    synth_mint_count BIGINT UNSIGNED NOT NULL,
    synth_mint_fees BIGINT UNSIGNED NOT NULL,
    synth_mint_volume BIGINT UNSIGNED NOT NULL,
    synth_mint_volume_usd BIGINT UNSIGNED NOT NULL,
    synth_redeem_average_slip DOUBLE NOT NULL,
    synth_redeem_count BIGINT UNSIGNED NOT NULL,
    synth_redeem_fees BIGINT UNSIGNED NOT NULL,
    synth_redeem_volume BIGINT UNSIGNED NOT NULL,
    synth_redeem_volume_usd BIGINT UNSIGNED NOT NULL,
    to_asset_average_slip DOUBLE NOT NULL,
    to_asset_count BIGINT UNSIGNED NOT NULL,
    to_asset_fees BIGINT UNSIGNED NOT NULL,
    to_asset_volume BIGINT UNSIGNED NOT NULL,
    to_asset_volume_usd BIGINT UNSIGNED NOT NULL,
    to_rune_average_slip DOUBLE NOT NULL,
    to_rune_count BIGINT UNSIGNED NOT NULL,
    to_rune_fees BIGINT UNSIGNED NOT NULL,
    to_rune_volume BIGINT UNSIGNED NOT NULL,
    to_rune_volume_usd BIGINT UNSIGNED NOT NULL,
    to_trade_average_slip DOUBLE NOT NULL,
    to_trade_count BIGINT UNSIGNED NOT NULL,
    to_trade_fees BIGINT UNSIGNED NOT NULL,
    to_trade_volume BIGINT UNSIGNED NOT NULL,
    to_trade_volume_usd BIGINT UNSIGNED NOT NULL,
    total_count BIGINT UNSIGNED NOT NULL,
    total_fees BIGINT UNSIGNED NOT NULL,
    total_volume BIGINT UNSIGNED NOT NULL,
    total_volume_usd BIGINT UNSIGNED NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_swap_time_range (start_time, end_time)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;