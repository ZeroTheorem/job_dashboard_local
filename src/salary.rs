use crate::database::RevenueStats;

const SALARY_PER_DAY: i64 = 1500;
const OLD_SALARY_PER_DAY: i64 = 3000;
const SALES_PLAN_PERCENT: f64 = 0.04;

pub fn count_monthly_salary(stats: &RevenueStats) -> i64 {
    (stats.total_shifts.unwrap_or(0) * SALARY_PER_DAY)
        + (stats.total_revenue.unwrap_or(0) as f64 * SALES_PLAN_PERCENT) as i64
}

pub fn count_old_salary(stats: &RevenueStats) -> i64 {
    stats.total_shifts.unwrap_or(0) * OLD_SALARY_PER_DAY
}
