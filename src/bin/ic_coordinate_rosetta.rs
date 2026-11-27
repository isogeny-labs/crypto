//! S59 coordinate-model comparison for prime-field index-calculus research.
//!
//! This instrument has two deliberately separate lanes:
//! - every prime-field EFD model/chart is round-tripped and its applicable
//!   addition/doubling formula is checked and counted;
//! - coordinate-defined factor bases on one NIST-shaped S59 short-Weierstrass
//!   curve are compared on exactly the same planted three-point relations.
//!
//! The first lane measures representation engineering.  The second measures
//! an index-calculus surface.  A projective storage chart is never presented
//! as a new intrinsic coordinate on the curve.

use crypto_lib::cryptanalysis::efd_representations::{chart_ids, EfdRepresentation};
use crypto_lib::cryptanalysis::prime_small_x::{Curve, PrimeField, Pt, SmallPrime};
use crypto_lib::ecc::field::FieldElement;
use crypto_lib::prime_ecc::efd::{catalogue, Bindings, OperationCounts, ProgramSource};
use num_bigint::BigUint;
use serde::Serialize;
use serde_json::json;
use std::collections::VecDeque;
use std::hint::black_box;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const S59_P: u64 = 288_230_445_959_117_953;
const DEFAULT_ITERATIONS: usize = 20_000;
const DEFAULT_SAMPLES: usize = 262_144;
const DEFAULT_SEED: u64 = 0x5335_395f_434f_4f52;

#[derive(Clone, Copy, Debug, Default, Serialize)]
struct Counts {
    multiplications: u64,
    squarings: u64,
    inversions: u64,
    additions: u64,
}

impl From<OperationCounts> for Counts {
    fn from(value: OperationCounts) -> Self {
        Self {
            multiplications: value.multiplications.into(),
            squarings: value.squarings.into(),
            inversions: value.inversions.into(),
            additions: value.additions.into(),
        }
    }
}

impl Counts {
    fn scaled(self, n: u64) -> Self {
        Self {
            multiplications: self.multiplications.saturating_mul(n),
            squarings: self.squarings.saturating_mul(n),
            inversions: self.inversions.saturating_mul(n),
            additions: self.additions.saturating_mul(n),
        }
    }

    fn score(self) -> u64 {
        self.inversions
            .saturating_mul(1_000)
            .saturating_add(self.multiplications.saturating_mul(10))
            .saturating_add(self.squarings.saturating_mul(8))
            .saturating_add(self.additions)
    }
}

#[derive(Clone, Debug, Serialize)]
struct FormulaChoice {
    id: String,
    operation: String,
    program_source: String,
    straight_line_counts: Counts,
    verified_cases: usize,
    iterations: usize,
    wall_ns: Option<u128>,
    checksum: Option<u64>,
}

#[derive(Clone, Debug, Serialize)]
struct ChartRow {
    schema: &'static str,
    field: &'static str,
    p: u64,
    model: String,
    chart: String,
    short_a: String,
    short_b: String,
    status: String,
    error: Option<String>,
    roundtrip_points: usize,
    roundtrip_iterations: usize,
    roundtrip_wall_ns: Option<u128>,
    roundtrip_checksum: Option<u64>,
    addition: Option<FormulaChoice>,
    doubling: Option<FormulaChoice>,
    best_diffadd: Option<FormulaChoice>,
    best_ladder: Option<FormulaChoice>,
    note: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Coordinate {
    X,
    XRepeat,
    NormalizedJacobianX,
    RawJacobianX,
    MobiusX,
    Y,
    XPlusY,
    XSquared,
    XOverY,
}

impl Coordinate {
    const ALL: [Self; 9] = [
        Self::X,
        Self::XRepeat,
        Self::NormalizedJacobianX,
        Self::RawJacobianX,
        Self::MobiusX,
        Self::Y,
        Self::XPlusY,
        Self::XSquared,
        Self::XOverY,
    ];

    fn id(self) -> &'static str {
        match self {
            Self::X => "x",
            Self::XRepeat => "x_aa",
            Self::NormalizedJacobianX => "jacobian_x_normalized",
            Self::RawJacobianX => "jacobian_X_raw_negative_control",
            Self::MobiusX => "mobius_x",
            Self::Y => "y",
            Self::XPlusY => "x_plus_y",
            Self::XSquared => "x_squared",
            Self::XOverY => "x_over_y",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|coordinate| coordinate.id() == id)
    }

    fn per_evaluation_counts(self) -> Counts {
        match self {
            Self::X | Self::XRepeat | Self::Y => Counts::default(),
            Self::NormalizedJacobianX => Counts {
                multiplications: 2,
                squarings: 1,
                inversions: 1,
                additions: 0,
            },
            Self::RawJacobianX => Counts {
                multiplications: 1,
                squarings: 1,
                inversions: 0,
                additions: 0,
            },
            Self::MobiusX => Counts {
                multiplications: 3,
                squarings: 0,
                inversions: 1,
                additions: 2,
            },
            Self::XPlusY => Counts {
                additions: 1,
                ..Counts::default()
            },
            Self::XSquared => Counts {
                squarings: 1,
                ..Counts::default()
            },
            Self::XOverY => Counts {
                multiplications: 1,
                inversions: 1,
                ..Counts::default()
            },
        }
    }

    fn interpretation(self) -> &'static str {
        match self {
            Self::X | Self::XRepeat => "degree-2 Kummer coordinate; valid IC surface",
            Self::NormalizedJacobianX => {
                "same degree-2 Kummer coordinate after projective normalization"
            }
            Self::RawJacobianX => {
                "negative control: raw projective X depends on scale and is not a curve function"
            }
            Self::MobiusX => "degree-2 Mobius reparameterization of the Kummer coordinate",
            Self::Y => "degree-3 function; not matched to the Semaev x-system",
            Self::XPlusY => "degree-3 function; not matched to the Semaev x-system",
            Self::XSquared => "degree-4 composition; rational image collisions can hide degree",
            Self::XOverY => {
                "rational function on this curve; the degree-2 Pornin class needs a compatible 2-torsion model"
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct RelationRow {
    schema: &'static str,
    field: &'static str,
    p: u64,
    curve: &'static str,
    a: u64,
    b: u64,
    coordinate: String,
    interpretation: &'static str,
    seed: u64,
    samples: usize,
    threshold: u64,
    coordinate_evaluations: u64,
    excluded_poles_or_infinity: u64,
    factor_base_hits: u64,
    factor_base_density: f64,
    admitted_relations: u64,
    relation_rate: f64,
    independence_ratio: f64,
    planted_zero_sum_passes: u64,
    negative_zero_sum_failures: u64,
    negation_invariant_matches: u64,
    scale_invariant_matches: u64,
    counted_coordinate_work: Counts,
    wall_ns: u128,
    checksum: u64,
}

fn field_element(p: u64, value: u64) -> FieldElement {
    FieldElement::new(BigUint::from(value % p), BigUint::from(p))
}

fn negative_field_element(p: u64, magnitude: u64) -> FieldElement {
    field_element(p, magnitude).neg()
}

fn params_for(model: &str, chart: &str, p: u64) -> Bindings {
    let mut params = Bindings::new();
    match model {
        "shortw" => {
            let (a, b) = if chart.ends_with("-0") && !chart.contains("w12") {
                (field_element(p, 0), field_element(p, 8))
            } else if chart.ends_with("-1") {
                (negative_field_element(p, 1), field_element(p, 9))
            } else if chart == "shortw/w12-0" {
                (field_element(p, 8), field_element(p, 0))
            } else {
                // NIST shape a = -3, with the planted point (1, 3).
                (negative_field_element(p, 3), field_element(p, 11))
            };
            params.insert("a".into(), a);
            params.insert("b".into(), b);
        }
        "montgom" => {
            params.insert("a".into(), field_element(p, 131));
            params.insert("b".into(), field_element(p, 1));
        }
        "edwards" => {
            params.insert("c".into(), field_element(p, 1));
            params.insert("d".into(), field_element(p, 64));
            if matches!(chart, "edwards/yz" | "edwards/yzsquared") {
                params.insert("r".into(), field_element(p, 8));
            }
        }
        "twisted" => {
            params.insert("a".into(), field_element(p, 2));
            params.insert("d".into(), field_element(p, 12));
        }
        "twistedhessian" => {
            params.insert("a".into(), field_element(p, 1));
            params.insert("d".into(), field_element(p, 6));
        }
        "hessian" => {
            let numerator = field_element(p, 29);
            let denominator = field_element(p, 9).inv().expect("9 is invertible");
            params.insert("d".into(), numerator.mul(&denominator));
        }
        "jintersect" | "jquartic" | "2dik" | "3dik" => {
            params.insert("a".into(), field_element(p, 2));
        }
        _ => {}
    }
    params
}

fn curve_for(representation: &EfdRepresentation) -> Result<Curve<SmallPrime>, String> {
    let (a, b) = representation.short_coefficients();
    let field = SmallPrime::new(S59_P);
    Ok(Curve {
        name: format!(
            "S59 EFD {}/{}",
            representation.model_id(),
            representation.chart_id()
        ),
        a: field.embed_big(&a)?,
        b: field.embed_big(&b)?,
        field,
        // Chart validation and arithmetic do not consume an order.  This
        // sentinel is deliberately never used for a subgroup or DLP claim.
        order: BigUint::from(5u32),
        order_is_prime: true,
        cofactor: 1,
    })
}

fn sample_points(curve: &Curve<SmallPrime>, wanted: usize) -> Vec<Pt<u64>> {
    let mut field = curve.field.clone();
    let mut points = Vec::new();
    for x in 0..1_000_000u64 {
        let rhs = curve.rhs(&mut field, x);
        if let Some(y) = curve.sqrt(&mut field, rhs) {
            points.push(Pt { x, y });
            if y != 0 && points.len() < wanted {
                points.push(Pt { x, y: field.neg(y) });
            }
            if points.len() >= wanted {
                break;
            }
        }
    }
    points
}

fn program_source(source: ProgramSource) -> String {
    match source {
        ProgramSource::Upstream => "upstream".into(),
        ProgramSource::RecordFallback => "record_fallback".into(),
    }
}

fn formula_choices(representation: &EfdRepresentation, operation: &str) -> Vec<FormulaChoice> {
    let mut choices = Vec::new();
    for id in representation.formula_ids(operation) {
        let Some(formula) = catalogue().formula(&id) else {
            continue;
        };
        let Ok(program) = formula.compile_any() else {
            continue;
        };
        choices.push(FormulaChoice {
            id,
            operation: operation.into(),
            program_source: program_source(program.source()),
            straight_line_counts: program.operation_counts().into(),
            verified_cases: 0,
            iterations: 0,
            wall_ns: None,
            checksum: None,
        });
    }
    choices.sort_by_key(|choice| (choice.straight_line_counts.score(), choice.id.clone()));
    choices
}

fn best_static(representation: &EfdRepresentation, operation: &str) -> Option<FormulaChoice> {
    formula_choices(representation, operation)
        .into_iter()
        .next()
}

fn choose_addition(
    representation: &EfdRepresentation,
    curve: &Curve<SmallPrime>,
    points: &[Pt<u64>],
) -> Option<(FormulaChoice, Pt<u64>, Pt<u64>)> {
    for mut choice in formula_choices(representation, "addition") {
        let mut verified = 0usize;
        let mut first = None;
        for (i, p) in points.iter().enumerate() {
            for q in points.iter().skip(i + 1) {
                let mut field = curve.field.clone();
                let Some(expected) = curve.add_points(&mut field, p, q) else {
                    continue;
                };
                if let Ok(got) = representation.chart_add(&curve.field, p, q, Some(&choice.id)) {
                    if got == expected {
                        verified += 1;
                        first.get_or_insert((*p, *q));
                    }
                }
                if verified == 8 {
                    break;
                }
            }
            if verified == 8 {
                break;
            }
        }
        if let Some((p, q)) = first {
            choice.verified_cases = verified;
            return Some((choice, p, q));
        }
    }
    None
}

fn choose_doubling(
    representation: &EfdRepresentation,
    curve: &Curve<SmallPrime>,
    points: &[Pt<u64>],
) -> Option<(FormulaChoice, Pt<u64>)> {
    for mut choice in formula_choices(representation, "doubling") {
        let mut verified = 0usize;
        let mut first = None;
        for p in points {
            let mut field = curve.field.clone();
            let Some(expected) = curve.add_points(&mut field, p, p) else {
                continue;
            };
            if let Ok(got) = representation.chart_double(&curve.field, p, Some(&choice.id)) {
                if got == expected {
                    verified += 1;
                    first.get_or_insert(*p);
                }
            }
            if verified == 8 {
                break;
            }
        }
        if let Some(p) = first {
            choice.verified_cases = verified;
            return Some((choice, p));
        }
    }
    None
}

fn benchmark_addition(
    representation: &EfdRepresentation,
    curve: &Curve<SmallPrime>,
    mut choice: FormulaChoice,
    p: Pt<u64>,
    q: Pt<u64>,
    iterations: usize,
) -> FormulaChoice {
    for _ in 0..128 {
        black_box(
            representation
                .chart_add(&curve.field, &p, &q, Some(&choice.id))
                .expect("qualified addition remains executable"),
        );
    }
    let start = Instant::now();
    let mut checksum = 0u64;
    for _ in 0..iterations {
        let point = representation
            .chart_add(&curve.field, &p, &q, Some(&choice.id))
            .expect("qualified addition remains executable");
        checksum ^= point.x.rotate_left(17) ^ point.y;
        black_box(point);
    }
    choice.iterations = iterations;
    choice.wall_ns = Some(start.elapsed().as_nanos());
    choice.checksum = Some(checksum);
    choice
}

fn benchmark_doubling(
    representation: &EfdRepresentation,
    curve: &Curve<SmallPrime>,
    mut choice: FormulaChoice,
    p: Pt<u64>,
    iterations: usize,
) -> FormulaChoice {
    for _ in 0..128 {
        black_box(
            representation
                .chart_double(&curve.field, &p, Some(&choice.id))
                .expect("qualified doubling remains executable"),
        );
    }
    let start = Instant::now();
    let mut checksum = 0u64;
    for _ in 0..iterations {
        let point = representation
            .chart_double(&curve.field, &p, Some(&choice.id))
            .expect("qualified doubling remains executable");
        checksum ^= point.x.rotate_left(29) ^ point.y;
        black_box(point);
    }
    choice.iterations = iterations;
    choice.wall_ns = Some(start.elapsed().as_nanos());
    choice.checksum = Some(checksum);
    choice
}

fn chart_row(model: &str, chart: &str, iterations: usize) -> ChartRow {
    let params = params_for(model, chart, S59_P);
    let representation = match EfdRepresentation::new(model, chart, &BigUint::from(S59_P), &params)
    {
        Ok(value) => value,
        Err(error) => {
            return ChartRow {
                schema: "ic.coordinate_rosetta.chart/v1",
                field: "S59 registry prime",
                p: S59_P,
                model: model.into(),
                chart: chart.into(),
                short_a: String::new(),
                short_b: String::new(),
                status: "inapplicable".into(),
                error: Some(error),
                roundtrip_points: 0,
                roundtrip_iterations: 0,
                roundtrip_wall_ns: None,
                roundtrip_checksum: None,
                addition: None,
                doubling: None,
                best_diffadd: None,
                best_ladder: None,
                note: "model/chart precondition failed; retained as an explicit outcome",
            };
        }
    };
    let (short_a, short_b) = representation.short_coefficients();
    let curve = match curve_for(&representation) {
        Ok(value) => value,
        Err(error) => {
            return ChartRow {
                schema: "ic.coordinate_rosetta.chart/v1",
                field: "S59 registry prime",
                p: S59_P,
                model: model.into(),
                chart: chart.into(),
                short_a: short_a.to_string(),
                short_b: short_b.to_string(),
                status: "invalid_fixture".into(),
                error: Some(error),
                roundtrip_points: 0,
                roundtrip_iterations: 0,
                roundtrip_wall_ns: None,
                roundtrip_checksum: None,
                addition: None,
                doubling: None,
                best_diffadd: None,
                best_ladder: None,
                note: "no result is inferred from an invalid fixture",
            };
        }
    };
    let points = sample_points(&curve, 24);
    if points.is_empty() {
        return ChartRow {
            schema: "ic.coordinate_rosetta.chart/v1",
            field: "S59 registry prime",
            p: S59_P,
            model: model.into(),
            chart: chart.into(),
            short_a: short_a.to_string(),
            short_b: short_b.to_string(),
            status: "invalid_fixture".into(),
            error: Some("no finite point found below x=1,000,000".into()),
            roundtrip_points: 0,
            roundtrip_iterations: 0,
            roundtrip_wall_ns: None,
            roundtrip_checksum: None,
            addition: None,
            doubling: None,
            best_diffadd: None,
            best_ladder: None,
            note: "no result is inferred from an invalid fixture",
        };
    }

    let mut usable = Vec::new();
    for point in &points {
        if let Ok(coords) = representation.encode_point(&curve.field, point) {
            if let Ok(decoded) = representation.decode_point(&curve.field, &coords, 0) {
                if let Ok(reencoded) = representation.encode_point(&curve.field, &decoded) {
                    if coords == reencoded {
                        usable.push(*point);
                    }
                }
            }
        }
    }
    let roundtrip_points = usable.len();
    let mut roundtrip_checksum = 0u64;
    let start = Instant::now();
    let mut completed = 0usize;
    if !usable.is_empty() {
        for i in 0..iterations {
            let point = usable[i % usable.len()];
            if let Ok(coords) = representation.encode_point(&curve.field, &point) {
                if let Ok(decoded) = representation.decode_point(&curve.field, &coords, 0) {
                    roundtrip_checksum ^= decoded.x.rotate_left(11) ^ decoded.y;
                    black_box(decoded);
                    completed += 1;
                }
            }
        }
    }
    let roundtrip_wall_ns = start.elapsed().as_nanos();

    let addition = choose_addition(&representation, &curve, &usable).map(|(choice, p, q)| {
        benchmark_addition(&representation, &curve, choice, p, q, iterations)
    });
    let doubling = choose_doubling(&representation, &curve, &usable)
        .map(|(choice, p)| benchmark_doubling(&representation, &curve, choice, p, iterations));
    let status = if completed == iterations {
        "verified"
    } else if roundtrip_points > 0 {
        "partial"
    } else {
        "unsupported_chart_points"
    };
    ChartRow {
        schema: "ic.coordinate_rosetta.chart/v1",
        field: "S59 registry prime",
        p: S59_P,
        model: model.into(),
        chart: chart.into(),
        short_a: short_a.to_string(),
        short_b: short_b.to_string(),
        status: status.into(),
        error: None,
        roundtrip_points,
        roundtrip_iterations: completed,
        roundtrip_wall_ns: Some(roundtrip_wall_ns),
        roundtrip_checksum: Some(roundtrip_checksum),
        addition,
        doubling,
        best_diffadd: best_static(&representation, "diffadd"),
        best_ladder: best_static(&representation, "ladder"),
        note: "wall fields require an admitted L2 run; straight-line counts exclude conversion and validation",
    }
}

fn all_chart_jobs() -> Vec<(String, String)> {
    let mut jobs = Vec::new();
    for model in &catalogue().models {
        for chart in chart_ids(&model.id) {
            jobs.push((model.id.clone(), chart));
        }
    }
    jobs
}

fn chart_all(workers: usize, iterations: usize) -> Vec<ChartRow> {
    let queue = Arc::new(Mutex::new(VecDeque::from(all_chart_jobs())));
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for _ in 0..workers.max(1) {
        let queue = Arc::clone(&queue);
        let output = Arc::clone(&output);
        handles.push(thread::spawn(move || loop {
            let job = queue.lock().expect("chart queue").pop_front();
            let Some((model, chart)) = job else {
                break;
            };
            let row = chart_row(&model, &chart, iterations);
            output.lock().expect("chart output").push(row);
        }));
    }
    for handle in handles {
        handle.join().expect("chart worker");
    }
    let mut rows = Arc::try_unwrap(output)
        .expect("all chart workers joined")
        .into_inner()
        .expect("chart output");
    rows.sort_by(|a, b| (&a.model, &a.chart).cmp(&(&b.model, &b.chart)));
    rows
}

fn mod_add(a: u64, b: u64) -> u64 {
    ((a as u128 + b as u128) % S59_P as u128) as u64
}

fn mod_mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % S59_P as u128) as u64
}

fn mod_inv(value: u64) -> Option<u64> {
    if value == 0 {
        return None;
    }
    let (mut old_r, mut r) = (S59_P as i128, value as i128);
    let (mut old_s, mut s) = (0i128, 1i128);
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }
    (old_r == 1).then_some(old_s.rem_euclid(S59_P as i128) as u64)
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn coordinate_value(
    coordinate: Coordinate,
    point: Pt<u64>,
    sample_index: u64,
    seed: u64,
    alternate_scale: bool,
) -> Option<u64> {
    match coordinate {
        Coordinate::X | Coordinate::XRepeat => Some(point.x),
        Coordinate::Y => Some(point.y),
        Coordinate::XPlusY => Some(mod_add(point.x, point.y)),
        Coordinate::XSquared => Some(mod_mul(point.x, point.x)),
        Coordinate::XOverY => Some(mod_mul(point.x, mod_inv(point.y)?)),
        Coordinate::MobiusX => {
            let numerator = mod_add(mod_mul(7, point.x), 11);
            let denominator = mod_add(mod_mul(13, point.x), 17);
            Some(mod_mul(numerator, mod_inv(denominator)?))
        }
        Coordinate::NormalizedJacobianX | Coordinate::RawJacobianX => {
            let domain = if alternate_scale {
                0x414c_545f_5343_414c
            } else {
                0x5052_494d_4152_595a
            };
            let mut z = splitmix64(seed ^ sample_index ^ domain) % S59_P;
            if z == 0 {
                z = 1;
            }
            let z2 = mod_mul(z, z);
            let raw_x = mod_mul(point.x, z2);
            if coordinate == Coordinate::RawJacobianX {
                Some(raw_x)
            } else {
                Some(mod_mul(raw_x, mod_inv(z2)?))
            }
        }
    }
}

fn primary_curve() -> Curve<SmallPrime> {
    let mut field = SmallPrime::new(S59_P);
    Curve {
        name: "S59 NIST-shaped y^2=x^3-3x+11".into(),
        a: field.neg(3),
        b: 11,
        field,
        order: BigUint::from(5u32),
        order_is_prime: false,
        cofactor: 1,
    }
}

fn relation_row(coordinate: Coordinate, samples: usize, seed: u64) -> RelationRow {
    let curve = primary_curve();
    let mut field = curve.field.clone();
    let generator = Pt { x: 1, y: 3 };
    assert!(curve.on_curve(&mut field, &generator));
    let two_g = curve
        .add_points(&mut field, &generator, &generator)
        .expect("planted generator does not double to infinity");
    let three_g = curve
        .add_points(&mut field, &two_g, &generator)
        .expect("planted generator has order above three");
    let mut p = curve
        .mul_big(
            &mut field,
            &generator,
            &BigUint::from((seed % 1_000_003) + 17),
        )
        .expect("seeded P is finite");
    let mut q = curve
        .mul_big(
            &mut field,
            &generator,
            &BigUint::from((splitmix64(seed) % 1_000_003) + 31),
        )
        .expect("seeded Q is finite");
    let threshold = S59_P / 16;
    let mut evaluations = 0u64;
    let mut excluded = 0u64;
    let mut hits = 0u64;
    let mut relations = 0u64;
    let mut positive = 0u64;
    let mut negative_failures = 0u64;
    let mut negation_matches = 0u64;
    let mut scale_matches = 0u64;
    let mut checksum = 0u64;
    let start = Instant::now();
    for index in 0..samples {
        p = curve
            .add_points(&mut field, &p, &two_g)
            .unwrap_or(generator);
        q = curve.add_points(&mut field, &q, &three_g).unwrap_or(two_g);
        let Some(sum) = curve.add_points(&mut field, &p, &q) else {
            evaluations += 3;
            excluded += 3;
            continue;
        };
        let r = curve.negate(&mut field, &sum);
        if curve.add_points(&mut field, &sum, &r).is_none() {
            positive += 1;
        }
        let wrong_r = curve.negate(&mut field, &p);
        if curve.add_points(&mut field, &sum, &wrong_r).is_none() {
            negative_failures += 1;
        }
        let mut all_hit = true;
        for (lane, point) in [p, q, r].into_iter().enumerate() {
            let sample_index = (index as u64).wrapping_mul(3).wrapping_add(lane as u64);
            evaluations += 1;
            let Some(value) = coordinate_value(coordinate, point, sample_index, seed, false) else {
                excluded += 1;
                all_hit = false;
                continue;
            };
            let negated = curve.negate(&mut field, &point);
            if coordinate_value(coordinate, negated, sample_index, seed, false) == Some(value) {
                negation_matches += 1;
            }
            if coordinate_value(coordinate, point, sample_index, seed, true) == Some(value) {
                scale_matches += 1;
            }
            if value < threshold {
                hits += 1;
            } else {
                all_hit = false;
            }
            checksum ^= value.rotate_left((lane * 13) as u32);
        }
        if all_hit {
            relations += 1;
        }
    }
    let wall_ns = start.elapsed().as_nanos();
    let denominator = evaluations.saturating_sub(excluded).max(1);
    let density = hits as f64 / denominator as f64;
    let rate = relations as f64 / samples.max(1) as f64;
    let expected = density * density * density;
    RelationRow {
        schema: "ic.coordinate_rosetta.relation/v1",
        field: "S59 registry prime",
        p: S59_P,
        curve: "y^2 = x^3 - 3x + 11",
        a: S59_P - 3,
        b: 11,
        coordinate: coordinate.id().into(),
        interpretation: coordinate.interpretation(),
        seed,
        samples,
        threshold,
        coordinate_evaluations: evaluations,
        excluded_poles_or_infinity: excluded,
        factor_base_hits: hits,
        factor_base_density: density,
        admitted_relations: relations,
        relation_rate: rate,
        independence_ratio: if expected == 0.0 {
            0.0
        } else {
            rate / expected
        },
        planted_zero_sum_passes: positive,
        negative_zero_sum_failures: negative_failures,
        negation_invariant_matches: negation_matches,
        scale_invariant_matches: scale_matches,
        counted_coordinate_work: coordinate.per_evaluation_counts().scaled(evaluations),
        wall_ns,
        checksum,
    }
}

fn relation_all(workers: usize, samples: usize, seed: u64) -> Vec<RelationRow> {
    let queue = Arc::new(Mutex::new(VecDeque::from(Coordinate::ALL.to_vec())));
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for _ in 0..workers.max(1) {
        let queue = Arc::clone(&queue);
        let output = Arc::clone(&output);
        handles.push(thread::spawn(move || loop {
            let coordinate = queue.lock().expect("relation queue").pop_front();
            let Some(coordinate) = coordinate else {
                break;
            };
            output
                .lock()
                .expect("relation output")
                .push(relation_row(coordinate, samples, seed));
        }));
    }
    for handle in handles {
        handle.join().expect("relation worker");
    }
    let mut rows = Arc::try_unwrap(output)
        .expect("all relation workers joined")
        .into_inner()
        .expect("relation output");
    rows.sort_by(|a, b| a.coordinate.cmp(&b.coordinate));
    rows
}

fn option_value(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find(|window| window[0] == name)
        .map(|window| window[1].clone())
}

fn usize_option(args: &[String], name: &str, default: usize) -> usize {
    option_value(args, name)
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn u64_option(args: &[String], name: &str, default: u64) -> u64 {
    option_value(args, name)
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn inventory() -> serde_json::Value {
    let formulas = &catalogue().formulas;
    let operation_counts = [
        "addition", "doubling", "tripling", "diffadd", "ladder", "scaling",
    ]
    .into_iter()
    .map(|operation| {
        (
            operation,
            formulas
                .iter()
                .filter(|formula| formula.operation == operation)
                .count(),
        )
    })
    .collect::<std::collections::BTreeMap<_, _>>();
    json!({
        "schema": "ic.coordinate_rosetta.inventory/v1",
        "p": S59_P,
        "field": "S59 registry prime",
        "primary_curve": {
            "equation": "y^2 = x^3 - 3x + 11",
            "a": S59_P - 3,
            "b": 11,
            "known_point": [1, 3],
            "scope": "known-scalar relation controls; no curve-order claim"
        },
        "efd": {
            "models": catalogue().models.iter().map(|model| &model.id).collect::<Vec<_>>(),
            "model_count": catalogue().models.len(),
            "charts": catalogue().representations.iter().map(|chart| &chart.id).collect::<Vec<_>>(),
            "chart_count": catalogue().representations.len(),
            "formula_count": formulas.len(),
            "operations": operation_counts
        },
        "coordinate_arms": Coordinate::ALL.iter().map(|coordinate| json!({
            "id": coordinate.id(),
            "interpretation": coordinate.interpretation(),
            "per_evaluation_counts": coordinate.per_evaluation_counts()
        })).collect::<Vec<_>>(),
        "scope_separation": {
            "intrinsic_ic_coordinates": "compared on one NIST-shaped S59 curve",
            "storage_charts": "round-trip and arithmetic engineering only",
            "higher_genus": "requires an explicit subgroup-preserving transfer and recovery path"
        }
    })
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  ic_coordinate_rosetta inventory\n  ic_coordinate_rosetta chart --model M --chart C [--iterations N]\n  ic_coordinate_rosetta chart-all [--workers N] [--iterations N]\n  ic_coordinate_rosetta relations --coordinate C [--samples N] [--seed N]\n  ic_coordinate_rosetta relations-all [--workers N] [--samples N] [--seed N]"
    );
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(command) = args.get(1).map(String::as_str) else {
        usage();
    };
    let value = match command {
        "inventory" => inventory(),
        "chart" => {
            let model = option_value(&args, "--model").unwrap_or_else(|| usage());
            let chart = option_value(&args, "--chart").unwrap_or_else(|| usage());
            let iterations = usize_option(&args, "--iterations", DEFAULT_ITERATIONS);
            serde_json::to_value(chart_row(&model, &chart, iterations)).expect("serialize chart")
        }
        "chart-all" => {
            let workers = usize_option(&args, "--workers", 4);
            let iterations = usize_option(&args, "--iterations", DEFAULT_ITERATIONS);
            json!({
                "schema": "ic.coordinate_rosetta.chart_batch/v1",
                "workers": workers,
                "iterations_per_chart": iterations,
                "wall_admissible": workers == 1,
                "rows": chart_all(workers, iterations)
            })
        }
        "relations" => {
            let id = option_value(&args, "--coordinate").unwrap_or_else(|| usage());
            let coordinate = Coordinate::from_id(&id).unwrap_or_else(|| usage());
            let samples = usize_option(&args, "--samples", DEFAULT_SAMPLES);
            let seed = u64_option(&args, "--seed", DEFAULT_SEED);
            serde_json::to_value(relation_row(coordinate, samples, seed))
                .expect("serialize relation")
        }
        "relations-all" => {
            let workers = usize_option(&args, "--workers", 4);
            let samples = usize_option(&args, "--samples", DEFAULT_SAMPLES);
            let seed = u64_option(&args, "--seed", DEFAULT_SEED);
            json!({
                "schema": "ic.coordinate_rosetta.relation_batch/v1",
                "workers": workers,
                "samples_per_coordinate": samples,
                "seed": seed,
                "wall_admissible": workers == 1,
                "rows": relation_all(workers, samples, seed)
            })
        }
        _ => usage(),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("serialize output")
    );
}
