//! Generic quadratic-resultant bookkeeping with the repository certificate API.
//!
//! identity.certificate/v1 fixes its evaluation field to M61. This auxiliary
//! formal-variable record is not an elliptic-curve fixture, an M31 test, or a
//! formal proof. Production field/curve arithmetic and solver tests are untouched.

use crypto_f4::cryptanalysis::identity_certificate::{
    check, check_with_extra, issue, statement_id, Counterexample, Expr,
    IdentityCertificate, IdentityStatement, IssueError, Verdict, MODULUS,
};
use crypto_f4::hash::sha256::sha256;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};

const ISSUE_SEED: u64 = 0x2026_1124_5359_4c56;
const CHECKER_SEED: u64 = 0x2026_1124_4558_5452;
const POINTS: u32 = 32;
const EXTRA_POINTS: u32 = 16;

fn subtract(left: Expr, right: Expr) -> Expr {
    Expr::plus(left, Expr::minus(right))
}

fn closed_resultant() -> Expr {
    let v = Expr::var;
    let mul = Expr::times;
    let af_cd = subtract(mul(v("a"), v("f")), mul(v("c"), v("d")));
    let ae_bd = subtract(mul(v("a"), v("e")), mul(v("b"), v("d")));
    let bf_ce = subtract(mul(v("b"), v("f")), mul(v("c"), v("e")));
    subtract(mul(af_cd.clone(), af_cd), mul(ae_bd, bf_ce))
}

fn sylvester_determinant() -> Expr {
    let v = Expr::var;
    // Coefficients of a U^2+b U+c and d U^2+e U+f, high to low.
    let matrix = [
        [v("a"), v("b"), v("c"), Expr::Const(0)],
        [Expr::Const(0), v("a"), v("b"), v("c")],
        [v("d"), v("e"), v("f"), Expr::Const(0)],
        [Expr::Const(0), v("d"), v("e"), v("f")],
    ];
    // Independent Leibniz construction: all 24 permutations, with parity
    // determined by inversion counting. No resultant formula is used here.
    let mut terms = Vec::with_capacity(24);
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                for d in 0..4 {
                    let permutation = [a, b, c, d];
                    if (0..4).any(|i| (i + 1..4).any(|j| permutation[i] == permutation[j])) {
                        continue;
                    }
                    let inversions: usize = (0..4)
                        .map(|i| (i + 1..4).filter(|&j| permutation[i] > permutation[j]).count())
                        .sum();
                    let product = Expr::product(
                        (0..4).map(|i| matrix[i][permutation[i]].clone()).collect(),
                    );
                    terms.push(if inversions % 2 == 0 { product } else { Expr::minus(product) });
                }
            }
        }
    }
    assert_eq!(terms.len(), 24);
    Expr::sum(terms)
}

fn statement() -> IdentityStatement {
    IdentityStatement {
        name: "Generic quadratic resultant versus its 4x4 Sylvester determinant".into(),
        source: "Standard quadratic resultant; structured/identity_certificate.rs constructs the closed formula and the full Leibniz determinant independently. Auxiliary fixed-M61 evaluated record only.".into(),
        variables: ["a", "b", "c", "d", "e", "f"].map(str::to_owned).to_vec(),
        degree_bound: 4,
        lhs: closed_resultant(),
        rhs: sylvester_determinant(),
    }
}

fn counterexample(value: Counterexample) -> Value {
    json!({
        "point_index": value.index,
        "assignment_in_variable_order": value.assignment,
        "lhs": value.lhs,
        "rhs": value.rhs,
    })
}

fn digest(bytes: &[u8]) -> String {
    sha256(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

fn write_unchanged_or_new(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        if fs::read(path)? != bytes {
            return Err(format!("refusing to replace differing artifact {}", path.display()).into());
        }
    } else {
        fs::write(path, bytes)?;
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 1 {
        return Err("usage: m31-structured-identity-certificate OUTPUT_DIRECTORY".into());
    }
    assert_eq!(MODULUS, (1u64 << 61) - 1);
    let certificate = issue(
        statement(), ISSUE_SEED, POINTS,
        "Issued and rechecked by the same auxiliary native program using the unchanged repository checker; 16 domain-separated extra points. No independent signed verdict or formal proof is claimed.",
        None,
    )?;
    // Recheck the serialized artifact, rather than just the producer's object.
    let mut certificate_bytes = serde_json::to_vec(&certificate)?;
    certificate_bytes.push(b'\n');
    let replay: IdentityCertificate = serde_json::from_slice(&certificate_bytes)?;
    let acceptance = match check_with_extra(&replay, Some((CHECKER_SEED, EXTRA_POINTS)))? {
        Verdict::Accept { points, extra_points, false_accept_log2 }
            if points == POINTS && extra_points == EXTRA_POINTS => json!({
                "verdict": "accept", "points": points, "extra_points": extra_points,
                "native_diagnostic_false_accept_log2": false_accept_log2,
            }),
        other => return Err(format!("valid certificate failed replay: {other:?}").into()),
    };

    // Adding one corrupts the formula at every field assignment. Exercise both
    // issuer refusal and checker evaluation, not merely sealed-ID rejection.
    let mut corrupted = replay.statement.clone();
    corrupted.lhs = Expr::plus(corrupted.lhs, Expr::Const(1));
    let issuer_refusal = match issue(corrupted.clone(), ISSUE_SEED, POINTS, "corruption control", None) {
        Err(IssueError::False(value)) => counterexample(value),
        other => return Err(format!("corrupted formula was not refused: {other:?}").into()),
    };
    let mut changed_record = replay.clone();
    changed_record.statement = corrupted;
    changed_record.id = statement_id(&changed_record.statement);
    let checker_refusal = match check(&changed_record)? {
        Verdict::Reject(value) => counterexample(value),
        other => return Err(format!("checker failed corruption control: {other:?}").into()),
    };
    let receipt = json!({
        "schema": "generic-resultant-check/v1",
        "certificate_id": replay.id,
        "certificate_sha256": digest(&certificate_bytes),
        "modulus": MODULUS,
        "scope": "generic auxiliary formal-variable algebraic record over fixed M61; not an elliptic-curve fixture, not a replacement for M31 tests, not a formal proof",
        "standing": "unsigned producer evaluation; pcc:unverified",
        "proof": null,
        "checker_seed": CHECKER_SEED,
        "check": acceptance,
        "bound_scope": "The native bound is a model diagnostic for derived evaluation points, not a formal identity proof or an independent signed assurance.",
        "corruption": {"change": "closed resultant plus one", "issuer_refused": issuer_refusal, "checker_rejected": checker_refusal},
        "producer_source_sha256": digest(include_bytes!("identity_certificate.rs")),
        "native_source_manifest_sha256": digest(crypto_f4::SOURCE_MANIFEST.as_bytes()),
    });
    let mut receipt_bytes = serde_json::to_vec_pretty(&receipt)?;
    receipt_bytes.push(b'\n');
    let output = Path::new(&args[0]);
    fs::create_dir_all(output)?;
    write_unchanged_or_new(&output.join("quadratic-resultant.identity.json"), &certificate_bytes)?;
    write_unchanged_or_new(&output.join("quadratic-resultant.check.json"), &receipt_bytes)?;
    println!("{}", json!({"certificate_id": certificate.id, "check": "accept", "issuer_corruption_control": "refused", "checker_corruption_control": "rejected", "modulus": MODULUS, "scope": "generic auxiliary evaluated algebra; no M31 curve fixture or formal proof"}));
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
