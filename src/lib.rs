use pyo3::prelude::*;

use dict_derive::{FromPyObject, IntoPyObject};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone, FromPyObject, IntoPyObject)]
pub struct Point {
    x: f64,
    y: f64,
    z: u64,
}

#[pyfunction]
pub fn fibonacci(nth: u64) -> u64 {
    if nth <= 2 {
        return 1;
    }

    fibonacci(nth - 1) + fibonacci(nth - 2)
}

#[pyfunction]
pub fn get_coordinate(point: Point) -> PyResult<Point> {
    Ok(Point {
        x: point.x + 1.0,
        y: point.y + 1.0,
        z: point.z + 1,
    })
}

#[pyfunction]
pub fn bubble_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut nums_copy = nums.clone();
    let n = nums_copy.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if nums_copy[j] > nums_copy[j + 1] {
                nums_copy.swap(j, j + 1);
            }
        }
    }
    nums_copy
}

#[pyfunction]
fn reverse_file(input_file: &str) -> std::io::Result<()> {
    let mut file = File::open(input_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut content_bytes = content.into_bytes();
    content_bytes.reverse();

    let mut output_file = File::create(format!("{}_reversed_rs", input_file))?;
    output_file.write_all(&content_bytes)?;

    Ok(())
}

#[pymodule]
fn python_rust_ffi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(get_coordinate, m)?)?;
    m.add_function(wrap_pyfunction!(bubble_sort, m)?)?;
    m.add_function(wrap_pyfunction!(reverse_file, m)?)?;

    Ok(())
}
