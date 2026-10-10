//! Fire number math for early retirement planning.
//!
//! A fire number is annual spending divided by a withdrawal rate, and the
//! honest version of the plan also counts the gap from your current
//! balance, the years of saving it takes to close, and what happens to
//! that timeline when the spending budget grows before you retire. These
//! functions compute each piece.

/// The balance a withdrawal rate has to support a given annual spending.
///
/// ```
/// let n = fire_number_calc::fire_number(40_000.0, 0.04);
/// assert!((n - 1_000_000.0).abs() < 1e-9);
/// ```
pub fn fire_number(annual_spending: f64, withdrawal_rate: f64) -> f64 {
    if withdrawal_rate <= 0.0 {
        return 0.0;
    }
    annual_spending / withdrawal_rate
}

/// Dollars still missing between today's balance and the fire number.
///
/// ```
/// let gap = fire_number_calc::gap_to_target(620_000.0, 1_000_000.0);
/// assert!((gap - 380_000.0).abs() < 1e-9);
/// ```
pub fn gap_to_target(current_balance: f64, target: f64) -> f64 {
    (target - current_balance).max(0.0)
}

/// Whole years until the portfolio (balance plus annual saving, compounding
/// at `return_rate`) covers the fire number. The target itself grows at
/// `spending_growth` each year, so a growing budget pushes the crossing
/// point out. Returns 0.0 if you are already there and `f64::INFINITY` if
/// the budget outruns the savings forever.
///
/// ```
/// let flat = fire_number_calc::years_to_target(620_000.0, 24_000.0, 1_000_000.0, 0.06, 0.0);
/// let grown = fire_number_calc::years_to_target(620_000.0, 24_000.0, 1_000_000.0, 0.06, 0.02);
/// assert!((flat - 6.0).abs() < 1e-9);
/// assert!((grown - 8.0).abs() < 1e-9);
/// assert!(grown > flat);
/// ```
pub fn years_to_target(
    current_balance: f64,
    annual_saving: f64,
    target: f64,
    return_rate: f64,
    spending_growth: f64,
) -> f64 {
    if current_balance >= target {
        return 0.0;
    }
    for n in 1..=600 {
        let grow = (1.0 + return_rate).powi(n);
        let bal = current_balance * grow
            + if return_rate > 0.0 {
                annual_saving * (grow - 1.0) / return_rate
            } else {
                annual_saving * n as f64
            };
        let tgt = target * (1.0 + spending_growth).powi(n);
        if bal >= tgt {
            return n as f64;
        }
    }
    f64::INFINITY
}

/// Both timelines: years to the target with a flat budget and with one that
/// grows each year until retirement. The difference is the price of the
/// growing budget in working years.
///
/// ```
/// let (flat, grown) =
///     fire_number_calc::budget_cost(620_000.0, 24_000.0, 1_000_000.0, 0.06, 0.02);
/// assert!((flat - 6.0).abs() < 1e-9);
/// assert!((grown - 8.0).abs() < 1e-9);
/// ```
pub fn budget_cost(
    current_balance: f64,
    annual_saving: f64,
    target: f64,
    return_rate: f64,
    spending_growth: f64,
) -> (f64, f64) {
    let flat = years_to_target(current_balance, annual_saving, target, return_rate, 0.0);
    let grown = years_to_target(current_balance, annual_saving, target, return_rate, spending_growth);
    (flat, grown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_percent_times_twenty_five() {
        assert!((fire_number(40_000.0, 0.04) - 1_000_000.0).abs() < 1e-9);
    }

    #[test]
    fn zero_rate_does_not_divide() {
        assert!(fire_number(40_000.0, 0.0) == 0.0);
    }

    #[test]
    fn gap_never_goes_negative() {
        assert!(gap_to_target(1_200_000.0, 1_000_000.0) == 0.0);
    }

    #[test]
    fn growing_budget_stretches_the_timeline() {
        let (flat, grown) = budget_cost(620_000.0, 24_000.0, 1_000_000.0, 0.06, 0.02);
        assert!(grown > flat);
    }

    #[test]
    fn losing_race_is_infinity() {
        let y = years_to_target(0.0, 10_000.0, 1_000_000.0, 0.01, 0.09);
        assert!(y.is_infinite());
    }
}
