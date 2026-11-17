//! Independent polynomial/Cantor and ordinary-u128 affine replay.
//! Does not call the producer's partial constructor, projector, or materializer.
use crate::{anchored, anchored_input as input, collection_check, driver, remainder};
use collection_check::accounting::{self as arithmetic, add, mul, pow};
use crypto_lib::cryptanalysis::{
    jv_cover::{self as cover, BaseEl, Div, Fld, Hyp, E2},
    jv_sieve::FpRing,
};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
#[allow(dead_code)]
#[path = "native_analysis/src/bin/two-lp-dlp-analysis.rs"]
mod full;
fn ensure(ok: bool, why: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn number(v: &Value, k: &str) -> Result<u64, String> {
    v[k].as_u64().ok_or(format!("missing {k}"))
}
fn signed(x: i64, l: u64) -> u64 {
    if x >= 0 {
        x as u64 % l
    } else {
        (l - ((-x) as u64 % l)) % l
    }
}
fn add_poly(f: &cover::Fq, a: &[E2], b: &[E2]) -> Vec<E2> {
    remainder::sub(f, a, &b.iter().map(|x| f.neg(x)).collect::<Vec<_>>())
}
fn monic(f: &cover::Fq, a: Vec<E2>) -> Vec<E2> {
    let a = remainder::trim(a);
    if let Some(&last) = a.last() {
        let inv = f.inv(&last);
        a.iter().map(|x| f.mul(x, &inv)).collect()
    } else {
        a
    }
}
fn gcd(f: &cover::Fq, a: &[E2], b: &[E2]) -> Result<Vec<E2>, String> {
    let (mut a, mut b) = (a.to_vec(), monic(f, b.to_vec()));
    while !b.is_empty() {
        if b.len() == 1 {
            return Ok(vec![E2::ONE]);
        }
        let r = remainder::modulo(f, &a, &b)?;
        a = b;
        b = monic(f, r);
    }
    Ok(monic(f, a))
}
fn derivative(f: &cover::Fq, a: &[E2]) -> Vec<E2> {
    remainder::trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, x)| f.scale(x, i as u64))
            .collect(),
    )
}

pub fn partial(
    jac: &Hyp<cover::Fq>,
    base: &[BaseEl],
    r: &Div<E2>,
    p: &anchored::Partial,
) -> Result<(), String> {
    let f = jac.f;
    let a = remainder::decode(&p.a);
    let c = remainder::decode(&p.c);
    ensure(
        a.len() <= 5
            && c.len() == 2
            && c[1] == E2::ONE
            && p.sextic.len() == 7
            && p.sextic[6] == 1
            && p.sextic.iter().all(|&x| x < f.p),
        "partial shapes",
    )?;
    ensure(
        r.u.len() == 4 && r.u[3] == E2::ONE && r.v.len() <= 3 && jac.is_valid(r),
        "partial residual",
    )?;
    ensure(
        p.terms.len() == 4
            && p.terms.windows(2).all(|w| w[0].0 < w[1].0)
            && [-1, 1].contains(&p.remainder.coefficient)
            && p.remainder.key.iter().all(|&x| x < f.p),
        "partial canonical support",
    )?;
    ensure(
        a.iter().chain(&c).all(|x| x.0.iter().all(|&v| v < f.p)),
        "partial coordinates",
    )?;
    let s = remainder::embed(&p.sextic);
    let norm = remainder::sub(
        f,
        &remainder::mul(f, &a, &a),
        &remainder::mul(f, &remainder::mul(f, &c, &c), &jac.h),
    );
    ensure(
        add_poly(f, &norm, &remainder::mul(f, &r.u, &s)).is_empty(),
        "independent exact principal norm",
    )?;
    ensure(
        remainder::modulo(f, &add_poly(f, &a, &remainder::mul(f, &c, &r.v)), &r.u)?.is_empty(),
        "residual ordinate",
    )?;
    for b in [&derivative(f, &r.u), &jac.h, &c, &s] {
        ensure(
            gcd(f, &r.u, b)?.len() == 1,
            "residual disjoint/coprime/squarefree",
        )?;
    }
    let mut product = vec![E2::ONE];
    let mut sum = r.clone();
    for &(i, sign) in &p.terms {
        let b = base.get(i).ok_or("partial point index")?;
        ensure(
            [-1, 1].contains(&sign) && b.y != E2::ZERO && b.x < f.p,
            "partial point/sign",
        )?;
        let y = if sign == 1 { b.y } else { f.neg(&b.y) };
        ensure(
            b.d == (Div {
                u: vec![f.neg(&f.from_fp(b.x)), E2::ONE],
                v: vec![b.y],
            }) && jac.is_valid(&b.d),
            "point descriptor",
        )?;
        let den = remainder::eval(f, &c, b.x);
        ensure(
            den != E2::ZERO && f.add(&remainder::eval(f, &a, b.x), &f.mul(&den, &y)) == E2::ZERO,
            "partial point ordinate",
        )?;
        product = remainder::mul(f, &product, &b.d.u);
        sum = jac.add(
            &sum,
            &if sign == 1 {
                b.d.clone()
            } else {
                jac.neg(&b.d)
            },
        );
    }
    let d = p.remainder.divisor();
    ensure(jac.is_valid(&d), "quadratic Mumford validity")?;
    let disc = (f.p + mul(d.u[1].0[0], d.u[1].0[0], f.p) - mul(4, d.u[0].0[0], f.p)) % f.p;
    ensure(
        disc != 0 && pow(disc, (f.p - 1) / 2, f.p) == f.p - 1,
        "quadratic irreducibility",
    )?;
    ensure(
        remainder::mul(f, &product, &d.u) == s,
        "exact four-plus-quadratic factorization",
    )?;
    ensure(gcd(f, &c, &d.u)?.len() == 1, "quadratic denominator")?;
    let v = if p.remainder.coefficient == 1 {
        d.v.clone()
    } else {
        d.v.iter().map(|x| f.neg(x)).collect()
    };
    ensure(
        remainder::modulo(f, &add_poly(f, &a, &remainder::mul(f, &c, &v)), &d.u)?.is_empty(),
        "quadratic principal sign",
    )?;
    let v0 = d.v.first().copied().unwrap_or(E2::ZERO);
    let v1 = d.v.get(1).copied().unwrap_or(E2::ZERO);
    let opposite = [
        d.u[0].0[0],
        d.u[1].0[0],
        f.neg(&v0).0[0],
        f.neg(&v0).0[1],
        f.neg(&v1).0[0],
        f.neg(&v1).0[1],
    ];
    ensure(p.remainder.key <= opposite, "canonical signed class")?;
    sum = jac.add(
        &sum,
        &if p.remainder.coefficient == 1 {
            d.clone()
        } else {
            jac.neg(&d)
        },
    );
    ensure(jac.is_identity(&sum), "independent partial Cantor identity")
}

#[derive(Default)]
pub struct Affine {
    rows: BTreeMap<usize, [u64; 3]>,
    order: Vec<usize>,
    pub products: u64,
}
impl Affine {
    pub fn insert(&mut self, mut row: [u64; 3], l: u64) -> Result<(), String> {
        ensure(row.iter().all(|&x| x < l), "affine canonicality")?;
        for &i in &self.order {
            let k = row[i];
            if k != 0 {
                for j in 0..2 {
                    if self.rows[&i][j] != 0 {
                        row[j] = add(row[j], l - mul(k, self.rows[&i][j], l), l);
                        self.products += 1;
                    }
                }
                row[2] = add(row[2], l - mul(k, self.rows[&i][2], l), l);
                self.products += 1;
            }
        }
        let Some(i) = (0..2).find(|&j| row[j] != 0) else {
            return ensure(row[2] == 0, "inconsistent affine equation");
        };
        let inv = pow(row[i], l - 2, l);
        for x in &mut row {
            *x = mul(*x, inv, l);
        }
        self.products += 3;
        for old in self.rows.values_mut() {
            let k = old[i];
            if k != 0 {
                for j in 0..2 {
                    if row[j] != 0 {
                        old[j] = add(old[j], l - mul(k, row[j], l), l);
                        self.products += 1;
                    }
                }
                old[2] = add(old[2], l - mul(k, row[2], l), l);
                self.products += 1;
            }
        }
        self.rows.insert(i, row);
        self.order.push(i);
        Ok(())
    }
    pub fn rank(&self) -> usize {
        self.rows.len()
    }
    pub fn target(&self) -> Option<u64> {
        self.rows
            .get(&1)
            .filter(|r| r[0] == 0 && r[1] == 1)
            .map(|r| r[2])
    }
}

fn replay_small(
    jac: &Hyp<cover::Fq>,
    base: &[BaseEl],
    small: &[(usize, i64)],
    r: &Div<E2>,
) -> Result<(), String> {
    ensure(
        small.windows(2).all(|w| w[0].0 < w[1].0)
            && small
                .iter()
                .all(|&(i, c)| i < base.len() && c != 0 && c.unsigned_abs() <= 2),
        "integer small witness",
    )?;
    let mut sum = jac.identity();
    for &(i, c) in small {
        let d = jac.mul(&base[i].d, c.unsigned_abs() as u128);
        sum = jac.add(&sum, &if c < 0 { jac.neg(&d) } else { d });
    }
    ensure(sum == *r, "integer small group equation")
}

pub fn qualify(
    input: &input::Input,
    input_bytes: &[u8],
    raw: &Value,
    source: &str,
) -> Result<Value, String> {
    let mode = raw["mode"].as_str().ok_or("mode")?;
    ensure(
        ["original-nagao", "full-only", "anchored"].contains(&mode)
            && raw["schema"] == "cover.anchored-descent.component/v1"
            && raw["source_revision"] == source
            && raw["input_sha256"] == driver::digest(input_bytes)
            && raw["seed"] == 401
            && raw["max_residuals"] == 8192
            && raw["max_rows"] == 32,
        "worker registration",
    )?;
    ensure(
        raw["complete_cost"] == false
            && raw["complete_total"].is_null()
            && raw["ratios"].is_null()
            && raw["comparative_eligibility"] == false
            && raw["fresh_dlp_instance"] == false
            && raw["collection_cost_included"] == false
            && raw["input_validation_cost_included"] == false
            && raw["foreign_thread_hooks"] == 0,
        "component cost/scope",
    )?;
    let (spec, ctx, base) = input::public_context(input)?;
    let l = spec.l;
    let jac = ctx.jac();
    ensure(
        raw["p"] == spec.p
            && raw["l"] == l
            && raw["other_modulus_primitives"]
                .as_array()
                .is_some_and(|a| a.len() == 15 && a.iter().all(|x| x == 0)),
        "domain/other modulus",
    )?;
    let mut stream = StdRng::seed_from_u64(401 ^ 0x51E7E);
    // Reproduce the public sampler without reading producer output as labels.
    let sample = |rng: &mut StdRng, lo, hi| {
        crypto_lib::cryptanalysis::jv_bitops::uniform_mod_u64(rng, lo, hi)
    };
    let alpha = sample(&mut stream, 1, l);
    let beta = sample(&mut stream, 1, l);
    let mut a = sample(&mut stream, 0, l);
    let mut b = sample(&mut stream, 1, l);
    ensure(
        raw["stream"] == json!({"alpha":alpha,"beta":beta,"initial_a":a,"initial_b":b}),
        "residual stream initialization",
    )?;
    let step = jac.add(
        &jac.mul(&spec.gj, alpha as u128),
        &jac.mul(&spec.qj, beta as u128),
    );
    let mut r = jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
    let trials = raw["trials"].as_array().ok_or("trials")?;
    let rows = raw["rows"].as_array().ok_or("rows")?;
    ensure(
        !trials.is_empty() && trials.len() <= 8192 && rows.len() <= 32,
        "recorded resource bounds",
    )?;
    let mut anchors = BTreeMap::new();
    for (i, s) in input.collection.sources.iter().enumerate() {
        if let Some(d) = &s.certificate.remainder {
            anchors.entry(d.key).or_insert(i);
        }
    }
    let (mut affine, mut selected, mut cohort) = (Affine::default(), 0usize, Sha256::new());
    let (mut partials, mut matched, mut unmatched, mut refused, mut timeouts, mut incomplete) =
        (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    for (i, t) in trials.iter().enumerate() {
        ensure(
            t["index"] == i
                && t["a"] == a
                && t["b"] == b
                && t["residual"]
                    == json!({"u":remainder::encode(&r.u),"v":remainder::encode(&r.v)}),
            "every residual label and native walk",
        )?;
        let stats = &t["stats"];
        timeouts += u64::from(stats["timed_out"] == true);
        incomplete += u64::from(stats["incomplete"] == true);
        let full: Vec<[(usize, i8); 6]> = t["full"]
            .as_array()
            .ok_or("full rows")?
            .iter()
            .map(|v| serde_json::from_value(v["terms"].clone()).map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()?;
        ensure(
            full.windows(2).all(|w| w[0] < w[1]),
            "unique ordered full rows",
        )?;
        for d in &full {
            ensure(
                d.windows(2).all(|w| w[0].0 < w[1].0)
                    && d.iter()
                        .all(|&(j, s)| j < base.len() && [-1, 1].contains(&s)),
                "full row shape",
            )?;
            replay_small(
                &jac,
                &base,
                &d.iter().map(|&(j, s)| (j, s as i64)).collect::<Vec<_>>(),
                &r,
            )?;
        }
        let mut checked_partials = Vec::new();
        let cofactors = t["cofactors"].as_array().ok_or("cofactors")?;
        if mode != "original-nagao" {
            ensure(
                t["algebraic_solutions"] == cofactors.len(),
                "all saved algebraic solutions",
            )?;
            let forms = if r.u.len() == 4 {
                let mut v = r.v.clone();
                v.resize(3, E2::ZERO);
                Some(cover::weil_forms(&ctx.f.f, &ctx.cov.hx, &r.u, &v))
            } else {
                None
            };
            for (cidx, c) in cofactors.iter().enumerate() {
                let lam: [[u64; 2]; 3] =
                    serde_json::from_value(c["lambda"].clone()).map_err(|e| e.to_string())?;
                ensure(
                    lam.iter().flatten().all(|&v| v < spec.p),
                    "canonical algebraic solution",
                )?;
                let lam = lam.map(E2);
                let forms = forms.as_ref().ok_or("cofactor of degenerate residual")?;
                let mut expected = Vec::new();
                for k in 0..6 {
                    let v = forms.eval(&ctx.f.f, k, &lam);
                    if v.0[1] != 0 {
                        break;
                    }
                    expected.push(v.0[0]);
                }
                if expected.len() == 6 {
                    expected.push(1);
                }
                ensure(c["sextic"] == json!(expected), "cofactor Weil-form binding")?;
                let roots: Vec<u64> =
                    serde_json::from_value(c["roots"].clone()).map_err(|e| e.to_string())?;
                if expected.len() == 7 {
                    let ring = FpRing::new(spec.p);
                    let actual = ring.roots(
                        &expected,
                        &mut StdRng::seed_from_u64(
                            0xA11CE ^ (i as u64).wrapping_mul(104729) ^ cidx as u64,
                        ),
                    );
                    ensure(roots == actual, "complete distinct Fp-root set")?;
                }
                cohort.update(
                    serde_json::to_vec(&json!([lam.map(|x| x.0), expected, roots]))
                        .map_err(|e| e.to_string())?,
                );
                if !c["partial"].is_null() {
                    ensure(
                        mode == "anchored" && roots.len() == 4,
                        "partial mode/root count",
                    )?;
                    let p: anchored::Partial =
                        serde_json::from_value(c["partial"].clone()).map_err(|e| e.to_string())?;
                    partial(&jac, &base, &r, &p)?;
                    ensure(p.sextic == expected, "partial cofactor binding")?;
                    let uc = |k: isize| {
                        if k < 0 {
                            E2::ZERO
                        } else {
                            r.u.get(k as usize).copied().unwrap_or(E2::ZERO)
                        }
                    };
                    let vc = |k: isize| {
                        if k < 0 {
                            E2::ZERO
                        } else {
                            r.v.get(k as usize).copied().unwrap_or(E2::ZERO)
                        }
                    };
                    let f = &ctx.f.f;
                    let actual_a: Vec<_> = (0..5isize)
                        .map(|k| {
                            f.sub(
                                &f.add(
                                    &f.add(&f.neg(&vc(k - 1)), &f.mul(&lam[0], &uc(k))),
                                    &f.mul(&lam[1], &uc(k - 1)),
                                ),
                                &f.mul(&lam[2], &vc(k)),
                            )
                        })
                        .collect();
                    ensure(
                        p.a == remainder::encode(&actual_a) && p.c == vec![lam[2].0, E2::ONE.0],
                        "partial lambda/function binding",
                    )?;
                    checked_partials.push(p);
                }
            }
            let events = stats["partial_events"].as_array().ok_or("partial events")?;
            ensure(
                events.len() == checked_partials.len(),
                "all partial events retained",
            )?;
            for (p, e) in checked_partials.iter().zip(events) {
                partials += 1;
                ensure(
                    e["partial"] == serde_json::to_value(p).map_err(|e| e.to_string())?,
                    "partial event source",
                )?;
                let anchor = anchors.get(&p.remainder.key).copied();
                ensure(
                    e["anchor_source"] == json!(anchor),
                    "exact signed class lookup",
                )?;
                if let Some(id) = anchor {
                    matched += 1;
                    if e["projection"].is_null() {
                        refused += 1;
                        ensure(!e["refusal"].is_null(), "projection refusal reason")?;
                    } else {
                        let expected = small_witness(p, &input.collection.sources[id].certificate)?;
                        ensure(
                            e["projection"]["small"] == json!(expected),
                            "integer anchor cancellation",
                        )?;
                        let eta = expected.iter().fold(0, |acc, &(j, c)| {
                            add(acc, mul(signed(c, l), input.kernel[j], l), l)
                        });
                        ensure(e["projection"]["eta"] == eta, "anchor log projection")?;
                        let coefficient = p.remainder.coefficient
                            * input.collection.sources[id]
                                .certificate
                                .remainder
                                .as_ref()
                                .ok_or("anchor remainder")?
                                .coefficient;
                        ensure(
                            e["projection"]["anchor_coefficient"] == coefficient
                                && e["refusal"].is_null(),
                            "signed anchor coefficient",
                        )?;
                        replay_small(&jac, &base, &expected, &r)?;
                    }
                } else {
                    unmatched += 1;
                    ensure(e["projection"].is_null(), "unmatched class cannot project")?;
                }
            }
        } else {
            ensure(
                cofactors.is_empty() && t["algebraic_solutions"] == stats["fp_solutions"],
                "original materializer record",
            )?;
        }
        ensure(
            t["cofactor_prefix_sha256"] == format!("{:x}", cohort.clone().finalize()),
            "acceptance-independent cofactor prefix",
        )?;
        if !t["selected_row"].is_null() {
            ensure(t["selected_row"] == selected, "selected row order")?;
            let row = rows.get(selected).ok_or("missing selected row")?;
            ensure(
                row["residual_index"] == i && row["a"] == a && row["b"] == b && row["rhs"] == a,
                "affine row labels",
            )?;
            let small: Vec<(usize, i64)> =
                serde_json::from_value(row["small"].clone()).map_err(|e| e.to_string())?;
            replay_small(&jac, &base, &small, &r)?;
            if row["witness"]["kind"] == "full" {
                ensure(
                    !full.is_empty()
                        && row["witness"]["decomposition"]["terms"] == json!(full[0])
                        && small
                            == full[0]
                                .iter()
                                .map(|&(j, s)| (j, s as i64))
                                .collect::<Vec<_>>(),
                    "selected full witness",
                )?;
            } else {
                ensure(
                    mode == "anchored" && full.is_empty() && row["witness"]["kind"] == "anchored",
                    "partial selection scope",
                )?;
                let p: anchored::Partial =
                    serde_json::from_value(row["witness"]["partial"].clone())
                        .map_err(|e| e.to_string())?;
                partial(&jac, &base, &r, &p)?;
                let id = number(&row["witness"], "anchor_source")? as usize;
                ensure(
                    anchors.get(&p.remainder.key) == Some(&id)
                        && checked_partials
                            .iter()
                            .any(|x| serde_json::to_value(x).ok() == serde_json::to_value(&p).ok()),
                    "selected original partial/anchor",
                )?;
                ensure(
                    small == small_witness(&p, &input.collection.sources[id].certificate)?,
                    "selected integer witness",
                )?;
            }
            let eta = small.iter().fold(0, |acc, &(j, c)| {
                add(acc, mul(signed(c, l), input.kernel[j], l), l)
            });
            ensure(
                row["coefficients"] == json!([eta, (l - b) % l]),
                "protected affine target coefficient",
            )?;
            affine.insert([eta, (l - b) % l, a], l)?;
            ensure(
                row["rank"] == affine.rank()
                    && row["products"] == affine.products
                    && row["normalization_products"] == 3 * affine.rank() as u64
                    && row["subtractions"] == affine.products - 3 * affine.rank() as u64
                    && row["candidate_scalar"] == json!(affine.target()),
                "every affine rank/product/target prefix",
            )?;
            selected += 1;
        } else {
            ensure(
                full.is_empty(),
                "full decomposition cannot be silently omitted",
            )?;
        }
        if i + 1 < trials.len() {
            ensure(
                affine.target().is_none() && selected < 32,
                "no work after solved/capped endpoint",
            )?;
            r = jac.add(&r, &step);
            a = add(a, alpha, l);
            b = add(b, beta, l);
        }
    }
    ensure(
        selected == rows.len()
            && raw["rank"] == affine.rank()
            && raw["partial_rows"] == partials
            && raw["matched_partials"] == matched
            && raw["unmatched_partials"] == unmatched
            && raw["projection_refusals"] == refused
            && raw["timeouts"] == timeouts
            && raw["incomplete_systems"] == incomplete,
        "all outcome counters",
    )?;
    let target = affine.target();
    if let Some(d) = target {
        ensure(
            d > 0 && d < l && cover::EllE::new(&ctx.f, &spec.alpha).mul(&spec.g, d) == spec.q,
            "public elliptic target replay",
        )?;
    }
    ensure(
        raw["actual_scalar"] == json!(target)
            && raw["verified"] == target.is_some()
            && raw["capped"] == target.is_none(),
        "actual endpoint",
    )?;
    let mut full_rows = input
        .collection
        .projections
        .iter()
        .map(|r| {
            (
                r.terms
                    .iter()
                    .map(|&(j, x)| (j as u64, x))
                    .collect::<Vec<_>>(),
                0u64,
            )
        })
        .collect::<Vec<_>>();
    for row in rows {
        let small: Vec<(usize, i64)> =
            serde_json::from_value(row["small"].clone()).map_err(|e| e.to_string())?;
        let mut terms = small
            .into_iter()
            .map(|(j, x)| (j as u64, signed(x, l)))
            .collect::<Vec<_>>();
        terms.push((base.len() as u64, (l - number(row, "b")?) % l));
        full_rows.push((terms, number(row, "a")?));
    }
    let (full_rank, full_target) = full::target_solution(&full_rows, base.len() + 1, l)?;
    ensure(
        full_rank + 1 == base.len() + affine.rank() && full_target == target,
        "unreduced full-matrix rank and target",
    )?;
    let phases = raw["phases"].as_array().ok_or("phases")?;
    let names = [
        "instance",
        "harness",
        "walk",
        "transport",
        "model",
        "factor_base",
        "relations",
        "descent",
        "linear_algebra",
        "verification",
        "precompute",
        "walk",
        "collision",
        "verification",
        "unattributed",
    ];
    ensure(phases.len() == names.len(), "phase count")?;
    let widths = arithmetic::Widths {
        field: arithmetic::Field::Prime {
            bits: 64 - spec.p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    };
    let mut subtotal = 0u128;
    for (p, name) in phases.iter().zip(names) {
        ensure(p["phase"] == name, "phase order")?;
        let c = arithmetic::counts(&p["primitives"])?;
        subtotal = subtotal
            .checked_add(arithmetic::price(&c, &widths)?)
            .ok_or("subtotal overflow")?;
    }
    ensure(
        raw["observed_incomplete_primitive_subtotal"] == subtotal.to_string(),
        "independent observed primitive price",
    )?;
    let la = phases
        .iter()
        .find(|p| p["phase"] == "linear_algebra")
        .ok_or("LA phase")?;
    let c = arithmetic::counts(&la["primitives"])?;
    ensure(
        c["modr_mul"] == affine.products
            && c["modr_add"] == affine.products - 3 * affine.rank() as u64 + rows.len() as u64
            && c["modr_inv"] == affine.rank() as u64,
        "affine primitive conservation",
    )?;
    Ok(
        json!({"mode":mode,"residuals":trials.len(),"accepted_rows":rows.len(),"partial_rows":partials,"matched_partials":matched,"unmatched_partials":unmatched,"projection_refusals":refused,"rank":affine.rank(),"actual_scalar":target,"observed_incomplete_primitive_subtotal":subtotal.to_string(),"all_saved_sources_replayed":true,"all_norms_and_class_signs_replayed":true,"all_residual_labels_and_affine_prefixes_replayed":true,"elliptic_target_replayed":target.is_some(),"complete_cost":false,"complete_total":null,"ratios":null}),
    )
}

fn small_witness(
    p: &anchored::Partial,
    anchor: &remainder::Certificate,
) -> Result<Vec<(usize, i64)>, String> {
    let d = anchor.remainder.as_ref().ok_or("nonpartial anchor")?;
    ensure(
        d.key == p.remainder.key && [-1, 1].contains(&d.coefficient),
        "anchor class/sign",
    )?;
    let mut row = BTreeMap::new();
    for &(i, s) in &p.terms {
        *row.entry(i).or_insert(0i64) -= s as i64;
    }
    for &(i, s) in &anchor.terms {
        *row.entry(i).or_insert(0i64) += (p.remainder.coefficient * d.coefficient * s) as i64;
    }
    Ok(row.into_iter().filter(|&(_, x)| x != 0).collect())
}

pub fn paired(a: &Value, b: &Value, original: &Value) -> Result<Value, String> {
    let x = a["trials"].as_array().ok_or("baseline trials")?;
    let y = b["trials"].as_array().ok_or("candidate trials")?;
    let z = original["trials"].as_array().ok_or("original trials")?;
    let n = x.len().min(y.len());
    for i in 0..n {
        ensure(
            x[i]["a"] == y[i]["a"]
                && x[i]["b"] == y[i]["b"]
                && x[i]["residual"] == y[i]["residual"]
                && x[i]["full"] == y[i]["full"]
                && x[i]["cofactor_prefix_sha256"] == y[i]["cofactor_prefix_sha256"],
            "matched full/cofactor stream prefix",
        )?;
    }
    let m = z.len().min(x.len());
    for i in 0..m {
        ensure(
            z[i]["residual"] == x[i]["residual"]
                && z[i]["full"] == x[i]["full"]
                && z[i]["algebraic_solutions"] == x[i]["algebraic_solutions"],
            "original Nagao full-output/system prefix",
        )?;
    }
    Ok(
        json!({"matched_full_only_anchored_residual_prefixes":n,"matched_original_full_only_residual_prefixes":m,"all_common_full_decomposition_sets_equal":true,"candidate_cofactor_stream_acceptance_independent":true,"supplied_state_component_only":true}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_partial_accepts_exact_fixture_and_refuses_changed_certificates() {
        let (f, h, base, r, a, c, s, terms, tail) = anchored::tests::fixture();
        let p = anchored::verify(&f, &h, &base, &r, &a, &c, &s, &terms, &tail)
            .unwrap()
            .into_certificate();
        let jac = Hyp { f: &f, h, g: 3 };
        partial(&jac, &base, &r, &p).unwrap();
        for k in 0..6 {
            let mut bad = p.clone();
            match k {
                0 => bad.a[0][0] = (bad.a[0][0] + 1) % f.p,
                1 => bad.sextic[0] = (bad.sextic[0] + 1) % f.p,
                2 => bad.remainder.coefficient = -bad.remainder.coefficient,
                3 => bad.remainder.key[2] = (bad.remainder.key[2] + 1) % f.p,
                4 => bad.terms[0].1 = -bad.terms[0].1,
                _ => bad.terms[1].0 = bad.terms[0].0,
            }
            assert!(partial(&jac, &base, &r, &bad).is_err(), "mutation {k}");
        }
        let mut bad_base = base.clone();
        bad_base[0].d = base[1].d.clone();
        assert!(partial(&jac, &bad_base, &r, &p).is_err());
    }
    #[test]
    fn independent_affine_replay_matches_actual_prefixes_and_refuses_inconsistency() {
        use crypto_lib::cryptanalysis::jv_sieve::two_large_prime::CountedRelationSystem;
        for l in [101, 263807419614139553] {
            let (mut a, mut b) = (Affine::default(), CountedRelationSystem::new(l, 2));
            for row in [[1, 2, 7], [0, 1, 3], [1, 0, 1], [0, 0, 0]] {
                a.insert(row, l).unwrap();
                b.insert(row[..2].to_vec(), row[2]);
                assert_eq!(a.rank(), b.rank());
                assert_eq!(a.products, b.ops());
                assert_eq!(a.target(), b.solved(1));
            }
            assert!(a.insert([0, 0, 1], l).is_err());
        }
    }
    #[test]
    fn signed_overlap_witness_keeps_integer_multiplicities() {
        let d = remainder::Remainder {
            key: [1, 2, 3, 4, 5, 6],
            coefficient: -1,
        };
        let p = anchored::Partial {
            terms: vec![(0, 1), (1, -1), (2, 1), (3, -1)],
            remainder: d.clone(),
            a: vec![],
            c: vec![],
            sextic: vec![],
        };
        let a = remainder::Certificate {
            terms: vec![(0, 1), (1, 1), (4, -1)],
            remainder: Some(d),
            a: vec![],
            c: vec![],
            norm: vec![],
        };
        assert_eq!(
            small_witness(&p, &a).unwrap(),
            vec![(1, 2), (2, -1), (3, 1), (4, -1)]
        );
    }
}
