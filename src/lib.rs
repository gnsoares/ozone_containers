use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
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

    fn __contains__(&self, py: Python<'_>, value: Py<PyAny>) -> bool {
        self.find(value.bind(py), 0, self.values.len()).is_some()
    }

    #[classattr]
    fn __hash__() -> Option<()> {
        None // type is unhashable since mutable
    }

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

    pub fn index(&self, py: Python<'_>, value: Py<PyAny>) -> PyResult<usize> {
        if let Some(idx) = self.find(value.bind(py), 0, self.values.len()) {
            Ok(idx)
        } else {
            Err(PyErr::new::<PyValueError, _>("TODO"))
        }
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

    fn find(&self, bound_value: &Bound<'_, PyAny>, beg: usize, end: usize) -> Option<usize> {
        if end == 0 || beg >= self.values.len() || beg >= end {
            return None;
        }
        match (beg + end) / 2 {
            mid if { mid < self.values.len() } => {
                if bound_value.eq(&self.values[mid]).unwrap_or(false)
                    && bound_value.is(&self.values[mid])
                {
                    Some(mid)
                } else {
                    match bound_value.lt(&self.values[mid]) {
                        Ok(true) => self.find(bound_value, beg, mid),
                        Ok(false) => self.find(bound_value, mid + 1, end),
                        Err(_) => None,
                    }
                }
            }
            _ => None,
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
mod ozone_containers {
    #[pymodule_export]
    use super::SortedList;
}
