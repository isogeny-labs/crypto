#[allow(dead_code, unused_imports)]
#[path = "../common.rs"]
mod common;
use common::*;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = Path::new(args.get(1).expect("usage: recover ROUND59"));
    let c: Value = serde_json::from_slice(&fs::read(root.join("carriers.json")).unwrap()).unwrap();
    let p = fp::parse(&c["p"]);
    let r = fp::parse(&c["r"]);
    let f = fp::Fp { p: p.clone() };
    let fr = fp::Fp { p: r.clone() };
    let input: Value =
        serde_json::from_slice(&fs::read(root.join("../round52-p192/input.json")).unwrap())
            .unwrap();
    let a = fp::parse(&input["a"]);
    let g = point(&input["G"]);
    let mut rows = vec![];
    let mut recovered = 0;
    let mut checked = 0;
    for row in c["bounded_point_search"]["targets"].as_array().unwrap() {
        let hits = row["matches"].as_array().unwrap();
        assert_eq!(hits.len(), row["match_count"].as_u64().unwrap() as usize);
        if hits.is_empty() {
            rows.push(json!({"target":row["name"],"status":"no bounded relation; outside carrier promise"}));
            continue;
        }
        let aa = fp::parse(&hits[0][0]);
        let u = hits[0][1].as_i64().unwrap();
        let k = fr.mul(&fr.c(u), &fr.inv(&aa));
        let q = point(&row["Q"]);
        assert_eq!(mul(&f, &a, &k, &g), q);
        for h in hits {
            let aa = fp::parse(&h[0]);
            let u = h[1].as_i64().unwrap();
            assert_eq!(fr.mul(&aa, &k), fr.c(u));
            checked += 1
        }
        recovered += 1;
        rows.push(json!({"target":row["name"],"status":"scalar recovered and point verified","a":aa.to_string(),"u":u,"scalar":k.to_string(),"Q":row["Q"],"all_match_scalars_consistent":true}));
    }
    assert_eq!(recovered, 4);
    assert_eq!(checked, 11605);
    println!("{}",serde_json::to_string_pretty(&json!({"all_checks_pass":true,"recovered_targets":recovered,"match_consistency_checks":checked,"rows":rows,"scope":"conditional carrier-to-scalar decoder; actual P192 small-scalar controls; hard fixtures have no bounded relation"})).unwrap());
}
