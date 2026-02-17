use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass(unsendable)]
struct SortedList {
    l: std::vec::Vec<Py<PyAny>>,
}

#[pymethods]
impl SortedList {
    #[new]
    fn new() -> Self {
        Self { l: vec![] }
    }

    pub fn add(&mut self, py: Python<'_>, value: Py<PyAny>) -> PyResult<()> {
        let index = self.bisect(value.bind(py), 0, self.l.len())?;
        self.l.insert(index, value);
        Ok(())
    }

    fn __str__(&self) -> String {
        format!(
            "[{}]",
            self.l
                .iter()
                .map(|v| v.to_string())
                .reduce(|acc, s| format!("{}, {}", acc, s))
                .unwrap_or(String::new())
        )
    }

    fn __repr__(&self) -> String {
        format!("SortedList({})", self.__str__())
    }
}

impl SortedList {
    fn bisect(&self, bound_value: &Bound<'_, PyAny>, beg: usize, end: usize) -> PyResult<usize> {
        match beg.cmp(&end) {
            std::cmp::Ordering::Equal => Ok(beg),
            std::cmp::Ordering::Greater => Err(PyErr::new::<PyRuntimeError, _>("TODO")),
            std::cmp::Ordering::Less => {
                let mid = (beg + end) / 2;
                match bound_value.lt(&self.l[mid]) {
                    Ok(true) => self.bisect(bound_value, beg, mid),
                    Ok(false) => self.bisect(bound_value, mid + 1, end),
                    Err(_) => Err(PyErr::new::<PyTypeError, _>("TODO")),
                }
            }
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
mod ozone_containers {
    #[pymodule_export]
    use super::SortedList;
}
