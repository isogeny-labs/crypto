//! Replay complete candidate flow and split RNG on the measurement runner.
//! Uses the shared line enumeration/sieve kernel, but separate coefficient
//! arithmetic to reconstruct/refuse every candidate and check retained history.
use crate::{
    pair_driver::{Record, Source, SplitRecord},
    pair_math_check::Domain,
};
use crypto_lib::cryptanalysis::{
    jv_cover::Ctx,
    jv_sieve::{self as sieve, Counters, FpRing, Line, SieveSetup},
};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
fn ensure(ok: bool, why: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn bind(hash: &mut Sha256, index: u64, line: &Line) {
    hash.update(index.to_le_bytes());
    for p in [&line.a0, &line.a1] {
        hash.update((p.len() as u64).to_le_bytes());
        for x in p {
            hash.update(x.to_le_bytes());
        }
    }
    hash.update((line.b.len() as u64).to_le_bytes());
    for x in &line.b {
        for y in x.0 {
            hash.update(y.to_le_bytes());
        }
    }
}
fn same_source(s: &Source, candidate: u64, b_index: u64, line: &Line, t: u64) -> bool {
    s.candidate_index == candidate
        && s.b_index == b_index
        && s.a0 == line.a0
        && s.a1 == line.a1
        && s.b == line.b.iter().map(|x| x.0).collect::<Vec<_>>()
        && s.s == t
}
fn same_split(s: &SplitRecord, candidate: u64, b_index: u64, line: &Line, t: u64) -> bool {
    s.candidate_index == candidate
        && s.b_index == b_index
        && s.a0 == line.a0
        && s.a1 == line.a1
        && s.b == line.b.iter().map(|x| x.0).collect::<Vec<_>>()
        && s.s == t
}
/// Separate range sampler arithmetic; the PRNG algorithm/seed is registered.
fn draw(rng: &mut StdRng, p: u64, raw: &mut u64) -> u64 {
    let zone = (p << p.leading_zeros()).wrapping_sub(1);
    loop {
        *raw += 1;
        let product = rng.next_u64() as u128 * p as u128;
        if product as u64 <= zone {
            return (product >> 64) as u64;
        }
    }
}
pub fn qualify(
    record: &Record,
    ctx: &Ctx,
    setup: &SieveSetup,
    domain: &Domain,
) -> Result<Value, String> {
    let p = record.public.p;
    let space = sieve::b_space(setup);
    let mut rng = StdRng::seed_from_u64(record.seed ^ 0xA17E_2E17);
    let scramble = sieve::coprime_scramble(space, &mut rng);
    let mut split_rng = StdRng::seed_from_u64(record.seed ^ 0x5155_4144_5041_4952);
    let ring = FpRing::new(p);
    let nu = sieve::nu_lines(setup);
    let mut slot = Vec::new();
    let mut ctr = Counters::new(p);
    let mut hash = Sha256::new();
    let mut seen = BTreeSet::new();
    let (mut source_index, mut split_index, mut checkpoint_index) = (0usize, 0usize, 0usize);
    let (mut candidates, mut refused, mut duplicates, mut lines, mut raw_draws, mut calls) =
        (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let cap = record.stop.ends_with("resource cap");
    let limit = record.counts.b_count + u64::from(cap);
    let mut begin = 0u64;
    let mut cap_reached = false;
    'enumerate: while begin < limit {
        let end = (begin as u128 + 1024).min(space) as u64;
        let batch: Vec<_> = (begin..end)
            .map(|i| sieve::b_from_index(setup, i as u128, scramble))
            .collect();
        let ab: Vec<_> = batch.iter().map(|b| (b[0].0[0], b[0].0[1])).collect();
        let hints = sieve::roots_by_incidence(p, &nu, &ab, &mut slot).0;
        for (offset, b) in batch.iter().enumerate() {
            let b_index = begin + offset as u64;
            ensure(b_index < limit, "enumeration passed retained endpoint")?;
            let found = sieve::lines_for_b_hint(
                &ring,
                &ctx.f.f,
                setup,
                b,
                &mut rng,
                true,
                hints[offset].as_deref(),
            );
            let shared = if found.len() >= 2 {
                Some(sieve::shared_for_b(&ring, &found[0], &mut 0))
            } else {
                None
            };
            for line in found {
                lines += 1;
                bind(&mut hash, b_index, &line);
                let mut counts = Vec::new();
                sieve::sieve_line(
                    &ring,
                    setup,
                    &line,
                    &mut ctr,
                    Some(&mut counts),
                    shared.as_ref(),
                );
                for (s, _) in counts
                    .into_iter()
                    .enumerate()
                    .skip(1)
                    .filter(|&(_, n)| n >= 5)
                {
                    let s = s as u64;
                    candidates += 1;
                    if split_index >= record.max_split_records {
                        ensure(
                            record.stop == "split trace resource cap"
                                && b_index == record.counts.b_count,
                            "split trace stop ordering",
                        )?;
                        cap_reached = true;
                        break 'enumerate;
                    }
                    let raw = json!({"a0":line.a0,"a1":line.a1,"b":line.b.iter().map(|x|x.0).collect::<Vec<_>>(),"s":s});
                    let proof = domain.line(&raw);
                    let mut factors = None;
                    if proof.as_ref().is_ok_and(|x| x.tail.len() == 5) {
                        let split = record
                            .splits
                            .get(split_index)
                            .ok_or("missing quartic invocation")?;
                        ensure(
                            same_split(split, candidates, b_index, &line, s),
                            "complete quartic invocation order/binding",
                        )?;
                        let proof = proof.as_ref().map_err(|e| e.clone())?;
                        ensure(split.tail == proof.tail, "complete split tail binding")?;
                        crate::pair_math_check::check_split(&split.report, &split.tail, 8)?;
                        for attempt in split.report["attempts"]
                            .as_array()
                            .ok_or("split attempts")?
                        {
                            let expected: [u64; 4] = std::array::from_fn(|_| {
                                calls += 1;
                                draw(&mut split_rng, p, &mut raw_draws)
                            });
                            ensure(
                                attempt["polynomial"] == json!(expected),
                                "exact split seed/draw stream",
                            )?;
                        }
                        factors = serde_json::from_value::<Option<[Vec<u64>; 2]>>(
                            split.report["factors"].clone(),
                        )
                        .map_err(|e| e.to_string())?;
                        split_index += 1;
                    } else {
                        ensure(
                            record
                                .splits
                                .get(split_index)
                                .is_none_or(|x| x.candidate_index != candidates),
                            "unexpected quartic invocation",
                        )?;
                    }
                    let certificate = proof.and_then(|x| domain.candidate(&x, factors.as_ref()));
                    let next = record.sources.get(source_index);
                    let Ok(certificate) = certificate else {
                        refused += 1;
                        ensure(
                            next.is_none_or(|x| x.candidate_index != candidates),
                            "refused candidate emitted",
                        )?;
                        continue;
                    };
                    if source_index >= 262_144 {
                        ensure(
                            record.stop == "source resource cap"
                                && b_index == record.counts.b_count,
                            "source cap ordering",
                        )?;
                        cap_reached = true;
                        break 'enumerate;
                    }
                    let row: crate::quadratic_pair_relation::Certificate =
                        serde_json::from_value(certificate.clone()).map_err(|e| e.to_string())?;
                    let orient = row
                        .remainders
                        .first()
                        .map_or(row.terms[0].1, |r| r.coefficient);
                    let key = (
                        row.terms
                            .iter()
                            .map(|&(i, c)| (i, c * orient))
                            .collect::<Vec<_>>(),
                        row.remainders
                            .iter()
                            .map(|r| (r.key, r.coefficient * orient))
                            .collect::<Vec<_>>(),
                    );
                    if !seen.insert(key) {
                        duplicates += 1;
                        ensure(
                            next.is_none_or(|x| x.candidate_index != candidates),
                            "duplicate candidate emitted",
                        )?;
                        continue;
                    }
                    let next = next.ok_or("missing admitted original source")?;
                    ensure(
                        same_source(next, candidates, b_index, &line, s),
                        "complete admitted source ordering/binding",
                    )?;
                    ensure(
                        serde_json::to_value(&next.certificate).map_err(|e| e.to_string())?
                            == certificate,
                        "complete reconstructed source certificate",
                    )?;
                    source_index += 1;
                }
            }
        }
        let checkpoint = record
            .checkpoints
            .get(checkpoint_index)
            .ok_or("missing enumeration checkpoint")?;
        ensure(
            checkpoint.b_count == end
                && checkpoint.lines == lines
                && checkpoint.sources == source_index
                && checkpoint.cohort_sha256 == format!("{:x}", hash.clone().finalize()),
            "complete enumerated cohort checkpoint",
        )?;
        checkpoint_index += 1;
        begin = end;
    }
    ensure(
        cap == cap_reached
            && candidates == record.counts.candidates
            && refused == record.counts.refused
            && duplicates == record.counts.duplicates
            && lines == record.counts.lines
            && source_index == record.sources.len()
            && split_index == record.splits.len()
            && checkpoint_index == record.checkpoints.len(),
        "complete enumeration/candidate/refusal/split flow",
    )?;
    let uniform = &record.work_native["uniform_sampling"];
    ensure(
        uniform["calls"].as_u64().is_some_and(|n| n >= calls)
            && uniform["raw_draws"]
                .as_u64()
                .is_some_and(|n| n >= raw_draws),
        "all split random draws charged",
    )?;
    Ok(
        json!({"complete_candidate_replay":true,"enumerated_lines":lines,"candidates":candidates,"refused":refused,"duplicates":duplicates,"all_quartic_invocations":split_index,"split_sampler_calls":calls,"split_raw_draws":raw_draws,"split_rejections":raw_draws-calls,"every_cohort_hash_reconstructed":true}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn separately_reconstructed_rng_stream_matches_registered_sampler() {
        for p in [5, 7, 101, 1009, 1511] {
            let mut a = StdRng::seed_from_u64(389 ^ 0x5155_4144_5041_4952);
            let mut b = a.clone();
            let mut raw = 0;
            for _ in 0..1024 {
                assert_eq!(
                    draw(&mut a, p, &mut raw),
                    crypto_lib::cryptanalysis::jv_bitops::uniform_mod_u64(&mut b, 0, p)
                );
            }
            assert!(raw >= 1024);
        }
    }
}
