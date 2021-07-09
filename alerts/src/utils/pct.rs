use super::PctDTORaw;

/// Compute the percentage of difference between a Vec containing two DTORaw
///
/// This give us the percentage of use of results[1] over results[0].
pub fn compute_pct(results: &[PctDTORaw]) -> f64 {
    // results must contains exactly two items.
    assert!(results.len() == 2);

    // Define temp variable
    // results[0] is the previous value in time
    // results[1] is the current value
    let (prev_div, curr_div) = (results[1].divisor, results[0].divisor);
    let (prev_num, curr_num) = (results[1].numerator, results[0].numerator);
    // Compute the delta value between both previous and current
    let total_d = (curr_div + curr_num) - (prev_div + prev_num);
    let divisor_d = curr_div - prev_div;

    // Return the computed percentage
    ((total_d - divisor_d) / total_d) * 100.0
}
