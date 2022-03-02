use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod fib_calcs;

use fib_calcs::fib_number::__pyo3_get_function_fibonacci_number;
use fib_calcs::fib_numbers::__pyo3_get_function_fibonacci_numbers;
//pub mod fib_numbers;

mod interface;
use interface::config::__pyo3_get_function_run_config;

use interface::object::__pyo3_get_function_object_interface;

mod class_module;
use class_module::fib_processor::FibProcessor;

#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust!");
}

#[pymodule]
fn flitton_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello));
    m.add_wrapped(wrap_pyfunction!(fibonacci_number));
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers));
    m.add_wrapped(wrap_pyfunction!(run_config));
    m.add_wrapped(wrap_pyfunction!(object_interface));
    m.add_class::<FibProcessor>()?;
    Ok(())
}

