# Second-host replay of #2866: launched, results pending

**Status: in flight.** The results are not in this file yet; they will be added as a versioned addendum, never by rewriting this text.

## What is being replayed

The result of `research/prime_ksum_vs_incumbent_20261029/` (isogeny-labs/crypto#2866) is UNQUALIFIED (one host, no `claims/*.json`). The incumbent study's PROTOCOL §13 required a replay on a distinct host class before a leaderboard change.

## Form 1: a fresh session on the frozen specs (the form accepted for the incumbent's own replay)

`research/prime_ksum_vs_incumbent_20261029/second-host/specs/vs-incumbent-P<b>.json` are the four frozen specs of #2866. They differ from the originals in four fields only (checked with `jq -S` after deleting them): `label`, `measurement.rounds` (2 to 6, because EKS needs at least 5 and `alternate` order balances only an even count), `measurement.isolation_required` (L0 to L2) and `measurement.timeout_seconds`. Curve, targets, arms, warm-up, order and seed are unchanged, and the algorithm seed of a round depends only on (seed, workload, round), so rounds 1 and 2 equal the original executions.

- **Route.** The `isolab-experiment.yml` GitHub Action (the documented route in `.agents/skills/isolab-eks/references/submission.md`), one dispatch per spec, from `main`, with `source_revision` = `932461bbfb886d52db539aff9e179c09ab02a6b9` (the commit that adds the specs), instance `c8a.2xlarge`, 3,000 s. No AWS credential of this session was used.
- **First dispatches, 2026-10-30 00:36 UTC.**

  | spec | run | outcome |
  |:--|:--|:--|
  | P20 | 40980783290 | failed at image publish: HTTP 429 pulling a public base image; no experiment was submitted |
  | P24 | 40980785749 | in flight when this was written |
  | P28 | 40980788090 | in flight when this was written |
  | P32 | 40980790202 | failed at image publish, same 429; no experiment was submitted |

  The 429 hit when four builds pulled the same base image at once. P20 was redispatched alone as run 40982789324; P32 follows after its image publish passes. A failure before the launcher means no job exists, so a new dispatch is a first submission and not a duplicate (isolab-eks skill: reconcile before resubmitting).
- **Matching.** When the results come back, every original record is matched to its partner by (arm, workload, round, algorithm seed). Every deterministic field must be identical: outcome and scalar, `cost`, `counters`, `boundaries`, `field_ops`, `solver` and each phase's counts. The incumbent's `research/prime_base_oracle_tuning_20261029/second-host/analysis/match-original.json` is the reference output. Rounds 3 to 6 add target-round pairs, not curves.

## Form 2: an exact replay of the committed sessions (what a `vs_rho` claim needs)

Operation counts depend only on code, workload and seed, so any machine can check them (`docs/ecbench/README.md` §9). On a second host of a different class:

```bash
git clone https://github.com/isogeny-labs/crypto && cd crypto
cargo build --release   # from a clean tree; the binary's SHA-256 is its identity
# pull research/prime_ksum_vs_incumbent_20261029/sessions/P*/records.jsonl from the artifact store
# (sizes and SHA-256 in each sessions/P<b>/.artifacts.json), then:
for b in 20 24 28 32; do
  ./target/release/ecbench verify --dir research/prime_ksum_vs_incumbent_20261029/sessions/P$b \
    --replay-all --exit-code --out receipt-P$b.json
done
```

Each receipt must report every replay reproduced with 0 problems. `ecbench claim build --independent-receipt` takes these receipts. Whether the EKS launcher accepts a verify-only job is not established here.

## Not yet replayed

The holdout of `research/prime_ksum_pooled_20261029/` (16 fresh curves) has not been replayed on another host. Its sessions are committed, so Form 2 applies to it directly; a Form 1 spec would be a fresh run of its frozen `specs/holdout-P<b>.json` with 6 rounds and L2.
