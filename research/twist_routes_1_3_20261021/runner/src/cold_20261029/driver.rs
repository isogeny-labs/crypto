use crate::{
    arithmetic::*,
    engine::{self, Task},
    pari::Pari,
    Result,
};
use serde_json::{json, Value};
use std::{fs, path::Path};
pub const GP: &str = "/workspace/shared/twist-tools-20261023/cas/root/usr/bin/gp";
pub fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(crate::source_sha256::sha256(&fs::read(p)?)))
}
pub fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
pub fn write(p: impl AsRef<Path>, x: &Value) -> Result<()> {
    if p.as_ref().exists() {
        return Err(format!("fresh output required: {}", p.as_ref().display()).into());
    }
    fs::write(p, serde_json::to_string_pretty(x)? + "\n")?;
    Ok(())
}
fn identity(f: &mut Fixture) -> Value {
    let mut m = crate::registry::identity(f.p, f.a, f.b, f.order as u128, None);
    f.id = m["slug"].as_str().unwrap().into();
    m["sources"] = json!(["research/twist_routes_1_3_20261021/execution-v15/FIXTURES.json"]);
    m["ec1_unresolved"]=json!("Exact field, whole order, prime subgroup order and canonical generator are recorded in the frozen fixture. EC1 metadata is separate; no cross-repository comparison is asserted.");
    m
}
fn make_fixture(e: Curve, n: u64, r: u64, family: &str, role: &str) -> Fixture {
    assert!(prime(e.p) && prime(r) && n % r == 0);
    let d = (2..e.p)
        .find(|x| pow(*x, (e.p - 1) / 2, e.p) == e.p - 1)
        .unwrap();
    let mut g = None;
    for x in 0..e.p {
        let q = e.mul(e.lift(x), n / r);
        if q.is_some() && e.mul(q, r).is_none() {
            g = e.canon(q);
            break;
        }
    }
    assert!(g.is_some());
    Fixture {
        id: "pending".into(),
        p: e.p,
        a: e.a,
        b: e.b,
        order: n,
        r,
        d,
        generator: g.unwrap(),
        family: family.into(),
        role: role.into(),
    }
}
fn inputs(original: &Path, out: &Path, so: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh fixture output required".into());
    }
    fs::create_dir_all(out)?;
    let mut gp = Pari::new(Path::new(GP), so, &out.join("gp.stderr"))?;
    let old = read(original)?;
    let mut development = vec![];
    let mut models = vec![];
    for f in old["fixtures"].as_array().ok_or("original fixtures")? {
        let n = f["order"].as_u64().unwrap();
        if !prime(n) {
            continue;
        }
        let mut x: Fixture = serde_json::from_value(
            json!({"id":f["id"],"p":f["p"],"a":f["a"],"b":f["b"],"order":n,"r":n,"d":f["d"],"generator":f["generator"],"family":"archived-development","role":f["role"]}),
        )?;
        assert_eq!(x.curve().card(), n);
        let oldid = x.id.clone();
        models.push(identity(&mut x));
        assert_eq!(oldid, x.id);
        development.push(x);
    }
    assert_eq!(development.len(), 10);
    let hc = gp.call("print(Vec(polclass(-67)))")?;
    assert_eq!(hc, json!([1, 147197952000u64]));
    let mut heldout = vec![];
    let mut sizing = vec![];
    let mut cm_count = [0usize; 3];
    let bins = [16u32, 18, 20];
    let mut candidates = vec![];
    for m in 2..=4096u64 {
        if cm_count.iter().all(|x| *x == 3) {
            break;
        }
        let p = 17 * m * m - m + 1;
        if !prime(p) {
            continue;
        }
        let n = p + 3 - m;
        let r = factors(n).last().unwrap().0;
        let bit = 64 - r.leading_zeros();
        let Some(bin) = bins
            .iter()
            .enumerate()
            .position(|(i, b)| bit >= *b && cm_count[i] < 3)
        else {
            continue;
        };
        if n / r > 16 || r >= (1 << 24) || p == 67 {
            continue;
        }
        let j = (p - 147197952000u64 % p) % p;
        let den = (1728 + p - j) % p;
        if den == 0 {
            continue;
        }
        let k = j * inv(den, p) % p;
        let mut e = Curve {
            p,
            a: 3 * k % p,
            b: 2 * k % p,
        };
        let mut actual = gp
            .call(&format!("print(ellcard(ellinit([{},{}],{})))", e.a, e.b, p))?
            .as_u64()
            .unwrap();
        let mut twisted = false;
        if actual != n {
            let d = (2..p).find(|d| pow(*d, (p - 1) / 2, p) == p - 1).unwrap();
            e.a = e.a * d % p * d % p;
            e.b = e.b * d % p * d % p * d % p;
            actual = gp
                .call(&format!("print(ellcard(ellinit([{},{}],{})))", e.a, e.b, p))?
                .as_u64()
                .unwrap();
            twisted = true;
        }
        candidates.push(json!({"family":"CM-candidate","m":m,"p":p,"predicted_N":n,"actual_N":actual,"r":r,"bin_bits":bit,"orientation_twisted":twisted,"accepted":actual==n}));
        if actual != n {
            continue;
        }
        assert_eq!(e.card(), n, "independent native CM point count");
        let mut f = make_fixture(
            e,
            n,
            r,
            "CM-degree17-candidate",
            &format!(
                "{}-min{}-cm-{}",
                if cm_count[bin] < 2 {
                    "heldout"
                } else {
                    "sizing"
                },
                bins[bin],
                cm_count[bin]
            ),
        );
        models.push(identity(&mut f));
        if cm_count[bin] < 2 {
            heldout.push(f);
        } else {
            sizing.push(f);
        }
        cm_count[bin] += 1;
    }
    if cm_count != [3, 3, 3] {
        write(
            out.join("SEARCH_FAILURE.json"),
            &json!({"cm_counts":cm_count,"candidates":candidates}),
        )?;
        return Err("fixed m<=4096 CM fixture search underfilled".into());
    }
    for bit in bins {
        let mut p = (1u64 << (bit - 1)) + 123;
        while !prime(p) {
            p += 1;
        }
        let mut count = 0;
        for b in 1..=512 {
            if count == 3 {
                break;
            }
            let e = Curve { p, a: 1, b };
            if (4 + 27 * b % p * b) % p == 0 {
                continue;
            }
            let n = gp
                .call(&format!("print(ellcard(ellinit([1,{b}],{p})))"))?
                .as_u64()
                .unwrap();
            let accepted = prime(n) && (64 - n.leading_zeros()) == bit;
            candidates.push(
                json!({"family":"general-control","p":p,"a":1,"b":b,"N":n,"accepted":accepted}),
            );
            if !accepted {
                continue;
            }
            assert_eq!(e.card(), n, "independent native general point count");
            let mut f = make_fixture(
                e,
                n,
                n,
                "general-curve-control",
                &format!(
                    "{}-min{bit}-general-{count}",
                    if count < 2 { "heldout" } else { "sizing" }
                ),
            );
            models.push(identity(&mut f));
            if count < 2 {
                heldout.push(f);
            } else {
                sizing.push(f);
            }
            count += 1;
        }
        if count != 3 {
            return Err("fixed general b<=512 fixture search underfilled".into());
        }
    }
    heldout.sort_by(|a, b| a.role.cmp(&b.role));
    let mut verify = String::from("default(parisizemax,4000000000);\nchecks=0;\n");
    for f in development
        .iter()
        .chain(heldout.iter())
        .chain(sizing.iter())
    {
        let g = crate::pari::point(f.g());
        verify.push_str(&format!("ee=ellinit([{},{}],{});tt=ellinit([{},{}],{});if(ellcard(ee)!={}||ellcard(tt)!={}||!isprime({})||ellmul(ee,{}, {})!=[0]||kronecker({},{})!=-1,error(\"fixture field/card/subgroup\"));checks++;\n",f.a,f.b,f.p,f.twist().a,f.twist().b,f.p,f.order,f.tn(),f.r,g,f.r,f.d,f.p));
        if f.family == "CM-degree17-candidate" {
            verify.push_str(&format!("mm={}+3-{};if(lift(ellj(ee))!=lift(Mod(-147197952000,{}))||{}!=17*mm^2-mm+1||{}!=17*mm^2,error(\"CM candidate recipe\"));\n",f.p,f.order,f.p,f.p,f.tn()));
        }
    }
    verify.push_str(
        "if(checks!=28,error(\"input population\"));print(\"ALL_COLD_INPUTS_OK \",checks);\n",
    );
    fs::write(out.join("verify.gp"), verify)?;
    let transcript = gp.close()?;
    fs::write(out.join("gp-transcript.txt"), transcript.join("\n"))?;
    write(
        out.join("FIXTURES.json"),
        &json!({"schema":"twist.cold-fixtures/v1","original_sha256":sha(original)?,"development":development,"heldout":heldout,"sizing_only":sizing,"minimum_heldout_group_bit_thresholds":[16,18,20],"actual_group_bits_recorded_per_fixture":true,"separate_from_E28_final_labels":true,"target_outcomes_observed":false}),
    )?;
    write(out.join("CANDIDATES.json"), &json!(candidates))?;
    write(out.join("registry-additions.json"), &json!(models))?;
    Ok(())
}
fn worker(task: &Path, out: &Path, so: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh worker output required".into());
    }
    let t: Task = serde_json::from_slice(&fs::read(task)?)?;
    let parent = out.parent().ok_or("worker output directory")?;
    let mut gp = Pari::new(Path::new(GP), so, &parent.join("gp.stderr"))?;
    let gp_pid = gp.child.id();
    let data = if ["rho", "rho-negation", "bsgs", "bsgs-negation", "gp-elllog"]
        .contains(&t.method.as_str())
    {
        engine::reference(&t, &mut gp)?
    } else {
        engine::ic(&t, &mut gp)?
    };
    let transcript = gp.close()?;
    fs::write(parent.join("gp-transcript.txt"), transcript.join("\n"))?;
    write(
        out,
        &json!({"schema":"twist.cold-worker/v1","native_pid":std::process::id(),"gp_pid":gp_pid,"result":data}),
    )
}
fn controls(out: &Path, so: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh controls required".into());
    }
    fs::create_dir_all(out)?;
    let mut gp = Pari::new(Path::new(GP), so, &out.join("gp.stderr"))?;
    let mut rows = vec![];
    for (p, b, n, g) in [(59, 13, 67, [1, 29]), (827, 15, 823, [2, 5])] {
        let f = Fixture {
            id: "development-only".into(),
            p,
            a: 1,
            b,
            order: n,
            r: n,
            d: 2,
            generator: g,
            family: "development-control".into(),
            role: "control".into(),
        };
        for policy in ["none", "augment"] {
            if p == 59 && policy == "augment" {
                continue;
            }
            for family in ["low-x", "curve-orbit", "twist-first", "twist-max"] {
                let base = Task {
                    id: format!("control-{p}-{policy}-{family}"),
                    fixture: f.clone(),
                    target: f.curve().mul(f.g(), 37),
                    target_label: 0,
                    method: "group".into(),
                    family: family.into(),
                    b: 8,
                    shift: 3,
                    alpha: policy.into(),
                    evaluation: "map".into(),
                    compression: false,
                    queries: 16,
                    seed: 20261029,
                };
                let a = engine::ic(&base, &mut gp)?;
                for method in ["group", "poly", "gp-poly"] {
                    for compression in [false, true] {
                        let mut t = base.clone();
                        t.method = method.into();
                        t.compression = compression;
                        let z = engine::ic(&t, &mut gp)?;
                        for field in [
                            "status",
                            "base",
                            "query_targets",
                            "witnesses",
                            "query_rows",
                            "recovered_scalar",
                        ] {
                            assert_eq!(
                                a[field], z[field],
                                "control {p}/{family}/{method}/{compression} {field}"
                            );
                        }
                        if a["fiber"].is_object() {
                            assert_eq!(a["fiber"]["rank"], z["fiber"]["rank"]);
                            assert_eq!(
                                a["fiber"]["kernel"].as_array().unwrap().len(),
                                z["fiber"]["kernel"].as_array().unwrap().len()
                            );
                        }
                        rows.push(json!({"p":p,"family":family,"alpha":policy,"method":method,"compression":compression,"status":z["status"],"all_equal":true}));
                    }
                }
                if p == 827 && family == "twist-max" && policy == "none" {
                    let mut t = base.clone();
                    t.family = "twist-max-scan".into();
                    let z = engine::ic(&t, &mut gp)?;
                    assert_eq!(z["base"], a["base"]);
                    assert_eq!(z["witnesses"], a["witnesses"]);
                    assert_eq!(z["construction"]["seed_order_tests"], 416);
                    rows.push(json!({"p":p,"comparison":"structure discovery vs archived exhaustive rule","order_tests_exhaustive":416,"bases_and_witnesses_identical":true}));
                }
            }
        }
        for method in ["rho", "rho-negation", "bsgs", "bsgs-negation", "gp-elllog"] {
            let t = Task {
                id: format!("reference-{p}-{method}"),
                fixture: f.clone(),
                target: f.curve().mul(f.g(), 37),
                target_label: 0,
                method: method.into(),
                family: "reference".into(),
                b: 0,
                shift: 0,
                alpha: "none".into(),
                evaluation: "map".into(),
                compression: false,
                queries: 0,
                seed: 20261029,
            };
            let z = engine::reference(&t, &mut gp)?;
            assert_eq!(z["recovered_scalar"], 37);
            rows.push(json!({"p":p,"method":method,"status":z["status"],"verified_scalar":37}));
        }
    }
    let transcript = gp.close()?;
    fs::write(out.join("gp-transcript.txt"), transcript.join("\n"))?;
    write(
        out.join("CONTROLS.json"),
        &json!({"schema":"twist.cold-development-controls/v1","rows":rows,"counter_measurement":false,"not_scientific_panel":true}),
    )
}
pub fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str){Some("worker")if args.len()==4=>worker(Path::new(&args[1]),Path::new(&args[2]),Path::new(&args[3])),Some("inputs")if args.len()==4=>inputs(Path::new(&args[1]),Path::new(&args[2]),Path::new(&args[3])),Some("controls")if args.len()==3=>controls(Path::new(&args[1]),Path::new(&args[2])),_=>Err("worker TASK FRESH_OUTPUT ROI_SO | inputs ORIGINAL_FIXTURES FRESH_OUT ROI_SO | controls FRESH_OUT ROI_SO".into())}
}
