use crate::core::models::runepool_units_history::RunepoolUnitsInterval;
use sqlx::MySqlPool;

pub async fn store_intervals(
    pool: &MySqlPool,
    intervals: &[RunepoolUnitsInterval],
) -> Result<(), sqlx::Error> {
    for interval in intervals {
        // Check if record exists
        let exists = sqlx::query!(
            r#"
            SELECT COUNT(*) as count 
            FROM `runepool_unit_intervals` 
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
                INSERT INTO `runepool_unit_intervals` (
                    start_time, end_time, count, units
                ) VALUES (?, ?, ?, ?)
                "#,
                interval.start_time.naive_utc(),
                interval.end_time.naive_utc(),
                interval.count as i64,
                interval.units as i64,
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
