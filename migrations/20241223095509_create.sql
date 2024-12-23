-- Create intervals table
CREATE TABLE `intervals` (
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
-- Create meta_snapshots table
CREATE TABLE `meta_snapshots` (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    start_asset_depth BIGINT UNSIGNED NOT NULL,
    end_asset_depth BIGINT UNSIGNED NOT NULL,
    start_lp_units BIGINT UNSIGNED NOT NULL,
    end_lp_units BIGINT UNSIGNED NOT NULL,
    start_member_count INT UNSIGNED NOT NULL,
    end_member_count INT UNSIGNED NOT NULL,
    start_rune_depth BIGINT UNSIGNED NOT NULL,
    end_rune_depth BIGINT UNSIGNED NOT NULL,
    start_synth_units BIGINT UNSIGNED NOT NULL,
    end_synth_units BIGINT UNSIGNED NOT NULL,
    luvi_increase DOUBLE NOT NULL,
    price_shift_loss DOUBLE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_meta_time_range (start_time, end_time)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;