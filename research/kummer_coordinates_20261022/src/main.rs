//! Exact fast-Kummer coordinate identity gate; independent Jacobian oracle.
use five_leads_study::three_leads::{Divisor, Jacobian, Point, Scholten};
use serde_json::{json, Value};
use std::fs;
use std::process::Command;

fn add(x: u64, y: u64, p: u64) -> u64 {
    (x + y) % p
}
fn sub(x: u64, y: u64, p: u64) -> u64 {
    (x + p - y) % p
}
fn mul(x: u64, y: u64, p: u64) -> u64 {
    x * y % p
}
fn pow(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n & 1 == 1 {
            z = mul(z, x, p)
        }
        x = mul(x, x, p);
        n >>= 1;
    }
    z
}
fn inv(x: u64, p: u64) -> u64 {
    assert_ne!(x, 0);
    pow(x, p - 2, p)
}
fn sqrt(x: u64, p: u64) -> Option<u64> {
    if x == 0 {
        return Some(0);
    }
    if pow(x, (p - 1) / 2, p) != 1 {
        return None;
    }
    let y = pow(x, (p + 1) / 4, p);
    assert_eq!(mul(y, y, p), x);
    Some(y)
}
fn poly_eval(coeff: &[u64], x: u64, p: u64) -> u64 {
    coeff.iter().rev().fold(0, |v, &a| add(mul(v, x, p), a, p))
}
fn transform(d: Divisor, p: u64, r0: u64, delta: u64, ys: u64) -> Divisor {
    if d.degree == 0 {
        return d;
    }
    let di = inv(delta, p);
    let yi = inv(ys, p);
    if d.degree == 1 {
        return Divisor {
            degree: 1,
            u: [mul(add(d.u[0], r0, p), di, p), 1, 0],
            v: [mul(d.v[0], yi, p), 0],
        };
    }
    Divisor {
        degree: 2,
        u: [
            mul(
                add(add(mul(r0, r0, p), mul(d.u[1], r0, p), p), d.u[0], p),
                mul(di, di, p),
                p,
            ),
            mul(add(add(r0, r0, p), d.u[1], p), di, p),
            1,
        ],
        v: [
            mul(add(d.v[0], mul(d.v[1], r0, p), p), yi, p),
            mul(mul(d.v[1], delta, p), yi, p),
        ],
    }
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
fn project(d: Divisor, t: &Value, p: u64) -> Option<[u64; 4]> {
    if d.degree == 0 {
        return Some(["a", "b", "c", "d"].map(|k| t[k].as_u64().unwrap()));
    }
    if d.degree != 2 {
        return None;
    }
    let l = t["lambda"].as_u64().unwrap();
    let mu = t["mu"].as_u64().unwrap();
    let nu = t["nu"].as_u64().unwrap();
    let a0 = d.u[0];
    let a1 = d.u[1];
    let b0 = d.v[0];
    let b02 = mul(b0, b0, p);
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
    let z = ["a", "b", "c", "d"].map(|k| t[k].as_u64().unwrap());
    let out = std::array::from_fn(|i| mul(z[i], sub(terms[i], b02, p), p));
    if out == [0; 4] {
        None
    } else {
        Some(out)
    }
}
fn xadd(x: [u64; 4], y: [u64; 4], diff: [u64; 4], t: &Value, p: u64) -> Option<[u64; 4]> {
    let hx = h(x, p);
    let hy = h(y, p);
    let pre = ["A", "B", "C", "D"].map(|k| t[k].as_u64().unwrap());
    let tmp = std::array::from_fn(|i| mul(mul(hx[i], hy[i], p), inv(pre[i], p), p));
    let out0 = h(tmp, p);
    let out0 = out0.map(|z| mul(z, z, p));
    let out = std::array::from_fn(|i| {
        let mut z = out0[i];
        for (j, &d) in diff.iter().enumerate() {
            if i != j {
                z = mul(z, d, p)
            }
        }
        z
    });
    if out == [0; 4] {
        None
    } else {
        Some(out)
    }
}
fn same(x: [u64; 4], y: [u64; 4], p: u64) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if mul(x[i], y[j], p) != mul(x[j], y[i], p) {
                return false;
            }
        }
    }
    true
}
fn linear(x: [u64; 4], m: [[u64; 4]; 4], p: u64) -> [u64; 4] {
    std::array::from_fn(|i| (0..4).fold(0, |z, j| add(z, mul(m[i][j], x[j], p), p)))
}
type Action = ([[u64; 4]; 4], usize);
fn torsion_action(
    j: &Jacobian,
    branch: Divisor,
    samples: &[Divisor],
    t: &Value,
    p: u64,
) -> Option<Action> {
    let pairs: Vec<_> = samples
        .iter()
        .filter_map(|&d| {
            let shifted = j.add(d, branch);
            match (project(d, t, p), project(shifted, t, p)) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            }
        })
        .collect();
    if pairs.len() < 3 {
        return None;
    }
    let mut system = Vec::new();
    for &(x, y) in &pairs {
        let anchor = y.iter().position(|&z| z != 0).unwrap();
        for i in 0..4 {
            if i == anchor {
                continue;
            }
            let mut row = [0u64; 16];
            for j in 0..4 {
                row[4 * i + j] = mul(y[anchor], x[j], p);
                row[4 * anchor + j] = sub(0, mul(y[i], x[j], p), p);
            }
            system.push(row);
        }
    }
    let mut rank = 0;
    let mut pivots = Vec::new();
    for col in 0..16 {
        let Some(hit) = (rank..system.len()).find(|&i| system[i][col] != 0) else {
            continue;
        };
        system.swap(rank, hit);
        let unit = inv(system[rank][col], p);
        for j in col..16 {
            system[rank][j] = mul(system[rank][j], unit, p)
        }
        for i in 0..system.len() {
            if i == rank {
                continue;
            }
            let factor = system[i][col];
            if factor != 0 {
                for j in col..16 {
                    system[i][j] = sub(system[i][j], mul(factor, system[rank][j], p), p)
                }
            }
        }
        pivots.push(col);
        rank += 1;
    }
    if rank != 15 {
        return None;
    }
    let free = (0..16).find(|j| !pivots.contains(j)).unwrap();
    let mut v = [0u64; 16];
    v[free] = 1;
    for (i, &col) in pivots.iter().enumerate() {
        v[col] = sub(0, system[i][free], p)
    }
    let m = std::array::from_fn(|i| std::array::from_fn(|j| v[4 * i + j]));
    if pairs
        .iter()
        .all(|&(x, y)| same(linear(x, m, p), y, p) && same(linear(linear(x, m, p), m, p), x, p))
    {
        Some((m, pairs.len()))
    } else {
        None
    }
}
fn project_full(
    d: Divisor,
    j: &Jacobian,
    branches: &[(Divisor, Action)],
    t: &Value,
    p: u64,
) -> Option<[u64; 4]> {
    project_full_depth(d, j, branches, t, p, 2)
}
fn project_full_depth(
    d: Divisor,
    j: &Jacobian,
    branches: &[(Divisor, Action)],
    t: &Value,
    p: u64,
    depth: usize,
) -> Option<[u64; 4]> {
    if let Some(k) = project(d, t, p) {
        return Some(k);
    }
    if depth == 0 {
        return None;
    }
    for &(branch, (matrix, _)) in branches {
        let shifted = j.add(d, branch);
        if shifted == d {
            continue;
        }
        if let Some(k) = project_full_depth(shifted, j, branches, t, p, depth - 1) {
            return Some(linear(k, matrix, p));
        }
    }
    None
}
fn coordinate_case(
    a: Divisor,
    b: Divisor,
    j: &Jacobian,
    branches: &[(Divisor, Action)],
    t: &Value,
    p: u64,
) -> (&'static str, usize, usize) {
    let sum = j.add(a, b);
    let diff = j.add(a, j.neg(b));
    match (
        project_full(a, j, branches, t, p),
        project_full(b, j, branches, t, p),
        project_full(sum, j, branches, t, p),
        project_full(diff, j, branches, t, p),
    ) {
        (Some(x), Some(y), Some(expected), Some(difference)) => {
            match xadd(x, y, difference, t, p) {
                Some(actual) => {
                    if same(actual, expected, p) {
                        ("pass", sum.degree, diff.degree)
                    } else {
                        ("fail", sum.degree, diff.degree)
                    }
                }
                None => ("zero_output", sum.degree, diff.degree),
            }
        }
        _ => ("projection_inapplicable", sum.degree, diff.degree),
    }
}
fn sample_points(j: &Jacobian, p: u64) -> Vec<Divisor> {
    let mut out = Vec::new();
    for x in 0..p {
        let v = poly_eval(&j.polynomial, x, p);
        if let Some(y) = sqrt(v, p) {
            if y != 0 {
                out.push(j.point(x, y));
                if out.len() == 8 {
                    break;
                }
            }
        }
    }
    assert_eq!(out.len(), 8);
    out
}
fn rosenhain_jacobian(s: &Scholten, t: &Value) -> Jacobian {
    let p = s.p;
    let l = t["lambda"].as_u64().unwrap();
    let mu = t["mu"].as_u64().unwrap();
    let nu = t["nu"].as_u64().unwrap();
    let mut f = vec![0; 6];
    f[1] = 1;
    for root in [1, l, mu, nu] {
        let mut g = vec![0; 6];
        for i in 0..5 {
            g[i + 1] = add(g[i + 1], f[i], p);
            g[i] = sub(g[i], mul(root, f[i], p), p)
        }
        f = g;
    }
    assert_eq!(f[5], 1);
    Jacobian {
        f: s.j.f.clone(),
        polynomial: f,
    }
}
fn model(raw: &Value, theta: &Value) -> Value {
    let p = raw["p"].as_u64().unwrap();
    assert_eq!(p, theta["p"].as_u64().unwrap());
    let s = Scholten::new(p);
    let sel = &theta["selected"];
    let t = &sel["constants"];
    let r0 = sel["r0"].as_u64().unwrap();
    let delta = sel["delta"].as_u64().unwrap();
    let ys = sel["sqrt_scale"].as_u64().unwrap();
    let jr = rosenhain_jacobian(&s, t);
    let points = sample_points(&s.j, p);
    let mut cases = Vec::new();
    let ds = [
        s.j.add(points[0], points[1]),
        s.j.add(points[2], points[3]),
        s.j.add(points[4], points[5]),
        s.j.add(points[6], points[7]),
    ];
    let mut torsion_samples = ds.to_vec();
    for k in 2..9 {
        torsion_samples.push(s.j.mul(ds[0], k));
    }
    let roots: Vec<u64> = raw["finite_rational_weierstrass_roots"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap())
        .collect();
    let mut actions = Vec::new();
    let mut branches = Vec::new();
    for &root in &roots {
        let branch = transform(s.j.point(root, 0), p, r0, delta, ys);
        let transformed_samples: Vec<_> = torsion_samples
            .iter()
            .copied()
            .map(|d| transform(d, p, r0, delta, ys))
            .collect();
        let action = torsion_action(&jr, branch, &transformed_samples, t, p);
        if let Some(a) = action {
            branches.push((branch, a));
        }
        actions.push(json!({"root":root,"action":action.map(|(matrix,checked)|json!({"matrix":matrix,"generic_samples":checked}))}));
    }
    let t0 = transform(s.j.point(r0, 0), p, r0, delta, ys);
    let mut exceptional = Vec::new();
    let mut exception_pass = 0;
    let mut exception_fail = 0;
    let mut exception_inapp = 0;
    let mut representatives = vec![Divisor::O];
    representatives.extend(
        roots
            .iter()
            .map(|&root| transform(s.j.point(root, 0), p, r0, delta, ys)),
    );
    representatives.extend(
        points
            .iter()
            .copied()
            .map(|d| transform(d, p, r0, delta, ys)),
    );
    for &d in &representatives {
        assert!(jr.valid(d));
        let x = project_full(d, &jr, &branches, t, p);
        let neg = project_full(jr.neg(d), &jr, &branches, t, p);
        let quotient_ok = matches!((x,neg),(Some(x),Some(y)) if same(x,y,p));
        if !quotient_ok {
            exception_fail += 1;
        }
        for (kind, a, b) in [
            ("identity", Divisor::O, d),
            ("inverse", d, jr.neg(d)),
            ("double", d, d),
            ("branch0", t0, d),
        ] {
            let (status, sum_degree, diff_degree) = coordinate_case(a, b, &jr, &branches, t, p);
            match status {
                "pass" => exception_pass += 1,
                "fail" => exception_fail += 1,
                _ => exception_inapp += 1,
            }
            let sum = jr.add(a, b);
            let diff = jr.add(a, jr.neg(b));
            let (kx, ky, ke, kd) = (
                project_full(a, &jr, &branches, t, p),
                project_full(b, &jr, &branches, t, p),
                project_full(sum, &jr, &branches, t, p),
                project_full(diff, &jr, &branches, t, p),
            );
            let observed = match (kx, ky, kd) {
                (Some(x), Some(y), Some(d)) => xadd(x, y, d, t, p),
                _ => None,
            };
            let witness = json!({"k_left":kx,"k_right":ky,"k_expected":ke,"k_difference":kd,"k_observed":observed,"jacobian_expected":sum});
            exceptional.push(json!({"kind":kind,"left_degree":a.degree,"right_degree":b.degree,"sum_degree":sum_degree,"difference_degree":diff_degree,"status":status,"quotient_ok":quotient_ok,"coordinate_witness":witness}));
        }
    }
    let mut map_checks = 0;
    let mut map_fail = 0;
    let mut dbl_pass = 0;
    let mut dbl_fail = 0;
    let mut dbl_inapp = 0;
    let mut add_pass = 0;
    let mut add_fail = 0;
    let mut add_inapp = 0;
    for &a in &ds {
        let at = transform(a, p, r0, delta, ys);
        assert!(jr.valid(at));
        let doubled = s.j.add(a, a);
        let dt = transform(doubled, p, r0, delta, ys);
        assert!(jr.valid(dt));
        map_checks += 1;
        if jr.add(at, at) != dt {
            map_fail += 1;
        }
        let checked = match (project(at, t, p), project(dt, t, p)) {
            (Some(x), Some(expected)) => match xadd(x, x, project(Divisor::O, t, p).unwrap(), t, p)
            {
                Some(z) => {
                    if same(z, expected, p) {
                        dbl_pass += 1;
                        "pass"
                    } else {
                        dbl_fail += 1;
                        "fail"
                    }
                }
                None => {
                    dbl_inapp += 1;
                    "zero_output"
                }
            },
            _ => {
                dbl_inapp += 1;
                "projection_inapplicable"
            }
        };
        cases.push(json!({"kind":"double","status":checked,"left_degree":at.degree,"sum_degree":dt.degree}));
    }
    for i in 0..4 {
        for j in (i + 1)..4 {
            let a = ds[i];
            let b = ds[j];
            let sum = s.j.add(a, b);
            let diff = s.j.add(a, s.j.neg(b));
            let (at, bt, st, dt) = (
                transform(a, p, r0, delta, ys),
                transform(b, p, r0, delta, ys),
                transform(sum, p, r0, delta, ys),
                transform(diff, p, r0, delta, ys),
            );
            for v in [at, bt, st, dt] {
                assert!(jr.valid(v));
            }
            map_checks += 2;
            if jr.add(at, bt) != st {
                map_fail += 1
            }
            if jr.add(at, jr.neg(bt)) != dt {
                map_fail += 1
            }
            let checked = match (
                project(at, t, p),
                project(bt, t, p),
                project(st, t, p),
                project(dt, t, p),
            ) {
                (Some(x), Some(y), Some(expected), Some(difference)) => {
                    match xadd(x, y, difference, t, p) {
                        Some(z) => {
                            if same(z, expected, p) {
                                add_pass += 1;
                                "pass"
                            } else {
                                add_fail += 1;
                                "fail"
                            }
                        }
                        None => {
                            add_inapp += 1;
                            "zero_output"
                        }
                    }
                }
                _ => {
                    add_inapp += 1;
                    "projection_inapplicable"
                }
            };
            cases.push(json!({"kind":"add","status":checked,"left_degree":at.degree,"right_degree":bt.degree,"sum_degree":st.degree,"difference_degree":dt.degree}));
        }
    }
    json!({"schema":"kummer-coordinate-generic/v1","p":p,"sample_point_count":points.len(),"map_checks":map_checks,"map_fail":map_fail,"xDBL_pass":dbl_pass,"xDBL_fail":dbl_fail,"xDBL_inapplicable":dbl_inapp,"xADD_pass":add_pass,"xADD_fail":add_fail,"xADD_inapplicable":add_inapp,"torsion_actions":actions,"exception_pass":exception_pass,"exception_fail":exception_fail,"exception_inapplicable":exception_inapp,"exceptional_cases":exceptional,"cases":cases})
}
fn archive_rows(bits: u64) -> Vec<Value> {
    let archive = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
    let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let out = Command::new("tar")
        .args(["--zstd", "-xOf", archive, &member])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str::<Value>(s).unwrap())
        .filter(|r| r["arm"] == "rho_neg")
        .collect()
}
fn target_point(v: &Value) -> Point {
    let x = |i| u64::from_str_radix(v[i].as_str().unwrap().trim_start_matches("0x"), 16).unwrap();
    Point::affine(x(0), x(1))
}
fn cohort_target(bits: u64, row: &Value, theta: &Value, model: &Value) -> Value {
    let w = &row["workload"];
    let spec = &w["curve_spec"];
    let p = spec["p"].as_u64().unwrap();
    let r = spec["r"].as_u64().unwrap();
    let s = Scholten::new(p);
    let source = s.short_curve();
    let g = Point::affine(spec["gx"].as_u64().unwrap(), spec["gy"].as_u64().unwrap());
    let q = target_point(&w["target"]);
    let planted = w["planted"].as_u64().unwrap();
    let source_scalar_verified = source.valid(g) && source.valid(q) && source.mul(g, planted) == q;
    let dg = s.tau(s.from_short(g));
    let dq = s.tau(s.from_short(q));
    let sel = &theta["selected"];
    let t = &sel["constants"];
    let (r0, delta, ys) = (
        sel["r0"].as_u64().unwrap(),
        sel["delta"].as_u64().unwrap(),
        sel["sqrt_scale"].as_u64().unwrap(),
    );
    let jr = rosenhain_jacobian(&s, t);
    let mut branches = Vec::new();
    for a in model["torsion_actions"].as_array().unwrap() {
        let root = a["root"].as_u64().unwrap();
        let matrix_value = &a["action"]["matrix"];
        let matrix =
            std::array::from_fn(|i| std::array::from_fn(|j| matrix_value[i][j].as_u64().unwrap()));
        let checked = a["action"]["generic_samples"].as_u64().unwrap() as usize;
        branches.push((
            transform(s.j.point(root, 0), p, r0, delta, ys),
            (matrix, checked),
        ));
    }
    let seed: u64 = row["algorithm_seed"].as_str().unwrap().parse().unwrap();
    let a = (seed ^ 0x9e37_79b9_7f4a_7c15) % r;
    let b = ((seed.rotate_left(17) ^ 0xd1b5_4a32_d192_ed03) % (r - 1)) + 1;
    let pairs = [
        (0, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (1, r - 1),
        (a, b),
        (r - 1, r - 1),
    ];
    let mut cases = Vec::new();
    for (case_index, (aa, bb)) in pairs.into_iter().enumerate() {
        let left = s.j.mul(dg, aa);
        let right = s.j.mul(dq, bb);
        let sum = s.j.add(left, right);
        let oracle = s.tau(s.from_short(source.add(source.mul(g, aa), source.mul(q, bb))));
        let (lt, rt, st) = (
            transform(left, p, r0, delta, ys),
            transform(right, p, r0, delta, ys),
            transform(sum, p, r0, delta, ys),
        );
        assert!(jr.valid(lt) && jr.valid(rt) && jr.valid(st));
        let map_correct = jr.add(lt, rt) == st;
        let (status, sum_degree, difference_degree) = coordinate_case(lt, rt, &jr, &branches, t, p);
        let difference = jr.add(lt, jr.neg(rt));
        let (kx, ky, ke, kd) = (
            project_full(lt, &jr, &branches, t, p),
            project_full(rt, &jr, &branches, t, p),
            project_full(st, &jr, &branches, t, p),
            project_full(difference, &jr, &branches, t, p),
        );
        let observed = match (kx, ky, kd) {
            (Some(x), Some(y), Some(d)) => xadd(x, y, d, t, p),
            _ => None,
        };
        let witness = json!({"k_left":kx,"k_right":ky,"k_expected":ke,"k_difference":kd,"k_observed":observed,"jacobian_expected":st});
        let gt = transform(dg, p, r0, delta, ys);
        let next = jr.add(st, gt);
        let other = jr.add(jr.neg(st), gt);
        let signed_lift_count = if st == jr.neg(st) {
            1
        } else {
            match (
                project_full(next, &jr, &branches, t, p),
                project_full(other, &jr, &branches, t, p),
            ) {
                (Some(x), Some(y)) => {
                    if same(x, y, p) {
                        2
                    } else {
                        1
                    }
                }
                _ => 0,
            }
        };
        cases.push(json!({"case_index":case_index,"a":aa,"b":bb,"sum_degree":sum_degree,"difference_degree":difference_degree,"coordinate_status":status,"source_label_correct":oracle==sum,"affine_map_correct":map_correct,"signed_lift_count":signed_lift_count,"coordinate_witness":witness}));
    }
    let all_source_checks = source_scalar_verified
        && cases
            .iter()
            .all(|x| x["source_label_correct"] == true && x["affine_map_correct"] == true);
    json!({"schema":"kummer-coordinate-target/v1","bits":bits,"p":p,"r":r,"target_index":w["target_index"],"workload_id":w["workload_id"],"workload_sha256":w["workload_sha256"],"algorithm_seed":row["algorithm_seed"],"planted":planted,"source_scalar_verified":source_scalar_verified,"all_source_checks":all_source_checks,"case_count":cases.len(),"cases":cases})
}
fn main() {
    let a = fs::read_to_string("research/four_followups_20261021/results/kummer-model-gate.jsonl")
        .unwrap();
    let b =
        fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl").unwrap();
    let mut out = String::new();
    let mut failures = String::new();
    let mut models = std::collections::BTreeMap::new();
    let mut theta = std::collections::BTreeMap::new();
    for (x, y) in a.lines().zip(b.lines()) {
        let r: Value = serde_json::from_str(x).unwrap();
        let t: Value = serde_json::from_str(y).unwrap();
        let p = r["p"].as_u64().unwrap();
        let m = model(&r, &t);
        for (case_index, c) in m["exceptional_cases"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            if c["status"] != "pass" {
                failures.push_str(&format!("{}\n",json!({"schema":"kummer-coordinate-exception/v1","source":"model","p":p,"case_index":case_index,"status":c["status"],"case":c})));
            }
        }
        out.push_str(&format!("{}\n", m));
        models.insert(p, m);
        theta.insert(p, t);
    }
    fs::create_dir_all("research/kummer_coordinates_20261022/results").unwrap();
    fs::write(
        "research/kummer_coordinates_20261022/results/generic-models.jsonl",
        out,
    )
    .unwrap();
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let rows = archive_rows(bits);
        assert_eq!(rows.len(), 180);
        let mut output = String::new();
        for row in rows {
            let p = row["workload"]["curve_spec"]["p"].as_u64().unwrap();
            let result = cohort_target(bits, &row, theta.get(&p).unwrap(), models.get(&p).unwrap());
            for c in result["cases"].as_array().unwrap() {
                if c["coordinate_status"] != "pass" {
                    failures.push_str(&format!("{}\n",json!({"schema":"kummer-coordinate-exception/v1","source":"target","bits":bits,"p":p,"target_index":result["target_index"],"workload_id":result["workload_id"],"case_index":c["case_index"],"status":c["coordinate_status"],"case":c})));
                }
            }
            output.push_str(&format!("{}\n", result));
        }
        fs::write(
            format!("research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"),
            output,
        )
        .unwrap();
    }
    fs::write(
        "research/kummer_coordinates_20261022/results/coordinate-exceptions.jsonl",
        failures,
    )
    .unwrap();
}
