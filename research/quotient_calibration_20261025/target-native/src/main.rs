//! Native target-frame calibration and a separate validation-only oracle.
mod coordinate_ring;
mod divisor;
mod kummer;
mod linear;

use coordinate_ring::{Monomial, Ring, Space};
use divisor::Jacobian;
use linear::{Row, Span};
use quotient_calibration_preflight::{
    field::{self, F},
    sections, small,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

const PUBLIC: &[u8] = include_bytes!("../fixtures/public-target.json");
const CONTROLS: &[u8] = include_bytes!("../fixtures/historical-controls.json");
const INPUTS: &[u8] = include_bytes!("../fixtures/coordinate-inputs.json");
const SOURCE: &[u8] = include_bytes!("../../fixtures/calibration-preflight.json");
const FRAME: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 11];

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn value_hash(v: &Value) -> String {
    digest(&serde_json::to_vec(v).unwrap())
}
fn load(path: &Path) -> Value {
    let bytes = fs::read(path).expect("read declared input");
    assert!(bytes.len() < 1_048_576, "input byte bound");
    serde_json::from_slice(&bytes).expect("JSON input")
}
fn pinned(path: &Path, expected: &[u8]) -> Value {
    let bytes = fs::read(path).expect("read pinned public input");
    assert_eq!(
        digest(&bytes),
        digest(expected),
        "public input hash changed"
    );
    serde_json::from_slice(&bytes).unwrap()
}
fn write_new(path: &Path, value: &Value) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("output must be fresh; overwriting/refitting is prohibited");
    serde_json::to_writer_pretty(&mut file, value).unwrap();
    file.write_all(b"\n").unwrap();
    file.sync_all().unwrap();
}
fn object_keys(value: &Value, allowed: &[&str]) {
    let mut actual: Vec<_> = value
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect();
    actual.sort_unstable();
    let mut expected = allowed.to_vec();
    expected.sort_unstable();
    assert_eq!(actual, expected, "unexpected or missing input fields");
}
fn f(value: &Value) -> F {
    let a = value.as_array().expect("field coefficient array");
    assert_eq!(a.len(), 18);
    F(std::array::from_fn(|i| {
        let c = a[i].as_u64().unwrap();
        assert!(c < 7);
        c as u8
    }))
}
fn row(value: &Value) -> Row {
    value.as_array().unwrap().iter().map(f).collect()
}
fn prediction_row(value: &Value) -> Row {
    let coordinates = row(value);
    assert_eq!(
        coordinates.len(),
        8,
        "prediction must contain exactly eight coordinates"
    );
    assert!(
        coordinates.iter().any(|c| *c != F::ZERO),
        "prediction must be a nonzero projective vector"
    );
    coordinates
}
struct Prediction {
    primary: Row,
    auxiliary: Row,
    swapped: Row,
}
/// Validate the entire prediction file before constructing any target divisor.
fn checked_predictions(
    predictions: &Value,
    certificate: &Value,
    inputs: &Value,
) -> Vec<Prediction> {
    object_keys(
        predictions,
        &[
            "schema",
            "matrix_sha256",
            "input_sha256",
            "points",
            "target_divisor_arithmetic_called",
            "discrete_log_or_source_enumeration_called",
            "declared_file_reads",
        ],
    );
    assert_eq!(predictions["schema"], "quotient-coordinate-predictions/v1");
    assert_eq!(predictions["matrix_sha256"], certificate["matrix_sha256"]);
    assert_eq!(predictions["input_sha256"], digest(INPUTS));
    assert_eq!(predictions["target_divisor_arithmetic_called"], false);
    assert_eq!(
        predictions["discrete_log_or_source_enumeration_called"],
        false
    );
    let reads = predictions["declared_file_reads"].as_array().unwrap();
    assert_eq!(reads.len(), 2);
    assert!(reads.iter().all(Value::is_string));
    let frozen = inputs["points"].as_array().unwrap();
    assert_eq!(frozen.len(), 32);
    let predicted = predictions["points"].as_array().unwrap();
    assert_eq!(predicted.len(), 32, "prediction case count must be 32");
    predicted
        .iter()
        .zip(frozen)
        .map(|(prediction, input)| {
            object_keys(
                prediction,
                &[
                    "case_id",
                    "point",
                    "point_plus_G",
                    "mapped_coordinates",
                    "mapped_auxiliary_coordinates",
                    "swapped_coordinate_control",
                ],
            );
            for key in ["case_id", "point", "point_plus_G"] {
                assert_eq!(
                    prediction[key], input[key],
                    "prediction input mismatch: {key}"
                );
            }
            Prediction {
                primary: prediction_row(&prediction["mapped_coordinates"]),
                auxiliary: prediction_row(&prediction["mapped_auxiliary_coordinates"]),
                swapped: prediction_row(&prediction["swapped_coordinate_control"]),
            }
        })
        .collect()
}
fn row_json(row: &[F]) -> Value {
    json!(row.iter().map(|v| v.0).collect::<Vec<_>>())
}
fn matrix_json(matrix: &[Row]) -> Value {
    Value::Array(matrix.iter().map(|r| row_json(r)).collect())
}
fn space_json(space: &Space) -> Value {
    json!({"homogeneous_degree":space.degree,"effective_divisor_degree":space.divisor_degree,"ideal_basis":matrix_json(&space.span.rows)})
}
fn point(value: &Value) -> [u16; 2] {
    let a = value.as_array().unwrap();
    assert_eq!(a.len(), 2);
    std::array::from_fn(|i| {
        let n = a[i].as_u64().unwrap();
        assert!(n < 343);
        n as u16
    })
}

struct Source {
    embedded: Vec<F>,
    weils: [sections::Weil; 3],
}
impl Source {
    fn new() -> Self {
        field::verify_modulus();
        let z = field::cubic_roots([F::c(1), F::ZERO, F::c(1), F::c(1)])[0];
        let embedded = (0..343)
            .map(|n| {
                F::c((n % 7) as u8) + z * F::c((n / 7 % 7) as u8) + z * z * F::c((n / 49) as u8)
            })
            .collect();
        let w = sections::Weil::new(z);
        let weils = [w.clone(), w.conjugate(1), w.conjugate(2)];
        Self { embedded, weils }
    }
    fn coordinates(&self, p: [u16; 2]) -> Row {
        let x = self.embedded[p[0] as usize];
        let y = self.embedded[p[1] as usize];
        assert_eq!(
            y * y,
            x.pow(3) + self.weils[0].a * x + self.weils[0].b,
            "source point membership"
        );
        linear::normalized(sections::sections(&self.weils, (x, y))[..8].to_vec())
    }
}

fn parse_target(value: &Value) -> (Ring, Vec<[F; 3]>, Vec<Vec<u8>>) {
    field::verify_modulus();
    object_keys(
        value,
        &[
            "schema",
            "original_quartic",
            "symmetric_quartic",
            "bitangents",
            "zeta",
            "source_input_sha256",
            "target_input_sha256",
        ],
    );
    assert_eq!(value["schema"], "quotient-target-public/v1");
    let parse_terms = |v: &Value| {
        v.as_array()
            .unwrap()
            .iter()
            .map(|term| {
                let m = std::array::from_fn(|i| term[0][i].as_u64().unwrap() as usize);
                (m, F::c(term[1].as_u64().unwrap() as u8))
            })
            .collect::<Vec<_>>()
    };
    let original = parse_terms(&value["original_quartic"]);
    let symmetric = parse_terms(&value["symmetric_quartic"]);
    let plane: Vec<Row> = vec![vec![5, 1, 1], vec![0, 1, 0], vec![1, 0, 0]]
        .into_iter()
        .map(|r| r.into_iter().map(F::c).collect())
        .collect();
    let cert: Vec<Row> = vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
        .into_iter()
        .map(|r| r.into_iter().map(F::c).collect())
        .collect();
    assert_eq!(
        substitute(&original, &plane),
        symmetric.iter().cloned().collect::<BTreeMap<_, _>>(),
        "original-to-symmetric polynomial identity"
    );
    let chart = linear::multiply(&linear::inverse(&plane), &cert);
    assert_eq!(
        substitute(&original, &cert),
        substitute(&symmetric, &chart),
        "certificate-to-symmetric polynomial identity"
    );
    for (from, to) in [
        ([1, 2, 4], [4, 6, 3]),
        ([1, 1, 0], [0, 1, 0]),
        ([1, 4, 1], [1, 5, 5]),
    ] {
        let a = linear::apply(&chart, &from.map(F::c));
        assert!(linear::projective_equal(&a, &to.map(F::c)));
    }
    let ring = Ring::new(symmetric, 27);
    assert_eq!(
        ring.smoothness_rank(),
        36,
        "degree-seven Jacobian ideal must span every homogeneous form"
    );
    for (m, c) in &ring.quartic {
        if m[2] == 0 {
            assert_eq!((*m, *c), ([2, 2, 0], F::c(2)), "fixed infinity divisor");
        }
    }
    let mut lines = vec![];
    object_keys(&value["bitangents"], &["9", "19", "26"]);
    for label in ["9", "19", "26"] {
        let original: Row = row(&value["bitangents"][label]);
        assert_eq!(original.len(), 3);
        let transformed: Row = (0..3)
            .map(|j| (0..3).fold(F::ZERO, |s, i| s + original[i] * plane[i][j]))
            .collect();
        lines.push(linear::normalized(transformed).try_into().unwrap());
    }
    let coefficients: Vec<Vec<u8>> = value["zeta"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            r.as_array()
                .unwrap()
                .iter()
                .map(|c| {
                    let n = c.as_u64().unwrap();
                    assert!(n < 7);
                    n as u8
                })
                .collect()
        })
        .collect();
    assert_eq!(coefficients.len(), 8);
    assert!(coefficients.iter().all(|r| r.len() == 120));
    (ring, lines, coefficients)
}

fn substitute(terms: &[(Monomial, F)], matrix: &[Row]) -> BTreeMap<Monomial, F> {
    let mut out = BTreeMap::new();
    for (m, c) in terms {
        let mut polynomial = BTreeMap::from([([0, 0, 0], *c)]);
        for i in 0..3 {
            for _ in 0..m[i] {
                let mut product = BTreeMap::new();
                for (mon, value) in &polynomial {
                    for j in 0..3 {
                        let mut n = *mon;
                        n[j] += 1;
                        let v = product.entry(n).or_insert(F::ZERO);
                        *v = *v + *value * matrix[i][j];
                    }
                }
                polynomial = product;
            }
        }
        for (m, c) in polynomial {
            let v = out.entry(m).or_insert(F::ZERO);
            *v = *v + c;
        }
    }
    out.retain(|_, c| *c != F::ZERO);
    out
}

fn fitting(source: &[Row], target: &[Row]) -> (Vec<Row>, usize) {
    assert_eq!(source.len(), 9);
    assert_eq!(target.len(), 9);
    let mut equations = vec![];
    for (q, z) in source.iter().zip(target) {
        let p = z.iter().position(|v| *v != F::ZERO).unwrap();
        for j in 0..8 {
            if j != p {
                let mut r = vec![F::ZERO; 64];
                for k in 0..8 {
                    r[j * 8 + k] = q[k] * z[p];
                    r[p * 8 + k] = -q[k] * z[j];
                }
                equations.push(r);
            }
        }
    }
    assert_eq!(equations.len(), 63);
    let span = Span::from_rows(64, equations);
    let kernel = span.kernel();
    assert_eq!(
        span.rows.len(),
        63,
        "target training frame is not a projective frame"
    );
    assert_eq!(kernel.len(), 1);
    let coefficients = linear::normalized(kernel[0].clone());
    let matrix: Vec<Row> = coefficients.chunks(8).map(|r| r.to_vec()).collect();
    let inverse = linear::inverse(&matrix);
    assert_eq!(linear::multiply(&matrix, &inverse), Span::full(8).rows);
    for (q, z) in source.iter().zip(target) {
        assert!(linear::projective_equal(&linear::apply(&matrix, q), z));
    }
    for omitted in 0..9 {
        assert_eq!(
            Span::from_rows(
                8,
                source
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != omitted)
                    .map(|(_, r)| r.clone())
            )
            .rows
            .len(),
            8
        );
        assert_eq!(
            Span::from_rows(
                8,
                target
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != omitted)
                    .map(|(_, r)| r.clone())
            )
            .rows
            .len(),
            8
        );
    }
    (matrix, span.rows.len())
}

fn calibrate(public: &Path, controls: &Path, source_file: &Path, out: &Path) {
    fs::create_dir(out).expect("fresh calibration directory");
    let public_value = pinned(public, PUBLIC);
    let controls = pinned(controls, CONTROLS);
    let frozen = pinned(source_file, SOURCE);
    let (ring, lines, zeta) = parse_target(&public_value);
    let positive = kummer::historical_controls(&ring, &zeta, &controls);
    write_new(&out.join("historical-controls.json"), &positive);
    let mut jac = Jacobian::new(ring, &lines);
    let sum = jac.add(&jac.generator.clone(), &jac.zero.clone());
    assert!(
        jac.equal_class(&sum, &jac.generator.clone()),
        "anchor plus origin must equal anchor"
    );
    let negative = jac.negative(&jac.generator.clone());
    let zero = jac.add(&negative, &jac.generator.clone());
    assert!(
        jac.equal_class(&zero, &jac.zero.clone()),
        "anchor and residual negative must sum to origin"
    );
    let source = Source::new();
    let mut current = jac.generator.clone();
    let mut q = vec![];
    let mut z = vec![];
    let mut targets = vec![];
    for n in 1..=11 {
        if FRAME.contains(&n) {
            let index = FRAME.iter().position(|k| *k == n).unwrap();
            let p = point(&frozen["training_frame"]["point_coordinates"][index]);
            let vector = source.coordinates(p);
            let frozen_hex = frozen["training_frame"]["eight_coordinate_vectors_hex"][index]
                .as_str()
                .unwrap();
            let actual = vector
                .iter()
                .flat_map(|v| v.0)
                .map(|c| format!("{c:02x}"))
                .collect::<String>();
            assert_eq!(actual, frozen_hex, "source training-vector replay");
            let target = kummer::evaluate(&jac.ring, &current, &zeta)
                .unwrap_or_else(|e| panic!("training scalar {n}: {e}"));
            targets.push(json!({"training_scalar":n,"point":p,"degree_three_divisor":space_json(&current),"target_zeta":row_json(&target)}));
            q.push(vector);
            z.push(target);
        }
        if n != 11 {
            current = jac.add(&current, &jac.generator.clone());
        }
    }
    let (matrix, rank) = fitting(&q, &z);
    let matrix_value = matrix_json(&matrix);
    let matrix_hash = value_hash(&matrix_value);
    let certificate = json!({"schema":"quotient-calibration-matrix/v1","public_target_sha256":digest(PUBLIC),"source_manifest_sha256":digest(SOURCE),"coordinate_inputs_sha256":digest(INPUTS),"matrix":matrix_value,"matrix_sha256":matrix_hash});
    write_new(&out.join("matrix.json"), &certificate);
    write_new(
        &out.join("frame-receipt.json"),
        &json!({"schema":"quotient-target-frame/v1","training_scalars":FRAME,"linear_system_shape":[63,64],"linear_system_rank":rank,"matrix_sha256":matrix_hash,"targets":targets,"divisor_operations":jac.receipts,"calibration_is_validation":false,"holdout_targets_computed":false,"source_scalar_labels_used_for_training_only":true}),
    );
    println!(
        "Frozen matrix {matrix_hash}; nine training pairs fitted. No holdout target computed."
    );
}

fn read_matrix(path: &Path) -> (Value, Vec<Row>) {
    let value = load(path);
    object_keys(
        &value,
        &[
            "schema",
            "public_target_sha256",
            "source_manifest_sha256",
            "coordinate_inputs_sha256",
            "matrix",
            "matrix_sha256",
        ],
    );
    assert_eq!(value["schema"], "quotient-calibration-matrix/v1");
    assert_eq!(value["public_target_sha256"], digest(PUBLIC));
    assert_eq!(value["source_manifest_sha256"], digest(SOURCE));
    assert_eq!(value["coordinate_inputs_sha256"], digest(INPUTS));
    assert_eq!(value["matrix_sha256"], value_hash(&value["matrix"]));
    let matrix: Vec<Row> = value["matrix"]
        .as_array()
        .unwrap()
        .iter()
        .map(row)
        .collect();
    assert_eq!(matrix.len(), 8);
    assert!(matrix.iter().all(|r| r.len() == 8));
    (value, matrix)
}
fn evaluate(matrix_path: &Path, inputs_path: &Path, output: &Path) {
    let (certificate, matrix) = read_matrix(matrix_path);
    let inputs = pinned(inputs_path, INPUTS);
    object_keys(&inputs, &["schema", "points"]);
    assert_eq!(inputs["schema"], "quotient-frozen-coordinate-inputs/v1");
    assert_eq!(inputs["points"].as_array().unwrap().len(), 32);
    let source = Source::new();
    let mut results = vec![];
    for input in inputs["points"].as_array().unwrap() {
        object_keys(input, &["case_id", "point", "point_plus_G"]);
        let p = point(&input["point"]);
        let auxiliary = point(&input["point_plus_G"]);
        assert_eq!(small::ec_add(Some(p), Some([0, 176])), Some(auxiliary));
        let q = source.coordinates(p);
        let prediction = linear::normalized(linear::apply(&matrix, &q));
        let auxiliary_prediction =
            linear::normalized(linear::apply(&matrix, &source.coordinates(auxiliary)));
        let mut swapped = q.clone();
        swapped.swap(0, 1);
        results.push(json!({"case_id":input["case_id"],"point":p,"point_plus_G":auxiliary,"mapped_coordinates":row_json(&prediction),"mapped_auxiliary_coordinates":row_json(&auxiliary_prediction),"swapped_coordinate_control":row_json(&linear::apply(&matrix,&swapped))}));
    }
    write_new(
        output,
        &json!({"schema":"quotient-coordinate-predictions/v1","matrix_sha256":certificate["matrix_sha256"],"input_sha256":digest(INPUTS),"points":results,"target_divisor_arithmetic_called":false,"discrete_log_or_source_enumeration_called":false,"declared_file_reads":[matrix_path,inputs_path]}),
    );
    println!(
        "Evaluated 32 frozen source coordinate inputs and their successors; no target oracle called."
    );
}

fn validation_reference(
    public_path: &Path,
    matrix_path: &Path,
    inputs_path: &Path,
    predictions_path: &Path,
    output: &Path,
) {
    let (certificate, _) = read_matrix(matrix_path);
    let inputs = pinned(inputs_path, INPUTS);
    let predictions = load(predictions_path);
    let predictions = checked_predictions(&predictions, &certificate, &inputs);
    let (ring, lines, zeta) = parse_target(&pinned(public_path, PUBLIC));
    let mut jac = Jacobian::new(ring, &lines);
    // This O(r) enumeration is deliberately validation-only, after matrix
    // publication and evaluator predictions. It is never an evaluator input.
    let mut labels = BTreeMap::new();
    let mut p = None;
    for n in 1..373 {
        p = small::ec_add(p, Some([0, 176]));
        assert!(labels.insert(p.unwrap(), n).is_none());
    }
    assert_eq!(small::ec_add(p, Some([0, 176])), None);
    let mut babies = vec![jac.zero.clone(), jac.generator.clone()];
    for _ in 2..16 {
        babies.push(jac.add(babies.last().unwrap(), &jac.generator.clone()));
    }
    let step = jac.add(babies.last().unwrap(), &jac.generator.clone());
    let mut giants = vec![jac.zero.clone(), step.clone()];
    for _ in 2..24 {
        giants.push(jac.add(giants.last().unwrap(), &step));
    }
    let mut results = vec![];
    let mut matches = 0;
    let mut auxiliary_matches = 0;
    let mut swapped_passes = 0;
    for (index, input) in inputs["points"].as_array().unwrap().iter().enumerate() {
        let predicted = &predictions[index];
        let n = labels[&point(&input["point"])];
        let na = labels[&point(&input["point_plus_G"])];
        let d = jac.add(&giants[n / 16], &babies[n % 16]);
        let da = jac.add(&giants[na / 16], &babies[na % 16]);
        let target = kummer::evaluate(&jac.ring, &d, &zeta)
            .unwrap_or_else(|e| panic!("holdout case {index}: {e}"));
        let target_aux = kummer::evaluate(&jac.ring, &da, &zeta)
            .unwrap_or_else(|e| panic!("holdout successor {index}: {e}"));
        let agrees = linear::projective_equal(&target, &predicted.primary);
        let auxiliary_agrees = linear::projective_equal(&target_aux, &predicted.auxiliary);
        let swapped_agrees = linear::projective_equal(&target, &predicted.swapped);
        matches += usize::from(agrees);
        auxiliary_matches += usize::from(auxiliary_agrees);
        swapped_passes += usize::from(swapped_agrees);
        results.push(json!({"case_id":input["case_id"],"exact_projective_match":agrees,"successor_match":auxiliary_agrees,"swapped_coordinates_match":swapped_agrees,"reference_zeta":row_json(&target),"reference_effective_divisor":space_json(&d),"validation_only_scalar_label":n}));
        eprintln!(
            "frozen holdout {}/32: match={agrees}, successor={auxiliary_agrees}",
            index + 1
        );
    }
    let status = if matches == 32 && auxiliary_matches == 32 && swapped_passes < 32 {
        "calibration_holdout_pass"
    } else {
        "calibration_holdout_failure"
    };
    write_new(
        output,
        &json!({"schema":"quotient-calibration-validation/v1","matrix_sha256":certificate["matrix_sha256"],"input_sha256":digest(INPUTS),"prediction_sha256":digest(&fs::read(predictions_path).unwrap()),"status":status,"primary_matches":matches,"primary_total":32,"successor_matches":auxiliary_matches,"swapped_coordinate_matches":swapped_passes,"reference_source_enumeration_points":372,"results":results,"divisor_operations":jac.receipts,"matrix_refitted":false,"challenges_replaced":false,"divisor_inversion_executed":false,"profile_B_executed":false}),
    );
    println!(
        "{status}: {matches}/32 primary, {auxiliary_matches}/32 successors, swapped control {swapped_passes}/32"
    );
    assert_eq!(
        status, "calibration_holdout_pass",
        "dependent inversion must stop; frozen matrix cannot be refitted"
    );
}

fn main() {
    let a: Vec<_> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    if a.is_empty() || matches!(a[0].to_str(), Some("--help" | "help")) {
        println!("quotient-target-calibration commands:\n  preflight PUBLIC CONTROLS FRESH_DIAGNOSTICS\n  calibrate PUBLIC CONTROLS SOURCE_MANIFEST FRESH_DIRECTORY\n  evaluate MATRIX COORDINATE_INPUTS FRESH_PREDICTIONS\n  validate PUBLIC MATRIX COORDINATE_INPUTS PREDICTIONS FRESH_REPORT\nNew experiments require a frozen protocol and a successful repository checkpoint gate.");
        return;
    }
    let command = a
        .first()
        .and_then(|s| s.to_str())
        .expect("command required: calibrate/evaluate/validate");
    match command {
        "preflight" => {
            assert_eq!(a.len(), 4);
            let started = std::time::Instant::now();
            let (ring, _, zeta) = parse_target(&pinned(&a[1], PUBLIC));
            let controls = pinned(&a[2], CONTROLS);
            let source_start = std::time::Instant::now();
            let source = Source::new();
            let source_setup_ms = source_start.elapsed().as_secs_f64() * 1000.0;
            let q_start = std::time::Instant::now();
            let _ = source.coordinates([0, 176]);
            let source_evaluation_ms = q_start.elapsed().as_secs_f64() * 1000.0;
            let target_start = std::time::Instant::now();
            let checks = kummer::historical_controls(&ring, &zeta, &controls);
            let target_controls_ms = target_start.elapsed().as_secs_f64() * 1000.0;
            write_new(
                &a[3],
                &json!({"schema":"quotient-native-preflight-diagnostics/v1","historical_control_checks":checks,"source_setup_wall_ms":source_setup_ms,"source_evaluation_wall_ms":source_evaluation_ms,"three_historical_target_controls_wall_ms":target_controls_ms,"total_wall_ms":started.elapsed().as_secs_f64()*1000.0,"clocks_are_resource_diagnostics_only":true,"new_target_frame_or_holdout_images_computed":false,"full_reference_divisor_setup_cost_measured":false}),
            );
        }
        "calibrate" => {
            assert_eq!(a.len(), 5);
            calibrate(&a[1], &a[2], &a[3], &a[4]);
        }
        "evaluate" => {
            assert_eq!(a.len(), 4);
            evaluate(&a[1], &a[2], &a[3]);
        }
        "validate" => {
            assert_eq!(a.len(), 6);
            validation_reference(&a[1], &a[2], &a[3], &a[4], &a[5]);
        }
        _ => panic!("unknown command"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_predictions() -> (Value, Value, Value) {
        let inputs: Value = serde_json::from_slice(INPUTS).unwrap();
        let certificate = json!({"matrix_sha256":"synthetic-matrix"});
        let coordinates = row_json(&[F::c(1); 8]);
        let points = inputs["points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|input| {
                json!({"case_id":input["case_id"],"point":input["point"],"point_plus_G":input["point_plus_G"],"mapped_coordinates":coordinates,"mapped_auxiliary_coordinates":coordinates,"swapped_coordinate_control":coordinates})
            })
            .collect::<Vec<_>>();
        let predictions = json!({"schema":"quotient-coordinate-predictions/v1","matrix_sha256":certificate["matrix_sha256"],"input_sha256":digest(INPUTS),"points":points,"target_divisor_arithmetic_called":false,"discrete_log_or_source_enumeration_called":false,"declared_file_reads":["synthetic-matrix.json","coordinate-inputs.json"]});
        (predictions, certificate, inputs)
    }

    #[test]
    fn projective_comparison_rejects_prefixes_and_extra_coordinates() {
        let target = vec![F::c(1); 8];
        for length in [0, 1, 7, 9] {
            let incomplete = vec![F::c(1); length];
            assert!(!linear::projective_equal(&target, &incomplete));
            assert!(!linear::projective_equal(&incomplete, &target));
        }
        assert!(linear::projective_equal(&target, &[F::c(3); 8]));
        assert!(!linear::projective_equal(&target, &[F::ZERO; 8]));
        let mut different = target.clone();
        different[7] = F::c(2);
        assert!(!linear::projective_equal(&target, &different));
    }

    #[test]
    fn prediction_parser_rejects_malformed_vectors_in_every_output_column() {
        let (predictions, certificate, inputs) = synthetic_predictions();
        assert_eq!(
            checked_predictions(&predictions, &certificate, &inputs).len(),
            32
        );
        for key in [
            "mapped_coordinates",
            "mapped_auxiliary_coordinates",
            "swapped_coordinate_control",
        ] {
            for length in [0, 1, 7, 9] {
                let mut malformed = predictions.clone();
                malformed["points"][31][key] = row_json(&vec![F::c(1); length]);
                assert!(std::panic::catch_unwind(|| {
                    checked_predictions(&malformed, &certificate, &inputs)
                })
                .is_err());
            }
            let mut malformed = predictions.clone();
            malformed["points"][31][key] = row_json(&[F::ZERO; 8]);
            assert!(std::panic::catch_unwind(|| {
                checked_predictions(&malformed, &certificate, &inputs)
            })
            .is_err());
            let mut malformed = predictions.clone();
            malformed["points"][31][key][0][17] = json!(7);
            assert!(std::panic::catch_unwind(|| {
                checked_predictions(&malformed, &certificate, &inputs)
            })
            .is_err());
        }
    }

    #[test]
    fn prediction_parser_requires_the_complete_frozen_case_list() {
        let (predictions, certificate, inputs) = synthetic_predictions();
        for count in [31, 33] {
            let mut malformed = predictions.clone();
            let points = malformed["points"].as_array_mut().unwrap();
            if count == 31 {
                points.pop();
            } else {
                points.push(points[0].clone());
            }
            assert!(std::panic::catch_unwind(|| {
                checked_predictions(&malformed, &certificate, &inputs)
            })
            .is_err());
        }
        for key in ["case_id", "point", "point_plus_G"] {
            let mut malformed = predictions.clone();
            malformed["points"][31][key] = Value::Null;
            assert!(std::panic::catch_unwind(|| {
                checked_predictions(&malformed, &certificate, &inputs)
            })
            .is_err());
        }
        let mut duplicate = predictions.clone();
        duplicate["points"][31] = duplicate["points"][0].clone();
        assert!(std::panic::catch_unwind(|| {
            checked_predictions(&duplicate, &certificate, &inputs)
        })
        .is_err());
    }

    #[test]
    fn planted_projective_map_recovers_the_same_matrix() {
        let mut matrix = Span::full(8).rows;
        matrix[0][1] = F::generator();
        matrix[2][4] = F::c(3);
        matrix[6][7] = F::generator().pow(2);
        let mut source = Span::full(8).rows;
        source.push(vec![F::c(1); 8]);
        let target: Vec<_> = source
            .iter()
            .enumerate()
            .map(|(i, q)| {
                linear::apply(&matrix, q)
                    .into_iter()
                    .map(|x| x * (F::generator() + F::c(i as u8)))
                    .collect()
            })
            .collect();
        let (fitted, rank) = fitting(&source, &target);
        assert_eq!(rank, 63);
        assert_eq!(
            linear::normalized(fitted.into_iter().flatten().collect()),
            linear::normalized(matrix.into_iter().flatten().collect())
        );
    }

    #[test]
    #[should_panic(expected = "target training frame is not a projective frame")]
    fn deficient_target_frame_is_rejected() {
        let mut source = Span::full(8).rows;
        source.push(vec![F::c(1); 8]);
        let target = vec![vec![F::c(1); 8]; 9];
        let _ = fitting(&source, &target);
    }

    #[test]
    #[should_panic(expected = "unexpected or missing input fields")]
    fn evaluator_envelope_rejects_reference_scalar_labels() {
        object_keys(
            &json!({"schema":"quotient-frozen-coordinate-inputs/v1","points":[],"source_scalar_labels":[37]}),
            &["schema", "points"],
        );
    }

    #[test]
    fn historical_target_vectors_are_reproduced_by_two_paths() {
        let public: Value = serde_json::from_slice(PUBLIC).unwrap();
        let controls: Value = serde_json::from_slice(CONTROLS).unwrap();
        let (ring, _, coefficients) = parse_target(&public);
        let result = kummer::historical_controls(&ring, &coefficients, &controls);
        assert_eq!(result["completed"], 3);
    }

    #[test]
    fn scalar_fast_path_preserves_extension_products() {
        let a = F::generator() + F::c(2);
        let b = F::generator().pow(3) + F::c(5);
        for c in 0..7 {
            assert_eq!(a * (b * F::c(c)), (a * b) * F::c(c));
            assert_eq!(F::c(c) * a, a * F::c(c));
        }
        assert_eq!((a + b) * F::c(3), a * F::c(3) + b * F::c(3));
    }
}
