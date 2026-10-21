//! Frozen native ecbench specs; no measurement or solver is implemented here.
use serde_json::{json, Value};
use std::{fs, path::Path};
fn arms(dimension: u32) -> Vec<Value> {
    let mut arms = vec![
        json!({"name":"rho-strong","role":"reference","method":{"id":"rho.signed_frobenius_strong"}}),
    ];
    let mut ic = |name: &str, role: &str, oracle: String| {
        arms.push(json!({"name":name,"role":role,"method":{"id":"ic.pipeline","params":{
        "factor_base":format!("binary-standard-subspace:dimension={dimension}"), "oracle":oracle,"max_trials":"1000000"}}}))
    };
    for (short, engine) in [("f4", "matrix-f4"), ("f5", "matrix-f5")] {
        for (suffix, role, runtime) in [
            ("baseline", "baseline", "baseline"),
            ("aa", "control", "baseline"),
            ("tail", "candidate", "quadratic-tail"),
        ] {
            ic(&format!("{short}-{suffix}"), role, format!("pdp3-binary:m=2,engine={engine},degree=3,node_budget=20000,matrix_runtime={runtime}"));
        }
    }
    ic(
        "f6-native",
        "candidate",
        "pdp3-binary:m=2,engine=f6-ic,degree=3,node_budget=20000,kernel_runtime=native-v1".into(),
    );
    ic(
        "generalized-typed",
        "candidate",
        format!(
            "pdp3-binary:m=2,engine=generalized,work_budget={}",
            1u64 << (2 * dimension)
        ),
    );
    ic(
        "semaev-batched",
        "candidate",
        "rootfind-mitm:m=2,batch_setup=1".into(),
    );
    ic(
        "semaev-full-table",
        "candidate",
        "pdp3-binary:m=2,engine=rootfind".into(),
    );
    arms
}
fn spec(n: u32, dimension: u32, smoke: bool) -> Value {
    json!({"schema":"ecbench.spec/v1","label":format!("F4/F5 continuation: complete cold recovery n{n} d{dimension}{}",if smoke{" plumbing"}else{""}),
        "description":"Every arm recovers one public DLP target cold. Same standard polynomial subspace within this session, distinct from the component study's Frobenius-invariant base. Baseline includes previously qualified GF2 kernels. New tail and generalized Gray work is counted unpriced; no ratio of lower bounds is promoted to full-cost S. No scaling claim.",
        "workloads":{"curves":[{"kind":"koblitz","a":1,"n":n}],"targets_per_curve":if smoke{2}else{8},"target_seed":20261019,"target_kind":"public"},
        "arms":arms(dimension),"measurement":{"rounds":if smoke{1}else{5},"warmup":if smoke{0}else{1},"order":"alternate","seed":230,"isolation_required":if smoke{"L0"}else{"L2"},"timeout_seconds":if smoke{120}else{300}}})
}
fn main() {
    let root = std::env::args().nth(1).expect("specs STUDY_DIRECTORY");
    let root = Path::new(&root);
    for (n, d, smoke) in [(11, 4, true), (17, 5, false), (23, 6, false)] {
        let name = if smoke {
            "smoke.json".into()
        } else {
            format!("ecbench-n{n}.json")
        };
        fs::write(
            root.join(name),
            format!(
                "{}\n",
                serde_json::to_string_pretty(&spec(n, d, smoke)).unwrap()
            ),
        )
        .unwrap();
    }
}
