-- Add migration script here
CREATE TABLE `earning_intervals` (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    avg_node_count DOUBLE NOT NULL,
    block_rewards BIGINT UNSIGNED NOT NULL,
    bonding_earnings BIGINT UNSIGNED NOT NULL,
    earnings BIGINT UNSIGNED NOT NULL,
    liquidity_earnings BIGINT UNSIGNED NOT NULL,
    liquidity_fees BIGINT UNSIGNED NOT NULL,
    rune_price_usd DOUBLE NOT NULL,
    pools JSON NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_earnings_time_range (start_time, end_time)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;