//! Exact bounded gate for the pinned m=83 same-field genus-three pullback.
//!
//! This certifies the transfer and runs a known-answer, bound-64 solve in
//! the destination image. It deliberately does not attempt the 81-bit DLP.

use clap::Parser;
use crypto_lib::binary_ecc::curve::{scalar_mul, BinaryCurve};
use crypto_lib::binary_ecc::{
    hcdlp_bsgs_counted, BinaryCoverPoint, BinaryPoint, F2mElement, F2mPoly, IrreduciblePoly,
    MumfordDivisor, OrdinaryBinaryCover, PullbackFiberKind, SubgroupTransferStatus,
};
use crypto_lib::cryptanalysis::curve_construction::weil::is_probable_prime;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

const SLUG: &str = "icv1-f2m83-tm6151469093347-debefd74";
const SUBGROUP_ORDER: &str = "2417851639230796216685689";
const P_X: &str = "477f77103dfad59850800";
const P_Y: &str = "2fa5e737d542c4e4fd5c3";
const Q_X: &str = "590a9c0709b34b0bcd3d4";
const Q_Y: &str = "6aa49dc52796827db931a";

#[derive(Parser)]
#[command(
    name = "n83-genus3-image-gate",
    about = "Certify the pinned m=83 cubic pullback and a bounded destination-image solve"
)]
struct Args {
    /// Write the deterministic certificate to this path; omit for stdout.
    #[arg(long)]
    output: Option<PathBuf>,
    /// Also emit a registry-harvestable `{curves:[...]}` representation file.
    #[arg(long)]
    curve_record_output: Option<PathBuf>,
}

fn integer(value: &str) -> Result<BigUint, String> {
    BigUint::parse_bytes(value.as_bytes(), 10).ok_or_else(|| format!("invalid integer `{value}`"))
}

fn element(value: &str) -> F2mElement {
    F2mElement::from_hex(value, 83)
}

fn point(x: &str, y: &str) -> BinaryPoint {
    BinaryPoint::Affine {
        x: element(x),
        y: element(y),
    }
}

fn polynomial_json(poly: &F2mPoly) -> Value {
    Value::Array(
        poly.coeffs
            .iter()
            .map(|coefficient| Value::String(format!("0x{:x}", coefficient.to_biguint())))
            .collect(),
    )
}

fn divisor_json(divisor: &MumfordDivisor) -> Value {
    json!({
        "u_coefficients_ascending": polynomial_json(&divisor.u),
        "v_coefficients_ascending": polynomial_json(&divisor.v),
    })
}

fn point_json(point: &BinaryPoint) -> Value {
    match point {
        BinaryPoint::Infinity => json!({"infinity": true}),
        BinaryPoint::Affine { x, y } => json!({
            "infinity": false,
            "x": format!("0x{:x}", x.to_biguint()),
            "y": format!("0x{:x}", y.to_biguint()),
        }),
    }
}

fn fiber_name(kind: &PullbackFiberKind) -> &'static str {
    match kind {
        PullbackFiberKind::InfinityRamified { .. } => "infinity_ramified_e3",
        PullbackFiberKind::ExceptionalTwoTorsionRamified { .. } => {
            "exceptional_two_torsion_ramified_e3"
        }
        PullbackFiberKind::GenericSeparable { .. } => "generic_separable_degree3",
    }
}

fn curve_record(group_order: &BigUint, order: &BigUint) -> Value {
    json!({
        "label": "pinned-n83-k0-polynomial-basis-generator",
        "n": 83,
        "a": 0,
        "b": 1,
        "group_order": group_order.to_string(),
        "field": {
            "characteristic": 2,
            "degree": 83,
            "representation": "polynomial",
            "modulus_exponents": [83, 45, 2, 1, 0],
            "element_encoding": "hex polynomial coefficient bitset, least significant bit is constant"
        },
        "curve": {
            "model": "binary Weierstrass",
            "coefficients": [1, 0, 0, 0, 1],
            "subgroup_order": order.to_string(),
            "cofactor": 4,
            "generator": [format!("0x{P_X}"), format!("0x{P_Y}")],
            "target_group": "prime-order subgroup"
        },
        "sources": [
            "AGENTS.md section 8a",
            "tools/ecdlp-surfaces/src/binary.rs",
            "research/n83_genus3_image_solver_20261031/PROTOCOL.md"
        ]
    })
}

fn execute() -> Result<(Value, Value), String> {
    let order = integer(SUBGROUP_ORDER)?;
    let group_order = &order * 4u32;
    let irreducible = IrreduciblePoly {
        degree: 83,
        low_terms: vec![0, 1, 2, 45],
    };
    let p = point(P_X, P_Y);
    let q = point(Q_X, Q_Y);
    let source = BinaryCurve {
        m: 83,
        irreducible: irreducible.clone(),
        a: F2mElement::zero(83),
        b: F2mElement::one(83),
        generator: p.clone(),
        order: order.clone(),
        cofactor: BigUint::from(4u32),
    };
    let cover =
        OrdinaryBinaryCover::new(83, irreducible, F2mElement::zero(83), F2mElement::one(83))
            .map_err(|e| e.to_string())?;
    cover.verify_certificate().map_err(|e| e.to_string())?;

    if !cover.target_is_on_curve(&p) || !cover.target_is_on_curve(&q) {
        return Err("a pinned point is not on the pinned source curve".into());
    }
    let p_order_check = scalar_mul(&source, &p, &order);
    let q_order_check = scalar_mul(&source, &q, &order);
    let q_from_p = scalar_mul(&source, &p, &BigUint::from(37u32));
    if p == BinaryPoint::Infinity
        || q == BinaryPoint::Infinity
        || p_order_check != BinaryPoint::Infinity
        || q_order_check != BinaryPoint::Infinity
        || q_from_p != q
    {
        return Err("pinned source subgroup checks failed".into());
    }
    if !is_probable_prime(&order) {
        return Err("the pinned prime-order factor failed its deterministic screen".into());
    }

    let image_p = cover.pullback_point_class(&p).map_err(|e| e.to_string())?;
    let image_q = cover.pullback_point_class(&q).map_err(|e| e.to_string())?;
    if image_p.is_identity()
        || image_q.is_identity()
        || !image_p
            .try_scalar_mul(&order, cover.source_curve())
            .map_err(|e| e.to_string())?
            .is_identity()
        || !image_q
            .try_scalar_mul(&order, cover.source_curve())
            .map_err(|e| e.to_string())?
            .is_identity()
        || image_p
            .try_scalar_mul(&BigUint::from(37u32), cover.source_curve())
            .map_err(|e| e.to_string())?
            != image_q
    {
        return Err("Jacobian image order or scalar-relation check failed".into());
    }

    let infinity = cover
        .pull_push_composition_certificate(&BinaryPoint::Infinity)
        .map_err(|e| e.to_string())?;
    let exceptional_source = BinaryCoverPoint::Affine {
        u: F2mElement::zero(83),
        v: F2mElement::zero(83),
    };
    let exceptional_target = cover
        .map_point(&exceptional_source)
        .map_err(|e| e.to_string())?;
    let exceptional = cover
        .pull_push_composition_certificate(&exceptional_target)
        .map_err(|e| e.to_string())?;
    let composition_p = cover
        .pull_push_composition_certificate(&p)
        .map_err(|e| e.to_string())?;
    let composition_q = cover
        .pull_push_composition_certificate(&q)
        .map_err(|e| e.to_string())?;
    if !infinity.identity_holds
        || !exceptional.identity_holds
        || !composition_p.identity_holds
        || !composition_q.identity_holds
    {
        return Err("a typed pull/push composition certificate failed".into());
    }

    let (inverse_three, subgroup_status) = match cover
        .subgroup_transfer_status(&order)
        .map_err(|e| e.to_string())?
    {
        SubgroupTransferStatus::InjectiveIfExactSubgroupOrder {
            degree_inverse_mod_order,
            ..
        } => (
            degree_inverse_mod_order,
            "injective_on_certified_r_subgroup",
        ),
        SubgroupTransferStatus::KernelIntersectionUnresolved { .. } => {
            return Err("degree three is not invertible modulo the subgroup order".into())
        }
    };

    let solve = hcdlp_bsgs_counted(
        &image_p,
        &image_q,
        &BigUint::from(64u32),
        cover.source_curve(),
    )
    .map_err(|e| e.to_string())?;
    if solve.recovered != Some(BigUint::from(37u32)) || solve.exhausted {
        return Err("the bound-64 destination solver did not recover scalar 37".into());
    }
    if scalar_mul(
        &source,
        &p,
        solve.recovered.as_ref().expect("checked above"),
    ) != q
    {
        return Err("the recovered scalar failed source-curve replay".into());
    }

    let cert = cover.certificate();
    let record = curve_record(&group_order, &order);
    let receipt = json!({
        "schema": "crypto.n83-genus3-image-gate/v1",
        "status": "pass",
        "claim_scope": "exact_transfer_and_bound_64_known_answer_destination_solve; not_a_full_81_bit_dlp",
        "curve": {
            "slug": SLUG,
            "field": "GF(2^83)",
            "modulus": "z^83 + z^45 + z^2 + z + 1",
            "equation": "y^2 + x*y = x^3 + 1",
            "group_order": group_order.to_string(),
            "subgroup_order": order.to_string(),
            "cofactor": 4,
            "p": point_json(&p),
            "q": point_json(&q),
            "q_relation": "Q=[37]P"
        },
        "cover": {
            "construction": cert.construction,
            "source_equation": "v^2 + u^2*v = u^7 + u",
            "map": "(u,v) -> (u^3, u*v + 1)",
            "genus": cert.source_genus,
            "degree": cert.map_degree,
            "separable": cert.map_separable,
            "riemann_hurwitz_ramification_total": cert.riemann_hurwitz_ramification_total,
            "certificate_replayed": true
        },
        "image_order": {
            "r_prime_factor_provenance": "AGENTS.md section 8a; independently passed fixed-base Miller-Rabin bases 2..37",
            "p_image_nonidentity": true,
            "q_image_nonidentity": true,
            "r_times_p_image_identity": true,
            "r_times_q_image_identity": true,
            "exact_image_order": order.to_string(),
            "reason": "nonidentity element killed by prime r",
            "subgroup_transfer_status": subgroup_status,
            "degree_inverse_mod_r": inverse_three.to_string()
        },
        "pullbacks": {
            "p": divisor_json(&image_p),
            "q": divisor_json(&image_q),
            "thirty_seven_times_p_equals_q": true
        },
        "fibers_and_composition": [
            {
                "input": "infinity",
                "kind": fiber_name(&infinity.fiber_kind),
                "pull_push_equals_three": infinity.identity_holds
            },
            {
                "input": point_json(&exceptional_target),
                "source_point": {"u": "0x0", "v": "0x0"},
                "kind": fiber_name(&exceptional.fiber_kind),
                "pull_push_equals_three": exceptional.identity_holds,
                "exceptional_pullback_two_torsion": true
            },
            {
                "input": "P",
                "kind": fiber_name(&composition_p.fiber_kind),
                "formal_pushforward": point_json(&composition_p.formal_pushforward),
                "three_times_input": point_json(&composition_p.three_times_input),
                "pull_push_equals_three": composition_p.identity_holds
            },
            {
                "input": "Q",
                "kind": fiber_name(&composition_q.fiber_kind),
                "formal_pushforward": point_json(&composition_q.formal_pushforward),
                "three_times_input": point_json(&composition_q.three_times_input),
                "pull_push_equals_three": composition_q.identity_holds
            }
        ],
        "destination_solver": {
            "name": "jacobian_image.bsgs",
            "input_bound_exclusive": "64",
            "received_planted_scalar": false,
            "recovered_scalar": "37",
            "destination_verified": true,
            "source_replay_verified": true,
            "setup": {
                "jacobian_additions": solve.setup.additions,
                "jacobian_doublings": solve.setup.doublings,
                "scalar_multiplications": solve.setup.scalar_multiplications,
                "table_inserts": solve.table_inserts
            },
            "search": {
                "jacobian_additions": solve.search.additions,
                "jacobian_doublings": solve.search.doublings,
                "table_lookups": solve.table_lookups
            },
            "verification": {
                "jacobian_additions": solve.verification.additions,
                "jacobian_doublings": solve.verification.doublings,
                "scalar_multiplications": solve.verification.scalar_multiplications
            }
        },
        "open_cost_obligation": "no certified conversion from genus-3 Cantor operations to source elliptic-curve GAE; no net speedup follows from this gate",
        "curve_record": record
    });
    Ok((receipt, json!({"curves": [record]})))
}

fn write_json(path: Option<PathBuf>, value: &Value) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    if let Some(path) = path {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
        }
        fs::write(&path, [&bytes[..], b"\n"].concat())
            .map_err(|e| format!("write {}: {e}", path.display()))?;
    } else {
        println!("{}", String::from_utf8_lossy(&bytes));
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    match execute() {
        Ok((receipt, curve_records)) => {
            if let Err(error) = write_json(args.output, &receipt).and_then(|_| {
                args.curve_record_output
                    .map(|path| write_json(Some(path), &curve_records))
                    .unwrap_or(Ok(()))
            }) {
                eprintln!("n83 genus-3 image gate: {error}");
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("n83 genus-3 image gate: {error}");
            std::process::exit(1);
        }
    }
}
