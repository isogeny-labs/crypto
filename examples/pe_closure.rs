//! Lead L2: the degree-`D` MutantXL closure by partial enlargement, on
//! committed refutation-degree draws ([`pe_closure_with`]).
//!
//! ```sh
//! cargo run --release --example pe_closure -- \
//!     --draws research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl \
//!     --draw 0 --degree 5 --mode partial --batch 64 --max-rows 120000
//! ```
//!
//! One JSON line per run on stdout: the draw, its committed outcome, the
//! mode, refuted / saturated / capped, the peak rows and bytes of the
//! eliminations, and the process's peak resident set (`VmHWM`).  A capped
//! run is unmeasured, never a refutation.  `--mode full` runs the dense
//! engine's order through the same code, the A/A reference for the peaks.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::partial_enlargement::{pe_closure_with, PeMode, PeOptions, PeOrder};
use num_bigint::BigUint;
use serde_json::Value;

fn vm_hwm_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        })
        .unwrap_or(0)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let draws = flag("--draws").expect("--draws FILE");
    let draw: u64 = flag("--draw").expect("--draw K").parse().expect("--draw K");
    let degree: u32 = flag("--degree")
        .expect("--degree D")
        .parse()
        .expect("--degree D");
    let batch: usize = flag("--batch").map_or(64, |v| v.parse().expect("--batch"));
    let mode = match flag("--mode").as_deref() {
        Some("full") => PeMode::Full,
        Some("partial") | None => PeMode::Partial { batch_rows: batch },
        Some(m) => panic!("--mode full|partial, got {m}"),
    };
    // Amendment 3: `--order low|high|random:S`, `--restrict`, `--seed-degree K`.
    let order = match flag("--order").as_deref() {
        None | Some("low") => PeOrder::LowFirst,
        Some("high") => PeOrder::HighFirst,
        Some(o) => PeOrder::Random(
            o.strip_prefix("random:")
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| panic!("--order low|high|random:S, got {o}")),
        ),
    };
    let opts = PeOptions {
        order,
        seed_degree: flag("--seed-degree").map(|v| v.parse().expect("--seed-degree")),
        restrict_vars: args.iter().any(|a| a == "--restrict"),
    };
    let max_cols: usize = flag("--max-cols").map_or(1_200_000, |v| v.parse().expect("--max-cols"));
    let max_rows: usize = flag("--max-rows").map_or(usize::MAX, |v| v.parse().expect("--max-rows"));
    let text = std::fs::read_to_string(&draws).unwrap_or_else(|e| panic!("{draws}: {e}"));
    let v: Value = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Value>(l).expect("json"))
        .find(|v| v["draw"].as_u64() == Some(draw))
        .unwrap_or_else(|| panic!("{draws}: no draw {draw}"));
    let m = v.get("m").and_then(Value::as_u64).unwrap_or(3) as usize;
    let n = v["n"].as_u64().expect("n") as u32;
    let ell = v["ell"].as_u64().expect("ell") as usize;
    let irr = find_irreducible_sparse(n).expect("field");
    let st = FieldStructure::new(n, &irr);
    let basis: Vec<F2mElement> = v["v_basis"]
        .as_array()
        .expect("v_basis")
        .iter()
        .map(|w| F2mElement::from_biguint(&BigUint::from(w.as_u64().unwrap()), n))
        .collect();
    assert_eq!(basis.len(), ell);
    let x_r = F2mElement::from_biguint(&BigUint::from(v["x_r"].as_u64().unwrap()), n);
    let sys =
        build_decomposition_system(&basis, &x_r, &F2mElement::one(n), m, &st).expect("system");
    assert_eq!(
        sys.n_vars as u64,
        v["n_vars"].as_u64().unwrap(),
        "rebuilt shape"
    );
    let r = pe_closure_with(
        &sys.equations,
        sys.n_vars,
        degree,
        mode,
        max_cols,
        max_rows,
        opts,
    );
    let hwm = vm_hwm_kb();
    let body = match &r {
        None => r#""built":false"#.to_string(),
        Some(r) => format!(
            r#""built":true,"mode":"{}","batch":{},"refuted":{},"saturated":{},"capped":{},"columns":{},"peak_basis_rows":{},"peak_matrix_rows":{},"peak_matrix_bytes":{},"final_rows":{},"absorbs":{},"expanded":{},"products":{},"word_ops":{},"order":"{:?}","restrict":{},"seed_degree":{},"completion_at_rows":{},"refuted_in_completion":{},"secs":{:.3}"#,
            r.mode,
            match mode {
                PeMode::Partial { batch_rows } => batch_rows,
                PeMode::Full => 0,
            },
            r.refuted,
            r.saturated,
            r.capped,
            r.columns,
            r.peak_basis_rows,
            r.peak_matrix_rows,
            r.peak_matrix_bytes(),
            r.final_rows,
            r.absorbs,
            r.expanded,
            r.products,
            r.word_ops,
            opts.order,
            opts.restrict_vars,
            opts.seed_degree.map_or("null".into(), |d| d.to_string()),
            r.completion_at_rows
                .map_or("null".into(), |d| d.to_string()),
            r.refuted_in_completion,
            r.secs
        ),
    };
    println!(
        r#"{{"source":"{draws}","cell":"{}","m":{m},"n":{n},"ell":{ell},"draw":{draw},"n_vars":{},"committed_outcome":{},"solutions":{},"degree":{degree},{body},"vm_hwm_kb":{hwm}}}"#,
        v["cell"].as_str().unwrap_or(""),
        sys.n_vars,
        v["outcome"],
        v["solutions"],
    );
}
