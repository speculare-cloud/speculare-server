use crate::utils::PctDTORaw;

/// Compute the percentage of difference between a Vec containing two DTORaw
///
/// This give us the percentage of use of results[1] over results[0].
pub fn compute_pct(results: &[PctDTORaw]) -> f64 {
    trace!("compute_pct: results are {:?}", results);
    let mut value = 0.0;
    for result in results {
        value += result.numerator / result.divisor;
    }

    (value / results.len() as f64) * 100.0
}
