//! Fixed-cohort component probe. No individual logarithm or rho comparison.
use crypto_lib::cryptanalysis::jv_bitops::{self as work, Phase};
use crypto_lib::cryptanalysis::jv_cover::{factor_base, generate_spec, Ctx};
use crypto_lib::cryptanalysis::jv_sieve::two_large_prime::TwoLargePrimeSieve;
use crypto_lib::cryptanalysis::jv_sieve::{
    b_from_index, b_space, coprime_scramble, lines_for_b_hint, nu_lines, roots_by_incidence,
    shared_for_b, sieve_line, Counters, FpRing, Line,
};
use crypto_lib::cryptanalysis::large_prime_filter::LargePrimeFilterOptions;
use rand::{rngs::StdRng, SeedableRng};
use serde_json::json;
use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
    fs::OpenOptions,
    time::Instant,
};

fn bind_line(h: &mut blake3::Hasher, k: u128, line: &Line) {
    h.update(&k.to_le_bytes());
    for p in [&line.a0, &line.a1, &line.p1, &line.p2, &line.p3, &line.p4] {
        h.update(&(p.len() as u64).to_le_bytes());
        for c in p {
            h.update(&c.to_le_bytes());
        }
    }
    h.update(&(line.b.len() as u64).to_le_bytes());
    for c in &line.b {
        for v in c.0 {
            h.update(&v.to_le_bytes());
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !args.len().is_multiple_of(2) {
        return Err("expected --key value pairs".into());
    }
    let mut flags = BTreeMap::new();
    for pair in args.chunks_exact(2) {
        if ![
            "--mode",
            "--p",
            "--seed",
            "--small-percent",
            "--b-count",
            "--out",
            "--source-revision",
        ]
        .contains(&pair[0].as_str())
            || flags.insert(pair[0].clone(), pair[1].clone()).is_some()
        {
            return Err("unknown/duplicate flag".into());
        }
    }
    let get = |k: &str| flags.get(k).ok_or_else(|| format!("missing {k}"));
    let mode = get("--mode")?;
    let p: u64 = get("--p")?.parse()?;
    let seed: u64 = get("--seed")?.parse()?;
    let percent: usize = get("--small-percent")?.parse()?;
    let count: u128 = get("--b-count")?.parse()?;
    let source = get("--source-revision")?;
    if !["smooth", "two-lp"].contains(&mode.as_str())
        || !(101..=1511).contains(&p)
        || !crypto_lib::cryptanalysis::residual_walk::is_prime_u64(p)
        || !(1..100).contains(&percent)
        || count == 0
        || count > 2_000_000
        || source.len() != 40
        || !source.bytes().all(|c| c.is_ascii_hexdigit())
    {
        return Err("invalid fixed-cohort configuration".into());
    }
    let output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(get("--out")?)?;
    // Public synthetic instance generation is outside the component boundary.
    // The relation collector never reads spec.d or solves an individual log.
    let spec = generate_spec(p, seed);
    work::enable(true);
    work::claim();
    work::set_moduli(p, spec.l);
    let before = work::snapshot();
    let uniforms_before = work::uniform_snapshot();
    let started = Instant::now();
    let ctx = {
        let _g = work::enter(Phase::Model);
        Ctx::new(&spec)
    };
    let mut enum_rng = StdRng::seed_from_u64(seed ^ 0xA17E_2E17);
    let mut extract_rng = StdRng::seed_from_u64(seed ^ 0xE17A_C710);
    let sieve = {
        let _g = work::enter(Phase::FactorBase);
        let base = factor_base(&ctx, &mut enum_rng);
        let small = (base.len() * percent / 100).max(1);
        TwoLargePrimeSieve::new(&ctx, &base, small, 9)?
    };
    if count > b_space(sieve.setup()) {
        return Err("B cohort exceeds unique space".into());
    }
    let ring = FpRing::new(p);
    let mut ctr = Counters::new(p);
    let mut seen = HashSet::new();
    let mut relations = Vec::new();
    let mut fingerprint = blake3::Hasher::new();
    let (mut line_count, mut candidates, mut refused, mut duplicates) = (0u64, 0u64, 0u64, 0u64);
    let mut kinds = [0u64; 3];
    {
        let _g = work::enter(Phase::Relations);
        let scramble = coprime_scramble(b_space(sieve.setup()), &mut enum_rng);
        let nu = nu_lines(sieve.setup());
        let mut slot = Vec::new();
        let mut start = 0;
        while start < count {
            let end = (start + 1024).min(count);
            let bs: Vec<_> = (start..end)
                .map(|k| b_from_index(sieve.setup(), k, scramble))
                .collect();
            let ab: Vec<_> = bs.iter().map(|b| (b[0].0[0], b[0].0[1])).collect();
            let hints = roots_by_incidence(p, &nu, &ab, &mut slot).0;
            for (offset, b) in bs.iter().enumerate() {
                let lines = lines_for_b_hint(
                    &ring,
                    &ctx.f.f,
                    sieve.setup(),
                    b,
                    &mut enum_rng,
                    true,
                    hints[offset].as_deref(),
                );
                let shared = if lines.len() >= 2 {
                    Some(shared_for_b(&ring, &lines[0], &mut 0))
                } else {
                    None
                };
                for line in lines {
                    line_count += 1;
                    bind_line(&mut fingerprint, start + offset as u128, &line);
                    let hits = if mode == "smooth" {
                        sieve_line(&ring, sieve.setup(), &line, &mut ctr, None, shared.as_ref()).0
                    } else {
                        sieve.sieve_line(&ring, &line, &mut ctr, shared.as_ref()).0
                    };
                    for s in hits {
                        candidates += 1;
                        let id = relations.len() as u64;
                        let relation = if mode == "smooth" {
                            sieve.verified_full_smooth_relation(
                                &ctx,
                                &ring,
                                &line,
                                s,
                                id,
                                spec.l,
                                &mut extract_rng,
                            )
                        } else {
                            sieve.verified_relation(&ctx, &ring, &line, s, id, spec.l)
                        };
                        let Some(r) = relation else {
                            refused += 1;
                            continue;
                        };
                        let negative: Vec<_> =
                            r.full().terms.iter().map(|&(i, e)| (i, -e)).collect();
                        let key_words = r.full().terms.len() as u64;
                        work::probe(key_words);
                        let duplicate = seen.contains(&r.full().terms) || {
                            work::probe(key_words);
                            seen.contains(&negative)
                        };
                        if duplicate {
                            duplicates += 1;
                            continue;
                        }
                        work::probe(key_words);
                        seen.insert(r.full().terms.clone());
                        kinds[r.partial().large.len()] += 1;
                        relations.push(r);
                        if relations.len() > 4096 {
                            return Err("relation cap exceeded".into());
                        }
                    }
                }
            }
            start = end;
        }
    }
    let filtered = {
        let _g = work::enter(Phase::LinearAlgebra);
        sieve.filter_checked(
            &ctx,
            &relations,
            &LargePrimeFilterOptions {
                modulus: spec.l,
                ..Default::default()
            },
        )?
    };
    let elapsed = started.elapsed().as_nanos();
    let tally = work::snapshot().since(&before);
    let uniforms = work::uniform_snapshot()
        .since(&uniforms_before)
        .ok_or("sampler counter regression")?;
    let widths = work::widths(p, spec.l);
    let phases:Vec<_>=Phase::IC.iter().map(|&phase|{
        let t=tally.phase(phase);
        json!({"phase":phase.name(),"primitives":work::prims_json(t),"diagnostic_priced_bitops":work::price(t,&widths).expect("known widths").to_string()})
    }).collect();
    let summary = json!({"schema":"cover.two-large-prime.component/v1","source_revision":source,
        "mode":mode,"p":p,"seed":seed,"l":spec.l,"m":9,"small_percent":percent,
        "full_base":sieve.base().len(),"small_base":sieve.small_columns(),"b_count":count,
        "line_count":line_count,"cohort_digest":fingerprint.finalize().to_hex().to_string(),
        "candidates":candidates,"refused":refused,"duplicates":duplicates,"unique_verified":relations.len(),
        "by_large_count":kinds,"graph":filtered.report,"replayed_rows":filtered.replayed_rows,
        "complete_rows":filtered.rows.len(),"output_nonzeros":filtered.rows.iter().map(|r|r.cols.len()).sum::<usize>(),
        "inner_wall_ns_practicality_only":elapsed,"phases":phases,
        "observed_primitives":work::prims_json(&tally.total()),"uniform_sampling":work::uniform_json(&uniforms.sum(&Phase::IC).ok_or("sampler sum overflow")?),
        "foreign_hook_calls":work::foreign(),"other_modulus_ops":work::snapshot_other(),
        "complete_cost":false,"total_bitops":null,"comparative_eligibility":false,
        "full_dlp_recovery":false,"rho_parity":false,
        "known_uncounted":["native containers/copies/normalization/scheduling","cohort hashing and serialization","PRNG internals and other sampling paths","some sieve native operations","individual-log descent/recovery not executed"],
        "boundary":"cold cover/base setup, fixed incidence/shared B cohort, source Jacobian verification, duplicate removal, graph filtering and all certificate replays; instance generation excluded"});
    let inputs: Vec<_> = relations
        .iter()
        .map(|r| json!({"full":r.full(),"partial":r.partial()}))
        .collect();
    serde_json::to_writer(
        output,
        &json!({"summary":summary,"inputs":inputs,"certificates":filtered.certificates}),
    )?;
    println!("{}", serde_json::to_string(&summary)?);
    Ok(())
}
