extern crate whatlang;
#[macro_use]
extern crate cpython;
use cpython::{PyErr, PyObject, PyResult, PyString, Python};
use whatlang::{detect, Lang, Script};

pub fn detect_language(py: Python, text: &PyObject) -> PyResult<(String, String)> {
    let text_s: String = text.cast_as::<PyString>(py)?.to_string(py)?.into_owned();
    let info_op = detect(&text_s);
    let info = info_op.ok_or_else(|| {
        PyErr::new::<cpython::exc::ValueError, String>(
            py,
            "Unable to determine language information".to_string(),
        )
    })?;
    Ok((
        info.lang().code().to_string(),
        info.script().name().to_string(),
    ))
}

py_module_initializer!(whatlang, initwhatlang, PyInit_whatlang, |py, m| {
    m.add(
        py,
        "__doc__",
        "The Rust 'whatlang' crate exposed to Python.",
    )?;
    m.add(
        py,
        "detect_language",
        py_fn!(py, detect_language(input: &PyObject)),
    )?;
    Ok(())
});
