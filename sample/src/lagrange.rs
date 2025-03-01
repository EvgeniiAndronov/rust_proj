pub fn lagrange_interpolation(x: f64, x_c: &[f64], y_c: &[f64]) -> f64 {
    let mut result = 0.0;
    let count_c = x_c.len();

    for i in 0..count_c {
        let mut term = y_c[i];
        for j in 0..count_c {
            if j != i {
                term *= (x - x_c[j]) / (x_c[i] - x_c[j]);
            }
        }
        result += term;
    }
    result
}