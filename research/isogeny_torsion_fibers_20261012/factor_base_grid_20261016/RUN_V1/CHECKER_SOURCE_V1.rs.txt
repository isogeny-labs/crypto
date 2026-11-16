//! Independent prime-field and curve replay of the χ-resultant pair sets.

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

fn velu(f: Field, point: Point, torsion: Point) -> Point {
    if point.infinity || point == torsion || point == neg(f, torsion) {
        return identity();
    }
    let plus = add(f, [190, 0], point, torsion);
    let minus = add(f, [190, 0], point, neg(f, torsion));
    Point {
        x: f.sub(
            f.add(f.add(point.x, plus.x), minus.x),
            f.scale(torsion.x, 2),
        ),
        y: f.add(f.add(point.y, plus.y), minus.y),
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

fn check_run(base_path: &str, heldout_path: &str, checker_path: &str, run_path: &str) {
    let base_bytes = fs::read(base_path).unwrap();
    let heldout_bytes = fs::read(heldout_path).unwrap();
    let checker_bytes = fs::read(checker_path).unwrap();
    let run_bytes = fs::read(run_path).unwrap();
    let base: Value = serde_json::from_slice(&base_bytes).unwrap();
    let heldout: Value = serde_json::from_slice(&heldout_bytes).unwrap();
    let checker: Value = serde_json::from_slice(&checker_bytes).unwrap();
    let run: Value = serde_json::from_slice(&run_bytes).unwrap();
    assert_eq!(run["schema"], "isogeny_fiber_base_grid_run/v1");
    assert_eq!(run["base_public_sha256"], sha256_hex(&base_bytes));
    assert_eq!(run["heldout_public_sha256"], sha256_hex(&heldout_bytes));
    assert_eq!(checker["public_sha256"], sha256_hex(&heldout_bytes));
    let columns = run["columns"].as_u64().unwrap() as usize;
    let position = run["position"].as_u64().unwrap() as usize;
    assert!([8, 16, 32, 64].contains(&columns) && position < 4);
    let arm = run["arm"].as_str().unwrap();
    assert!(matches!(
        arm,
        "chi-inverse"
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
        "chi-inverse" | "chi-full" | "direct-inverse" | "direct-full" | "destination-direct"
    );
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
        for index in 0..columns {
            let witness = point(&base["base_witness_source"][index]);
            let subgroup = scalar_mul(f, [190, 0], witness, 195);
            assert_eq!(
                velu(f, subgroup, torsion),
                point(&base["base_projected_destination"][index])
            );
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
    assert!(run["total_cpu_ns"].as_u64().unwrap() >= setup_cpu + query_cpu);
    assert!(run["total_wall_ns"].as_u64().unwrap() > 0);
    assert!(run["peak_rss_bytes"].as_u64().unwrap() <= 512 * 1024 * 1024);
    println!(
        "{}",
        json!({"schema":"isogeny_fiber_base_grid_run_replay/v1",
            "run_sha256":sha256_hex(&run_bytes),
            "columns":columns,"position":position,"arm":arm,
            "scalar_checks":2,"relation_witnesses":rows.len(),
            "rank":run["rank"],"status":"pass"})
    );
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("run") {
        assert_eq!(args.len(), 6);
        check_run(&args[2], &args[3], &args[4], &args[5]);
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
