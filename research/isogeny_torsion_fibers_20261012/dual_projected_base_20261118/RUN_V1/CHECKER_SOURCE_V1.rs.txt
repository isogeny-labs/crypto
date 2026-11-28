//! Independent field, dual-map, relation, row and scalar replay.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use serde_json::{json, Value};

type El = [u64; 2];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: El,
    y: El,
    infinity: bool,
}

#[derive(Clone, Copy)]
struct Field {
    p: u64,
    nu: u64,
}

impl Field {
    fn add(self, x: El, y: El) -> El {
        [(x[0] + y[0]) % self.p, (x[1] + y[1]) % self.p]
    }
    fn sub(self, x: El, y: El) -> El {
        [
            (x[0] + self.p - y[0]) % self.p,
            (x[1] + self.p - y[1]) % self.p,
        ]
    }
    fn mul(self, x: El, y: El) -> El {
        let p = self.p as u128;
        let real =
            (x[0] as u128 * y[0] as u128 + self.nu as u128 * x[1] as u128 * y[1] as u128) % p;
        let imag = (x[0] as u128 * y[1] as u128 + x[1] as u128 * y[0] as u128) % p;
        [real as u64, imag as u64]
    }
    fn scale(self, x: El, scalar: u64) -> El {
        self.mul(x, [scalar % self.p, 0])
    }
    fn inv(self, x: El) -> El {
        let norm = self.sub(
            self.mul([x[0], 0], [x[0], 0]),
            self.scale([x[1] * x[1] % self.p, 0], self.nu),
        )[0];
        assert_ne!(norm, 0);
        let inv = pow_mod(norm, self.p - 2, self.p);
        self.mul([x[0], (self.p - x[1]) % self.p], [inv, 0])
    }
    fn neg(self, x: El) -> El {
        self.sub([0, 0], x)
    }
}

fn pow_mod(mut x: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = ((result as u128 * x as u128) % modulus as u128) as u64;
        }
        x = ((x as u128 * x as u128) % modulus as u128) as u64;
        exponent >>= 1;
    }
    result
}

fn identity() -> Point {
    Point {
        x: [0, 0],
        y: [0, 0],
        infinity: true,
    }
}

fn neg(f: Field, point: Point) -> Point {
    Point {
        y: f.neg(point.y),
        ..point
    }
}

fn add(f: Field, a: El, p: Point, q: Point) -> Point {
    if p.infinity {
        return q;
    }
    if q.infinity {
        return p;
    }
    if p.x == q.x && p.y == f.neg(q.y) {
        return identity();
    }
    let slope = if p == q {
        let numerator = f.add(f.scale(f.mul(p.x, p.x), 3), a);
        f.mul(numerator, f.inv(f.scale(p.y, 2)))
    } else {
        f.mul(f.sub(q.y, p.y), f.inv(f.sub(q.x, p.x)))
    };
    let x = f.sub(f.sub(f.mul(slope, slope), p.x), q.x);
    let y = f.sub(f.mul(slope, f.sub(p.x, x)), p.y);
    Point {
        x,
        y,
        infinity: false,
    }
}

fn scalar_mul(f: Field, a: El, point: Point, mut scalar: u64) -> Point {
    let mut result = identity();
    let mut base = point;
    while scalar != 0 {
        if scalar & 1 != 0 {
            result = add(f, a, result, base);
        }
        base = add(f, a, base, base);
        scalar >>= 1;
    }
    result
}

fn on_curve(f: Field, a: El, b: El, p: Point) -> bool {
    p.infinity || f.mul(p.y, p.y) == f.add(f.add(f.mul(f.mul(p.x, p.x), p.x), f.mul(a, p.x)), b)
}

fn point(v: &Value) -> Point {
    if v["infinity"] == true {
        return identity();
    }
    Point {
        x: [v["x"][0].as_u64().unwrap(), v["x"][1].as_u64().unwrap()],
        y: [v["y"][0].as_u64().unwrap(), v["y"][1].as_u64().unwrap()],
        infinity: false,
    }
}

fn expected_pairs(f: Field, a: El, signed: &[Point], target: Point) -> BTreeSet<(usize, usize)> {
    let mut out = BTreeSet::new();
    for (i, &p) in signed.iter().enumerate() {
        for (j, &q) in signed.iter().enumerate().skip(i) {
            if add(f, a, p, q) == target {
                out.insert((i, j));
            }
        }
    }
    out
}

fn velu_on_curve(f: Field, a: El, point: Point, torsion: Point) -> Point {
    if point.infinity || point == torsion || point == neg(f, torsion) {
        return identity();
    }
    let plus = add(f, a, point, torsion);
    let minus = add(f, a, point, neg(f, torsion));
    Point {
        x: f.sub(
            f.add(f.add(point.x, plus.x), minus.x),
            f.scale(torsion.x, 2),
        ),
        y: f.add(f.add(point.y, plus.y), minus.y),
        infinity: false,
    }
}

fn velu(f: Field, point: Point, torsion: Point) -> Point {
    velu_on_curve(f, [190, 0], point, torsion)
}

fn independent_dual_kernel(f: Field) -> (Point, u64) {
    let mut squares = BTreeMap::<El, Vec<El>>::new();
    for packed_y in 0..f.p * f.p {
        let y = [packed_y % f.p, packed_y / f.p];
        squares.entry(f.mul(y, y)).or_default().push(y);
    }
    let mut scanned = 0;
    for packed_x in 0..f.p * f.p {
        let x = [packed_x % f.p, packed_x / f.p];
        let rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul([130, 0], x)), [179, 0]);
        if let Some(roots) = squares.get(&rhs) {
            let mut lifts: Vec<Point> = roots
                .iter()
                .map(|&y| Point {
                    x,
                    y,
                    infinity: false,
                })
                .collect();
            lifts.sort_by_key(|point| point_key(f, *point));
            for point in lifts {
                scanned += 1;
                let kernel = scalar_mul(f, [130, 0], point, 14_885);
                if !kernel.infinity {
                    assert_eq!(scalar_mul(f, [130, 0], kernel, 3), identity());
                    return (kernel, scanned);
                }
            }
        }
    }
    panic!("independent dual kernel search exhausted the destination")
}

fn dual_codomain_check(f: Field, kernel: Point) -> (El, El) {
    let x2 = f.mul(kernel.x, kernel.x);
    let x3 = f.mul(x2, kernel.x);
    let t = f.add(f.scale(x2, 3), [130, 0]);
    let w = f.add(
        f.add(f.scale(x3, 5), f.scale(f.mul([130, 0], kernel.x), 3)),
        f.scale([179, 0], 2),
    );
    (
        f.sub([130, 0], f.scale(t, 10)),
        f.sub([179, 0], f.scale(w, 14)),
    )
}

fn dual_eval_check(f: Field, kernel: Point, point: Point) -> Point {
    let image = velu_on_curve(f, [130, 0], point, kernel);
    if image.infinity {
        return image;
    }
    Point {
        x: f.mul(image.x, f.inv([9, 0])),
        y: f.mul(image.y, f.inv([27, 0])),
        infinity: false,
    }
}

fn point_key(f: Field, point: Point) -> u64 {
    if point.infinity {
        return 0;
    }
    let pack = |value: El| value[0] + f.p * value[1];
    let sign = u64::from(pack(point.y) > pack(f.neg(point.y)));
    ((pack(point.x) + 1) << 1) | sign
}

fn destination_direct_base_check(f: Field, columns: usize) -> (Vec<Point>, u64) {
    let mut squares = BTreeMap::<El, Vec<El>>::new();
    for packed_y in 0..f.p * f.p {
        let y = [packed_y % f.p, packed_y / f.p];
        squares.entry(f.mul(y, y)).or_default().push(y);
    }
    let mut selected = Vec::new();
    let mut seen = BTreeSet::new();
    let mut scanned = 0u64;
    for packed_x in 0..f.p * f.p {
        let x = [packed_x % f.p, packed_x / f.p];
        let rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul([130, 0], x)), [179, 0]);
        if let Some(roots) = squares.get(&rhs) {
            let mut lifts: Vec<Point> = roots
                .iter()
                .map(|&y| Point {
                    x,
                    y,
                    infinity: false,
                })
                .collect();
            lifts.sort_by_key(|point| point_key(f, *point));
            for point in lifts {
                scanned += 1;
                let subgroup = scalar_mul(f, [130, 0], point, 195);
                if subgroup.infinity {
                    continue;
                }
                let orbit = point_key(f, subgroup).min(point_key(f, neg(f, subgroup)));
                if seen.insert(orbit) {
                    selected.push(subgroup);
                    if selected.len() == columns {
                        return (selected, scanned);
                    }
                }
            }
        }
    }
    panic!("independent destination scan did not find {columns} columns")
}

fn modmul_229(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % 229) as u64
}

fn independent_row_solve(rows: &[Vec<u64>], variables: usize) -> (usize, Option<Vec<u64>>) {
    let mut matrix = rows.to_vec();
    assert!(matrix.iter().all(|row| row.len() == variables + 1));
    let mut pivots = Vec::new();
    let mut rank = 0;
    for column in 0..variables {
        let Some(next) = (rank..matrix.len()).find(|&index| matrix[index][column] != 0) else {
            continue;
        };
        matrix.swap(rank, next);
        let inverse = pow_mod(matrix[rank][column], 227, 229);
        for j in column..=variables {
            matrix[rank][j] = modmul_229(matrix[rank][j], inverse);
        }
        for i in 0..matrix.len() {
            if i == rank || matrix[i][column] == 0 {
                continue;
            }
            let factor = matrix[i][column];
            for j in column..=variables {
                let pivot_value = matrix[rank][j];
                let updated = (matrix[i][j] + 229 - modmul_229(factor, pivot_value)) % 229;
                matrix[i][j] = updated;
            }
        }
        pivots.push(column);
        rank += 1;
    }
    for row in &matrix {
        if row[..variables].iter().all(|&coefficient| coefficient == 0) {
            assert_eq!(row[variables], 0, "inconsistent witness rows");
        }
    }
    if rank != variables {
        return (rank, None);
    }
    let mut solution = vec![0u64; variables];
    for (index, &column) in pivots.iter().enumerate() {
        solution[column] = matrix[index][variables];
    }
    (rank, Some(solution))
}

fn verify_dual_identity_check(raw_path: &str) {
    let raw_bytes = fs::read(raw_path).unwrap();
    let raw: Value = serde_json::from_slice(&raw_bytes).unwrap();
    assert_eq!(raw["schema"], "isogeny_dual_identity/v1");
    assert_eq!(raw["status"], "pass");
    let f = Field { p: 211, nu: 2 };
    let forward_kernel = Point {
        x: [3, 0],
        y: [1, 0],
        infinity: false,
    };
    let (kernel, scanned) = independent_dual_kernel(f);
    assert_eq!(point(&raw["kernel"]), kernel);
    assert_eq!(raw["kernel_points_scanned"], scanned);
    assert_eq!(dual_codomain_check(f, kernel), ([198, 0], [176, 0]));
    let mut squares = BTreeMap::<El, Vec<El>>::new();
    for packed_y in 0..f.p * f.p {
        let y = [packed_y % f.p, packed_y / f.p];
        squares.entry(f.mul(y, y)).or_default().push(y);
    }
    let mut source_checked = 1u64;
    let mut destination_checked = 1u64;
    for packed_x in 0..f.p * f.p {
        let x = [packed_x % f.p, packed_x / f.p];
        for (a, b, source_side) in [([190, 0], [37, 0], true), ([130, 0], [179, 0], false)] {
            let rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul(a, x)), b);
            if let Some(roots) = squares.get(&rhs) {
                for &y in roots {
                    let p = Point {
                        x,
                        y,
                        infinity: false,
                    };
                    if source_side {
                        let mapped = velu(f, p, forward_kernel);
                        let back = dual_eval_check(f, kernel, mapped);
                        assert_eq!(back, scalar_mul(f, [190, 0], p, 3));
                        source_checked += 1;
                    } else {
                        let back = dual_eval_check(f, kernel, p);
                        assert!(on_curve(f, [190, 0], [37, 0], back));
                        assert_eq!(velu(f, back, forward_kernel), scalar_mul(f, [130, 0], p, 3));
                        destination_checked += 1;
                    }
                }
            }
        }
    }
    assert_eq!(source_checked, 44_655);
    assert_eq!(destination_checked, 44_655);
    assert_eq!(raw["source_points_checked"], source_checked);
    assert_eq!(raw["destination_points_checked"], destination_checked);
    println!(
        "{}",
        json!({"schema":"isogeny_dual_identity_replay/v1",
        "status":"pass","source_points_checked":source_checked,
        "destination_points_checked":destination_checked,
        "kernel_points_scanned":scanned,"raw_sha256":sha256_hex(&raw_bytes)})
    );
}

fn check_run(
    base_path: &str,
    heldout_path: &str,
    checker_path: &str,
    run_path: &str,
    full_base_path: &str,
) {
    let base_bytes = fs::read(base_path).unwrap();
    let heldout_bytes = fs::read(heldout_path).unwrap();
    let checker_bytes = fs::read(checker_path).unwrap();
    let run_bytes = fs::read(run_path).unwrap();
    let base: Value = serde_json::from_slice(&base_bytes).unwrap();
    let full_base_bytes = fs::read(full_base_path).unwrap();
    let full_base: Value = serde_json::from_slice(&full_base_bytes).unwrap();
    assert_eq!(full_base["schema"], "isogeny_fiber_base_grid_public/v1");
    let heldout: Value = serde_json::from_slice(&heldout_bytes).unwrap();
    let checker: Value = serde_json::from_slice(&checker_bytes).unwrap();
    let run: Value = serde_json::from_slice(&run_bytes).unwrap();
    assert_eq!(run["schema"], "isogeny_dual_projected_run/v1");
    assert_eq!(run["base_public_sha256"], sha256_hex(&base_bytes));
    assert_eq!(run["heldout_public_sha256"], sha256_hex(&heldout_bytes));
    assert_eq!(checker["public_sha256"], sha256_hex(&heldout_bytes));
    let columns = run["columns"].as_u64().unwrap() as usize;
    let position = run["position"].as_u64().unwrap() as usize;
    assert!([8, 16, 32, 64].contains(&columns) && position < 4);
    let arm = run["arm"].as_str().unwrap();
    assert!(matches!(
        arm,
        "chi-dual"
            | "direct-dual"
            | "chi-inverse"
            | "chi-full"
            | "direct-inverse"
            | "direct-full"
            | "destination-direct"
            | "source-bsgs"
            | "rho-negation"
            | "rho-negation-aa"
    ));
    let f = Field { p: 211, nu: 2 };
    let torsion = Point {
        x: [3, 0],
        y: [1, 0],
        infinity: false,
    };
    let g = point(&heldout["generator"]);
    let q = point(&heldout["targets"][position]["source"]);
    let mapped_g = point(&heldout["mapped_generator"]);
    let mapped_q = point(&heldout["targets"][position]["destination"]);
    assert_eq!(point(&run["target"]), q);
    assert_eq!(point(&run["mapped_target"]), mapped_q);
    assert!(on_curve(f, [190, 0], [37, 0], g));
    assert!(on_curve(f, [190, 0], [37, 0], q));
    assert!(on_curve(f, [130, 0], [179, 0], mapped_g));
    assert!(on_curve(f, [130, 0], [179, 0], mapped_q));
    assert_eq!(scalar_mul(f, [190, 0], g, 229), identity());
    assert_eq!(scalar_mul(f, [190, 0], q, 229), identity());
    assert_eq!(velu(f, g, torsion), mapped_g);
    assert_eq!(velu(f, q, torsion), mapped_q);
    let scalar = run["recovered"].as_u64().expect("missing target scalar");
    assert_eq!(scalar, checker["scalars"][position].as_u64().unwrap());
    assert_eq!(scalar_mul(f, [190, 0], g, scalar), q);
    assert_eq!(scalar_mul(f, [130, 0], mapped_g, scalar), mapped_q);
    assert_eq!(run["verified"], true);
    let destination_relation = arm == "destination-direct";
    let query_a = if destination_relation {
        [130, 0]
    } else {
        [190, 0]
    };
    let query_g = if destination_relation { mapped_g } else { g };
    let query_q = if destination_relation { mapped_q } else { q };
    let relation_arm = matches!(
        arm,
        "chi-dual"
            | "direct-dual"
            | "chi-inverse"
            | "chi-full"
            | "direct-inverse"
            | "direct-full"
            | "destination-direct"
    );
    let dual_arm = matches!(arm, "chi-dual" | "direct-dual");
    if dual_arm {
        assert_eq!(base["schema"], "isogeny_dual_projected_public/v1");
        assert!(base.get("base_witness_source").is_none());
        assert_eq!(
            base["parent_full_base_sha256"],
            sha256_hex(&full_base_bytes)
        );
    } else {
        assert_eq!(base_bytes, full_base_bytes);
    }
    let mut signed = Vec::new();
    if destination_relation {
        let (expected, scanned) = destination_direct_base_check(f, columns);
        assert_eq!(run["candidate_count"], scanned);
        let reported: Vec<Point> = run["positive_base"]
            .as_array()
            .unwrap()
            .iter()
            .map(point)
            .collect();
        assert_eq!(reported, expected);
        let matches_frozen = expected.iter().enumerate().all(|(index, &point)| {
            point == self::point(&base["base_projected_destination"][index])
        });
        assert_eq!(run["base_matches_frozen"], matches_frozen);
        for subgroup in expected {
            assert_eq!(scalar_mul(f, [130, 0], subgroup, 229), identity());
            signed.push(subgroup);
            signed.push(neg(f, subgroup));
        }
    } else if relation_arm {
        assert_eq!(run["base_matches_frozen"], true);
        let dual_kernel = if dual_arm {
            let (kernel, searched) = independent_dual_kernel(f);
            assert_eq!(run["dual_stats"]["kernel_x"][0], kernel.x[0]);
            assert_eq!(run["dual_stats"]["kernel_x"][1], kernel.x[1]);
            assert_eq!(run["dual_stats"]["kernel_y"][0], kernel.y[0]);
            assert_eq!(run["dual_stats"]["kernel_y"][1], kernel.y[1]);
            assert_eq!(run["dual_stats"]["kernel_points_scanned"], searched);
            assert_eq!(dual_codomain_check(f, kernel), ([198, 0], [176, 0]));
            let (projected, scanned) = destination_direct_base_check(f, columns);
            for (index, &y) in projected.iter().enumerate() {
                assert_eq!(y, point(&base["base_projected_destination"][index]));
            }
            assert_eq!(run["dual_stats"]["destination_points_scanned"], scanned);
            assert_eq!(run["candidate_count"], searched + scanned);
            assert_eq!(run["dual_stats"]["dual_evaluations"], columns as u64);
            assert_eq!(run["dual_stats"]["source_multiplications"], columns as u64);
            assert_eq!(run["dual_stats"]["forward_checks"], columns as u64);
            Some(kernel)
        } else {
            None
        };
        for index in 0..columns {
            let witness = point(&full_base["base_witness_source"][index]);
            let subgroup = scalar_mul(f, [190, 0], witness, 195);
            assert_eq!(
                velu(f, subgroup, torsion),
                point(&base["base_projected_destination"][index])
            );
            if let Some(kernel) = dual_kernel {
                let mapped = point(&base["base_projected_destination"][index]);
                let back = dual_eval_check(f, kernel, mapped);
                assert!(on_curve(f, [190, 0], [37, 0], back));
                assert_eq!(scalar_mul(f, [190, 0], back, 153), subgroup);
                assert_eq!(velu(f, back, torsion), scalar_mul(f, [130, 0], mapped, 3));
            }
            assert_eq!(point(&run["positive_base"][index]), subgroup);
            signed.push(subgroup);
            signed.push(neg(f, subgroup));
        }
    }
    let mut rows = Vec::new();
    for witness in run["relation_witnesses"].as_array().unwrap() {
        let a = witness["a"].as_u64().unwrap();
        let b = witness["b"].as_u64().unwrap();
        let i = witness["i"].as_u64().unwrap() as usize;
        let j = witness["j"].as_u64().unwrap() as usize;
        assert!(a < 229 && b < 229 && i <= j && j < signed.len());
        let probe = add(
            f,
            query_a,
            scalar_mul(f, query_a, query_g, a),
            scalar_mul(f, query_a, query_q, b),
        );
        let projected = scalar_mul(f, query_a, probe, 195);
        assert_eq!(point(&witness["projected"]), projected);
        assert_eq!(add(f, query_a, signed[i], signed[j]), projected);
        let mut row = vec![0u64; columns + 2];
        for index in [i, j] {
            let coefficient = if index % 2 == 0 { 1 } else { 228 };
            row[index / 2] = (row[index / 2] + coefficient) % 229;
        }
        row[columns] = (229 - modmul_229(195, b)) % 229;
        row[columns + 1] = modmul_229(195, a);
        rows.push(row);
    }
    if relation_arm {
        assert_eq!(run["relations"], rows.len());
        assert!(run["trials"].as_u64().unwrap() <= 1832);
        let (rank, solution) = independent_row_solve(&rows, columns + 1);
        assert_eq!(run["rank"], rank);
        assert_eq!(rank, columns + 1);
        assert_eq!(solution.unwrap()[columns], scalar);
        assert!(run["candidate_count"].as_u64().unwrap() > 0);
    } else {
        assert!(rows.is_empty());
        assert_eq!(run["relations"], 0);
        assert_eq!(run["rank"], 0);
        assert_eq!(run["candidate_count"], 0);
    }
    if matches!(arm, "chi-inverse" | "direct-inverse") {
        let stats = &run["inverse_stats"];
        let destination_points = stats["destination_points"].as_u64().unwrap();
        assert_eq!(run["candidate_count"], destination_points);
        assert_eq!(stats["polynomial_calls"], destination_points);
        let classified = stats["rejected_no_root"].as_u64().unwrap()
            + stats["rejected_no_lift"].as_u64().unwrap()
            + stats["rejected_map_mismatch"].as_u64().unwrap()
            + stats["accepted_image_points"].as_u64().unwrap();
        assert_eq!(classified, destination_points);
    }
    let setup_cpu = run["setup_cpu_ns"].as_u64().unwrap();
    let query_cpu = run["query_cpu_ns"].as_u64().unwrap();
    let relation_cpu = run["relation_cpu_ns"].as_u64().unwrap();
    let row_cpu = run["row_cpu_ns"].as_u64().unwrap();
    let recovery_cpu = run["recovery_cpu_ns"].as_u64().unwrap();
    let verify_cpu = run["verify_cpu_ns"].as_u64().unwrap();
    let other_cpu = run["other_cpu_ns"].as_u64().unwrap();
    assert_eq!(relation_cpu + row_cpu + recovery_cpu, query_cpu);
    assert_eq!(
        setup_cpu + query_cpu + verify_cpu + other_cpu,
        run["total_cpu_ns"].as_u64().unwrap()
    );
    if !relation_arm {
        assert_eq!(row_cpu + recovery_cpu, 0);
    }
    assert!(run["chi_stats"]["exhaustive_fallback_evaluations"]
        .as_u64()
        .is_some());
    assert!(run["total_wall_ns"].as_u64().unwrap() > 0);
    assert!(run["peak_rss_bytes"].as_u64().unwrap() <= 512 * 1024 * 1024);
    println!(
        "{}",
        json!({"schema":"isogeny_fiber_base_grid_run_replay/v2",
            "run_sha256":sha256_hex(&run_bytes),
            "columns":columns,"position":position,"arm":arm,
            "scalar_checks":2,"relation_witnesses":rows.len(),
            "rank":run["rank"],"phase_cpu_sum_checked":true,"status":"pass"})
    );
}

fn check_batch(
    base_path: &str,
    full_base_path: &str,
    prior_path: &str,
    public_path: &str,
    checker_path: &str,
    run_path: &str,
) {
    let base_bytes = fs::read(base_path).unwrap();
    let full_base_bytes = fs::read(full_base_path).unwrap();
    let prior_bytes = fs::read(prior_path).unwrap();
    let public_bytes = fs::read(public_path).unwrap();
    let checker_bytes = fs::read(checker_path).unwrap();
    let run_bytes = fs::read(run_path).unwrap();
    let base: Value = serde_json::from_slice(&base_bytes).unwrap();
    let full_base: Value = serde_json::from_slice(&full_base_bytes).unwrap();
    let prior: Value = serde_json::from_slice(&prior_bytes).unwrap();
    let public: Value = serde_json::from_slice(&public_bytes).unwrap();
    let checker: Value = serde_json::from_slice(&checker_bytes).unwrap();
    let records: Vec<Value> = run_bytes
        .split(|&byte| byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).unwrap())
        .collect();
    assert_eq!(
        records.len(),
        65,
        "64 target records plus one summary required"
    );
    let summary = &records[64];
    let arm = summary["arm"].as_str().unwrap();
    let round = summary["round"].as_u64().unwrap();
    assert!(round < 3);
    assert!(matches!(
        arm,
        "direct-dual"
            | "direct-inverse"
            | "destination-direct"
            | "source-bsgs"
            | "rho-negation"
            | "rho-negation-aa"
    ));
    let relation_arm = matches!(arm, "direct-dual" | "direct-inverse" | "destination-direct");
    let destination_relation = arm == "destination-direct";
    assert_eq!(public["schema"], "isogeny_dual_batch_public/v1");
    assert_eq!(checker["schema"], "isogeny_dual_batch_checker/v1");
    assert!(public.get("selection_seed").is_none());
    assert_eq!(checker["selection_seed"], 0x2026_1118_0064u64);
    assert_eq!(checker["public_sha256"], sha256_hex(&public_bytes));
    assert_eq!(public["prior_public_sha256"], sha256_hex(&prior_bytes));
    assert_eq!(public["targets"].as_array().unwrap().len(), 64);
    assert_eq!(checker["scalars"].as_array().unwrap().len(), 64);
    assert_eq!(summary["schema"], "isogeny_dual_batch_summary/v1");
    assert_eq!(summary["base_public_sha256"], sha256_hex(&base_bytes));
    assert_eq!(summary["batch_public_sha256"], sha256_hex(&public_bytes));
    assert_eq!(summary["columns"], 64);
    assert_eq!(summary["target_count"], 64);
    assert_eq!(summary["completed_targets"], 64);
    assert_eq!(summary["status"], "pass");
    assert_eq!(full_base["schema"], "isogeny_fiber_base_grid_public/v1");
    if arm == "direct-dual" {
        assert_eq!(base["schema"], "isogeny_dual_projected_public/v1");
        assert!(base.get("base_witness_source").is_none());
        assert_eq!(
            base["parent_full_base_sha256"],
            sha256_hex(&full_base_bytes)
        );
    } else {
        assert_eq!(base_bytes, full_base_bytes);
    }
    let f = Field { p: 211, nu: 2 };
    let torsion = Point {
        x: [3, 0],
        y: [1, 0],
        infinity: false,
    };
    let generator = point(&public["generator"]);
    let mapped_generator = point(&public["mapped_generator"]);
    assert_eq!(generator, point(&prior["generator"]));
    assert_eq!(mapped_generator, point(&prior["mapped_generator"]));
    assert_eq!(velu(f, generator, torsion), mapped_generator);
    assert_eq!(scalar_mul(f, [190, 0], generator, 229), identity());
    let mut signed = Vec::new();
    let mut expected_positive = Vec::new();
    if relation_arm {
        if destination_relation {
            let (projected, scanned) = destination_direct_base_check(f, 64);
            assert_eq!(summary["candidate_count"], scanned);
            expected_positive = projected;
        } else {
            let dual_kernel = if arm == "direct-dual" {
                let (kernel, searched) = independent_dual_kernel(f);
                assert_eq!(summary["dual_stats"]["kernel_x"][0], kernel.x[0]);
                assert_eq!(summary["dual_stats"]["kernel_x"][1], kernel.x[1]);
                assert_eq!(summary["dual_stats"]["kernel_y"][0], kernel.y[0]);
                assert_eq!(summary["dual_stats"]["kernel_y"][1], kernel.y[1]);
                assert_eq!(summary["dual_stats"]["kernel_points_scanned"], searched);
                assert_eq!(dual_codomain_check(f, kernel), ([198, 0], [176, 0]));
                let (_, scanned) = destination_direct_base_check(f, 64);
                assert_eq!(summary["candidate_count"], searched + scanned);
                assert_eq!(summary["dual_stats"]["destination_points_scanned"], scanned);
                assert_eq!(summary["dual_stats"]["dual_evaluations"], 64);
                assert_eq!(summary["dual_stats"]["source_multiplications"], 64);
                assert_eq!(summary["dual_stats"]["forward_checks"], 64);
                Some(kernel)
            } else {
                None
            };
            for index in 0..64 {
                let witness = point(&full_base["base_witness_source"][index]);
                let subgroup = scalar_mul(f, [190, 0], witness, 195);
                let projected = point(&full_base["base_projected_destination"][index]);
                assert_eq!(velu(f, subgroup, torsion), projected);
                assert_eq!(point(&base["base_projected_destination"][index]), projected);
                if let Some(kernel) = dual_kernel {
                    let back = dual_eval_check(f, kernel, projected);
                    assert_eq!(scalar_mul(f, [190, 0], back, 153), subgroup);
                }
                expected_positive.push(subgroup);
            }
            if arm == "direct-inverse" {
                let stats = &summary["inverse_stats"];
                let scanned = stats["destination_points"].as_u64().unwrap();
                assert_eq!(summary["candidate_count"], scanned);
                assert_eq!(stats["polynomial_calls"], scanned);
                assert_eq!(
                    stats["rejected_no_root"].as_u64().unwrap()
                        + stats["rejected_no_lift"].as_u64().unwrap()
                        + stats["rejected_map_mismatch"].as_u64().unwrap()
                        + stats["accepted_image_points"].as_u64().unwrap(),
                    scanned
                );
            }
        }
        let reported: Vec<Point> = summary["positive_base"]
            .as_array()
            .unwrap()
            .iter()
            .map(point)
            .collect();
        assert_eq!(reported, expected_positive);
        for subgroup in expected_positive {
            signed.push(subgroup);
            signed.push(neg(f, subgroup));
        }
    } else {
        assert_eq!(summary["candidate_count"], 0);
        assert!(summary["positive_base"].as_array().unwrap().is_empty());
    }
    let query_a = if destination_relation {
        [130, 0]
    } else {
        [190, 0]
    };
    let query_g = if destination_relation {
        mapped_generator
    } else {
        generator
    };
    let mut used_scalars = BTreeSet::new();
    let mut total_query_adds = 0u64;
    let mut total_query_doubles = 0u64;
    let mut total_verify_adds = 0u64;
    let mut total_verify_doubles = 0u64;
    let mut total_query_fp_muls = 0u64;
    let mut total_query_fp_invs = 0u64;
    let mut total_verify_fp_muls = 0u64;
    let mut total_verify_fp_invs = 0u64;
    let mut total_query_cpu = 0u64;
    let mut total_verify_cpu = 0u64;
    let mut total_trials = 0u64;
    let mut total_witnesses = 0usize;
    for (position, record) in records.iter().take(64).enumerate() {
        assert_eq!(record["schema"], "isogeny_dual_batch_target/v1");
        assert_eq!(record["base_public_sha256"], sha256_hex(&base_bytes));
        assert_eq!(record["batch_public_sha256"], sha256_hex(&public_bytes));
        assert_eq!(record["arm"], arm);
        assert_eq!(record["round"], round);
        assert_eq!(record["position"], position);
        let seed = (0x2026_1015u64 ^ 211)
            .wrapping_add(round.wrapping_mul(0x9e37_79b9))
            .wrapping_add(((position + 64) as u64).wrapping_mul(0x0123_4567));
        assert_eq!(record["seed"], seed);
        let target = point(&public["targets"][position]["source"]);
        let mapped_target = point(&public["targets"][position]["destination"]);
        if position < 4 {
            assert_eq!(public["targets"][position], prior["targets"][position]);
        }
        assert_eq!(point(&record["target"]), target);
        assert_eq!(point(&record["mapped_target"]), mapped_target);
        assert!(on_curve(f, [190, 0], [37, 0], target));
        assert!(on_curve(f, [130, 0], [179, 0], mapped_target));
        assert_eq!(velu(f, target, torsion), mapped_target);
        let expected_scalar = checker["scalars"][position].as_u64().unwrap();
        assert!(expected_scalar > 0 && expected_scalar < 229);
        assert!(used_scalars.insert(expected_scalar));
        assert_eq!(scalar_mul(f, [190, 0], generator, expected_scalar), target);
        assert_eq!(
            scalar_mul(f, [130, 0], mapped_generator, expected_scalar),
            mapped_target
        );
        assert_eq!(record["recovered"], expected_scalar);
        assert_eq!(record["verified"], true);
        assert_eq!(record["status"], "pass");
        let query_q = if destination_relation {
            mapped_target
        } else {
            target
        };
        let mut rows = Vec::new();
        for witness in record["relation_witnesses"].as_array().unwrap() {
            let a = witness["a"].as_u64().unwrap();
            let b = witness["b"].as_u64().unwrap();
            let i = witness["i"].as_u64().unwrap() as usize;
            let j = witness["j"].as_u64().unwrap() as usize;
            assert!(a < 229 && b < 229 && i <= j && j < signed.len());
            let probe = add(
                f,
                query_a,
                scalar_mul(f, query_a, query_g, a),
                scalar_mul(f, query_a, query_q, b),
            );
            let projected = scalar_mul(f, query_a, probe, 195);
            assert_eq!(point(&witness["projected"]), projected);
            assert_eq!(add(f, query_a, signed[i], signed[j]), projected);
            let mut row = vec![0u64; 66];
            for index in [i, j] {
                let coefficient = if index % 2 == 0 { 1 } else { 228 };
                row[index / 2] = (row[index / 2] + coefficient) % 229;
            }
            row[64] = (229 - modmul_229(195, b)) % 229;
            row[65] = modmul_229(195, a);
            rows.push(row);
        }
        if relation_arm {
            assert_eq!(record["relations"], rows.len());
            assert!(record["trials"].as_u64().unwrap() <= 1832);
            let (rank, answer) = independent_row_solve(&rows, 65);
            assert_eq!(record["rank"], rank);
            assert_eq!(rank, 65);
            assert_eq!(answer.unwrap()[64], expected_scalar);
        } else {
            assert!(rows.is_empty());
            assert_eq!(record["relations"], 0);
            assert_eq!(record["rank"], 0);
        }
        let query_cpu = record["query_cpu_ns"].as_u64().unwrap();
        assert_eq!(
            record["relation_cpu_ns"].as_u64().unwrap()
                + record["row_cpu_ns"].as_u64().unwrap()
                + record["recovery_cpu_ns"].as_u64().unwrap(),
            query_cpu
        );
        total_query_cpu += query_cpu;
        total_verify_cpu += record["verify_cpu_ns"].as_u64().unwrap();
        total_query_adds += record["query_group_ops"]["adds"].as_u64().unwrap();
        total_query_doubles += record["query_group_ops"]["doubles"].as_u64().unwrap();
        total_verify_adds += record["verify_group_ops"]["adds"].as_u64().unwrap();
        total_verify_doubles += record["verify_group_ops"]["doubles"].as_u64().unwrap();
        total_query_fp_muls += record["query_fp_muls"].as_u64().unwrap();
        total_query_fp_invs += record["query_fp_invs"].as_u64().unwrap();
        total_verify_fp_muls += record["verify_fp_muls"].as_u64().unwrap();
        total_verify_fp_invs += record["verify_fp_invs"].as_u64().unwrap();
        total_trials += record["trials"].as_u64().unwrap();
        total_witnesses += rows.len();
    }
    assert_eq!(used_scalars.len(), 64);
    assert_eq!(summary["query_group_ops"]["adds"], total_query_adds);
    assert_eq!(summary["query_group_ops"]["doubles"], total_query_doubles);
    assert_eq!(summary["verify_group_ops"]["adds"], total_verify_adds);
    assert_eq!(summary["verify_group_ops"]["doubles"], total_verify_doubles);
    assert_eq!(summary["query_fp_muls"], total_query_fp_muls);
    assert_eq!(summary["query_fp_invs"], total_query_fp_invs);
    assert_eq!(summary["verify_fp_muls"], total_verify_fp_muls);
    assert_eq!(summary["verify_fp_invs"], total_verify_fp_invs);
    assert_eq!(summary["query_cpu_ns"], total_query_cpu);
    assert_eq!(summary["verify_cpu_ns"], total_verify_cpu);
    assert_eq!(summary["trials"], total_trials);
    assert_eq!(summary["relation_witnesses"], total_witnesses);
    assert_eq!(
        summary["setup_cpu_ns"].as_u64().unwrap()
            + total_query_cpu
            + total_verify_cpu
            + summary["other_cpu_ns"].as_u64().unwrap(),
        summary["total_cpu_ns"].as_u64().unwrap()
    );
    assert!(summary["total_wall_ns"].as_u64().unwrap() > 0);
    assert!(summary["peak_rss_bytes"].as_u64().unwrap() <= 512 * 1024 * 1024);
    println!(
        "{}",
        json!({"schema":"isogeny_dual_batch_replay/v1",
        "run_sha256":sha256_hex(&run_bytes),"arm":arm,"round":round,
        "scalar_checks":128,"target_count":64,"relation_witnesses":total_witnesses,
        "rank_checks":if relation_arm {64} else {0},"status":"pass"})
    );
}

fn audit_batch(run_dir: &str) {
    let arms = [
        "direct-dual",
        "direct-inverse",
        "destination-direct",
        "source-bsgs",
        "rho-negation",
        "rho-negation-aa",
    ];
    let mut compared = 0usize;
    let mut matched_witnesses = 0usize;
    let mut mismatches = Vec::new();
    for round in 0..3 {
        let mut records = BTreeMap::new();
        for arm in arms {
            let path = format!("{run_dir}/r{round}-{arm}.jsonl");
            let bytes = fs::read(&path).unwrap();
            let parsed: Vec<Value> = bytes
                .split(|&byte| byte == b'\n')
                .filter(|line| !line.is_empty())
                .map(|line| serde_json::from_slice(line).unwrap())
                .collect();
            assert_eq!(parsed.len(), 65, "{path} has an incomplete target panel");
            assert_eq!(parsed[64]["status"], "pass");
            records.insert(arm, parsed);
        }
        let dual = &records["direct-dual"];
        let inverse = &records["direct-inverse"];
        let destination = &records["destination-direct"];
        let rho = &records["rho-negation"];
        let rho_aa = &records["rho-negation-aa"];
        assert_eq!(dual[64]["positive_base"], inverse[64]["positive_base"]);
        for position in 0..64 {
            let d = &dual[position];
            let i = &inverse[position];
            let e = &destination[position];
            for key in [
                "seed",
                "position",
                "target",
                "mapped_target",
                "trials",
                "relations",
                "rank",
                "recovered",
                "query_group_ops",
                "relation_witnesses",
            ] {
                if d[key] != i[key] {
                    mismatches.push(format!(
                        "round {round} target {position} dual/inverse {key}"
                    ));
                }
            }
            for key in [
                "seed",
                "position",
                "trials",
                "relations",
                "rank",
                "recovered",
            ] {
                if d[key] != e[key] {
                    mismatches.push(format!("round {round} target {position} destination {key}"));
                }
            }
            let d_witnesses = d["relation_witnesses"].as_array().unwrap();
            let e_witnesses = e["relation_witnesses"].as_array().unwrap();
            if d_witnesses.len() != e_witnesses.len() {
                mismatches.push(format!(
                    "round {round} target {position} destination witness count"
                ));
            } else {
                for (dw, ew) in d_witnesses.iter().zip(e_witnesses) {
                    for key in ["a", "b", "i", "j"] {
                        if dw[key] != ew[key] {
                            mismatches.push(format!(
                                "round {round} target {position} destination witness {key}"
                            ));
                        }
                    }
                }
            }
            for key in [
                "seed",
                "position",
                "target",
                "mapped_target",
                "trials",
                "recovered",
                "query_group_ops",
            ] {
                if rho[position][key] != rho_aa[position][key] {
                    mismatches.push(format!("round {round} target {position} rho A/A {key}"));
                }
            }
            matched_witnesses += d_witnesses.len();
            compared += 1;
        }
    }
    let passed = mismatches.is_empty();
    println!(
        "{}",
        json!({"schema":"isogeny_dual_batch_pair_audit/v1",
        "paired_targets":compared,"relation_witnesses":matched_witnesses,
        "mismatches":mismatches,"status":if passed {"pass"} else {"fail"}})
    );
    assert!(passed, "paired batch relation or control mismatch");
}

fn summarize_batch(run_dir: &str, summary_path: &str, svg_path: &str) {
    let arms = [
        "direct-dual",
        "direct-inverse",
        "destination-direct",
        "source-bsgs",
        "rho-negation",
        "rho-negation-aa",
    ];
    let denominator = 64.0 * (229.0f64).sqrt();
    let mut rows = Vec::new();
    let mut per_target = BTreeMap::new();
    for arm in arms {
        let mut totals = Vec::new();
        let mut setup = Vec::new();
        let mut query = Vec::new();
        let mut verify = Vec::new();
        let mut cpu = Vec::new();
        let mut wall = Vec::new();
        let mut witnesses = 0u64;
        for round in 0..3 {
            let path = format!("{run_dir}/children/r{round}-{arm}.jsonl");
            let bytes = fs::read(path).unwrap();
            let last = bytes
                .split(|&byte| byte == b'\n')
                .rfind(|line| !line.is_empty())
                .unwrap();
            let record: Value = serde_json::from_slice(last).unwrap();
            assert_eq!(record["schema"], "isogeny_dual_batch_summary/v1");
            assert_eq!(record["arm"], arm);
            assert_eq!(record["round"], round);
            assert_eq!(record["completed_targets"], 64);
            assert_eq!(record["status"], "pass");
            let count = |key: &str| {
                record[key]["adds"].as_u64().unwrap() + record[key]["doubles"].as_u64().unwrap()
            };
            let setup_count = count("setup_group_ops");
            let query_count = count("query_group_ops");
            let verify_count = count("verify_group_ops");
            setup.push(setup_count as f64);
            query.push(query_count as f64);
            verify.push(verify_count as f64);
            totals.push((setup_count + query_count + verify_count) as f64);
            cpu.push(record["total_cpu_ns"].as_u64().unwrap() as f64 / 1e6);
            wall.push(record["total_wall_ns"].as_u64().unwrap() as f64 / 1e6);
            witnesses += record["relation_witnesses"].as_u64().unwrap();
        }
        let mean = |values: &[f64]| values.iter().sum::<f64>() / values.len() as f64;
        let s = mean(&totals) / denominator;
        per_target.insert(arm, s);
        rows.push(json!({"arm":arm,"rounds":3,"targets_per_round":64,
            "mean_whole_batch_group_ops":mean(&totals),
            "mean_setup_group_ops":mean(&setup),
            "mean_query_group_ops":mean(&query),
            "mean_verify_group_ops":mean(&verify),
            "min_whole_batch_group_ops":totals.iter().copied().reduce(f64::min).unwrap(),
            "max_whole_batch_group_ops":totals.iter().copied().reduce(f64::max).unwrap(),
            "mean_s_group_per_target":s,
            "mean_total_cpu_ms_l0":mean(&cpu),
            "mean_total_wall_ms_l0":mean(&wall),
            "relation_witnesses_across_rounds":witnesses}));
    }
    let result = json!({"schema":"isogeny_dual_batch_summary_table/v1",
        "population":"same 64 public order-229 targets in each of three rounds",
        "unit":"mean whole-batch additions and doublings / (64 sqrt(229))",
        "isolation":"L0 shared Darwin",
        "complete_portable_s":null,
        "rows":rows,
        "ratios":{"inverse_over_dual_s_group":per_target["direct-inverse"]/per_target["direct-dual"],
            "dual_over_rho_s_group":per_target["direct-dual"]/per_target["rho-negation"],
            "destination_over_dual_s_group":per_target["destination-direct"]/per_target["direct-dual"]}});
    fs::write(
        summary_path,
        format!("{}\n", serde_json::to_string_pretty(&result).unwrap()),
    )
    .unwrap();
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"980\" height=\"540\" viewBox=\"0 0 980 540\"><rect width=\"980\" height=\"540\" fill=\"white\"/><text x=\"45\" y=\"38\" font-family=\"sans-serif\" font-size=\"23\">64-target reused-setup fiber panel</text><text x=\"45\" y=\"65\" font-family=\"sans-serif\" font-size=\"14\">Mean whole-batch group additions and doublings / (64 sqrt(229)); three rounds, L0 Darwin</text>");
    for exponent in 0..=4 {
        let y = 420 - exponent * 75;
        svg.push_str(&format!("<line x1=\"75\" y1=\"{y}\" x2=\"945\" y2=\"{y}\" stroke=\"#dddddd\"/><text x=\"38\" y=\"{}\" font-family=\"sans-serif\" font-size=\"12\">10^{exponent}</text>", y + 4));
    }
    for (index, arm) in arms.iter().enumerate() {
        let value = per_target[arm];
        let height = value.log10().max(0.0) * 75.0;
        let x = 92 + index * 143;
        let y = 420.0 - height;
        let color = if *arm == "direct-dual" {
            "#2769a8"
        } else if *arm == "rho-negation" {
            "#b06520"
        } else {
            "#6b858e"
        };
        svg.push_str(&format!("<rect x=\"{x}\" y=\"{y:.1}\" width=\"78\" height=\"{height:.1}\" fill=\"{color}\"/><text x=\"{x}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"13\">{value:.2}</text><text x=\"{x}\" y=\"448\" font-family=\"sans-serif\" font-size=\"12\">{arm}</text>", y - 7.0));
    }
    svg.push_str("<text x=\"45\" y=\"505\" font-family=\"sans-serif\" font-size=\"13\">Counted group work only; native field, square-root, table, allocation, startup, and I/O conversion remains open.</text></svg>\n");
    fs::write(svg_path, svg).unwrap();
    println!(
        "{}",
        json!({"schema":"isogeny_dual_batch_summary_written/v1",
        "summary_sha256":sha256_hex(&fs::read(summary_path).unwrap()),
        "svg_sha256":sha256_hex(&fs::read(svg_path).unwrap()),
        "status":"pass"})
    );
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("summarize-batch") {
        assert_eq!(args.len(), 5);
        summarize_batch(&args[2], &args[3], &args[4]);
        return;
    }
    if args.get(1).map(String::as_str) == Some("audit-batch") {
        assert_eq!(args.len(), 3);
        audit_batch(&args[2]);
        return;
    }
    if args.get(1).map(String::as_str) == Some("check-batch") {
        assert_eq!(args.len(), 8);
        check_batch(&args[2], &args[3], &args[4], &args[5], &args[6], &args[7]);
        return;
    }
    if args.get(1).map(String::as_str) == Some("verify-dual") {
        assert_eq!(args.len(), 3);
        verify_dual_identity_check(&args[2]);
        return;
    }
    if args.get(1).map(String::as_str) == Some("run") {
        assert_eq!(args.len(), 7);
        check_run(&args[2], &args[3], &args[4], &args[5], &args[6]);
        return;
    }
    assert_eq!(
        args.len(),
        5,
        "usage: checker BASE.json PARENT.json B RAW.jsonl"
    );
    let base_bytes = fs::read(&args[1]).unwrap();
    let parent_bytes = fs::read(&args[2]).unwrap();
    let raw_bytes = fs::read(&args[4]).unwrap();
    let base: Value = serde_json::from_slice(&base_bytes).unwrap();
    let parent: Value = serde_json::from_slice(&parent_bytes).unwrap();
    let columns: usize = args[3].parse().unwrap();
    assert!([8, 16, 32, 64].contains(&columns));
    assert_eq!(base["schema"], "isogeny_fiber_base_grid_public/v1");
    assert_eq!(
        base["parent_public_sha256"].as_str().unwrap(),
        sha256_hex(&parent_bytes)
    );
    assert_eq!(base["columns"], 64);
    assert_eq!(base["source_points"], 44_655);
    assert_eq!(base["image_points"], 14_885);
    let f = Field { p: 211, nu: 2 };
    let torsion = Point {
        x: [3, 0],
        y: [1, 0],
        infinity: false,
    };
    let generator = point(&parent["generator"]);
    assert!(on_curve(f, [190, 0], [37, 0], generator));
    assert_eq!(scalar_mul(f, [190, 0], generator, 229), identity());
    let mut signed = Vec::with_capacity(2 * columns);
    let mut orbits = BTreeSet::new();
    for index in 0..64 {
        let witness = point(&base["base_witness_source"][index]);
        let raw = point(&base["base_raw_destination"][index]);
        let projected = point(&base["base_projected_destination"][index]);
        assert!(on_curve(f, [190, 0], [37, 0], witness));
        assert!(on_curve(f, [130, 0], [179, 0], raw));
        assert!(on_curve(f, [130, 0], [179, 0], projected));
        assert_eq!(velu(f, witness, torsion), raw);
        let subgroup = scalar_mul(f, [190, 0], witness, 195);
        assert!(!subgroup.infinity);
        assert_eq!(scalar_mul(f, [190, 0], subgroup, 229), identity());
        assert_eq!(velu(f, subgroup, torsion), projected);
        assert_eq!(scalar_mul(f, [130, 0], projected, 229), identity());
        let orbit = {
            let negated = neg(f, projected);
            let a = (projected.x, projected.y);
            let b = (negated.x, negated.y);
            a.min(b)
        };
        assert!(orbits.insert(orbit), "duplicate signed orbit {index}");
        if index < 8 {
            assert_eq!(raw, point(&parent["full_base_raw"][index]));
            assert_eq!(projected, point(&parent["full_base_projected"][index]));
            assert_eq!(witness, point(&parent["full_base_witness_source"][index]));
        }
        if index < columns {
            signed.push(subgroup);
            signed.push(neg(f, subgroup));
        }
    }
    let mut rows = raw_bytes
        .split(|&byte| byte == b'\n')
        .filter(|row| !row.is_empty())
        .map(|row| serde_json::from_slice::<Value>(row).unwrap());
    let mut residual = identity();
    let mut supported = 0usize;
    let mut pairs = 0usize;
    let mut diagonal_pairs = 0usize;
    for scalar_label in 0..229u64 {
        let row = rows.next().expect("missing exactness row");
        assert_eq!(row["schema"], "isogeny_fiber_base_grid_case/v1");
        assert_eq!(row["public_sha256"], sha256_hex(&base_bytes));
        assert_eq!(row["columns"], columns);
        assert_eq!(row["scalar_label"], scalar_label);
        assert_eq!(row["status"], "pass");
        assert_eq!(point(&row["residual"]), residual);
        let reported: BTreeSet<(usize, usize)> = row["pairs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|pair| {
                let i = pair[0].as_u64().unwrap() as usize;
                let j = pair[1].as_u64().unwrap() as usize;
                assert!(i <= j && j < signed.len());
                (i, j)
            })
            .collect();
        assert_eq!(reported.len(), row["pairs"].as_array().unwrap().len());
        assert_eq!(reported, expected_pairs(f, [190, 0], &signed, residual));
        supported += usize::from(!reported.is_empty());
        pairs += reported.len();
        diagonal_pairs += reported.iter().filter(|(i, j)| i == j).count();
        residual = add(f, [190, 0], residual, generator);
    }
    assert_eq!(residual, identity());
    let summary = rows.next().expect("missing summary");
    assert!(rows.next().is_none(), "extra rows");
    assert_eq!(summary["schema"], "isogeny_fiber_base_grid_exhaustive/v1");
    assert_eq!(summary["public_sha256"], sha256_hex(&base_bytes));
    assert_eq!(summary["columns"], columns);
    assert_eq!(summary["cases"], 229);
    assert_eq!(summary["supported_residues"], supported);
    assert_eq!(summary["zero_residues"], 229 - supported);
    assert_eq!(summary["unordered_pairs"], pairs);
    assert_eq!(summary["diagonal_pairs"], diagonal_pairs);
    assert_eq!(summary["status"], "pass");
    println!(
        "{}",
        json!({"schema":"isogeny_fiber_base_grid_replay/v1",
            "base_public_sha256":sha256_hex(&base_bytes),
            "raw_sha256":sha256_hex(&raw_bytes),
            "columns":columns,"cases_replayed":229,
            "base_columns_replayed":64,"supported_residues":supported,
            "unordered_pairs":pairs,"diagonal_pairs":diagonal_pairs,
            "status":"pass"})
    );
}
