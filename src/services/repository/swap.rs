use crate::core::models::swap_history::SwapInterval;
use sqlx::MySqlPool;

pub async fn store_intervals(
    pool: &MySqlPool,
    intervals: &[SwapInterval],
) -> Result<(), sqlx::Error> {
    for interval in intervals {
        // Check if record exists
        let exists = sqlx::query!(
            r#"
            SELECT COUNT(*) as count 
            FROM `swap_intervals` 
            WHERE start_time = ? AND end_time = ?
            "#,
            interval.start_time.naive_utc(),
            interval.end_time.naive_utc()
        )
        .fetch_one(pool)
        .await?
        .count
            > 0;

        if !exists {
            sqlx::query!(
                r#"
                INSERT INTO `swap_intervals` (
                    start_time, end_time, average_slip, from_trade_average_slip,
                    from_trade_count, from_trade_fees, from_trade_volume,
                    from_trade_volume_usd, rune_price_usd, synth_mint_average_slip,
                    synth_mint_count, synth_mint_fees, synth_mint_volume,
                    synth_mint_volume_usd, synth_redeem_average_slip, synth_redeem_count,
                    synth_redeem_fees, synth_redeem_volume, synth_redeem_volume_usd,
                    to_asset_average_slip, to_asset_count, to_asset_fees,
                    to_asset_volume, to_asset_volume_usd, to_rune_average_slip,
                    to_rune_count, to_rune_fees, to_rune_volume, to_rune_volume_usd,
                    to_trade_average_slip, to_trade_count, to_trade_fees,
                    to_trade_volume, to_trade_volume_usd, total_count, total_fees,
                    total_volume, total_volume_usd
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                interval.start_time.naive_utc(),
                interval.end_time.naive_utc(),
                interval.average_slip,
                interval.from_trade_average_slip,
                interval.from_trade_count as i64,
                interval.from_trade_fees as i64,
                interval.from_trade_volume as i64,
                interval.from_trade_volume_usd as i64,
                interval.rune_price_usd,
                interval.synth_mint_average_slip,
                interval.synth_mint_count as i64,
                interval.synth_mint_fees as i64,
                interval.synth_mint_volume as i64,
                interval.synth_mint_volume_usd as i64,
                interval.synth_redeem_average_slip,
                interval.synth_redeem_count as i64,
                interval.synth_redeem_fees as i64,
                interval.synth_redeem_volume as i64,
                interval.synth_redeem_volume_usd as i64,
                interval.to_asset_average_slip,
                interval.to_asset_count as i64,
                interval.to_asset_fees as i64,
                interval.to_asset_volume as i64,
                interval.to_asset_volume_usd as i64,
                interval.to_rune_average_slip,
                interval.to_rune_count as i64,
                interval.to_rune_fees as i64,
                interval.to_rune_volume as i64,
                interval.to_rune_volume_usd as i64,
                interval.to_trade_average_slip,
                interval.to_trade_count as i64,
                interval.to_trade_fees as i64,
                interval.to_trade_volume as i64,
                interval.to_trade_volume_usd as i64,
                interval.total_count as i64,
                interval.total_fees as i64,
                interval.total_volume as i64,
                interval.total_volume_usd as i64,
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
