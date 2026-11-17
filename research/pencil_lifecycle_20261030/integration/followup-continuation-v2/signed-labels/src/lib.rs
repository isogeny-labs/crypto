//! Concrete saved-stream label checks using the existing native affine Cantor law.
//! This is not independent arithmetic or a universal transfer/source-fidelity proof.
use five_leads_study::three_leads::{Divisor, Point, Scholten};
use serde::Serialize;
use serde_json::{json, Value};
use signed_collector_replay::{replay_jsonl, Limits};
fn require(b: bool, m: &str) -> Result<(), String> {
    if b {
        Ok(())
    } else {
        Err(m.into())
    }
}
fn num(v: &Value, k: &str) -> Result<u64, String> {
    v[k].as_u64().ok_or_else(|| format!("unsigned {k}"))
}
fn div(v: &Value, p: u64) -> Result<Divisor, String> {
    let d: Divisor = serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
    require(
        d.degree <= 2 && d.u.iter().chain(&d.v).all(|x| *x < p),
        "divisor domain",
    )?;
    require(
        d.u[d.degree] == 1
            && d.u[d.degree + 1..].iter().all(|x| *x == 0)
            && d.v[d.degree..].iter().all(|x| *x == 0),
        "divisor padding",
    )?;
    require(
        serde_json::to_value(d).map_err(|e| e.to_string())? == *v,
        "divisor fields",
    )?;
    Ok(d)
}
fn point(v: &Value, q: u64) -> Result<Point, String> {
    let p: Point = serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
    require(!p.inf && p.x < q && p.y < q, "source point domain")?;
    require(
        serde_json::to_value(p).map_err(|e| e.to_string())? == *v,
        "source point fields",
    )?;
    Ok(p)
}
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Equation {
    pub sequence: u64,
    pub kind: String,
    pub attempt: u64,
    pub a: u64,
    pub b: u64,
    pub divisor: Divisor,
    pub matches: bool,
}
#[derive(Debug, Serialize)]
pub struct Verified {
    pub schema: &'static str,
    pub arithmetic: &'static str,
    pub candidate_equations: usize,
    pub restart_equations: usize,
    pub source_and_transfer_header_match: bool,
    pub source_prime_torsion_checked: bool,
    pub transferred_prime_torsion_checked: bool,
    pub recovered: Option<u64>,
    pub source_scalar_verified: bool,
    pub equations: Vec<Equation>,
    pub independent_arithmetic: bool,
    pub source_curve_order_recounted: bool,
    pub scheduler_verified: bool,
}
/// Header must be supplied from separately pinned input; decoded data are only used
/// after strict replay. This checker supports the explicit Scholten family and a
/// conservative p <= i32::MAX arithmetic domain. Limits are not total RSS bounds.
pub fn verify(input: &[u8], header: &Value, limits: Limits) -> Result<Verified, String> {
    let replay = replay_jsonl(input, header, limits)?;
    let p = num(header, "p")?;
    let r = num(header, "r")?;
    require(
        p > 13 && p % 4 == 3 && p <= i32::MAX as u64,
        "Scholten field domain",
    )?;
    let s = Scholten::new(p);
    let curve = s.short_curve();
    let q = curve.f.size();
    require(
        header["source_field"] == serde_json::to_value(&curve.f).map_err(|e| e.to_string())?
            && header["source_coefficients"] == json!(curve.c)
            && header["source_binary"] == false,
        "source model mismatch",
    )?;
    require(
        header["polynomial"] == json!(s.j.polynomial),
        "destination model mismatch",
    )?;
    let g = point(&header["generator"], q)?;
    let target = point(&header["target"], q)?;
    require(
        curve.valid(g) && curve.valid(target),
        "source point off curve",
    )?;
    require(
        curve.mul(g, r).inf && curve.mul(target, r).inf,
        "source r-torsion",
    )?;
    let pg = s.tau(s.from_short(g));
    let tq = s.tau(s.from_short(target));
    require(
        pg != Divisor::O && tq != Divisor::O && s.j.valid(pg) && s.j.valid(tq),
        "invalid/zero transferred point",
    )?;
    require(
        pg == div(&header["transferred_generator"], p)?
            && tq == div(&header["transferred_target"], p)?,
        "transferred point mismatch",
    )?;
    require(
        s.j.mul(pg, r) == Divisor::O && s.j.mul(tq, r) == Divisor::O,
        "destination r-torsion",
    )?;
    let mut equations = vec![];
    let (mut candidates, mut restarts) = (0, 0);
    for line in std::str::from_utf8(input)
        .map_err(|e| e.to_string())?
        .lines()
    {
        let rec: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
        let e = &rec["event"];
        let kind = e["kind"].as_str().ok_or("event kind")?;
        if kind != "candidate" && kind != "restart" {
            continue;
        }
        let a = num(e, "a")?;
        let b = num(e, "b")?;
        let observed = div(&e["divisor"], p)?;
        require(s.j.valid(observed), "recorded divisor off Jacobian")?;
        let expected = s.j.add(s.j.mul(pg, a), s.j.mul(tq, b));
        if expected != observed {
            return Err(format!(
                "label equation mismatch at sequence {}",
                num(&rec, "sequence")?
            ));
        }
        if kind == "candidate" {
            candidates += 1;
        } else {
            restarts += 1;
        }
        equations.push(Equation {
            sequence: num(&rec, "sequence")?,
            kind: kind.into(),
            attempt: num(
                e,
                if kind == "candidate" {
                    "attempt"
                } else {
                    "after_attempt"
                },
            )?,
            a,
            b,
            divisor: observed,
            matches: true,
        });
    }
    require(
        candidates as u64 == replay.candidates,
        "candidate count disagreement",
    )?;
    let source_scalar_verified = if let Some(k) = replay.recovered {
        require(curve.mul(g, k) == target, "source scalar mismatch")?;
        true
    } else {
        false
    };
    Ok(Verified {
        schema: "signed.stream.label-check/v1",
        arithmetic: "native affine Cantor; shared with producer fallback",
        candidate_equations: candidates,
        restart_equations: restarts,
        source_and_transfer_header_match: true,
        source_prime_torsion_checked: true,
        transferred_prime_torsion_checked: true,
        recovered: replay.recovered,
        source_scalar_verified,
        equations,
        independent_arithmetic: false,
        source_curve_order_recounted: false,
        scheduler_verified: false,
    })
}
