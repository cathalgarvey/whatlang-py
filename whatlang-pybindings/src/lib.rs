extern crate whatlang;
#[macro_use] extern crate cpython;
use whatlang::{detect, Lang, Script};
use cpython::{Python, PyObject, PyString, PyResult, PyErr};

pub fn detect_language(py: Python, text: &PyObject) -> PyResult<(String, String)> {
    let text_s : String = text.cast_as::<PyString>(py)?.to_string(py)?.into_owned();
    let info_op = detect(&text_s);
    let info = info_op.ok_or_else(|| PyErr::new::<cpython::exc::ValueError, String>(py, "Unable to determine language information".to_string()))?;
    Ok((lang_to_string(info.lang), script_to_string(info.script)))
}

py_module_initializer!(whatlang, initwhatlang, PyInit_whatlang, |py, m| {
    m.add(py, "__doc__", "The Rust 'whatlang' crate exposed to Python.")?;
    m.add(py, "detect_language", py_fn!(py, detect_language(input: &PyObject)))?;
    Ok(())
});

fn lang_to_string(l: Lang) -> String {
    let ls = match l {
        Lang::Aka => "Aka",
        Lang::Amh => "Amh",
        Lang::Arb => "Arb",
        Lang::Azj => "Azj",
        Lang::Bel => "Bel",
        Lang::Ben => "Ben",
        Lang::Bho => "Bho",
        Lang::Bul => "Bul",
        Lang::Ceb => "Ceb",
        Lang::Ces => "Ces",
        Lang::Cmn => "Cmn",
        Lang::Dan => "Dan",
        Lang::Deu => "Deu",
        Lang::Ell => "Ell",
        Lang::Eng => "Eng",
        Lang::Epo => "Epo",
        Lang::Est => "Est",
        Lang::Fin => "Fin",
        Lang::Fra => "Fra",
        Lang::Guj => "Guj",
        Lang::Hat => "Hat",
        Lang::Hau => "Hau",
        Lang::Heb => "Heb",
        Lang::Hin => "Hin",
        Lang::Hrv => "Hrv",
        Lang::Hun => "Hun",
        Lang::Ibo => "Ibo",
        Lang::Ilo => "Ilo",
        Lang::Ind => "Ind",
        Lang::Ita => "Ita",
        Lang::Jav => "Jav",
        Lang::Jpn => "Jpn",
        Lang::Kan => "Kan",
        Lang::Kat => "Kat",
        Lang::Khm => "Khm",
        Lang::Kin => "Kin",
        Lang::Kor => "Kor",
        Lang::Kur => "Kur",
        Lang::Lav => "Lav",
        Lang::Lit => "Lit",
        Lang::Mai => "Mai",
        Lang::Mal => "Mal",
        Lang::Mar => "Mar",
        Lang::Mkd => "Mkd",
        Lang::Mlg => "Mlg",
        Lang::Mya => "Mya",
        Lang::Nep => "Nep",
        Lang::Nld => "Nld",
        Lang::Nno => "Nno",
        Lang::Nob => "Nob",
        Lang::Nya => "Nya",
        Lang::Ori => "Ori",
        Lang::Orm => "Orm",
        Lang::Pan => "Pan",
        Lang::Pes => "Pes",
        Lang::Pol => "Pol",
        Lang::Por => "Por",
        Lang::Ron => "Ron",
        Lang::Run => "Run",
        Lang::Rus => "Rus",
        Lang::Sin => "Sin",
        Lang::Skr => "Skr",
        Lang::Slv => "Slv",
        Lang::Sna => "Sna",
        Lang::Som => "Som",
        Lang::Spa => "Spa",
        Lang::Srp => "Srp",
        Lang::Swe => "Swe",
        Lang::Tam => "Tam",
        Lang::Tel => "Tel",
        Lang::Tgl => "Tgl",
        Lang::Tha => "Tha",
        Lang::Tir => "Tir",
        Lang::Tuk => "Tuk",
        Lang::Tur => "Tur",
        Lang::Uig => "Uig",
        Lang::Ukr => "Ukr",
        Lang::Urd => "Urd",
        Lang::Uzb => "Uzb",
        Lang::Vie => "Vie",
        Lang::Ydd => "Ydd",
        Lang::Yor => "Yor",
        Lang::Zul => "Zul",
    };
    ls.to_string()
}

fn script_to_string(s: Script) -> String {
    let ss = match s {
        Script::Latin => "Latin",
        Script::Cyrillic => "Cyrillic",
        Script::Arabic => "Arabic",
        Script::Devanagari => "Devanagari",
        Script::Hiragana => "Hiragana",
        Script::Katakana => "Katakana",
        Script::Ethiopic => "Ethiopic",
        Script::Hebrew => "Hebrew",
        Script::Bengali => "Bengali",
        Script::Georgian => "Georgian",
        Script::Mandarin => "Mandarin",
        Script::Hangul => "Hangul",
        Script::Greek => "Greek",
        Script::Kannada => "Kannada",
        Script::Tamil => "Tamil",
        Script::Thai => "Thai",
        Script::Gujarati => "Gujarati",
        Script::Gurmukhi => "Gurmukhi",
        Script::Telugu => "Telugu",
        Script::Malayalam => "Malayalam",
        Script::Oriya => "Oriya",
        Script::Myanmar => "Myanmar",
        Script::Sinhala => "Sinhala",
        Script::Khmer => "Khmer",
    };
    ss.to_string()
}
