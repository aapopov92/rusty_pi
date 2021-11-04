use pyo3::prelude::*;
use rand::distributions::{Distribution, Uniform};

/// Estimate Pi using Monte-Carlo method
#[pyfunction]
fn estimate(mut iterations: usize) -> PyResult<f64> {
    let mut dots_in_circle: i64 = 0;
    let mut dots_total: i64 = 0;

    let mut range = rand::thread_rng();
    let generator = Uniform::from(0.0..1.0);

    loop{
        if iterations == 0 {
            break;
        }
        let point1 = generator.sample(&mut range);
        let point2 = generator.sample(&mut range);

        if (point1 * point1 + point2 * point2) <= 1.0 {
            dots_in_circle += 1;
        }
        dots_total += 1;
        iterations -= 1;
    }

    Ok(4.0 * (dots_in_circle as f64 / dots_total as f64))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rusty_pi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(estimate, m)?)?;

    Ok(())
}
