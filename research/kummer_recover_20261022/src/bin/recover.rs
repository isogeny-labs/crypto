//! Signed Recover with the locally verified Mumford-coordinate equations.
use five_leads_study::three_leads::{Divisor, Field, Jacobian};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs};
fn m(x: u64, y: u64, p: u64) -> u64 {
    x * y % p
}
fn a(x: u64, y: u64, p: u64) -> u64 {
    (x + y) % p
}
fn s(x: u64, y: u64, p: u64) -> u64 {
    (x + p - y) % p
}
fn pow(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n & 1 == 1 {
            z = m(z, x, p)
        }
        x = m(x, x, p);
        n >>= 1
    }
    z
}
fn inv(x: u64, p: u64) -> u64 {
    assert_ne!(x, 0);
    pow(x, p - 2, p)
}
fn vec4(v: &Value) -> [u64; 4] {
    std::array::from_fn(|i| v[i].as_u64().unwrap())
}
fn matrix(v: &Value) -> [[u64; 4]; 4] {
    std::array::from_fn(|i| vec4(&v[i]))
}
fn linear(x: [u64; 4], m0: [[u64; 4]; 4], p: u64) -> [u64; 4] {
    std::array::from_fn(|i| (0..4).fold(0, |z, j| a(z, m(m0[i][j], x[j], p), p)))
}
fn h(x: [u64; 4], p: u64) -> [u64; 4] {
    let [w, x, y, z] = x;
    [
        a(a(w, x, p), a(y, z, p), p),
        s(a(w, x, p), a(y, z, p), p),
        a(s(w, x, p), s(y, z, p), p),
        s(s(w, x, p), s(y, z, p), p),
    ]
}
fn xadd(x: [u64; 4], y: [u64; 4], d: [u64; 4], t: [u64; 4], p: u64) -> Option<[u64; 4]> {
    let hx = h(x, p);
    let hy = h(y, p);
    let z = h(
        std::array::from_fn(|i| m(m(hx[i], hy[i], p), inv(t[i], p), p)),
        p,
    );
    let out = std::array::from_fn(|i| {
        (0..4)
            .filter(|&j| j != i)
            .fold(m(z[i], z[i], p), |v, j| m(v, d[j], p))
    });
    if out == [0; 4] {
        None
    } else {
        Some(out)
    }
}
fn xadd_repaired(
    x: [u64; 4],
    y: [u64; 4],
    d: [u64; 4],
    t: [u64; 4],
    branches: &[(u64, [[u64; 4]; 4])],
    p: u64,
) -> (Option<[u64; 4]>, Option<u64>) {
    if let Some(z) = xadd(x, y, d, t, p) {
        return (Some(z), None);
    }
    for &(root, act) in branches {
        let tx = linear(x, act, p);
        let td = linear(d, act, p);
        if let Some(z) = xadd(tx, y, td, t, p) {
            return (Some(linear(z, act, p)), Some(root));
        }
    }
    (None, None)
}
fn solve4(mut mat: [[u64; 5]; 4], p: u64) -> Option<[u64; 4]> {
    for col in 0..4 {
        let pivot = (col..4).find(|&i| mat[i][col] != 0)?;
        mat.swap(col, pivot);
        let unit = inv(mat[col][col], p);
        for j in col..5 {
            mat[col][j] = m(mat[col][j], unit, p)
        }
        for i in 0..4 {
            if i != col {
                let z = mat[i][col];
                for j in col..5 {
                    mat[i][j] = s(mat[i][j], m(z, mat[col][j], p), p)
                }
            }
        }
    }
    Some(std::array::from_fn(|i| mat[i][4]))
}
fn general(k: [u64; 4], theta: &Value, p: u64) -> Option<[u64; 4]> {
    let (l, mu, nu) = (
        theta["lambda"].as_u64().unwrap(),
        theta["mu"].as_u64().unwrap(),
        theta["nu"].as_u64().unwrap(),
    );
    let diag = ["a", "b", "c", "d"].map(|z| theta[z].as_u64().unwrap());
    let mat = [
        [
            m(mu, a(l, nu, p), p),
            m(m(nu, l, p), a(1, mu, p), p),
            m(nu, a(l, mu, p), p),
            m(m(mu, l, p), a(1, nu, p), p),
        ],
        [
            s(0, mu, p),
            s(0, m(nu, l, p), p),
            s(0, nu, p),
            s(0, m(mu, l, p), p),
        ],
        [a(mu, 1, p), a(l, nu, p), a(nu, 1, p), a(l, mu, p)],
        [p - 1, p - 1, p - 1, p - 1],
    ];
    let system = std::array::from_fn(|j| {
        let mut row = [0u64; 5];
        for i in 0..4 {
            row[i] = m(mat[i][j], diag[j], p)
        }
        row[4] = k[j];
        row
    });
    let out = solve4(system, p)?;
    if out[0] == 0 {
        return None;
    }
    let unit = inv(out[0], p);
    Some(out.map(|z| m(z, unit, p)))
}
fn curve(theta: &Value, p: u64) -> Vec<u64> {
    let (l, mu, nu) = (
        theta["lambda"].as_u64().unwrap(),
        theta["mu"].as_u64().unwrap(),
        theta["nu"].as_u64().unwrap(),
    );
    let mut f = vec![0; 7];
    f[1] = 1;
    for root in [1, l, mu, nu] {
        let mut g = vec![0; 7];
        for i in 0..6 {
            g[i + 1] = a(g[i + 1], f[i], p);
            g[i] = s(g[i], m(root, f[i], p), p)
        }
        f = g;
    }
    assert_eq!(f[5], 1);
    f
}
fn gen_project(d: Divisor, f: &[u64], p: u64) -> Option<[u64; 4]> {
    if d.degree != 2 {
        return None;
    }
    let (a0, a1, b1) = (d.u[0], d.u[1], d.v[1]);
    let z = s(m(a1, a1, p), a0, p);
    let xi4 = s(
        a(
            a(
                m(b1, b1, p),
                m(z, s(m(f[5], a1, p), m(f[6], z, p), p), p),
                p,
            ),
            m(a1, s(f[3], m(f[4], a1, p), p), p),
            p,
        ),
        f[2],
        p,
    );
    Some([1, s(0, a1, p), a0, xi4])
}
fn exceptional_lift(
    anchor: Divisor,
    q: [u64; 4],
    plus: [u64; 4],
    f: &[u64],
    j: &Jacobian,
    p: u64,
) -> (Option<Divisor>, usize) {
    // A bounded exceptional branch: enumerate square roots of b1^2, solve
    // b(x)^2=f(x) mod a(x), then use the signed anchor to choose Q over -Q.
    // This deliberately exposes its O(p) cost; it is not the fast Recover law.
    let (a0, a1) = (q[2], s(0, q[1], p));
    let z = s(m(a1, a1, p), a0, p);
    let b1sq = a(
        s(
            s(q[3], m(z, s(m(f[5], a1, p), m(f[6], z, p), p), p), p),
            m(a1, s(f[3], m(f[4], a1, p), p), p),
            p,
        ),
        f[2],
        p,
    );
    let mut power = [1, 0];
    let mut rem = [0, 0];
    for &coeff in f {
        rem[0] = a(rem[0], m(coeff, power[0], p), p);
        rem[1] = a(rem[1], m(coeff, power[1], p), p);
        power = [
            s(0, m(a0, power[1], p), p),
            s(power[0], m(a1, power[1], p), p),
        ];
    }
    let mut tested = 0;
    for b1 in 0..p {
        if m(b1, b1, p) != b1sq {
            continue;
        }
        let candidates: Vec<u64> = if b1 != 0 {
            vec![m(a(rem[1], m(a1, b1sq, p), p), inv(m(2, b1, p), p), p)]
        } else {
            (0..p).collect()
        };
        for b0 in candidates {
            let candidate = Divisor {
                degree: 2,
                u: [a0, a1, 1],
                v: [b0, b1],
            };
            tested += 1;
            if j.valid(candidate) && gen_project(j.add(anchor, candidate), f, p) == Some(plus) {
                return (Some(candidate), tested);
            }
        }
    }
    (None, tested)
}
fn recover(
    anchor: Divisor,
    q: [u64; 4],
    plus: [u64; 4],
    minus: [u64; 4],
    f: &[u64],
    p: u64,
) -> (Option<Divisor>, &'static str) {
    if anchor.degree != 2 {
        return (None, "special_anchor");
    }
    let (a0, a1, b0, b1) = (anchor.u[0], anchor.u[1], anchor.v[0], anchor.v[1]);
    let (z1, z2) = (s(0, a(q[1], a1, p), p), s(q[2], a0, p));
    let z3 = a(m(a1, q[2], p), m(a0, q[1], p), p);
    let d = s(m(z2, z2, p), m(z1, z3, p), p);
    if d == 0 {
        return (None, "D_zero");
    }
    let g1 = m(s(m(z2, b1, p), m(z1, b0, p), p), inv(d, p), p);
    let g2 = m(
        a(
            s(0, m(m(a0, z1, p), b1, p), p),
            m(s(m(a1, z1, p), z2, p), b0, p),
            p,
        ),
        inv(d, p),
        p,
    );
    let delta = s(
        a(
            s(0, m(4, m(g2, g2, p), p), p),
            m(2, m(a(plus[1], minus[1], p), m(g1, g2, p), p), p),
            p,
        ),
        m(2, m(a(plus[2], minus[2], p), m(g1, g1, p), p), p),
        p,
    );
    if delta == 0 {
        return (None, "Delta_zero");
    }
    let v3 = a(
        m(s(minus[2], plus[2], p), g1, p),
        m(s(plus[1], minus[1], p), g2, p),
        p,
    );
    let v4 = a(
        m(
            s(m(plus[1], minus[2], p), m(minus[1], plus[2], p), p),
            g1,
            p,
        ),
        m(s(plus[2], minus[2], p), g2, p),
        p,
    );
    // a_P mod a_Q = -(Z1*x + Z2); reduce (G3*x-G4)*a_P modulo a_Q.
    let c1 = s(0, a(m(q[1], z1, p), z2, p), p);
    let h1 = a(m(c1, v3, p), m(z1, v4, p), p);
    if h1 == 0 {
        return (None, "b1_coefficient_zero");
    }
    // Equation (6) provides b1(Q)^2. Its sign is fixed by the signed anchor.
    let a1q = s(0, q[1], p);
    let z = s(m(a1q, a1q, p), q[2], p);
    let b1sq = a(
        s(
            s(q[3], m(z, s(m(f[5], a1q, p), m(f[6], z, p), p), p), p),
            m(a1q, s(f[3], m(f[4], a1q, p), p), p),
            p,
        ),
        f[2],
        p,
    );
    let factor_sq = m(b1sq, inv(m(h1, h1, p), p), p);
    let g3sq = m(factor_sq, m(v3, v3, p), p);
    let t = s(s(f[6], m(g1, g1, p), p), g3sq, p);
    let factor = m(t, inv(delta, p), p);
    let g3 = m(factor, v3, p);
    let g4 = m(factor, v4, p);
    let q_b1 = a(m(c1, g3, p), m(z1, g4, p), p);
    let q_b0 = a(m(m(q[2], z1, p), g3, p), m(z2, g4, p), p);
    (
        Some(Divisor {
            degree: 2,
            u: [q[2], s(0, q[1], p), 1],
            v: [q_b0, q_b1],
        }),
        "computed",
    )
}
fn main() {
    let theta_rows =
        fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl").unwrap();
    let mut theta = BTreeMap::new();
    for line in theta_rows.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        theta.insert(v["p"].as_u64().unwrap(), v["selected"]["constants"].clone());
    }
    let model_rows =
        fs::read_to_string("research/kummer_coordinates_20261022/results/generic-models.jsonl")
            .unwrap();
    let mut actions = BTreeMap::new();
    for line in model_rows.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        let a: Vec<_> = v["torsion_actions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|z| (z["root"].as_u64().unwrap(), matrix(&z["action"]["matrix"])))
            .collect();
        actions.insert(p, a);
    }
    let mut good = 0;
    let mut wrong = 0;
    let mut inapp = 0;
    let mut general_fail = 0;
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let input =
            format!("research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl");
        let rows = fs::read_to_string(input).unwrap();
        let mut out = String::new();
        for line in rows.lines() {
            let v: Value = serde_json::from_str(line).unwrap();
            let p = v["p"].as_u64().unwrap();
            let t = &theta[&p];
            let f = curve(t, p);
            let j = Jacobian {
                f: Field::new(p, 1, 0),
                polynomial: f[..6].to_vec(),
            };
            let cs = v["cases"].as_array().unwrap();
            let (pp, qq, pq) = (
                &cs[1]["coordinate_witness"],
                &cs[2]["coordinate_witness"],
                &cs[3]["coordinate_witness"],
            );
            let anchor: Divisor = serde_json::from_value(pp["jacobian_expected"].clone()).unwrap();
            let expected: Divisor =
                serde_json::from_value(qq["jacobian_expected"].clone()).unwrap();
            let (kp, kq, kplus) = (
                vec4(&pp["k_expected"]),
                vec4(&qq["k_expected"]),
                vec4(&pq["k_expected"]),
            );
            let constants = ["A", "B", "C", "D"].map(|z| t[z].as_u64().unwrap());
            let (kminus, repair_root) = xadd_repaired(kq, kp, kplus, constants, &actions[&p], p);
            let q_general = general(kq, t, p);
            let plus_general = general(kplus, t, p);
            let minus_general = kminus.and_then(|z| general(z, t, p));
            let direct = gen_project(expected, &f, p);
            let general_conversion_ok = match (q_general, direct) {
                (Some(x), Some(y)) => x == y,
                _ => false,
            };
            if expected.degree == 2 && !general_conversion_ok {
                general_fail += 1
            }
            let (mut computed, mut status) = match (q_general, plus_general, minus_general) {
                (Some(q), Some(plus), Some(minus)) => recover(anchor, q, plus, minus, &f, p),
                _ => (None, "general_conversion_inapplicable"),
            };
            let mut shifted_root = None;
            if status == "D_zero" {
                if let Some(km) = kminus {
                    for &(root, act) in &actions[&p] {
                        let torsion = Divisor {
                            degree: 1,
                            u: [s(0, root, p), 1, 0],
                            v: [0, 0],
                        };
                        if !j.valid(torsion) {
                            continue;
                        }
                        let (ps, ms) = (
                            general(linear(kplus, act, p), t, p),
                            general(linear(km, act, p), t, p),
                        );
                        if let (Some(qs), Some(ps), Some(ms)) = (q_general, ps, ms) {
                            let shifted_anchor = j.add(anchor, torsion);
                            let (candidate, shifted_status) =
                                recover(shifted_anchor, qs, ps, ms, &f, p);
                            if let Some(candidate) = candidate {
                                computed = Some(candidate);
                                status = "anchor_torsion_shift_computed";
                                shifted_root = Some(root);
                                break;
                            } else if shifted_status != "D_zero" {
                                status = shifted_status;
                            }
                        }
                    }
                }
            }
            let mut exceptional_candidates_tested = 0;
            if computed.is_none() && status == "D_zero" {
                if let (Some(q), Some(plus)) = (q_general, plus_general) {
                    let (lifted, tested) = exceptional_lift(anchor, q, plus, &f, &j, p);
                    exceptional_candidates_tested = tested;
                    if lifted.is_some() {
                        computed = lifted;
                        status = "enumerated_exception_lift";
                    }
                }
            }
            let correct = computed == Some(expected) && computed.is_some_and(|d| j.valid(d));
            if correct {
                good += 1
            } else if computed.is_some() {
                wrong += 1
            } else {
                inapp += 1
            }
            out.push_str(&format!("{}\n",json!({"schema":"kummer-fast-recover-target/v1","bits":bits,"p":p,"target_index":v["target_index"],"workload_id":v["workload_id"],"source_scalar_verified":v["source_scalar_verified"],"anchor":anchor,"expected":expected,"k_anchor":kp,"k_target":kq,"k_plus":kplus,"k_minus":kminus,"repair_root":repair_root,"recover_shift_root":shifted_root,"exceptional_candidates_tested":exceptional_candidates_tested,"general_target":q_general,"general_plus":plus_general,"general_minus":minus_general,"general_conversion_ok":general_conversion_ok,"status":status,"computed":computed,"correct":correct})));
        }
        fs::create_dir_all("research/kummer_recover_20261022/results").unwrap();
        fs::write(
            format!("research/kummer_recover_20261022/results/recover-target-{bits}.jsonl"),
            out,
        )
        .unwrap();
    }
    println!("recover_targets=1260 correct={good} wrong={wrong} inapplicable={inapp} general_conversion_fail={general_fail}");
}
