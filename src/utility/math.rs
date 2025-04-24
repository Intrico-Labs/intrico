/// Round off to nearest relevant value or to 8 decimal places
pub fn round_if_close(val: f64, tol: f64) -> f64 {
    let candidates = [0.0, 0.5, -0.5, 1.0, -1.0];
    for &cand in &candidates {
        if (val - cand).abs() < tol {
            return cand;
        }
    }
    // Round to 8 decimal places
    (val * 1e8).round() / 1e8
}