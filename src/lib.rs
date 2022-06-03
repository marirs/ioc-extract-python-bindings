use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass(dict)]
#[derive(Default)]
pub struct Artifacts {
    pub urls: Option<Vec<String>>,
    pub domains: Option<Vec<String>>,
    pub emails: Option<Vec<String>>,
    pub ip_address: Option<Vec<String>>,
    pub crypto: Option<Vec<String>>,
    pub registry_keys: Option<Vec<String>>,
    pub sql: Option<Vec<String>>,
    pub file_paths: Option<Vec<String>>,
}

impl Artifacts {
    pub fn new(a: Artifacts) -> Self {
        Self {
            urls: a.urls,
            domains: a.domains,
            emails: a.emails,
            ip_address: a.ip_address,
            crypto: a.crypto,
            registry_keys: a.registry_keys,
            sql: a.sql,
            file_paths: a.file_paths,
        }
    }
}

#[pyfunction]
fn from_str(s: &str) -> HashMap<&'static str, Option<Vec<String>>> {
    match ioc_extract::Artifacts::from_str(s) {
        Some(s) => HashMap::from([
            ("urls", s.urls),
            ("domains", s.domains),
            ("emails", s.emails),
            ("ip_address", s.ip_address),
            ("crypto", s.crypto),
            ("registry_keys", s.registry_keys),
            ("sql", s.sql),
            ("file_paths", s.file_paths),
        ]),
        None => HashMap::new(),
    }
}

#[pyfunction]
fn from_file(file_name: &str) -> PyResult<HashMap<&'static str, Option<Vec<String>>>> {
    match ioc_extract::Artifacts::from_file(file_name)? {
        Some(s) => Ok(HashMap::from([
            ("urls", s.urls),
            ("domains", s.domains),
            ("emails", s.emails),
            ("ip_address", s.ip_address),
            ("crypto", s.crypto),
            ("registry_keys", s.registry_keys),
            ("sql", s.sql),
            ("file_paths", s.file_paths),
        ])),
        None => Ok(HashMap::new()),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ioc_extract_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(from_str, m)?)?;
    m.add_function(wrap_pyfunction!(from_file, m)?)?;
    Ok(())
}
