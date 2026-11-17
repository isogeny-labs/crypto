//! Number and text formats shared with the tool's first (Python) version.
use optlevel::util::*;
use serde_json::json;

#[test]
fn python_float_formats() {
    assert_eq!(py_repr(1.0), "1.0");
    assert_eq!(py_repr(0.1), "0.1");
    assert_eq!(py_repr(799516217.8528097), "799516217.8528097");
    assert_eq!(py_repr(1e16), "1e+16");
    assert_eq!(py_repr(1.25e17), "1.25e+17");
    assert_eq!(py_repr(0.0001), "0.0001");
    assert_eq!(py_repr(0.00005), "5e-05");
    assert_eq!(py_repr(-2.5), "-2.5");
    assert_eq!(py_repr(378.0), "378.0");
    assert_eq!(py_e(2794000000.0, 3), "2.794e+09");
    assert_eq!(py_e(0.000123456, 3), "1.235e-04");
    assert_eq!(py_g3(17.61), "17.6");
    assert_eq!(py_g3(1.0), "1");
    assert_eq!(py_g3(100.0), "100");
    assert_eq!(py_g3(999.9), "1e+03");
    assert_eq!(py_g3(0.001), "0.001");
    assert_eq!(py_g3(0.00001), "1e-05");
    assert_eq!(py_g3(2.5), "2.5");
}

#[test]
fn python_text_formats() {
    assert_eq!(sh_quote("optlevel:fma_f64:single:ops_per_s"), "optlevel:fma_f64:single:ops_per_s");
    assert_eq!(sh_quote("optlevel T-timing; not a speed claim"), "'optlevel T-timing; not a speed claim'");
    assert_eq!(sh_quote("it's"), "'it'\"'\"'s'");
    assert_eq!(sh_quote(""), "''");
    assert_eq!(html_escape("t & <t> \"q\" 'a'"), "t &amp; &lt;t&gt; &quot;q&quot; &#x27;a&#x27;");
    // json.dumps(..., sort_keys=True): the text the host fingerprint hashes
    assert_eq!(py_dumps(&json!(["Intel(R) Xeon(R)", "x86_64", 4, ["avx", "fma"], {"L2": 2097152, "L1d": 49152}, null, true])),
               "[\"Intel(R) Xeon(R)\", \"x86_64\", 4, [\"avx\", \"fma\"], {\"L1d\": 49152, \"L2\": 2097152}, null, true]");
    assert_eq!(py_dumps(&json!("caf\u{e9} \u{1f600}\n")), "\"caf\\u00e9 \\ud83d\\ude00\\n\"");
}

#[test]
fn utc_and_hashes() {
    assert_eq!(utc_from_unix(0), "1970-01-01T00:00:00Z");
    assert_eq!(utc_from_unix(1794873600), "2026-11-17T00:00:00Z");
    assert_eq!(utc_from_unix(951782399), "2000-02-28T23:59:59Z");
    assert_eq!(utc_from_unix(951868800), "2000-03-01T00:00:00Z");
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    assert_eq!(to_pretty(&json!({"b": [1, 2.0], "a": {}})), "{\n \"a\": {},\n \"b\": [\n  1,\n  2.0\n ]\n}\n");
}

#[test]
fn fingerprint_is_the_one_the_first_version_computed_for_this_kind_of_host() {
    // the committed calibration of the validation host: its fingerprint must be
    // reproducible from the host fields the same way the Python tool hashed them
    let cal: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../research/optlevel_20261117/controls-run2/calibration.json")).unwrap()).unwrap();
    let h = &cal["host"];
    assert_eq!(h["fingerprint"], "44113cdb2d87e834");
    // the flag set itself is not stored, so only the format is checked here; the live check is
    // research/optlevel_rs_20261117 (Python and Rust probes on the same host give the same fingerprint)
    assert_eq!(h["fingerprint"].as_str().unwrap().len(), 16);
}
