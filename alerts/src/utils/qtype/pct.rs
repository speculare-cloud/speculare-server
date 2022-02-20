use crate::utils::PctDTORaw;

/// Compute the percentage of difference between a Vec containing two DTORaw
///
/// This give us the percentage of use of results[1] over results[0].
pub fn compute_pct(results: &[PctDTORaw]) -> f64 {
    // Define temp variable
    // results[0] is the previous value in time
    // results[1] is the current value
    let (prev_div, curr_div) = (results[1].divisor, results[0].divisor);
    let (prev_num, curr_num) = (results[1].numerator, results[0].numerator);

    let prev_val = prev_num / prev_div;
    let curr_val = curr_num / curr_div;

    // Return the computed percentage
    ((prev_val + curr_val) / 2.0) * 100.0
}
