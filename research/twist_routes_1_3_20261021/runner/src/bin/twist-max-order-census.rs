//! Exact target-blind first-point/maximal-order twist-generator diagnostic.
#![allow(dead_code)]
#[path = "../../../../../src/cryptanalysis/twist_ic_field.rs"]
mod field;
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use field::{Quad, P, Q};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::Path,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn write(p: impl AsRef<Path>, x: &Value) -> Result<()> {
    Ok(fs::write(p, serde_json::to_string_pretty(x)? + "\n")?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn val(f: &Value, k: &str) -> u64 {
    f[k].as_u64().unwrap()
}
fn factors(mut n: u64) -> Vec<u64> {
    let mut out = vec![];
    let mut l = 2;
    while l * l <= n {
        if n % l == 0 {
            out.push(l);
            while n % l == 0 {
                n /= l;
            }
        }
        l += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}
fn order(e: Quad, p: P, n: u64) -> Result<u64> {
    if p.is_none() || e.pm(p, n as u128).is_some() {
        return Err("point/multiple check failed".into());
    }
    let mut r = n;
    for l in factors(n) {
        while r % l == 0 && e.pm(p, (r / l) as u128).is_none() {
            r /= l;
        }
    }
    Ok(r)
}
fn pair(p: P) -> [u64; 2] {
    let (x, y) = p.unwrap();
    assert_eq!((x.1, y.1), (0, 0));
    [x.0, y.0]
}
fn card(e: Quad) -> u64 {
    1 + (0..e.p)
        .map(|x| match field::sqrt(e.rhs(Q(x, 0)).0, e.p) {
            None => 0,
            Some(0) => 1,
            Some(_) => 2,
        })
        .sum::<u64>()
}
fn seed(e: Quad, n: u64, full: bool) -> Result<(P, u64, u64)> {
    let mut tests = 0;
    let mut best = None;
    let mut best_order = 0;
    for x in 0..e.p {
        if let Some(y) = field::sqrt(e.rhs(Q(x, 0)).0, e.p).filter(|y| *y != 0) {
            let p = Some((Q(x, 0), Q(y.min(e.p - y), 0)));
            tests += 1;
            if !full {
                return Ok((p, x + 1, tests));
            }
            let found = order(e, p, n)?;
            if found > best_order {
                best = p;
                best_order = found;
            }
            if found == n {
                return Ok((p, x + 1, tests));
            }
        }
    }
    if best.is_some() {
        Ok((best, e.p, tests))
    } else {
        Err("no nonzero point in fixed search".into())
    }
}
fn base(source: Quad, twist: Quad, h: P, d: u64, n: u64, c: u64, b: usize) -> (Vec<u64>, u64) {
    let mut r = h;
    let mut xs = BTreeSet::new();
    let mut terms = 0;
    let di = field::inv(d, source.p);
    for _ in 0..n {
        let Some((x, _)) = r else {
            break;
        };
        terms += 1;
        let xx = (x.0 * di + c) % source.p;
        if field::sqrt(source.rhs(Q(xx, 0)).0, source.p).is_some_and(|y| y != 0) {
            xs.insert(xx);
        }
        if xs.len() == b {
            break;
        }
        r = twist.pa(r, h);
    }
    (xs.into_iter().collect(), terms)
}
fn produce(input: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh output directory required".into());
    }
    fs::create_dir_all(out)?;
    let raw = read(input)?;
    let rows = raw.as_array().ok_or("geometry rows missing")?;
    let mut selected = BTreeMap::new();
    let mut legacy = BTreeMap::new();
    for row in rows {
        if row["construction"]["family"] == "twist-orbit" {
            let f = &row["fixture"];
            let id = f["id"].as_str().unwrap().to_owned();
            legacy.insert(
                (
                    id.clone(),
                    row["construction"]["shift"].as_u64().unwrap(),
                    row["construction"]["B"].as_u64().unwrap(),
                ),
                row["construction"].clone(),
            );
            selected.insert(id, f.clone());
        }
    }
    if selected.len() != 10 || legacy.len() != 60 {
        return Err("all ten fixtures/60 legacy twist bases required".into());
    }
    let mut gp=String::from("default(parisizemax,4000000000);\nseed(A,B,p,n,want)={my(E=ellinit([Mod(A,p),Mod(B,p)]));for(x=0,p-1,for(y=1,(p-1)/2,if((y*y-x^3-A*x-B)%p==0,if(!want||ellorder(E,[Mod(x,p),Mod(y,p)])==n,return([x,y]));break)));error(\"no seed\");};\nbase(A,B,a,b,p,d,H,n,c,k)={my(E=ellinit([Mod(A,p),Mod(B,p)]),R=[Mod(H[1],p),Mod(H[2],p)],T=R,S=[]);for(i=1,n,if(T==[0],break);my(x=lift(T[1]/Mod(d,p)+c));if(kronecker((x^3+a*x+b)%p,p)==1,S=setunion(S,[x]));if(#S==k,break);T=elladd(E,T,R));S;};\n");
    gp += "firstFilled=0;maximalFilled=0;\n";
    let mut records = vec![];
    let mut cases = vec![];
    let (mut old_filled, mut new_filled, mut matched) = (0usize, 0usize, 0usize);
    for (id, f) in selected {
        let (p, a, b, d, n) = (
            val(&f, "p"),
            val(&f, "a"),
            val(&f, "b"),
            val(&f, "d"),
            val(&f, "order"),
        );
        let nt = 2 * (p + 1) - n;
        let source = Quad { p, d, a, b };
        let twist = Quad {
            p,
            d,
            a: a * d % p * d % p,
            b: b * d % p * d % p * d % p,
        };
        if card(source) != n || card(twist) != nt {
            return Err("source/twist cardinality check failed".into());
        }
        let (old, old_trials, _) = seed(twist, nt, false)?;
        let (new, new_trials, order_tests) = seed(twist, nt, true)?;
        let old_order = order(twist, old, nt)?;
        let new_order = order(twist, new, nt)?;
        let g: [u64; 2] = serde_json::from_value(f["generator"].clone())?;
        if order(source, Some((Q(g[0], 0), Q(g[1], 0))), n)? != n {
            return Err("source generator check failed".into());
        }
        let recorded: P =
            serde_json::from_value(legacy[&(id.clone(), 1, 8)]["orbit_generator"].clone())?;
        if old != recorded {
            return Err("legacy seed mismatch".into());
        }
        let mut u = n;
        let mut v = nt;
        while v != 0 {
            (u, v) = (v, u % v);
        }
        records.push(json!({"fixture":f,"old_generator":pair(old),"maximal_order_generator":pair(new),"old_order":old_order,"maximal_order":new_order,"twist_order":nt,"group_cyclic":new_order==nt,"gcd_source_twist_orders":u,"first_seed_x_trials":old_trials,"maximal_seed_x_trials":new_trials,"maximal_seed_order_tests":order_tests}));
        let oh = pair(old);
        let nh = pair(new);
        gp+=&format!("E=ellinit([Mod({},{p}),Mod({},{p})]);Es=ellinit([Mod({a},{p}),Mod({b},{p})]);if(ellcard(E)!={nt}||ellcard(Es)!={n}||ellorder(Es,[{},{}])!={n},error(\"card/source generator\"));if(seed({},{},{p},{nt},0)!={:?}||seed({},{},{p},{new_order},1)!={:?},error(\"canonical seed\"));if(ellorder(E,{:?})!={old_order}||ellorder(E,{:?})!={new_order},error(\"order mismatch\"));\n",twist.a,twist.b,g[0],g[1],twist.a,twist.b,oh,twist.a,twist.b,nh,oh,nh);
        for (strategy, h) in [("first", old), ("maximal-order", new)] {
            for c in [0u64, 1, 3] {
                for bsz in [8usize, 16] {
                    let (xs, terms) = base(source, twist, h, d, nt, c, bsz);
                    let filled = xs.len() == bsz;
                    if c == 0 && !xs.is_empty() {
                        return Err("zero-shift negative control".into());
                    }
                    if strategy == "first" {
                        if legacy[&(id.clone(), c, bsz as u64)]["base"] != json!(xs) {
                            return Err("legacy complete-base mismatch".into());
                        }
                        matched += 1;
                    }
                    if c != 0 && filled {
                        if strategy == "first" {
                            old_filled += 1;
                        } else {
                            new_filled += 1;
                        }
                    }
                    let counter = if strategy == "first" {
                        "firstFilled"
                    } else {
                        "maximalFilled"
                    };
                    gp+=&format!("V=base({},{},{a},{b},{p},{d},{:?},{nt},{c},{bsz});if(V!={:?},error(\"base mismatch\"));if({c}!=0&&#V=={bsz},{counter}++);\n",twist.a,twist.b,pair(h),xs);
                    cases.push(json!({"fixture":id,"strategy":strategy,"shift":c,"B":bsz,"base":xs,"filled":filled,"terms_attempted":terms}));
                }
            }
        }
        gp += &format!("if(vecmax(ellgroup(E))!={new_order},error(\"group exponent mismatch\"));print(\"GENERATOR_OK\\t{id}\\t{old_order}\\t{new_order}\\t\",vecsort(ellgroup(E)));\n");
    }
    gp += "print(\"FILLED_COUNTS\\t\",firstFilled,\"\\t\",maximalFilled);\nprint(\"ALL_TWIST_MAXIMAL_GENERATORS_OK 10 120\");\n";
    fs::write(out.join("verify.gp"), gp)?;
    write(
        out.join("RECORDS.json"),
        &json!({"schema":"twist.maximal-generator-census/v1","input_sha256":sha(input)?,"fixtures":records,"cases":cases,"summary":{"fixtures":10,"base_cases":cases.len(),"legacy_bases_equal":matched,"nonzero_cases_per_strategy":40,"first_filled_nonzero":old_filled,"maximal_order_filled_nonzero":new_filled,"zero_shift_cases":40},"gp_script_sha256":sha(out.join("verify.gp"))?,"timing_or_solving_advantage_claim":false}),
    )
}
fn admit(raw: &Path, receipt: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh admission path required".into());
    }
    let r = read(receipt)?;
    let data = read(raw.join("RECORDS.json"))?;
    if r["exit_code"] != 0
        || !r["signal"].is_null()
        || r["script_sha256"] != sha(raw.join("verify.gp"))?
    {
        return Err("GP transport failed".into());
    }
    let op = r["stdout"].as_str().ok_or("stdout missing")?;
    let ep = r["stderr"].as_str().ok_or("stderr missing")?;
    if r["stdout_sha256"] != sha(op)? || r["stderr_sha256"] != sha(ep)? {
        return Err("GP output hash mismatch".into());
    }
    let err = fs::read_to_string(ep)?;
    if !err.lines().all(|l| {
        l.trim().is_empty()
            || l.trim() == "***   Warning: new maximum stack size = 4000002048 (3814.699 Mbytes)."
    }) {
        return Err(format!("GP diagnostics rejected: {err}").into());
    }
    let text = fs::read_to_string(op)?;
    let expected = data["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| {
            format!(
                "GENERATOR_OK\t{}\t{}\t{}\t{:?}",
                x["fixture"]["id"].as_str().unwrap(),
                val(x, "old_order"),
                val(x, "maximal_order"),
                if val(x, "twist_order") == val(x, "maximal_order") {
                    vec![val(x, "maximal_order")]
                } else {
                    vec![
                        val(x, "twist_order") / val(x, "maximal_order"),
                        val(x, "maximal_order"),
                    ]
                }
            )
        })
        .chain(std::iter::once(format!(
            "FILLED_COUNTS\t{}\t{}",
            val(&data["summary"], "first_filled_nonzero"),
            val(&data["summary"], "maximal_order_filled_nonzero")
        )))
        .chain(std::iter::once(
            "ALL_TWIST_MAXIMAL_GENERATORS_OK 10 120".into(),
        ))
        .collect::<Vec<_>>();
    if text.lines().map(str::to_owned).collect::<Vec<_>>() != expected {
        return Err("GP marker/order/equality check failed".into());
    }
    write(
        out,
        &json!({"schema":"twist.maximal-generator-admission/v1","status":"native-and-independent-PARI-orders-seeds-all-bases-verified","summary":data["summary"],"fixtures":data["fixtures"],"records_sha256":sha(raw.join("RECORDS.json"))?,"receipt_sha256":sha(receipt)?,"no_scalar_map_or_speed_claim":true}),
    )
}
fn export(v: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh export path required".into());
    }
    let p = read(v.join("protocol.json"))?;
    let f = read(v.join("FROZEN.json"))?;
    let a = read(v.join("ADMISSION.json"))?;
    let h = read(v.join("HOST.json"))?;
    if a["status"] != "native-and-independent-PARI-orders-seeds-all-bases-verified" {
        return Err("admission absent".into());
    }
    let root = fs::canonicalize(v)?;
    let file = |path: &Path, kind: &str| -> Result<Value> {
        let abs = fs::canonicalize(path)?;
        let ac = abs.components().collect::<Vec<_>>();
        let bc = root.components().collect::<Vec<_>>();
        let common = ac.iter().zip(&bc).take_while(|(x, y)| x == y).count();
        let mut rel = std::path::PathBuf::new();
        for _ in common..bc.len() {
            rel.push("..");
        }
        for x in &ac[common..] {
            rel.push(x.as_os_str());
        }
        Ok(json!({"path":rel,"sha256":sha(path)?,"bytes":fs::metadata(path)?.len(),"kind":kind}))
    };
    let script =
        Path::new("research/twist_routes_1_3_20261021/runner/src/bin/twist-max-order-census.rs");
    let producer = read(v.join("runs/producer-transport.json"))?;
    let artifacts = vec![
        file(&v.join("runs.tar.xz"), "raw")?,
        file(&v.join("RAW_MEMBERS.json"), "receipt")?,
        file(&v.join("ADMISSION.json"), "receipt")?,
        file(&v.join("pari-replay-v1/RECEIPT.json"), "receipt")?,
    ];
    let runs=p["arms"].as_array().unwrap().iter().map(|arm|json!({"run_id":format!("generator-{}-000",arm["id"].as_str().unwrap()),"arm":arm["id"],"instance_id":"original-ten-fixtures-two-seeds-all-bases","seed":0,"replicate":0,"warmup":false,"status":"succeeded","outcome_class":"completed","attempts":1,"resumed":0,"checkpoint":{"policy":"none","generations_written":0,"restored_from_generation":null},"measurements":{"counted":{"base_cases":a["summary"]["base_cases"]},"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":{"legacy_bases_equal":a["summary"]["legacy_bases_equal"],"zero_shift_cases":a["summary"]["zero_shift_cases"]}},"isolation":{"tier":"L0","contended":null,"blockers":["Exact construction/count experiment only; no performance claim."]},"verification":{"status":"verified","verifier":"native field/group arithmetic and independent PARI seed/order/complete-base replay"},"exclusion":{"excluded":false,"rule":null},"artifacts":artifacts,"started_at":if arm["id"]=="native"{producer["started"].clone()}else{Value::Null},"finished_at":if arm["id"]=="native"{producer["finished"].clone()}else{Value::Null},"worker":arm["instrument"]["binary_sha256"],"backend_ref":if arm["id"]=="native"{"runs/generator-v1"}else{"pari-replay-v1"}})).collect::<Vec<_>>();
    write(
        out,
        &json!({"schema":"experiment.export/v1","export_id":"twist-max-order-census-20261027-v1","created":String::from_utf8(std::process::Command::new("date").args(["-u","+%Y-%m-%dT%H:%M:%SZ"]).output()?.stdout)?.trim(),"protocol":{"path":"protocol.json","sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]},"producer":{"runtime":"codex","model":"GPT-6","session":"twist-max-order-census-20261027","role":"research-and-implementation"},"source":{"repo":"crypto","commit":f["source_revision"],"dirty":false},"backend":"local","host":{"path":"HOST.json","sha256":sha(v.join("HOST.json"))?,"class_id":null,"summary":h["summary"]},"arms":p["arms"].as_array().unwrap().iter().map(|x|json!({"id":x["id"],"role":x["role"],"binary_sha256":x["instrument"]["binary_sha256"],"commit":x["instrument"]["commit"]})).collect::<Vec<_>>(),"runs":runs,"flow":{"planned":2,"attempted":2,"completed":2,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]},"files":[file(&v.join("FROZEN.json"),"receipt")?,file(&v.join("protocol.json"),"protocol")?],"analysis":{"script":file(script,"analysis")?,"blinded":false,"blinding_key_sha256":null,"outputs":[file(&v.join("ADMISSION.json"),"analysis")?],"unmasked_at":null},"notes":[{"kind":"observation","text":"Two actual whole-panel native/CAS executions check the same cases; do not pool them as independent targets. Different generators produce different bases. Discovery occurs in the actual producer and its trial/order-test counts are retained, but total primitive cost, decomposition and scalar recovery are unmeasured. GP epoch timestamps remain in its actual receipt, not reconstructed here.","by":"native exporter"}],"reviews_dir":"reviews"}),
    )
}
fn main() -> Result<()> {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    match a.first().map(String::as_str) {
        Some("produce") if a.len() == 3 => produce(Path::new(&a[1]), Path::new(&a[2])),
        Some("admit") if a.len() == 4 => {
            admit(Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]))
        }
        Some("export") if a.len() == 3 => export(Path::new(&a[1]), Path::new(&a[2])),
        _ => Err("produce GEOMETRY_ROWS FRESH_RAW | admit RAW GP_RECEIPT OUT.json".into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_prime_order_and_factor_reduction() {
        let e = Quad {
            p: 7,
            d: 3,
            a: 1,
            b: 1,
        };
        let p = Some((Q(0, 0), Q(1, 0)));
        assert_eq!(card(e), 5);
        assert_eq!(order(e, p, 5).unwrap(), 5);
        assert_eq!(order(e, p, 15).unwrap(), 5);
        assert!(order(e, p, 6).is_err());
    }
    #[test]
    fn maximal_seed_covers_a_noncyclic_control() {
        let e = Quad {
            p: 7,
            d: 3,
            a: 0,
            b: 1,
        };
        assert_eq!(card(e), 12);
        let (h, trials, _) = seed(e, 12, true).unwrap();
        assert_eq!(pair(h), [1, 3]);
        assert_eq!(order(e, h, 12).unwrap(), 6);
        assert_eq!(trials, 7);
    }
}
