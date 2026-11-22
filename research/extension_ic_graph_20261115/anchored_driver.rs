//! Recorded-state descent experiment: no collection, no supplied target answer.
use crate::{anchored, anchored_input as input, driver, nagao, remainder};
use crypto_lib::cryptanalysis::{
    jv_bitops::{self as work, Phase},
    jv_cover::{self as cover, EllE},
    jv_sieve::two_large_prime::CountedRelationSystem,
};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

pub const SEED: u64 = 401;
pub const MAX_RESIDUALS: u64 = 8192;
pub const MAX_ROWS: usize = 32;

fn add(a: u64, b: u64, l: u64) -> u64 {
    work::hook_add(l);
    ((a as u128 + b as u128) % l as u128) as u64
}
fn signed(x: i64, l: u64) -> u64 {
    if x >= 0 {
        x as u64 % l
    } else {
        (l - ((-x) as u64 % l)) % l
    }
}
fn dot(small: &[(usize, i64)], h: &[u64], l: u64) -> u64 {
    let mut eta = 0;
    for &(i, x) in small {
        let x = signed(x, l);
        if x != 0 && h[i] != 0 {
            work::hook_mul(l);
            eta = add(eta, (x as u128 * h[i] as u128 % l as u128) as u64, l);
        }
        work::words(5);
    }
    eta
}

pub fn run(bytes: &[u8], mode: &str, source_revision: &str) -> Result<Value, String> {
    if !["original-nagao", "full-only", "anchored"].contains(&mode)
        || source_revision.len() != 40
        || !source_revision.bytes().all(|c| c.is_ascii_hexdigit())
    {
        return Err("mode/source registration".into());
    }
    work::enable(false);
    let input = input::parse(bytes)?;
    // Full source/kernel replay is a separate input-admission step, reported
    // explicitly as excluded supplied-state validation rather than attack work.
    let input_validation = input::check_homogeneous(&input)?;
    let l = input.collection.public.l;
    work::claim();
    work::set_moduli(input.collection.public.p, l);
    work::enable(true);
    let before = work::snapshot();
    let before_other = work::snapshot_other();
    let start = Instant::now();
    let (spec, ctx, base) = {
        let _phase = work::enter(Phase::Transport);
        input::public_context(&input)?
    };
    let jac = ctx.jac();
    let by_x: HashMap<_, _> = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
    let mut anchors = BTreeMap::new();
    if mode == "anchored" {
        let _phase = work::enter(Phase::Descent);
        for (i, s) in input.collection.sources.iter().enumerate() {
            if let Some(r) = &s.certificate.remainder {
                work::probe(6);
                work::words(7);
                anchors.entry(r.key).or_insert(i);
            }
        }
    }
    let opts = cover::opts_for_mode(24, 600.0, cover::StopMode::Staircase(64));
    let _phase = work::enter(Phase::Descent);
    let mut stream = StdRng::seed_from_u64(SEED ^ 0x51E7E);
    let alpha = work::uniform_mod_u64(&mut stream, 1, l);
    let beta = work::uniform_mod_u64(&mut stream, 1, l);
    let step = jac.add(
        &jac.mul(&spec.gj, alpha as u128),
        &jac.mul(&spec.qj, beta as u128),
    );
    let mut a = work::uniform_mod_u64(&mut stream, 0, l);
    let mut b = work::uniform_mod_u64(&mut stream, 1, l);
    let mut residual = jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
    let initial_a = a;
    let initial_b = b;
    let mut affine = CountedRelationSystem::new(l, 2);
    let mut trials = Vec::new();
    let mut rows = Vec::new();
    let mut scalar = None;
    let mut refusal = None;
    let mut timeouts = 0u64;
    let mut incomplete = 0u64;
    let mut cohort = Sha256::new();
    let (mut partials, mut matched, mut unmatched, mut projected_refusals) =
        (0u64, 0u64, 0u64, 0u64);
    for i in 0..MAX_RESIDUALS {
        let mut rng = StdRng::seed_from_u64(SEED ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let mut partial_rows = Vec::new();
        let (full, cofactors, stats, solutions, degenerate) = if mode == "original-nagao" {
            let (full, cost) =
                cover::nagao_decompose(&ctx, &base, &by_x, &residual, &opts, &mut rng);
            timeouts += u64::from(cost.timed_out);
            incomplete += u64::from(cost.incomplete);
            let solutions = cost.fp_solutions;
            let degenerate = cost.degenerate;
            (
                full,
                Vec::new(),
                serde_json::to_value(cost).map_err(|e| e.to_string())?,
                solutions,
                degenerate,
            )
        } else {
            let out = nagao::decompose(
                &ctx,
                &base,
                &by_x,
                &residual,
                &opts,
                &mut rng,
                mode == "anchored",
            );
            timeouts += u64::from(out.stats.timed_out);
            incomplete += u64::from(out.stats.incomplete);
            partials += out.partial.len() as u64;
            let mut partial_events = Vec::new();
            for verified in out.partial {
                let certificate = verified.certificate().clone();
                work::probe(6);
                let anchor = anchors.get(&certificate.remainder.key).copied();
                let mut event = json!({"partial":certificate,"anchor_source":anchor,"projection":null,"refusal":null});
                if let Some(id) = anchor {
                    matched += 1;
                    match anchored::project(
                        &jac,
                        &base,
                        &verified,
                        &input.collection.sources[id].certificate,
                        &input.kernel,
                        l,
                    ) {
                        Ok(projected) => {
                            event["projection"] =
                                serde_json::to_value(&projected).map_err(|e| e.to_string())?;
                            partial_rows.push((projected.small, projected.eta, id, certificate));
                        }
                        Err(reason) => {
                            projected_refusals += 1;
                            event["refusal"] = json!(reason);
                        }
                    }
                } else {
                    unmatched += 1;
                }
                partial_events.push(event);
            }
            let mut stats = serde_json::to_value(out.stats).map_err(|e| e.to_string())?;
            stats["partial_events"] = json!(partial_events);
            let solutions = out.cofactors.len();
            (out.full, out.cofactors, stats, solutions, out.degenerate)
        };
        // This fingerprint is independent of acceptance, selected anchor and
        // target solution. Candidate and full-only must share every prefix.
        for c in &cofactors {
            cohort.update(
                serde_json::to_vec(&json!([c.lambda, c.sextic, c.roots]))
                    .map_err(|e| e.to_string())?,
            );
        }
        let mut selected = None;
        for d in &full {
            if !cover::verify_dec(&jac, &base, &residual, d) {
                return Err("full decomposition group replay".into());
            }
            if selected.is_none() {
                let small: Vec<_> = d.terms.iter().map(|&(i, s)| (i, s as i64)).collect();
                selected = Some((
                    small.clone(),
                    dot(&small, &input.kernel, l),
                    json!({"kind":"full","decomposition":d}),
                ));
            }
        }
        if selected.is_none() {
            if let Some((small, eta, id, partial)) = partial_rows.into_iter().next() {
                selected = Some((
                    small,
                    eta,
                    json!({"kind":"anchored","anchor_source":id,"partial":partial}),
                ));
            }
        }
        let selected_row = rows.len();
        if let Some((small, eta, witness)) = selected {
            // Check the integer small-point equation before reducing mod ell.
            let _verification = work::enter(Phase::Verification);
            let mut sum = jac.identity();
            for &(j, c) in &small {
                let d = jac.mul(&base[j].d, c.unsigned_abs() as u128);
                sum = jac.add(&sum, &if c < 0 { jac.neg(&d) } else { d });
            }
            if sum != residual {
                return Err("projected small row does not equal residual".into());
            }
            drop(_verification);
            let _la = work::enter(Phase::LinearAlgebra);
            let coefficients = [eta, (l - b) % l];
            work::hook_add(l);
            affine.insert(coefficients.to_vec(), a);
            let candidate = affine.solved(1);
            rows.push(json!({"residual_index":i,"a":a,"b":b,"small":small,"coefficients":coefficients,"rhs":a,"witness":witness,"rank":affine.rank(),"products":affine.ops(),"normalization_products":affine.normalization_products(),"subtractions":affine.subtraction_ops(),"candidate_scalar":candidate}));
            if let Some(d) = candidate {
                let _verification = work::enter(Phase::Verification);
                let ec = EllE::new(&ctx.f, &spec.alpha);
                if d > 0 && d < l && ec.mul(&spec.g, d) == spec.q {
                    scalar = Some(d);
                } else {
                    refusal = Some("elliptic candidate check failed");
                }
            }
        }
        trials.push(json!({"index":i,"a":a,"b":b,"residual":{"u":remainder::encode(&residual.u),"v":remainder::encode(&residual.v)},"full":full,"cofactors":cofactors,"stats":stats,"algebraic_solutions":solutions,"degenerate":degenerate,"cofactor_prefix_sha256":format!("{:x}",cohort.clone().finalize()),"selected_row":if rows.len()>selected_row{Some(selected_row)}else{None}}));
        if scalar.is_some() || refusal.is_some() {
            break;
        }
        if rows.len() >= MAX_ROWS {
            refusal = Some("accepted descent cap");
            break;
        }
        residual = jac.add(&residual, &step);
        a = add(a, alpha, l);
        b = add(b, beta, l);
    }
    drop(_phase);
    let wall_ns = start.elapsed().as_nanos();
    let tally = work::snapshot().since(&before);
    let other: Vec<_> = work::snapshot_other()
        .iter()
        .zip(before_other)
        .map(|(a, b)| a - b)
        .collect();
    let phases:Vec<_>=Phase::ALL.iter().map(|&phase|json!({"phase":phase.name(),"primitives":work::prims_json(tally.phase(phase))})).collect();
    let observed =
        work::price(&tally.total(), &work::widths(spec.p, l)).map_err(|_| "unpriced primitive")?;
    work::enable(false);
    Ok(
        json!({"schema":"cover.anchored-descent.component/v1","source_revision":source_revision,"input_sha256":driver::digest(bytes),"mode":mode,"seed":SEED,"p":spec.p,"l":l,"max_residuals":MAX_RESIDUALS,"max_rows":MAX_ROWS,"stream":{"alpha":alpha,"beta":beta,"initial_a":initial_a,"initial_b":initial_b},"input_validation":input_validation,"trials":trials,"rows":rows,"partial_rows":partials,"matched_partials":matched,"unmatched_partials":unmatched,"projection_refusals":projected_refusals,"timeouts":timeouts,"incomplete_systems":incomplete,"rank":affine.rank(),"actual_scalar":scalar,"verified":scalar.is_some(),"capped":scalar.is_none(),"resource_refusal":refusal,"phases":phases,"other_modulus_primitives":other,"foreign_thread_hooks":work::foreign(),"observed_incomplete_primitive_subtotal":observed.to_string(),"complete_cost":false,"complete_total":null,"ratios":null,"collection_cost_included":false,"input_validation_cost_included":false,"inner_wall_ns":wall_ns,"known_uncounted":["native allocation/container scans and comparisons","hashing and serialization","PRNG internals beyond uniform-word hooks","fixed-factor setup and incremental matrix scans"],"fresh_dlp_instance":false,"comparative_eligibility":false}),
    )
}
