//! Summarize the already replayed complete panel; no solver or clock calls.
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 3, "usage: review OUTCOME REPORT_DATA");
    let input: Value = serde_json::from_slice(&fs::read(&a[1]).unwrap()).unwrap();
    assert_eq!(input["actual_f4_calls"], 41);
    assert_eq!(input["unsupported_f4_not_called"], 41);
    let mut pair_count = 0;
    let mut pair_degree4 = 0;
    let mut proper_gate_loss = 0;
    let mut full_gate_pass = 0;
    let mut degree4_gate_loss = 0;
    let mut valid = 0;
    let mut rows = vec![];
    for case in input["cases"].as_array().unwrap() {
        let id = case["case"].as_str().unwrap();
        let obs = case["observations"].as_array().unwrap();
        let n = id.rsplit_once("-n").unwrap().1.parse::<usize>().unwrap();
        let get = |name: &str| obs.iter().find(|o| o["arm"]["id"] == name).unwrap();
        for o in obs {
            let full = o["arm"]["id"] == "full";
            if full {
                if o["polynomial_space_gate_pass"] == true {
                    full_gate_pass += 1;
                }
            } else if o["polynomial_space_gate_pass"] == false {
                proper_gate_loss += 1;
            }
            if o["f4_supported"] == true {
                if o["complete_basis_valid"] == true {
                    valid += 1;
                }
                if !full && o["productive_degree"] == 4 && o["polynomial_space_gate_pass"] == false
                {
                    degree4_gate_loss += 1;
                }
                let ids = o["arm"]["aux"].as_array().unwrap();
                if ids.len() == 2 && ids.iter().all(|i| i.as_u64().unwrap() % 2 == 1) {
                    pair_count += 1;
                    if o["productive_degree"] == 4 && o["complete_basis_valid"] == true {
                        pair_degree4 += 1;
                    }
                }
            }
        }
        let names = if n == 3 {
            vec!["original-a", "full", "syzygies", "full"]
        } else {
            vec!["original-a", "full", "pair-00-01", "pair-differences-00-01"]
        };
        rows.push(json!({"case":id,"n":n,"baseline_ranks":case["baseline_ranks_D2_D3_D4"],
            "fresh_aa_exact":case["fresh_aa_exact"],"prior_original_full_exact":case["prior_original_full_exact"],
            "original":get(names[0]),"full":get(names[1]),"first_pair_only":get(names[2]),"first_pair_with_differences":get(names[3])}));
    }
    assert_eq!(valid, 41);
    assert_eq!(full_gate_pass, 6);
    assert_eq!(proper_gate_loss, 76);
    assert_eq!(pair_count, 14);
    assert_eq!(pair_degree4, 14);
    assert_eq!(degree4_gate_loss, 29);
    let result = json!({"schema":"syzygy-pair-report-data/v1","actual_f4_calls":41,"complete_bases_valid":valid,
        "pair_only_arms":pair_count,"pair_only_valid_degree4":pair_degree4,
        "full_pool_polynomial_gate_pass":full_gate_pass,"proper_subsets_rejected":proper_gate_loss,
        "degree4_proper_arms_rejected":degree4_gate_loss,"rows":rows,
        "report_selection":"lexicographically first distinct-edge pair, selected in this review source before report computation; no best-runtime selection",
        "original_15x_achieved":false,"timing_observations":0,"intrinsic_regularity":null});
    let path = Path::new(&a[2]);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    file.write_all(&serde_json::to_vec_pretty(&result).unwrap())
        .unwrap();
    file.sync_all().unwrap();
    println!("{pair_degree4}/{pair_count} pair-only arms: independently valid degree4; all76proper-subset arms lose a declared space;29degree4 proper arms cannot pass pruning");
    for r in result["rows"].as_array().unwrap() {
        if r["original"]["f4_supported"] == true {
            for name in [
                "original",
                "full",
                "first_pair_only",
                "first_pair_with_differences",
            ] {
                println!(
                    "{} {} degree={} rows={} cols={} row-mults={}",
                    r["case"].as_str().unwrap(),
                    name,
                    r[name]["productive_degree"],
                    r[name]["max_rows"],
                    r[name]["max_cols"],
                    r[name]["row_reduction_multiplications_only"]
                );
            }
        }
    }
}
