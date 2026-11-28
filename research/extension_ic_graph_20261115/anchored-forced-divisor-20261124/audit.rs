//! Select/replay one public one-LP anchor, without emitting raw target fields.
use super::forced_divisor::Family;
use crypto_lib::cryptanalysis::jv_cover::{Div, Fld, Fq, Hyp, E2};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const RAW_SHA: &str = "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d";
const H_SHA: &str = "37554399c543d282fcfe6e58021b2fdf8f329178cade0aea19944259523a939a";
fn ensure(b: bool, s: &str) -> Result<()> {
    if b {
        Ok(())
    } else {
        Err(s.into())
    }
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn native(v: &Value) -> Result<Vec<E2>> {
    Ok(serde_json::from_value::<Vec<[u64; 2]>>(v.clone())?
        .into_iter()
        .map(E2)
        .collect())
}
fn encode(v: &[E2]) -> Vec<[u64; 2]> {
    v.iter().map(|e| e.0).collect()
}
fn selected(sources: &[Value]) -> Option<usize> {
    sources.iter().position(|s| {
        s["certificate"]["remainders"]
            .as_array()
            .is_some_and(|r| r.len() == 1)
            && s["certificate"]["terms"]
                .as_array()
                .is_some_and(|r| r.len() == 7)
    })
}
fn public_small(base: &[Value], terms: &[(usize, i8)]) -> Result<Vec<Value>> {
    ensure(
        terms.len() == 7 && terms.windows(2).all(|w| w[0].0 < w[1].0),
        "seven distinct ordered small indices",
    )?;
    terms
        .iter()
        .map(|&(i, sign)| {
            ensure([-1, 1].contains(&sign), "unit small sign")?;
            let b = base.get(i).ok_or("public base index")?;
            let x = b["x"].as_u64().ok_or("base x")?;
            let y: [u64; 2] = serde_json::from_value(b["y"].clone())?;
            Ok(json!({"original_index":i,"x":x,"y":y,"sign":sign}))
        })
        .collect()
}
fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    writeln!(file)?;
    file.sync_all()?;
    Ok(())
}
fn audit(raw_path: &str, h_path: &str, out: &str) -> Result<()> {
    let out = Path::new(out);
    ensure(!out.exists(), "exclusive audit output")?;
    let bytes = fs::read(raw_path)?;
    ensure(sha(&bytes) == RAW_SHA, "pinned retained source")?;
    let h_bytes = fs::read(h_path)?;
    ensure(sha(&h_bytes) == H_SHA, "pinned public H fixture")?;
    fs::create_dir(out)?;
    write_new(
        &out.join("started.json"),
        &json!({"schema":"graph-ic.anchor-audit-start/v1","raw_sha256":RAW_SHA,"h_fixture_sha256":H_SHA}),
    )?;
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(120));
        eprintln!("audit watchdog expired");
        std::process::exit(124);
    });
    let raw: Value = serde_json::from_slice(&bytes)?;
    let input: Value = serde_json::from_slice(&h_bytes)?;
    let collection = &raw["work"]["collection"];
    let sources = collection["sources"].as_array().ok_or("source list")?;
    let base = collection["base"].as_array().ok_or("public base list")?;
    let mut histogram = [0u64; 4];
    for s in sources {
        let count = s["certificate"]["remainders"]
            .as_array()
            .ok_or("source cofactor list")?
            .len();
        histogram[count.min(3)] += 1;
    }
    let Some(index) = selected(sources) else {
        return write_new(
            &out.join("summary.json"),
            &json!({"schema":"graph-ic.anchor-audit/v1","raw_sha256":RAW_SHA,"source_count":sources.len(),"cofactor_histogram_0_1_2_more":histogram,"selected_source":null,"valid_anchor":false,"new_system_solves":0,"signed_verification":false}),
        );
    };
    let row = &sources[index]["certificate"];
    let terms: Vec<(usize, i8)> = serde_json::from_value(row["terms"].clone())?;
    let small = public_small(base, &terms)?;
    let f = Fq::new(1009);
    ensure(
        input["p"] == 1009 && input["raw_sha256"] == RAW_SHA,
        "public H domain",
    )?;
    let h = native(&input["h"])?;
    let remainder = &row["remainders"][0];
    let key: [u64; 6] = serde_json::from_value(remainder["key"].clone())?;
    let sign = remainder["coefficient"].as_i64().ok_or("remainder sign")?;
    ensure(
        [-1, 1].contains(&sign) && key.iter().all(|&x| x < f.p),
        "canonical signed quadratic",
    )?;
    let u = vec![f.from_fp(key[0]), f.from_fp(key[1]), E2::ONE];
    let mut v = vec![E2([key[2], key[3]]), E2([key[4], key[5]])];
    if sign == -1 {
        for x in &mut v {
            *x = f.neg(x);
        }
    }
    while v.last() == Some(&E2::ZERO) {
        v.pop();
    }
    let family = Family::new(&f, &h, &u, &v)?;
    let a = native(&row["a"])?;
    let c = native(&row["c"])?;
    let point = family.recover_parameters(&f, &a, &c)?;
    let function = family.instantiate(&f, &point)?;
    let norm: Vec<u64> = serde_json::from_value(row["norm"].clone())?;
    ensure(
        norm.len() == 10 && norm.iter().all(|&x| x < f.p),
        "canonical degree9 norm",
    )?;
    let scale = f.inv(&f.sq(&c[1]));
    let expected: Vec<_> = norm.iter().map(|&x| f.mul(&f.from_fp(x), &scale)).collect();
    ensure(function.norm == expected, "original normalized norm replay")?;
    let roots: Vec<_> = small.iter().map(|b| b["x"].as_u64().unwrap()).collect();
    let support = family.materialize(&f, &function, &roots)?;
    ensure(
        support.quadratic
            == vec![Div {
                u: u.clone(),
                v: v.clone(),
            }],
        "forced quadratic support",
    )?;
    let mut expected_small = vec![];
    for b in &small {
        let x = b["x"].as_u64().ok_or("small x")?;
        let y: [u64; 2] = serde_json::from_value(b["y"].clone())?;
        ensure(
            x < f.p && y.iter().all(|&x| x < f.p),
            "canonical small point",
        )?;
        let y = if b["sign"] == 1 { E2(y) } else { f.neg(&E2(y)) };
        expected_small.push((x, y));
    }
    ensure(
        support.small == expected_small,
        "actual original small ordinates",
    )?;
    let jac = Hyp {
        f: &f,
        h: h.clone(),
        g: 3,
    };
    let mut sum = jac.identity();
    for (x, y) in &support.small {
        sum = jac.add(
            &sum,
            &Div {
                u: vec![f.neg(&f.from_fp(*x)), E2::ONE],
                v: vec![*y],
            },
        );
    }
    sum = jac.add(&sum, &support.quadratic[0]);
    ensure(jac.is_identity(&sum), "original Jacobian anchor relation")?;
    let public = json!({"schema":"graph-ic.anchored-forced-divisor.fixture/v1","raw_sha256":RAW_SHA,"p":1009,"source_id":index,"h":encode(&h),"a":row["a"],"c":row["c"],"norm":norm,"small":small,"remainders":[{"key":key,"coefficient":sign}],"normalized_coordinates":point});
    write_new(&out.join("fixture.json"), &public)?;
    write_new(
        &out.join("summary.json"),
        &json!({"schema":"graph-ic.anchor-audit/v1","raw_sha256":RAW_SHA,"h_fixture_sha256":H_SHA,"source_count":sources.len(),"cofactor_histogram_0_1_2_more":histogram,"selected_source":index,"valid_anchor":true,"fixture_sha256":sha(&fs::read(out.join("fixture.json"))?),"norm_support_group_replay":true,"legacy_fq_products":f.muls(),"new_system_solves":0,"signed_verification":false,"new_graph_rows":0}),
    )
}
pub fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    ensure(
        a.len() == 5 && a[1] == "audit",
        "audit RETAINED_RAW PUBLIC_H NEW_OUTPUT",
    )?;
    audit(&a[2], &a[3], &a[4])
}
#[cfg(test)]
mod controls {
    use super::*;
    #[test]
    fn first_one_lp_is_deterministic_and_absence_is_explicit() {
        let one = json!({"certificate":{"remainders":[{}],"terms":[0,1,2,3,4,5,6]}});
        let two = json!({"certificate":{"remainders":[{},{}],"terms":[0,1,2,3,4]}});
        assert_eq!(selected(&[two.clone(), one.clone(), one]), Some(1));
        assert_eq!(selected(&[two]), None);
    }
    #[test]
    fn public_point_allowlist_drops_answers_and_bad_indices() {
        let base: Vec<_> = (0..7)
            .map(|x| json!({"x":x,"y":[1,0],"target_answer":99}))
            .collect();
        let terms: Vec<_> = (0..7).map(|x| (x, 1)).collect();
        let public = public_small(&base, &terms).unwrap();
        assert!(public
            .iter()
            .all(|b| b.get("target_answer").is_none() && b.as_object().unwrap().len() == 4));
        let mut bad = terms.clone();
        bad[6].0 = 7;
        assert!(public_small(&base, &bad).is_err());
        bad = terms;
        bad[6].1 = 0;
        assert!(public_small(&base, &bad).is_err());
    }
}
