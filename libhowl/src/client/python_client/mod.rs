use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use pyo3::prelude::*;
use pyo3::types::PyString;
use url::Url;
use crate::structs::InitCommandType;
use crate::client::native_client::Client as NativeClient;

#[pyclass]
struct Client {
    native_client: Arc<RwLock<NativeClient>>
}

#[pymethods]
impl Client {
    #[new]
    fn new(r#type: String) -> Self {
        let r#type = InitCommandType::from_str(&r#type).unwrap();
        Self {
            native_client: Arc::new(RwLock::new(NativeClient::new(r#type))),
        }
    }

    fn connect<'p>(&self, py: Python<'p>, addr: &'p PyAny) -> PyResult<&'p PyAny> {
        let addr: String = addr.extract()?;
        let native_client = self.native_client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            native_client.write().await.connect(Url::from_str(&addr).unwrap()).await;
            Python::with_gil(|py| Ok(py.None()))
        })
    }

    fn share_data<'p>(&self, py: Python<'p>, json: &'p PyString) -> PyResult<&'p PyAny> {
        let json: String = json.extract()?;
        let native_client = self.native_client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            native_client.read().await.share_data(serde_json::from_str(&json).unwrap()).await;
            Python::with_gil(|py| Ok(py.None()))
        })
    }
}

#[pymodule]
fn libhowl(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>().unwrap();
    Ok(())
}