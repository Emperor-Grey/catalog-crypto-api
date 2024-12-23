use crate::model::depth_history::DepthInterval;
use sqlx::MySqlPool;

pub async fn store_intervals(
    pool: &MySqlPool,
    intervals: &[DepthInterval],
) -> Result<(), sqlx::Error> {
    for interval in intervals {
        sqlx::query!(
            r#"
            INSERT INTO intervals (
                start_time, end_time, asset_depth, asset_price, 
                asset_price_usd, liquidity_units, luvi, members_count,
                rune_depth, synth_supply, synth_units, units
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            interval.start_time.naive_utc(),
            interval.end_time.naive_utc(),
            interval.asset_depth as i64,
            interval.asset_price,
            interval.asset_price_usd,
            interval.liquidity_units as i64,
            interval.luvi,
            interval.members_count as i32,
            interval.rune_depth as i64,
            interval.synth_supply as i64,
            interval.synth_units as i64,
            interval.units as i64
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
