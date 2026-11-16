//! Native export of a kernel-checked fixed fixture; no search or timing run.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

mod hash {
    pub mod sha256 {
        use sha2::{Digest, Sha256};
        pub fn sha256(bytes: &[u8]) -> [u8; 32] {
            Sha256::digest(bytes).into()
        }
    }
}
#[allow(dead_code)]
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity;
use identity::{Expr as E, IdentityStatement, Verdict};

const COORDS: &[(u64, u64)] = &[
    (0, 6),
    (3, 1),
    (5, 1),
    (6, 3),
    (7, 6),
    (9, 1),
    (10, 6),
    (16, 4),
];
#[cfg(test)]
const OLD: &[(u64, u64)] = &[(5, 1), (6, 3), (10, 6), (16, 4)];

fn digest(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
#[cfg(test)]
fn graph(x: u64) -> u64 {
    (4 * x * x + 9 * x + 9) % 17
}
#[cfg(test)]
fn curve(x: u64) -> u64 {
    (x * x * x + 2 * x + 2) % 17
}
#[cfg(test)]
fn label_member(x: u64, y: u64) -> bool {
    [0, 2, 10, 12].into_iter().any(|t| {
        let h = (graph(x) + t) % 17;
        y == h || y == (17 - h) % 17
    })
}

fn dump(coords: &[(u64, u64)]) -> Value {
    let prior: Value = serde_json::from_str(include_str!("../../secant/fixture.json")).unwrap();
    let curve = prior["candidate"]["curve"].clone();
    let slug = curve["slug"].as_str().unwrap();
    let mut points = Vec::new();
    let mut keys = Vec::new();
    for (col, &(x, y)) in coords.iter().enumerate() {
        for (v, c) in [(y, 1), (17 - y, 18)] {
            points.push(json!({"x":format!("0x{x:x}"),"y":format!("0x{v:x}"),"col":col,"coef":c}));
            keys.push(((x + 1) << 1) | u64::from(v > 17 - v));
        }
    }
    keys.sort_unstable();
    let bytes: Vec<_> = keys.into_iter().flat_map(u64::to_be_bytes).collect();
    let points_sha = digest(&bytes);
    let params = json!({"xs":coords.iter().map(|p|p.0.to_string()).collect::<Vec<_>>().join(";")});
    let preimage = json!({"schema":"ecbench.factor_base/v1","curve":slug,"family":"explicit-x","params":params,"columns":coords.len(),"signed_points":points.len(),"points_sha256":points_sha});
    let sha = digest(&serde_json::to_vec(&preimage).unwrap());
    json!({"schema":"ecbench.factor_base_dump/v1","curve":curve,
        "factor_base":{"fb_id":format!("FB1h{}",&sha[..12]),"fb_sha256":sha,"family":"explicit-x","params":params,"description":"explicit x coordinates; both signs, one column per supplied x","signed_points":points.len(),"abscissae":coords.len(),"columns":coords.len(),"dimension":null,"points_sha256":points_sha},
        "build_adds":0,"build_doubles":0,"points":points})
}

fn membership_coefficients() -> Vec<u64> {
    COORDS.iter().fold(vec![1], |p, &(x, _)| {
        let mut q = vec![0; p.len() + 1];
        for (i, &c) in p.iter().enumerate() {
            q[i] = (q[i] + c * (17 - x)) % 17;
            q[i + 1] = (q[i + 1] + c) % 17;
        }
        q
    })
}

fn statement() -> IdentityStatement {
    let [d, l, u] = ["d", "l", "u"].map(E::var);
    let lu = E::product(vec![l.clone(), u.clone()]);
    let q = E::sum(vec![d.clone(), lu.clone()]);
    let complement = E::sum(vec![d.clone(), l.clone(), E::minus(lu)]);
    let bool_eq = E::sum(vec![E::product(vec![u.clone(), u.clone()]), E::minus(u)]);
    IdentityStatement {
        name: "Exact elimination of one affine Boolean selector".into(),
        source: "PencilElimination.elimination_identity; complete root projection requires a field"
            .into(),
        variables: vec!["d".into(), "l".into(), "u".into()],
        degree_bound: 4,
        lhs: E::product(vec![d.clone(), E::sum(vec![d, l.clone()])]),
        rhs: E::sum(vec![
            E::product(vec![q, complement]),
            E::product(vec![l.clone(), l, bool_eq]),
        ]),
    }
}

fn write_once(path: &Path, value: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(path)?;
    f.write_all(serde_json::to_string_pretty(value)?.as_bytes())?;
    f.write_all(b"\n")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 || args[1] != "emit" {
        return Err("usage: pencil-offset-fixture emit NEW_OUTPUT_DIR".into());
    }
    let out = Path::new(&args[2]);
    fs::create_dir(out)?;
    let data = dump(COORDS);
    let id = data["factor_base"]["fb_id"].as_str().unwrap();
    write_once(&out.join(format!("{id}.json")), &data)?;
    let cert=identity::issue(statement(),1101,32,
        "same-session native issuer and replay; Lean kernel checks the universal identity separately",None)?;
    assert!(matches!(identity::check(&cert)?, Verdict::Accept { .. }));
    write_once(&out.join("identity.json"), &serde_json::to_value(cert)?)?;
    write_once(
        &out.join("fixture.json"),
        &json!({
            "schema":"quadratic-ordinate-pencil-fixture/v1","curve":data["curve"],"factor_base":data["factor_base"],
            "construction":"hand-derived mathematical fixture, not a prospective search trial",
            "graph_coefficients_low_to_high":[9,9,4],"offset_origin":0,"offset_weights":[2,10],"offsets":[0,2,10,12],
            "positive_fibre_sizes":[4,1,2,2],"positive_labelled_points":9,"signed_labelled_points":18,
            "deduplicated_signed_points":16,"duplicate_signed_labels":2,
            "membership_coefficients_low_to_high":membership_coefficients(),
            "canonical_homogeneous_degree":7,"same_base_dense_degree":9,
            "measured_f4_degree":null,"measured_relation_yield":null,"measured_runtime":null,
            "global_selector_variables":7,"one_bit_projection_variables":5,
            "projection_canonical_degree":"not separately formalized",
            "degree_metric":"first vanishing of canonical highest-form quotient; not F4 degree"
        }),
    )?;
    println!(
        "{}",
        json!({"factor_base":id,"signed_points":16,"columns":8,"fixture_only":true})
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_membership_agrees_with_kernel_fixture() {
        for x in 0..17 {
            for y in 0..17 {
                if y * y % 17 == curve(x) {
                    assert_eq!(label_member(x, y), COORDS.iter().any(|p| p.0 == x));
                }
            }
        }
        for &(x, y) in COORDS {
            assert_eq!(y * y % 17, curve(x));
            assert!(y < 17 - y);
        }
    }
    #[test]
    fn fb1_codec_reproduces_existing_record() {
        assert_eq!(dump(OLD)["factor_base"]["fb_id"], "FB1hed5b19904cc8");
    }
    #[test]
    fn known_new_column_pair_replays() {
        let (x, y, z, v, m) = (0u64, 6u64, 3u64, 1u64, 4u64);
        assert!(label_member(x, y) && label_member(z, v));
        let r = (m * m + 34 - x - z) % 17;
        let s = (m * ((x + 17 - r) % 17) + 17 - y) % 17;
        assert_eq!((r, s), (13, 10));
        assert_eq!((v + 17 - y) % 17, (m * ((z + 17 - x) % 17)) % 17);
    }
    #[test]
    fn identity_certificate_rejects_changed_statement() {
        let c = identity::issue(statement(), 1101, 32, "development test", None).unwrap();
        assert!(matches!(
            identity::check(&c).unwrap(),
            Verdict::Accept { .. }
        ));
        let mut changed = c;
        changed.statement.lhs = E::sum(vec![changed.statement.lhs, E::Const(1)]);
        assert!(identity::check(&changed).is_err());
    }
}
