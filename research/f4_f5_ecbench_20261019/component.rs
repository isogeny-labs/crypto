//! Copy precomputed study values to a compact dashboard input; no pooling.
use serde_json::{json, Value};
use std::{fs, path::Path};
fn main() {
    let root = std::env::args().nth(1).expect("component STUDY");
    let root = Path::new(&root);
    let read = |name| {
        serde_json::from_str::<Value>(&fs::read_to_string(root.join(name)).unwrap()).unwrap()
    };
    let native = read("native-summary.json");
    let ecbench = read("ecbench-summary.json");
    let costs:Vec<_>=native["costs"].as_array().unwrap().iter().filter(|r|r["n"]==23).map(|r|json!({"boundary":r["boundary"],"arm":r["arm"],"random":r["random"],"cold":r["cold"]})).collect();
    let improvements:Vec<_>=native["changes"].as_array().unwrap().iter().filter(|r|r["seed"]==230).map(|r|json!({"arm":r["arm"],"n":r["n"],"random":r["random"],"before":r["before"],"candidate":r["candidate"],"comparison":r["comparison"]})).collect();
    let sessions:Vec<_>=ecbench["sessions"].as_array().unwrap().iter().map(|s|{
        let arms:Vec<_>=s["arms"].as_array().unwrap_or(&Vec::new()).iter().map(|a|json!({"name":a["name"],"cold_solve":a["cold_solve"],"process":a["process"],"admitted":a["admitted"],"requested":a["requested"],"complete":a["complete"],"s_charged_mean":a["s_charged_mean"],"s_over_rho_diagnostic":a["charged_s_over_rho_diagnostic"],"lower_bound":a["lower_bound"]})).collect();
        let comparisons:Vec<_>=s["comparisons"].as_array().unwrap_or(&Vec::new()).iter().map(|c|json!({"pair":c["pair"],"wall":c["result"]["wall"],"ops_status":c["result"]["ops"]["status"],"ops_bounded":c["result"]["ops"]["bounded"]})).collect();
        json!({"n":s["n"],"complete":s["complete"],"audit_ok":s["audit"]["ok"],"arms":arms,"comparisons":comparisons})
    }).collect();
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-recursive-component/v1","report":"research/f4_f5_ecbench_20261019/REPORT.pdf","study":"research/f4_f5_ecbench_20261019/","scope":"Final source9bed104b0a: opt-in adaptive F4/F5 policies and one-worker full-table construction; complete original roots and first-witness decomposition kept separate. Full ecbench DLP recovery uses separately declared standard bases. Earlier full-pipeline regressions remain in their own reports; different hosts are not pooled.","costs_n23":costs,"improvements":improvements,"four_worker_controls":native["four_worker_controls"],"full_table_four_worker_controls":native["full_table_four_worker_controls"],"ecbench":sessions,
        "limits":"All policies remain opt-in. Gray work is counted unpriced; S/rho ratios are diagnostic lower-bound comparisons. No external-fastest, other-hardware, n19 admission or hard-family8x qualification. Native comparison and full-DLP timing intervals have different bases and workloads."})).unwrap());
}
