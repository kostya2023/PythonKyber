use pyo3::prelude::*;
use pqcrypto_kyber::{
    kyber512::{keypair as keypair512, encapsulate as encapsulate512, decapsulate as decapsulate512},
    kyber768::{keypair as keypair768, encapsulate as encapsulate768, decapsulate as decapsulate768},
    kyber1024::{keypair as keypair1024, encapsulate as encapsulate1024, decapsulate as decapsulate1024},
};
use pqcrypto_traits::kem::{PublicKey, SecretKey, Ciphertext, SharedSecret};

// Общий трейт для обработки ошибок
fn to_py_result<T, E: ToString>(result: Result<T, E>) -> PyResult<T> {
    result.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

// Модуль Kyber512
#[pymodule]
fn Kyber512(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfunction]
    fn generate_keypair() -> PyResult<(Vec<u8>, Vec<u8>)> {
        let (pk, sk) = keypair512();
        Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
    }

    #[pyfunction]
    fn encapsulate(public_key: Vec<u8>) -> PyResult<(Vec<u8>, Vec<u8>)> {
        let pk = to_py_result(PublicKey::from_bytes(&public_key))?;
        let (ct, ss) = encapsulate512(&pk);
        Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
    }

    #[pyfunction]
    fn decapsulate(secret_key: Vec<u8>, ciphertext: Vec<u8>) -> PyResult<Vec<u8>> {
        let sk = to_py_result(SecretKey::from_bytes(&secret_key))?;
        let ct = to_py_result(Ciphertext::from_bytes(&ciphertext))?;
        Ok(decapsulate512(&ct, &sk).as_bytes().to_vec())
    }

    m.add_function(wrap_pyfunction!(generate_keypair, m)?)?;
    m.add_function(wrap_pyfunction!(encapsulate, m)?)?;
    m.add_function(wrap_pyfunction!(decapsulate, m)?)?;
    Ok(())
}

// Модуль Kyber768
#[pymodule]
fn Kyber768(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfunction]
    fn generate_keypair() -> PyResult<(Vec<u8>, Vec<u8>)> {
        let (pk, sk) = keypair768();
        Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
    }

    #[pyfunction]
    fn encapsulate(public_key: Vec<u8>) -> PyResult<(Vec<u8>, Vec<u8>)> {
        let pk = to_py_result(PublicKey::from_bytes(&public_key))?;
        let (ct, ss) = encapsulate768(&pk);
        Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
    }

    #[pyfunction]
    fn decapsulate(secret_key: Vec<u8>, ciphertext: Vec<u8>) -> PyResult<Vec<u8>> {
        let sk = to_py_result(SecretKey::from_bytes(&secret_key))?;
        let ct = to_py_result(Ciphertext::from_bytes(&ciphertext))?;
        Ok(decapsulate768(&ct, &sk).as_bytes().to_vec())
    }

    m.add_function(wrap_pyfunction!(generate_keypair, m)?)?;
    m.add_function(wrap_pyfunction!(encapsulate, m)?)?;
    m.add_function(wrap_pyfunction!(decapsulate, m)?)?;
    Ok(())
}

// Модуль Kyber1024
#[pymodule]
fn Kyber1024(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfunction]
    fn generate_keypair() -> PyResult<(Vec<u8>, Vec<u8>)> {
        let (pk, sk) = keypair1024();
        Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
    }

    #[pyfunction]
    fn encapsulate(public_key: Vec<u8>) -> PyResult<(Vec<u8>, Vec<u8>)> {
        let pk = to_py_result(PublicKey::from_bytes(&public_key))?;
        let (ct, ss) = encapsulate1024(&pk);
        Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
    }

    #[pyfunction]
    fn decapsulate(secret_key: Vec<u8>, ciphertext: Vec<u8>) -> PyResult<Vec<u8>> {
        let sk = to_py_result(SecretKey::from_bytes(&secret_key))?;
        let ct = to_py_result(Ciphertext::from_bytes(&ciphertext))?;
        Ok(decapsulate1024(&ct, &sk).as_bytes().to_vec())
    }

    m.add_function(wrap_pyfunction!(generate_keypair, m)?)?;
    m.add_function(wrap_pyfunction!(encapsulate, m)?)?;
    m.add_function(wrap_pyfunction!(decapsulate, m)?)?;
    Ok(())
}

// Главный модуль PythonKyber
#[pymodule]
fn PythonKyber(py: Python, m: &PyModule) -> PyResult<()> {
    // Добавляем подмодули
    let kyber512_module = PyModule::new(py, "Kyber512")?;
    Kyber512(py, kyber512_module)?;
    m.add_submodule(kyber512_module)?;

    let kyber768_module = PyModule::new(py, "Kyber768")?;
    Kyber768(py, kyber768_module)?;
    m.add_submodule(kyber768_module)?;

    let kyber1024_module = PyModule::new(py, "Kyber1024")?;
    Kyber1024(py, kyber1024_module)?;
    m.add_submodule(kyber1024_module)?;

    Ok(())
}