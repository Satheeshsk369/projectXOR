use pyo3::prelude::*;

#[pyfunction]
fn angle_to_repr(angle: f64) -> u64 {
    // generator = 360 / (2^64 -1)
    // repr = angle / generator
    (angle / 1.951563910473908e-17) as u64
}

#[pyfunction]
fn repr_to_angle(repr: u64) -> f64 {
    // generator = 360 / (2^64 -1)
    // angle = repr * generator
    (repr as f64) * 1.951563910473908e-17
}

#[pyfunction]
fn shift_range(angle: f64, start: f64, end: f64) -> f64 {
    let x = angle_to_repr(start);
    let y = angle_to_repr(end);
    let val = angle_to_repr(angle);
    let z = (val & (y - x)) + x;
    repr_to_angle(z)
}

#[pyfunction]
fn xor(angle1: f64, angle2: f64) -> f64 {
    let a = angle_to_repr(angle1);
    let b = angle_to_repr(angle2);
    repr_to_angle(a ^ b)
}

#[pyfunction]
fn generate(row: Vec<f64>, col: Vec<f64>) -> Vec<Vec<f64>> {
    let n = row.len();
    let mut val = vec![vec![0.0; n]; n];

    val[0] = row;
    for i in 0..n {
        val[i][0] = col[i];
    }
    
    for i in 1..n {
        for j in 1..n {
            val[i][j] = xor(val[i - 1][j], val[i][j - 1])
        }
    }

    val
}

#[pymodule]
fn algo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(angle_to_repr, m)?)?;
    m.add_function(wrap_pyfunction!(repr_to_angle, m)?)?;
    m.add_function(wrap_pyfunction!(shift_range, m)?)?;
    m.add_function(wrap_pyfunction!(xor, m)?)?;
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    Ok(())
}
