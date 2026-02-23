use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass(unsendable, module = "ozone_containers")]
struct SortedList {
    values: std::vec::Vec<Py<PyAny>>,
}

#[pyclass(unsendable, module = "ozone_containers")]
struct SortedListIterator {
    inner: std::vec::IntoIter<Py<PyAny>>,
}

#[pymethods]
impl SortedListIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Py<PyAny>> {
        slf.inner.next()
    }
}

#[pymethods]
impl SortedList {
    #[new]
    fn new() -> Self {
        Self { values: vec![] }
    }

    const __hash__: Option<Py<PyAny>> = None;

    fn __iter__(&self, py: Python<'_>) -> SortedListIterator {
        SortedListIterator {
            inner: self
                .values
                .iter()
                .map(|value| value.clone_ref(py))
                .collect::<std::vec::Vec<Py<PyAny>>>()
                .into_iter(),
        }
    }

    fn __str__(&self) -> String {
        format!(
            "[{}]",
            self.values
                .iter()
                .map(|v| v.to_string())
                .reduce(|acc, s| format!("{}, {}", acc, s))
                .unwrap_or(String::new())
        )
    }

    fn __repr__(&self) -> String {
        format!("SortedList({})", self.__str__())
    }

    pub fn add(&mut self, py: Python<'_>, value: Py<PyAny>) -> PyResult<()> {
        let index = self.bisect(value.bind(py), 0, self.values.len())?;
        self.values.insert(index, value);
        Ok(())
    }
}

impl SortedList {
    fn bisect(&self, bound_value: &Bound<'_, PyAny>, beg: usize, end: usize) -> PyResult<usize> {
        match beg.cmp(&end) {
            std::cmp::Ordering::Equal => Ok(beg),
            std::cmp::Ordering::Greater => Err(PyErr::new::<PyRuntimeError, _>("TODO")),
            std::cmp::Ordering::Less => {
                let mid = (beg + end) / 2;
                match bound_value.lt(&self.values[mid]) {
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
