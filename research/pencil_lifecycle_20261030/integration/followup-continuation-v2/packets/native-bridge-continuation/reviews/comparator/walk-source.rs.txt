use five_leads_study::three_leads::{Divisor, Field, Jacobian};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs};
#[derive(Clone,Copy,Default,serde::Serialize)]
pub struct Ops { pub m:u64,pub a:u64,pub i:u64 }
thread_local! { static OPS:std::cell::Cell<Ops>=std::cell::Cell::new(Ops{m:0,a:0,i:0}); }
pub fn reset_ops(){ OPS.with(|o|o.set(Ops::default())); }
pub fn read_ops()->Ops{ OPS.with(|o|o.get()) }
fn tally_m(){OPS.with(|o|{let mut v=o.get();v.m+=1;o.set(v);});}
fn tally_a(){OPS.with(|o|{let mut v=o.get();v.a+=1;o.set(v);});}
fn tally_i(){OPS.with(|o|{let mut v=o.get();v.i+=1;o.set(v);});}
fn raw_mul(x:u64,y:u64,p:u64)->u64{x*y%p}

fn mul(x: u64, y: u64, p: u64) -> u64 {
    tally_m();
    x * y % p
}
fn add(x: u64, y: u64, p: u64) -> u64 {
    tally_a();
    (x + y) % p
}
fn sub(x: u64, y: u64, p: u64) -> u64 {
    tally_a();
    (x + p - y) % p
}
fn pow(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n > 0 {
        if n & 1 != 0 {
            out = raw_mul(out, x, p);
        }
        x = raw_mul(x, x, p);
        n >>= 1;
    }
    out
}
fn inv(x: u64, p: u64) -> u64 {
    assert_ne!(x, 0);
    tally_i();
    pow(x, p - 2, p)
}
fn vec4(v: &Value) -> [u64; 4] {
    std::array::from_fn(|i| v[i].as_u64().unwrap())
}
fn matrix(v: &Value) -> [[u64; 4]; 4] {
    std::array::from_fn(|i| vec4(&v[i]))
}
fn linear(x: [u64; 4], mat: [[u64; 4]; 4], p: u64) -> [u64; 4] {
    std::array::from_fn(|i| (0..4).fold(0, |v, j| add(v, mul(mat[i][j], x[j], p), p)))
}
fn same(x: [u64; 4], y: [u64; 4], p: u64) -> bool {
    (0..4).all(|i| (0..4).all(|j| mul(x[i], y[j], p) == mul(x[j], y[i], p)))
}
fn h(v: [u64; 4], p: u64) -> [u64; 4] {
    let [a, b, c, d] = v;
    [
        add(add(a, b, p), add(c, d, p), p),
        sub(add(a, b, p), add(c, d, p), p),
        add(sub(a, b, p), sub(c, d, p), p),
        sub(sub(a, b, p), sub(c, d, p), p),
    ]
}
fn xadd(x: [u64; 4], y: [u64; 4], diff: [u64; 4], t: [u64; 4], p: u64) -> Option<[u64; 4]> {
    let hx = h(x, p);
    let hy = h(y, p);
    let out0 = h(
        std::array::from_fn(|i| mul(mul(hx[i], hy[i], p), inv(t[i], p), p)),
        p,
    );
    let out = std::array::from_fn(|i| {
        (0..4)
            .filter(|&j| j != i)
            .fold(mul(out0[i], out0[i], p), |v, j| mul(v, diff[j], p))
    });
    (out != [0; 4]).then_some(out)
}
fn xadd_repaired(
    x: [u64; 4],
    y: [u64; 4],
    diff: [u64; 4],
    t: [u64; 4],
    acts: &[(u64, [[u64; 4]; 4])],
    p: u64,
) -> (Option<[u64; 4]>, Option<u64>) {
    if let Some(z) = xadd(x, y, diff, t, p) {
        return (Some(z), None);
    }
    for &(root, act) in acts {
        if let Some(z) = xadd(linear(x, act, p), y, linear(diff, act, p), t, p) {
            return (Some(linear(z, act, p)), Some(root));
        }
    }
    (None, None)
}
fn project(d: Divisor, t: &Value, p: u64) -> Option<[u64; 4]> {
    if d.degree == 0 {
        return Some(["a", "b", "c", "d"].map(|k| t[k].as_u64().unwrap()));
    }
    if d.degree != 2 {
        return None;
    }
    let (l, mu, nu) = (
        t["lambda"].as_u64().unwrap(),
        t["mu"].as_u64().unwrap(),
        t["nu"].as_u64().unwrap(),
    );
    let (a0, a1, b0) = (d.u[0], d.u[1], d.v[0]);
    let terms = [
        mul(a0, mul(sub(mu, a0, p), add(add(l, a1, p), nu, p), p), p),
        mul(
            a0,
            mul(sub(mul(nu, l, p), a0, p), add(add(1, a1, p), mu, p), p),
            p,
        ),
        mul(a0, mul(sub(nu, a0, p), add(add(l, a1, p), mu, p), p), p),
        mul(
            a0,
            mul(sub(mul(mu, l, p), a0, p), add(add(1, a1, p), nu, p), p),
            p,
        ),
    ];
    let scale = ["a", "b", "c", "d"].map(|k| t[k].as_u64().unwrap());
    let out = std::array::from_fn(|i| mul(scale[i], sub(terms[i], mul(b0, b0, p), p), p));
    (out != [0; 4]).then_some(out)
}
fn project_full(
    d: Divisor,
    j: &Jacobian,
    t: &Value,
    acts: &[(u64, [[u64; 4]; 4])],
    p: u64,
) -> Option<[u64; 4]> {
    if let Some(k) = project(d, t, p) {
        return Some(k);
    }
    for &(root, act) in acts {
        let torsion = Divisor {
            degree: 1,
            u: [sub(0, root, p), 1, 0],
            v: [0, 0],
        };
        if let Some(k) = project(j.add(d, torsion), t, p) {
            return Some(linear(k, act, p));
        }
    }
    None
}
fn curve(t: &Value, p: u64) -> Vec<u64> {
    let mut f = vec![0; 7];
    f[1] = 1;
    for root in [
        1,
        t["lambda"].as_u64().unwrap(),
        t["mu"].as_u64().unwrap(),
        t["nu"].as_u64().unwrap(),
    ] {
        let mut g = vec![0; 7];
        for i in 0..6 {
            g[i + 1] = add(g[i + 1], f[i], p);
            g[i] = sub(g[i], mul(root, f[i], p), p);
        }
        f = g;
    }
    f
}
fn branch(k: [u64; 4], p: u64) -> u64 {
    let den = if k[2] != 0 {
        k[2]
    } else if k[1] != 0 {
        k[1]
    } else if k[3] != 0 {
        k[3]
    } else {
        return 0;
    };
    mul(k[0], inv(den, p), p) & 1
}
fn main() {
    let theta_rows =
        fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl").unwrap();
    let mut theta = BTreeMap::new();
    let mut normalization = BTreeMap::new();
    for line in theta_rows.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        normalization.insert(
            p,
            (
                v["selected"]["r0"].as_u64().unwrap(),
                v["selected"]["delta"].as_u64().unwrap(),
            ),
        );
        theta.insert(p, v["selected"]["constants"].clone());
    }
    let model_rows =
        fs::read_to_string("research/kummer_coordinates_20261022/results/generic-models.jsonl")
            .unwrap();
    let mut acts = BTreeMap::new();
    for line in model_rows.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        let (r0, delta) = normalization[&p];
        let a: Vec<_> = v["torsion_actions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|z| {
                (
                    mul(sub(z["root"].as_u64().unwrap(), r0, p), inv(delta, p), p),
                    matrix(&z["action"]["matrix"]),
                )
            })
            .collect();
        acts.insert(p, a);
    }
    let (mut passed, mut failed, mut inapp, mut repaired) = (0, 0, 0, 0);
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let input = fs::read_to_string(format!(
            "research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"
        ))
        .unwrap();
        let mut output = String::new();
        for line in input.lines() {
            let v: Value = serde_json::from_str(line).unwrap();
            let p = v["p"].as_u64().unwrap();
            let r = v["r"].as_u64().unwrap();
            let t = &theta[&p];
            let f = curve(t, p);
            let j = Jacobian {
                f: Field::new(p, 1, 0),
                polynomial: f[..6].to_vec(),
            };
            let cases = v["cases"].as_array().unwrap();
            let g: Divisor =
                serde_json::from_value(cases[1]["coordinate_witness"]["jacobian_expected"].clone())
                    .unwrap();
            let mut d: Divisor =
                serde_json::from_value(cases[2]["coordinate_witness"]["jacobian_expected"].clone())
                    .unwrap();
            let mut k = vec4(&cases[2]["coordinate_witness"]["k_expected"]);
            let mut kp = vec4(&cases[3]["coordinate_witness"]["k_expected"]);
            let kg = vec4(&cases[1]["coordinate_witness"]["k_expected"]);
            let k0 = ["a", "b", "c", "d"].map(|key| t[key].as_u64().unwrap());
            let consts = ["A", "B", "C", "D"].map(|key| t[key].as_u64().unwrap());
            let (mut a, mut b) = (0u64, 1u64);
            for step in 0..3 {
                let choice = branch(k, p);
                let (kd, root_d) = xadd_repaired(k, k, k0, consts, &acts[&p], p);
                let (km, root_m) = xadd_repaired(k, kp, kg, consts, &acts[&p], p);
                let (kpp, root_pp) = xadd_repaired(kp, kp, k0, consts, &acts[&p], p);
                let mut status = "pass";
                let roots = [root_d, root_m, root_pp];
                repaired += roots.iter().filter(|x| x.is_some()).count();
                if kd.is_none() || km.is_none() || kpp.is_none() {
                    status = "xadd_inapplicable";
                    inapp += 1;
                }
                let nd = if choice == 0 {
                    j.add(d, d)
                } else {
                    j.add(j.add(d, d), g)
                };
                let nplus = j.add(nd, g);
                let expected_d = project_full(nd, &j, t, &acts[&p], p);
                let expected_plus = project_full(nplus, &j, t, &acts[&p], p);
                let next = if choice == 0 { kd } else { km };
                let next_plus = if choice == 0 { km } else { kpp };
                let label_a = add(add(a, a, r), choice, r);
                let label_b = add(b, b, r);
                let label_expected = j.add(
                    j.mul(g, label_a),
                    j.mul(
                        serde_json::from_value(
                            cases[2]["coordinate_witness"]["jacobian_expected"].clone(),
                        )
                        .unwrap(),
                        label_b,
                    ),
                );
                if status == "pass" {
                    if !same(next.unwrap(), expected_d.unwrap(), p)
                        || !same(next_plus.unwrap(), expected_plus.unwrap(), p)
                        || label_expected != nd
                    {
                        status = "mismatch";
                        failed += 1;
                    } else {
                        passed += 1;
                    }
                }
                output.push_str(&format!("{}\n",json!({"schema":"kummer-differential-transition/v1","bits":bits,"p":p,"r":r,"target_index":v["target_index"],"workload_id":v["workload_id"],"source_scalar_verified":v["source_scalar_verified"],"step":step,"branch":choice,"input_label":[a,b],"output_label":[label_a,label_b],"input_k":k,"input_k_plus_g":kp,"output_k":next,"output_k_plus_g":next_plus,"expected_k":expected_d,"expected_k_plus_g":expected_plus,"expected_signed":nd,"label_signed":label_expected,"torsion_repair_roots":roots,"status":status})));
                if status != "pass" {
                    break;
                }
                d = nd;
                k = next.unwrap();
                kp = next_plus.unwrap();
                a = label_a;
                b = label_b;
            }
        }
        fs::create_dir_all("research/kummer_differential_walk_20261022/results").unwrap();
        fs::write(
            format!("research/kummer_differential_walk_20261022/results/transition-{bits}.jsonl"),
            output,
        )
        .unwrap();
    }
    println!("transitions pass={passed} fail={failed} inapp={inapp} repaired_xadd={repaired}");
}
