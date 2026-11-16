//! Exact original-top regularity from checked disjoint pair-product relations.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const N: usize = 83;
const MASK: u128 = (1u128 << N) - 1;
const LOW: u128 = (1u128 << 45) | 7;
type Row = Vec<u64>;
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap()
}
fn mul(mut a: u128, mut b: u128) -> u128 {
    assert!(a <= MASK && b <= MASK);
    let mut r = 0;
    while b != 0 {
        if b & 1 != 0 {
            r ^= a;
        }
        b >>= 1;
        let c = a >> 82;
        a = (a << 1) & MASK;
        if c != 0 {
            a ^= LOW;
        }
    }
    r
}
fn pow(mut a: u128, mut e: u128) -> u128 {
    let mut r = 1;
    while e != 0 {
        if e & 1 != 0 {
            r = mul(r, a);
        }
        a = mul(a, a);
        e >>= 1;
    }
    r
}
fn slow_mul(a: u128, b: u128) -> u128 {
    let mut bits = [false; 166];
    for i in 0..83 {
        for j in 0..83 {
            if a >> i & 1 != 0 && b >> j & 1 != 0 {
                bits[i + j] ^= true;
            }
        }
    }
    for k in (83..166).rev() {
        if bits[k] {
            for j in [0, 1, 2, 45, 83] {
                bits[k - 83 + j] ^= true;
            }
        }
    }
    (0..83).fold(0, |v, i| v | ((bits[i] as u128) << i))
}
fn field_certificate() -> Value {
    assert!((2..=9).all(|d| 83 % d != 0));
    let f = (1u128 << 83) | LOW;
    assert_eq!(f & 1, 1);
    assert_eq!(f.count_ones() % 2, 1);
    let mut z = 2;
    let mut ladder = vec![z.to_string()];
    for _ in 0..83 {
        let fast = mul(z, z);
        assert_eq!(fast, slow_mul(z, z));
        z = fast;
        ladder.push(z.to_string());
    }
    assert_eq!(z, 2);
    json!({"schema":"n83.binary-field-proof/v1","modulus":"z^83+z^45+z^2+z+1","modulus_bits":format!("{f:x}"),"degree_prime":83,"f_at_0":1,"f_at_1":1,"frobenius_ladder":ladder,"irreducible":true,"argument":"83 is prime. z^(2^83)=z mod f; gcd(f,z^2+z)=1 since f(0)=f(1)=1. Rabin criterion. Every squaring cross-checked by full carryless product and long reduction."})
}
fn bit(r: &Row, j: usize) -> bool {
    r[j / 64] >> (j % 64) & 1 != 0
}
fn flip(r: &mut Row, j: usize) {
    r[j / 64] ^= 1u64 << (j % 64);
}
fn xor(a: &mut Row, b: &Row) {
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter_mut().zip(b) {
        *x ^= *y;
    }
}
fn pivot(r: &Row) -> Option<usize> {
    r.iter().enumerate().rev().find_map(|(i, w)| {
        if *w == 0 {
            None
        } else {
            Some(i * 64 + 63 - w.leading_zeros() as usize)
        }
    })
}
fn hexrow(r: &Row) -> Vec<String> {
    r.iter().map(|x| format!("{x:016x}")).collect()
}
fn basis(rows: &[Row]) -> BTreeMap<usize, (Row, u128)> {
    assert!(rows.len() <= 128);
    let mut b: BTreeMap<usize, (Row, u128)> = BTreeMap::new();
    for (i, row) in rows.iter().enumerate() {
        let mut r = row.clone();
        let mut w = 1u128 << i;
        while let Some(p) = pivot(&r) {
            if let Some((s, v)) = b.get(&p) {
                xor(&mut r, s);
                w ^= v;
            } else {
                b.insert(p, (r, w));
                break;
            }
        }
    }
    b
}
fn decompose(row: &Row, b: &BTreeMap<usize, (Row, u128)>) -> (Row, u128) {
    let mut r = row.clone();
    let mut w = 0;
    while let Some(p) = pivot(&r) {
        if let Some((s, v)) = b.get(&p) {
            xor(&mut r, s);
            w ^= v;
        } else {
            break;
        }
    }
    (r, w)
}
fn coordinate_rows(weights: &[u128], target: u128) -> (Vec<Row>, Vec<Row>) {
    let n = weights.len();
    let words = (n * n).div_ceil(64);
    let mut c = vec![vec![0; words]; 83];
    let mut q = c.clone();
    for i in 0..n {
        for j in 0..n {
            let p = mul(weights[i], weights[j]);
            let a = mul(p, p) ^ mul(target, p);
            for k in 0..83 {
                if p >> k & 1 != 0 {
                    flip(&mut c[k], i * n + j);
                }
                if a >> k & 1 != 0 {
                    flip(&mut q[k], i * n + j);
                }
            }
        }
    }
    (c, q)
}
fn witness_suffix(input: &Value, n: usize) -> String {
    if input["target_kind"] == "natural" {
        assert!(input["witness"].is_null());
        return String::new();
    }
    assert_eq!(input["target_kind"], "planted");
    let w = input["witness"].as_array().unwrap();
    assert_eq!(w.len(), 2 * n);
    let bits: Vec<_> = w
        .iter()
        .map(|v| {
            let s = v.as_str().unwrap();
            assert!(s == "0" || s == "1");
            s
        })
        .collect();
    format!("map witness=r,{};\nif(size(witness(I))!=0){{print(\"WITNESS_FAILED\");exit;}} print(\"WITNESS_OK\");\n",bits.join(","))
}
fn source_template(
    weights: &[u128],
    target: u128,
    b: u128,
    suffix: &str,
) -> (String, Vec<usize>, bool) {
    let n = weights.len();
    let mut poly: BTreeMap<Vec<usize>, u128> = BTreeMap::new();
    poly.insert(vec![], b);
    for i in 0..n {
        let a = mul(mul(weights[i], weights[i]), mul(target, target));
        poly.insert(vec![i], a);
        poly.insert(vec![n + i], a);
        for j in 0..n {
            let c = mul(weights[i], weights[j]);
            poly.insert(vec![i, n + j], mul(c, c) ^ mul(target, c));
        }
    }
    let mut text = format!("ring r=2,(v(1..{})),dp;\nideal I;\n", 2 * n);
    let mut ids = vec![0; 83];
    let mut eq = 0;
    let mut all_bilinear = true;
    for k in 0..83 {
        let terms: Vec<_> = poly.iter().filter(|(_, c)| *c >> k & 1 != 0).collect();
        if terms.is_empty() {
            continue;
        }
        eq += 1;
        ids[k] = eq;
        all_bilinear &= terms.iter().map(|(m, _)| m.len()).max() == Some(2);
        let expr = terms
            .iter()
            .map(|(m, _)| {
                if m.is_empty() {
                    "1".into()
                } else {
                    m.iter()
                        .map(|v| format!("v({})", v + 1))
                        .collect::<Vec<_>>()
                        .join("*")
                }
            })
            .collect::<Vec<_>>()
            .join("+");
        text += &format!("I[{eq}]={expr};\n");
    }
    for i in 0..2 * n {
        eq += 1;
        text += &format!("I[{eq}]=v({})^2-v({});\n", i + 1, i + 1);
    }
    text += suffix;
    (text, ids, all_bilinear)
}
fn inverse(m: &[Vec<u128>]) -> Option<Vec<Vec<u128>>> {
    let n = m.len();
    let mut a = vec![vec![0; 2 * n]; n];
    for i in 0..n {
        assert_eq!(m[i].len(), n);
        a[i][..n].copy_from_slice(&m[i]);
        a[i][n + i] = 1;
    }
    for c in 0..n {
        let p = (c..n).find(|i| a[*i][c] != 0)?;
        a.swap(p, c);
        let inv = pow(a[c][c], MASK - 1);
        assert_eq!(mul(a[c][c], inv), 1);
        for j in 0..2 * n {
            a[c][j] = mul(a[c][j], inv);
        }
        for i in 0..n {
            if i == c {
                continue;
            }
            let q = a[i][c];
            for j in 0..2 * n {
                a[i][j] ^= mul(q, a[c][j]);
            }
        }
    }
    Some(a.iter().map(|r| r[n..].to_vec()).collect())
}
fn check_inverse(m: &[Vec<u128>], v: &[Vec<u128>]) -> bool {
    let n = m.len();
    for i in 0..n {
        for j in 0..n {
            let a = (0..n).fold(0, |x, k| x ^ mul(m[i][k], v[k][j]));
            let b = (0..n).fold(0, |x, k| x ^ mul(v[i][k], m[k][j]));
            if a != u128::from(i == j) || b != u128::from(i == j) {
                return false;
            }
        }
    }
    true
}
fn matrix_json(m: &[Vec<u128>]) -> Value {
    json!(m
        .iter()
        .map(|r| r.iter().map(u128::to_string).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn choose(n: usize, k: usize) -> u64 {
    if k > n {
        return 0;
    }
    let mut v = 1u128;
    for j in 0..k.min(n - k) {
        v = v * (n - j) as u128 / (j + 1) as u128;
    }
    v.try_into().unwrap()
}
fn pair_certificate(weights: &[u128], q: &[Row]) -> Value {
    let n = weights.len();
    assert!(n > 0 && n <= 27);
    let (c, _) = coordinate_rows(weights, 0);
    let bas = basis(q);
    let mut witnesses = vec![];
    let mut failures = vec![];
    let mut proof_rows = vec![];
    for (k, row) in c.iter().enumerate() {
        let (rem, w) = decompose(row, &bas);
        let mut check = vec![0; row.len()];
        for j in 0..83 {
            if w >> j & 1 != 0 {
                xor(&mut check, &q[j]);
            }
        }
        assert_eq!(
            check
                .iter()
                .zip(&rem)
                .map(|(a, b)| a ^ b)
                .collect::<Vec<_>>(),
            *row
        );
        if pivot(&rem).is_some() {
            failures.push(json!({"coordinate":k,"remainder":hexrow(&rem)}));
        }
        witnesses.push(w);
        proof_rows.push(json!({"coordinate":k,"product_row":hexrow(row),"original_coordinate_rows_xor_mask":format!("{w:021x}")}));
    }
    if !failures.is_empty() {
        return json!({"status":"product_span_not_contained","q_rank":bas.len(),"product_rank":basis(&c).len(),"failed_coordinates":failures,"coordinate_witnesses":proof_rows});
    }
    let mut m = vec![weights.to_vec()];
    for k in 1..n {
        m.push(m[k - 1].iter().map(|a| mul(*a, *a)).collect());
    }
    let inv = inverse(&m).expect("singular coordinate matrix");
    assert!(check_inverse(&m, &inv));
    let mut frob_basis: Vec<_> = (0..83).map(|i| 1u128 << i).collect();
    let mut pairs = vec![];
    for k in 0..n {
        let mut coeff = vec![0u128; 83];
        for r in 0..83 {
            for j in 0..83 {
                if witnesses[r] >> j & 1 != 0 {
                    coeff[j] ^= frob_basis[r];
                }
            }
        }
        let mut all_coeff = vec![];
        for i in 0..n {
            for j in 0..n {
                let got = (0..83)
                    .filter(|r| bit(&q[*r], i * n + j))
                    .fold(0, |a, r| a ^ coeff[r]);
                let want = mul(m[k][i], m[k][j]);
                assert_eq!(got, want, "pair-product coefficient");
                all_coeff.extend_from_slice(&got.to_le_bytes());
            }
        }
        pairs.push(json!({"pair":k,"original_coordinate_coefficients":coeff.iter().map(u128::to_string).collect::<Vec<_>>(),"verified_coefficients":n*n,"coefficient_u128le_sha256":sha(&all_coeff)}));
        for a in &mut frob_basis {
            *a = mul(*a, *a);
        }
    }
    let lower: Vec<_> = (0..=2 * n + 1).map(|d| choose(n, d)).collect();
    let upper: Vec<_> = (0..=2 * n + 1)
        .map(|d| {
            if d <= n {
                choose(n, d) * (1u64 << d)
            } else {
                0
            }
        })
        .collect();
    json!({"schema":"n83.binary-pair-upper/v1","status":"exact","ell":n,"dreg_exact":n+1,"dreg_lower_bound":n+1,"dreg_upper_bound":n+1,"q_rank":bas.len(),"product_rank":basis(&c).len(),"original_coordinate_rows":q.iter().map(hexrow).collect::<Vec<_>>(),"coordinate_witnesses":proof_rows,"moore_matrix":matrix_json(&m),"inverse":matrix_json(&inv),"both_inverse_products_verified":true,"pair_relations":pairs,"hilbert_lower_bound":lower,"hilbert_upper_bound":upper,"hilbert_function_exact":false,"upper_quotient":"K[A_i,B_i]/(A_i^2,B_i^2,A_i*B_i), i=0..ell-1; Hilbert series (1+2t)^ell","lower_quotient":"F2[a_i]/(a_i^2), Hilbert series (1+t)^ell; requires every other highest form cross-bilinear"})
}
fn rank_generic(rows: Vec<Row>) -> usize {
    let mut b: BTreeMap<usize, Row> = BTreeMap::new();
    for mut r in rows {
        while let Some(p) = pivot(&r) {
            if let Some(s) = b.get(&p) {
                xor(&mut r, s);
            } else {
                b.insert(p, r);
                break;
            }
        }
    }
    b.len()
}
fn explicit_hilbert(n: usize, q: &[Row]) -> Vec<usize> {
    let mut h = vec![];
    for d in 0..=2 * n {
        let columns: Vec<_> = (0usize..1 << (2 * n))
            .filter(|x| x.count_ones() as usize == d)
            .collect();
        let map: BTreeMap<_, _> = columns.iter().enumerate().map(|(i, x)| (*x, i)).collect();
        let mut rows = vec![];
        if d >= 2 {
            for mon in (0usize..1 << (2 * n)).filter(|x| x.count_ones() as usize == d - 2) {
                for qr in q {
                    let mut r = vec![0; columns.len().div_ceil(64)];
                    for i in 0..n {
                        for j in 0..n {
                            let term = (1 << i) | (1 << (n + j));
                            if mon & term == 0 && bit(qr, i * n + j) {
                                flip(&mut r, map[&(mon | term)]);
                            }
                        }
                    }
                    rows.push(r);
                }
            }
        }
        h.push(columns.len() - rank_generic(rows));
    }
    h.push(0);
    h
}
fn controls() -> Value {
    let mut arithmetic = 0;
    let mut s = 0x123456789abcdefu128;
    for _ in 0..128 {
        s = (s.wrapping_mul(6364136223846793005).wrapping_add(17)) & MASK;
        let a = s;
        s = (s.wrapping_mul(1442695040888963407).wrapping_add(91)) & MASK;
        assert_eq!(mul(a, s), slow_mul(a, s));
        arithmetic += 1;
    }
    let field = field_certificate();
    let weights: Vec<_> = (0..4).map(|i| 1u128 << i).collect();
    let mut s3_checks = 0;
    for target in [1, 1u128 << 82] {
        let (_, q) = coordinate_rows(&weights, target);
        for assignment in 0usize..256 {
            let mut x = 0;
            let mut y = 0;
            for (i, w) in weights.iter().enumerate() {
                if assignment >> i & 1 != 0 {
                    x ^= w;
                }
                if assignment >> (4 + i) & 1 != 0 {
                    y ^= w;
                }
            }
            let direct = mul(mul(x, x), mul(y, y))
                ^ mul(mul(x, x), mul(target, target))
                ^ mul(mul(y, y), mul(target, target))
                ^ mul(mul(x, y), target)
                ^ 1;
            let mut descended = 1;
            for i in 0..4 {
                if (assignment >> i ^ assignment >> (4 + i)) & 1 != 0 {
                    descended ^= mul(mul(weights[i], weights[i]), mul(target, target));
                }
                for j in 0..4 {
                    if assignment >> i & 1 != 0 && assignment >> (4 + j) & 1 != 0 {
                        for k in 0..83 {
                            if bit(&q[k], i * 4 + j) {
                                descended ^= 1u128 << k;
                            }
                        }
                    }
                }
            }
            assert_eq!(direct, descended);
            s3_checks += 1;
        }
    }
    let mut rows = vec![];
    for n in 1..=4 {
        let w: Vec<_> = (0..n).map(|i| 1u128 << i).collect();
        for (t, positive) in [(1u128 << 82, true), (1, false)] {
            let (_, q) = coordinate_rows(&w, t);
            let cert = pair_certificate(&w, &q);
            let h = explicit_hilbert(n, &q);
            let degree = h.iter().position(|v| *v == 0).unwrap();
            if positive {
                assert_eq!(cert["status"], "exact");
                assert_eq!(degree, n + 1);
            } else {
                assert_eq!(cert["status"], "product_span_not_contained");
            }
            rows.push(json!({"ell":n,"target":t.to_string(),"expected_product_inclusion":positive,"certificate_status":cert["status"],"explicit_hilbert_function":h,"explicit_dreg":degree}));
        }
    }
    let mut bad = vec![vec![1u128, 1], vec![1, 1]];
    assert!(inverse(&bad).is_none());
    bad[1][1] = 2;
    let inv = inverse(&bad).unwrap();
    assert!(check_inverse(&bad, &inv));
    let mut broken = inv.clone();
    broken[0][0] ^= 1;
    assert!(!check_inverse(&bad, &broken));
    json!({"schema":"n83.binary-pair-controls/v1","arithmetic_cross_checks":arithmetic,"s3_boolean_assignments_checked":s3_checks,"irreducibility_verified":field["irreducible"],"small_matrix_cases":rows,"singular_and_corrupted_inverse_rejected":true,"all_passed":true})
}
fn certify(case: &Value, root: &Path) -> Value {
    let dir = root.join("inputs").join(case["key"].as_str().unwrap());
    let bytes = fs::read(dir.join("input.json")).unwrap();
    assert_eq!(sha(&bytes), case["input_sha256"].as_str().unwrap());
    let input: Value = serde_json::from_slice(&bytes).unwrap();
    let text = fs::read_to_string(dir.join("system.sing")).unwrap();
    assert_eq!(
        sha(text.as_bytes()),
        case["source_sha256"].as_str().unwrap()
    );
    assert_eq!(input["source_sha256"], case["source_sha256"]);
    for (k, v) in [("n", 83), ("ell", 27), ("m", 2), ("n_vars", 54)] {
        assert_eq!(input[k], v);
    }
    assert!(input["family"] == "binary" || input["family"] == "koblitz");
    assert_eq!(input["field_modulus"], "z^83+z^45+z^2+z+1");
    assert_eq!(
        input["factor_base_manifest_sha256"],
        case["factor_base_manifest_sha256"]
    );
    let arm = case["dataset"].as_str().unwrap();
    assert!(["original", "baseline", "spike40"].contains(&arm));
    let pos: Vec<_> = (0..27)
        .map(|i| if i == 26 && arm == "spike40" { 40 } else { i })
        .collect();
    if arm != "original" {
        assert_eq!(input["arm"], arm);
        assert_eq!(input["coordinate_bit_positions"], json!(pos));
    }
    let w: Vec<_> = pos.iter().map(|i| 1u128 << i).collect();
    let target = input["target_x"].as_str().unwrap().parse::<u128>().unwrap();
    let b = input["curve_b"].as_str().unwrap().parse::<u128>().unwrap();
    assert!(target <= MASK);
    if input["family"] == "koblitz" {
        assert_eq!(input["curve_a"], "0");
        assert_eq!(b, 1);
    } else {
        assert_eq!(input["curve_a"], "1");
        assert_eq!(b, (1u128 << 82) | 0x51b);
    }
    let (expected, ids, lower) = source_template(&w, target, b, &witness_suffix(&input, 27));
    assert_eq!(
        text, expected,
        "full source template including witness suffix"
    );
    assert_eq!(
        input["input_equations"],
        ids.iter().filter(|i| **i != 0).count() + 54
    );
    assert!(
        lower,
        "non-bilinear original highest form invalidates lower certificate"
    );
    let (_, q) = coordinate_rows(&w, target);
    let mut proof = pair_certificate(&w, &q);
    proof["input"] = input;
    proof["key"] = case["key"].clone();
    proof["coordinate_equation_ids"] = json!(ids);
    proof["source_template_verified"] = json!(true);
    proof["lower_projection_verified"] = json!(true);
    proof["lower_witness_monomial_variables_zero_based"] = json!((0..27).collect::<Vec<_>>());
    proof["field_certificate_sha256"] =
        json!(sha(&fs::read(root.join("certificates/field.json")).unwrap()));
    proof
}
fn run(root: &Path, replay: bool) {
    let p = read(&root.join("protocol-binary-pair.json"));
    let cases = p["extensions"]["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 12);
    let out = root.join("certificates");
    if !replay {
        fs::create_dir(&out).unwrap();
    }
    for (name, v) in [("field", field_certificate()), ("controls", controls())] {
        let path = out.join(format!("{name}.json"));
        if replay {
            assert_eq!(read(&path), v);
        } else {
            save(&path, &v);
        }
    }
    let mut rows = vec![];
    let mut errors = 0;
    for case in cases {
        let key = case["key"].as_str().unwrap();
        let computed = std::panic::catch_unwind(|| certify(case, root));
        match computed {
            Ok(proof) => {
                let path = out.join(format!("{key}.json"));
                if replay {
                    assert_eq!(read(&path), proof);
                } else {
                    save(&path, &proof);
                }
                let exact = proof["status"] == "exact";
                if !exact {
                    errors += 1;
                }
                rows.push(json!({"key":key,"arm":case["arm"],"dataset":case["dataset"],"family":proof["input"]["family"],"m":2,"ell":27,"target_kind":proof["input"]["target_kind"],"source_sha256":proof["input"]["source_sha256"],"factor_base_manifest_sha256":proof["input"]["factor_base_manifest_sha256"],"status":proof["status"],"dreg_exact":proof["dreg_exact"],"dreg_lower_bound":proof["dreg_lower_bound"],"dreg_upper_bound":proof["dreg_upper_bound"],"q_rank":proof["q_rank"],"product_rank":proof["product_rank"],"certificate":format!("{key}.json"),"verified":exact}));
                println!(
                    "{} {} {}",
                    if replay { "replayed" } else { "certified" },
                    key,
                    proof["status"]
                );
            }
            Err(_) => {
                errors += 1;
                rows.push(json!({"key":key,"arm":case["arm"],"status":"error","verified":false}));
            }
        }
    }
    let summary = json!({"schema":"n83.binary-pair-summary/v1","rows":rows,"errors":errors,"all_exact":errors==0,"scope":"Original highest forms; no affine preprocessing. Exact degree, not exact Hilbert function, solving degree or LFD."});
    if replay {
        assert_eq!(read(&out.join("summary.json")), summary);
        save(
            &root.join("replay.json"),
            &json!({"schema":"n83.binary-pair-replay/v1","cases":12,"all_passed":errors==0,"summary_sha256":sha(&fs::read(out.join("summary.json")).unwrap())}),
        );
    } else {
        save(&out.join("summary.json"), &summary);
    }
    assert_eq!(errors, 0, "incomplete certificates preserved");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "controls" => save(Path::new(&a[2]), &controls()),
        "run" | "replay" => run(Path::new(&a[2]), a[1] == "replay"),
        _ => panic!("run ROOT | replay ROOT | controls OUT"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_explicit_controls() {
        assert_eq!(controls()["all_passed"], true);
    }
    #[test]
    fn xor_witness_and_mutation() {
        let mut rows = vec![vec![3u64], vec![6]];
        let b = basis(&rows);
        let (rem, w) = decompose(&vec![5], &b);
        assert_eq!(rem, vec![0]);
        assert_eq!(w, 3);
        rows[0][0] = 1;
        assert_ne!(decompose(&vec![5], &basis(&rows)).0, vec![0]);
    }
    #[test]
    fn pair_slot_hilbert_upper() {
        for n in 1..=6 {
            let mut counts = vec![0u64; 2 * n + 1];
            for mask in 0usize..1 << (2 * n) {
                if (0..n).all(|i| mask >> i & 1 == 0 || mask >> (n + i) & 1 == 0) {
                    counts[mask.count_ones() as usize] += 1;
                }
            }
            for (d, v) in counts.iter().enumerate() {
                assert_eq!(*v, if d <= n { choose(n, d) * (1 << d) } else { 0 });
            }
        }
    }
    #[test]
    fn natural_and_planted_suffix() {
        assert_eq!(
            witness_suffix(&json!({"target_kind":"natural","witness":null}), 2),
            ""
        );
        assert!(witness_suffix(
            &json!({"target_kind":"planted","witness":["0","1","0","1"]}),
            2
        )
        .starts_with("map witness=r,0,1,0,1;"));
        assert!(std::panic::catch_unwind(|| witness_suffix(
            &json!({"target_kind":"planted","witness":["0","1","2","1"]}),
            2
        ))
        .is_err());
    }
    #[test]
    fn expected_source_constants_and_fields() {
        let (text, ids, _) = source_template(&[1, 2], 1 << 82, 1, "");
        assert!(text.starts_with("ring r=2,(v(1..4)),dp;\nideal I;\nI[1]=1"));
        let count = ids.iter().filter(|x| **x != 0).count();
        assert!(text.ends_with(&format!("I[{}]=v(4)^2-v(4);\n", count + 4)));
    }
}
