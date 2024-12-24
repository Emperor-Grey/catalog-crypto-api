-- Create intervals table
CREATE TABLE `depth_intervals` (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    asset_depth BIGINT UNSIGNED NOT NULL,
    asset_price DOUBLE NOT NULL,
    asset_price_usd DOUBLE NOT NULL,
    liquidity_units BIGINT UNSIGNED NOT NULL,
    luvi DOUBLE NOT NULL,
    members_count INT UNSIGNED NOT NULL,
    rune_depth BIGINT UNSIGNED NOT NULL,
    synth_supply BIGINT UNSIGNED NOT NULL,
    synth_units BIGINT UNSIGNED NOT NULL,
    units BIGINT UNSIGNED NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_time_range (start_time, end_time)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;