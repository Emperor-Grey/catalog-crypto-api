use crate::model::earnings_history::IntervalData;
use sqlx::MySqlPool;

pub async fn store_intervals(
    pool: &MySqlPool,
    intervals: &[IntervalData],
) -> Result<(), sqlx::Error> {
    for interval in intervals {
        sqlx::query!(
            r#"
            INSERT INTO `earning_intervals` (
                start_time, end_time, avg_node_count, block_rewards,
                bonding_earnings, earnings, liquidity_earnings,
                liquidity_fees, rune_price_usd
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            interval.start_time.naive_utc(),
            interval.end_time.naive_utc(),
            interval.avg_node_count,
            interval.block_rewards as i64,
            interval.bonding_earnings as i64,
            interval.earnings as i64,
            interval.liquidity_earnings as i64,
            interval.liquidity_fees as i64,
            interval.rune_price_usd,
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
