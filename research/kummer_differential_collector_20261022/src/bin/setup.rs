#![allow(dead_code)]
use five_leads_study::three_leads::Point;
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, process::Command};

mod theta {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/theta_impl.rs"));
    pub fn search(p: u64, roots: &[u64], c5: u64) -> (Value, Ops, usize) {
        reset_ops();
        let mut selected = None;
        let mut eligible = 0;
        for &r0 in roots {
            for &r1 in roots {
                if r0 == r1 {
                    continue;
                }
                let delta = sub(r1, r0, p);
                let scale = mul(c5, pow(delta, 5, p), p);
                let Some(ys) = sqrt(scale, p) else {
                    continue;
                };
                if ys == 0 {
                    continue;
                }
                let di = inv(delta, p);
                let remain: Vec<_> = roots
                    .iter()
                    .copied()
                    .filter(|&r| r != r0 && r != r1)
                    .map(|r| mul(sub(r, r0, p), di, p))
                    .collect();
                for perm in [
                    [0, 1, 2],
                    [0, 2, 1],
                    [1, 0, 2],
                    [1, 2, 0],
                    [2, 0, 1],
                    [2, 1, 0],
                ] {
                    eligible += 1;
                    let (l, mu, nu) = (remain[perm[0]], remain[perm[1]], remain[perm[2]]);
                    let tuples = theta_tuple(p, l, mu, nu);
                    if selected.is_none() {
                        if let Some(constants) = tuples.into_iter().next() {
                            selected = Some(json!({"r0":r0,"r1":r1,"delta":delta,"sqrt_scale":ys,
                        "lambda_mu_nu":[l,mu,nu],"constants":constants}));
                        }
                    }
                }
            }
        }
        (selected.unwrap(), read_ops(), eligible)
    }
}
mod coordinate {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/coordinate_impl.rs"
    ));
    pub fn roots(
        p: u64,
    ) -> (
        Vec<u64>,
        Ops,
        five_leads_study::three_leads::Counts,
        five_leads_study::three_leads::Counts,
    ) {
        reset_ops();
        let s = Scholten::new(p);
        let roots: Vec<u64> = (0..p)
            .filter(|&x| poly_eval(&s.j.polynomial, x, p) == 0)
            .collect();
        assert_eq!(roots.len(), 5);
        (roots, read_ops(), s.e.f.counts(), s.j.f.counts())
    }
    pub fn actions(
        p: u64,
        roots: &[u64],
        sel: &Value,
    ) -> (
        Vec<[[u64; 4]; 4]>,
        Ops,
        five_leads_study::three_leads::Counts,
        five_leads_study::three_leads::Counts,
    ) {
        reset_ops();
        let s = Scholten::new(p);
        let t = &sel["constants"];
        let (r0, delta, ys) = (
            sel["r0"].as_u64().unwrap(),
            sel["delta"].as_u64().unwrap(),
            sel["sqrt_scale"].as_u64().unwrap(),
        );
        let jr = rosenhain_jacobian(&s, t);
        let points = sample_points(&s.j, p);
        let ds = [
            s.j.add(points[0], points[1]),
            s.j.add(points[2], points[3]),
            s.j.add(points[4], points[5]),
            s.j.add(points[6], points[7]),
        ];
        let mut samples = ds.to_vec();
        for k in 2..9 {
            samples.push(s.j.mul(ds[0], k));
        }
        let transformed: Vec<_> = samples
            .iter()
            .copied()
            .map(|d| transform(d, p, r0, delta, ys))
            .collect();
        let mut out = Vec::new();
        for &root in roots {
            let branch = transform(s.j.point(root, 0), p, r0, delta, ys);
            let (matrix, _checked) = torsion_action(&jr, branch, &transformed, t, p).unwrap();
            out.push(matrix);
        }
        (out, read_ops(), s.e.f.counts(), s.j.f.counts())
    }
    pub fn transfer(
        p: u64,
        sel: &Value,
        g: Point,
        q: Point,
        scalar: u64,
    ) -> (
        Divisor,
        Divisor,
        [u64; 4],
        [u64; 4],
        [u64; 4],
        Ops,
        five_leads_study::three_leads::Counts,
        five_leads_study::three_leads::Counts,
    ) {
        reset_ops();
        let s = Scholten::new(p);
        let t = &sel["constants"];
        let source = s.short_curve();
        assert!(source.valid(g) && source.valid(q));
        assert_eq!(source.mul(g, scalar), q);
        let (r0, delta, ys) = (
            sel["r0"].as_u64().unwrap(),
            sel["delta"].as_u64().unwrap(),
            sel["sqrt_scale"].as_u64().unwrap(),
        );
        let dg = transform(s.tau(s.from_short(g)), p, r0, delta, ys);
        let dq = transform(s.tau(s.from_short(q)), p, r0, delta, ys);
        let jr = rosenhain_jacobian(&s, t);
        assert!(jr.valid(dg) && jr.valid(dq));
        let dplus = jr.add(dg, dq);
        let kg = project(dg, t, p).unwrap();
        let kq = project(dq, t, p).unwrap();
        let kp = project(dplus, t, p).unwrap();
        (
            dg,
            dq,
            kg,
            kq,
            kp,
            read_ops(),
            s.e.f.counts(),
            s.j.f.counts(),
        )
    }
}

fn point(v: &Value) -> Point {
    let f = |i: usize| {
        u64::from_str_radix(v[i].as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
    };
    Point::affine(f(0), f(1))
}
fn counts(c: five_leads_study::three_leads::Counts) -> Value {
    json!({"m":c.m,"s":c.s,"i":c.i,"field_adds":c.field_adds,"bit_ops":c.bit_ops,
        "ec_adds":c.ec_adds,"jac_adds":c.jac_adds,"sqrt_calls":c.sqrt_calls})
}
fn main() {
    let theta_rows =
        fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl").unwrap();
    let frozen_theta: BTreeMap<u64, Value> = theta_rows
        .lines()
        .map(|l| {
            let v: Value = serde_json::from_str(l).unwrap();
            (v["p"].as_u64().unwrap(), v)
        })
        .collect();
    let generic_rows =
        fs::read_to_string("research/kummer_coordinates_20261022/results/generic-models.jsonl")
            .unwrap();
    let frozen_actions: BTreeMap<u64, Value> = generic_rows
        .lines()
        .map(|l| {
            let v: Value = serde_json::from_str(l).unwrap();
            (v["p"].as_u64().unwrap(), v)
        })
        .collect();
    let mut model_out = String::new();
    let mut target_out = String::new();
    for (&p, theta_row) in &frozen_theta {
        let (roots, root_ops, root_e, root_j) = coordinate::roots(p);
        let frozen_model =
            fs::read_to_string("research/four_followups_20261021/results/kummer-model-gate.jsonl")
                .unwrap();
        let model: Value = frozen_model
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).unwrap())
            .find(|v| v["p"].as_u64() == Some(p))
            .unwrap();
        let expected_roots: Vec<u64> = model["finite_rational_weierstrass_roots"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect();
        assert_eq!(roots, expected_roots);
        let c5 = model["quintic_coefficients"][5].as_u64().unwrap();
        let (selected, theta_ops, eligible) = theta::search(p, &roots, c5);
        assert_eq!(selected, theta_row["selected"]);
        let (actions, action_ops, action_e, action_j) = coordinate::actions(p, &roots, &selected);
        let expected_actions = &frozen_actions[&p]["torsion_actions"];
        assert_eq!(actions.len(), 5);
        for (a, b) in actions.iter().zip(expected_actions.as_array().unwrap()) {
            let expected: [[u64; 4]; 4] = std::array::from_fn(|i| {
                std::array::from_fn(|j| b["action"]["matrix"][i][j].as_u64().unwrap())
            });
            assert_eq!(*a, expected);
        }
        model_out.push_str(&format!(
            "{}\n",
            json!({"schema":"kummer-cold-model-setup/v1","p":p,
            "roots":roots,"theta_eligible_orders":eligible,"selected_theta":selected,
            "torsion_actions_match":true,"root_ops":root_ops,"root_extension_counts":counts(root_e),
            "root_base_counts":counts(root_j),"theta_ops":theta_ops,"torsion_ops":action_ops,
            "torsion_extension_counts":counts(action_e),"torsion_base_counts":counts(action_j)})
        ));
    }
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
        let output = Command::new("tar")
            .args([
                "--zstd",
                "-xOf",
                "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst",
                &member,
            ])
            .output()
            .unwrap();
        assert!(output.status.success());
        for line in String::from_utf8(output.stdout).unwrap().lines() {
            let row: Value = serde_json::from_str(line).unwrap();
            if row["arm"] != "rho_neg" {
                continue;
            }
            let w = &row["workload"];
            let spec = &w["curve_spec"];
            let p = spec["p"].as_u64().unwrap();
            let sel = &frozen_theta[&p]["selected"];
            let g = Point::affine(spec["gx"].as_u64().unwrap(), spec["gy"].as_u64().unwrap());
            let q = point(&w["target"]);
            let scalar = w["planted"].as_u64().unwrap();
            let (dg, dq, kg, kq, kp, ops, e, j) = coordinate::transfer(p, sel, g, q, scalar);
            target_out.push_str(&format!(
                "{}\n",
                json!({"schema":"kummer-cold-target-setup/v1",
                "bits":bits,"p":p,"workload_id":w["workload_id"],"target_index":w["target_index"],
                "source_scalar_verified":row["outcome"]["status"]=="verified",
                "anchor":dg,"target":dq,"k_anchor":kg,"k_target":kq,"k_plus":kp,
                "transfer_ops":ops,"extension_counts":counts(e),"base_counts":counts(j)})
            ));
        }
    }
    fs::create_dir_all("research/kummer_differential_collector_20261022/results").unwrap();
    fs::write(
        "research/kummer_differential_collector_20261022/results/cold-model-setup.jsonl",
        model_out,
    )
    .unwrap();
    fs::write(
        "research/kummer_differential_collector_20261022/results/cold-target-setup.jsonl",
        target_out,
    )
    .unwrap();
    println!("cold_models=21 cold_targets=1260");
}
