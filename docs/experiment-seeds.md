# Experiment seeds and paired A/B reproducibility

Policy ID: isogeny-labs/experiment-seeds/v1. Applies prospectively to randomized experiments, measurements, simulations, searches, model training and A/B comparisons. Existing task-specific protocols and stricter gates remain binding.

## Freeze randomness before outcomes

- Before execution, commit or otherwise immutably register a seed manifest alongside the experiment protocol. Record the study ID, manifest/schema version, master seed as an exact-width hexadecimal string, explicit seed panel and replicate IDs, every PRNG algorithm/library/version, seed width/encoding and initialization, and the exact child-stream derivation. Pin the manifest hash. Preserve an existing frozen seed schedule; never silently replace it.
- Specify every source of randomness: curve/instance/target generation, factor-base construction, relation/order sampling, walk/solver randomness, randomized algebra, training/splits/initialization/loaders and resampling/analysis when applicable. Use an explicit seed argument/configuration where supported; never rely on wall-clock time, process ID, host identity, entropy defaults, language-randomized hashes or an undocumented global seed.
- Derive streams using a versioned stable hash/KDF or a documented PRNG stream-splitting facility with domain separation. Key them by study/suite, replicate, instance, phase and logical stream or shard identity. Scheduling order, physical worker identity and machine count must not change a case's random inputs. Specify the algorithm, encoding, byte order and truncation; retain derived seeds. Do not use ad hoc master-plus-worker arithmetic. Detect duplicate seeds/stream identities and declare intentional reuse.
- Shared workload keys exclude the A/B arm. Algorithm-internal streams may include an arm label when independent streams are appropriate. Declare whether streams are shared, paired or independent; preserve any existing independently selected walk-seed design. Persist/checkpoint PRNG states or counters, logical indices and input/stream hashes for exact continuation. Allocate disjoint logical work across workers.

## Pair A/B on the same experiment

- Freeze the compared source revisions/configurations and intended difference. Use the same public instances, curves/subgroups, targets, factor bases and ordered input/relation streams for a compatible stage comparison. Equal numeric seeds are insufficient when the implementations draw randomness differently: materialize replayable input streams or use stable per-case keyed generation, and verify their hashes.
- When the algorithm deliberately changes factor-base selection, relation generation or another input-producing stage, pair the upstream instances and resource budget, record that difference, retain each arm's resulting stream and include its production cost. Do not force identical downstream outputs or call different stages the same benchmark. Keep coordinate field p separate from subgroup/scalar modulus r.
- Keep resources, thread counts, stopping rules, time/memory limits, compiler flags and measurement conditions matched. Record concurrency/affinity and relevant nondeterminism. Use the established A/A noise control and interleaved or counterbalanced A/B order, with a separately recorded scheduling seed if randomized. Preserve stricter repository timing and admission rules.
- Predeclare multiple independent replicate seeds and sample size for general performance claims. Replays of one seed test repeatability, not independent replication. Retain all preregistered trials, failures, timeouts and exclusions with reasons; never reroll or select seeds after seeing outcomes. Report per-pair results and an appropriate paired summary/uncertainty, separating runtime noise from between-instance variation. Exploratory single-seed observations must be labeled as such. Use fresh held-out seeds/instances for validation after tuning and disclose intentional corpus reuse.

## Save a replayable record

For each run retain the manifest and hash; full seed/stream mapping; exact commands/configuration; repository commit(s), source/diff and executable hashes as applicable; dependency locks/toolchain/container/environment versions; hardware, worker/thread count and limits; exact input/corpus and output hashes; raw per-trial records, correctness checks and failures; and a replay command. Record actual artifacts and checks only. Protect secrets: seeds are for public/synthetic experiment data and must not reveal real private keys or secret data.

Keep target-generation seeds or scalar witnesses that reveal a challenge logarithm in evaluator custody and outside solver inputs, according to the existing protocol; solver success must use only the permitted public problem data. Record the custody/release arrangement so replay remains possible without leaking the answer during a solve.

Before describing a new result as reproducible, replay at least one representative case for every compared arm/configuration with the frozen manifest. Compare exact mathematical outputs, witnesses, useful independent rank and recovered public logs where applicable. Canonicalize only order that the protocol explicitly treats as irrelevant. Declare bitwise versus semantic/tolerance-based reproducibility in advance; predeclare numerical tolerances and document irreducible GPU/thread/library nondeterminism. Timing need not match bitwise. A repeated deterministic run is a plumbing/replay check, not a noise estimate or independent replication.

If seed control, source/data pins, input-stream recovery or replay is missing, record the exact gap and label the result unqualified for a reproducible A/B or speedup claim; perform only work already permitted by the task. Do not fabricate seeds for historical runs, rewrite frozen evidence, or expand maintenance tasks into experiments. Review/report tasks audit these fields in cited evidence. Preserve this policy in handoffs and future scheduled-task prompt updates. Existing M31 requirements, cost-accounting boundaries, parallel-worker limits, impact scoring, privacy, compute admission and publication restrictions still apply. [pcc:unverified]

## Integration in crypto

This is an additional experiment contract, not a replacement for `AGENTS.md`
or a change to any frozen protocol. In particular:

- Section 10 still requires the recorded clean baseline, same-host resource
  isolation, an A/A noise check, at least five interleaved A/B rounds, exact
  output checks, and one-thread/many-thread checks for parallel changes. A
  replicate-seed panel and repeated timing rounds are different requirements.
- Keep current M31 task requirements and distinguish field modulus `p` from
  subgroup/scalar modulus `r`. Preserve independently selected walk-seed
  designs and evaluator custody of target scalars. The primary IC comparison
  remains a cold single-target solve with setup charged in full.
- Integrate seed manifests into the existing frozen protocol, task card,
  checkpoint, `bench.record/v1` record, artifact custody and validation/PCC
  process. A seed manifest or matching replay alone grants no execution,
  publication, benchmark admission or signed verification status.
- Use the existing native experiment implementation. This policy adds no Python
  research harness and does not change historical fixtures, seeds or evidence.
- Review and maintenance tasks audit this information in existing evidence;
  missing metadata is a qualification gap, not authorization to rerun a study.

## Sources checked

- Existing repository `AGENTS.md`, especially section 10, the IC measurement
  rules, checkpoint/artifact custody and the persistent research loop.
- [Rust Rand: StdRng portability](https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html):
  the PRNG algorithm and environment belong in the replay contract; a numeric
  seed alone does not specify a portable output stream.
- [NumPy: parallel random number generation](https://numpy.org/doc/stable/reference/random/parallel.html):
  deterministic logical stream identities and explicit stream derivation avoid
  dependence on task scheduling. This is a design reference, not a choice of
  experiment implementation language.

## Point-index fresh public panel (prepared 2026-11-27)

Study ID `crypto-point-index-fresh-panel-20261127`. The prepared, **unrun** 84-case input panel and its complete seed mapping are under `research/unified_point_index_20261127/panel-bridge-20261127/fresh-inputs-20261127/`. SHA-256 of `panel/seed-manifest.json` is `119f95bec637ec55675dec63e5e20f5fd379c80eec92140968d6bbc3bd864b36`; SHA-256 of `inputs.sha256` is `be7ceff3ac847d71f9ebb7ea464db687a47c4a40c6607c9fa2c9d008d553cc01`. The panel manifest and hash list both bind the seed manifest. No solver, image, S3 checkpoint or benchmark was run to prepare these public files. [pcc:unverified]

The exact-width 256-bit public master seed is `64cb17f4c82a0e3cce0c0c2f982c7f53a55a8137aac7a9a3c2ed70dc20e020f3`, defined as SHA-256 of UTF-8 `isogeny-labs/crypto/point-index/fresh-panel/2026-11-27/v1`. For every curve `p`, split `development` or `holdout`, replicate ID and target index, derive a fixture seed as the unsigned big-endian first eight bytes of SHA-256 of `point-index/fixture/v1`, a NUL byte, the 32 raw master bytes, `p` as u64 big-endian, split tag `d` or `h`, replicate ID as u8 and target index as u8. The three schedule seeds use domain `point-index/schedule/v1`, a NUL byte, the same raw master bytes and schedule replicate ID as u8; their decimal values are `16818245236990737041`, `14481075801228219236`, and `1511192426156936103`. This is public deterministic input selection fixed before outcomes. The original panel's seeds 1–10 and 101–104 and its original protocol stay untouched. [pcc:unverified]

The 84 distinct fixture seeds are listed individually in the seed manifest. Each public problem shares its fixture seed and target across both arms. The native solver's `Seeded` generator is the source-defined u64 SplitMix64 with wrapping state increment and little-endian output chunks; the panel scheduler uses the separate source-defined u64 SplitMix64. The target point comes from an independent exhaustive tiny-curve oracle using scalars 2 and `r-3`, held only in the separate manifest. Ten seed indices per curve are development cases and four are fresh holdout cases; both have two targets. The manifest's three schedule IDs are independent from fixture IDs. Equal numeric fixture seeds do not guarantee equal downstream draw streams if either solver changes draw order, so receipt/replay and full pipeline accounting remain required. The prepared manifest deliberately says `pending-final-source-freeze`: it does not admit a qualified timing run until reviewed final sources and a frozen protocol/spec replace that marker in a new add-only record. [pcc:unverified]
