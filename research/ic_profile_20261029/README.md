# IC phase profile: where the time actually goes (2026-10-29)

Profile-only study. Two ecbench sessions on one x86 host, three arms each, to
attribute index-calculus cost to phases in one unit before optimising anything.
No new baseline, no speedup claim, no security claim.

**Why this exists.** "Make index calculus fast" has no target until the phases
are priced. The bet before this run was that linear algebra or setup would
dominate; the measurement says otherwise.

## What ran

| run | curve | arms | admission |
|:--|:--|:--|:--|
| `crypto-ic-profile-prime-20261029` | `prime_search` 20 bits, seed 59297, 8 targets, 5 rounds | `rho-neg`, `ic-n45` (`prime-abscissa:size=45`, `mitm:negation_folded=1`), `ic-n45-aa` | **admitted**: L2 144/144, exit 0 |
| `crypto-ic-profile-koblitz-20261029` | `koblitz a=0 n=31` (`icv1-f2m31-tm90707-c95f16f5`), 16 targets, 5 rounds | `rho-neg`, `ic-frob-m3` (`koblitz-orbit:divisor=0;1`, `mitm-frobenius:m=3`), `ic-frob-m3-aa` | **rejected**: seq 42 graded L1 at 64.8 preemptions/s; 287/288 L2 |

Both ran on the deployed lab image (`ecbench` sha256 `583e62f0…`, built from
`cb2ec6873f30b683f7885b53c410d69965f02c20`, linux/amd64), not on current `main`.
The A/A arms are bit-identical to their candidates, which is what a
deterministic counted unit should do.

Raw records (1.7 MB and 848 KB) stay in the private results bucket:
`runs/crypto-ic-profile-{prime,koblitz}-20261029/<pod-uid>/session/records.jsonl`.
Receipts, requests, L2 gates and build identity are committed beside this file.

## The profile

20-bit prime field, `ic.pipeline`, medians over 40 measured solves:

| phase | GAE (median) | share of priced GAE |
|:--|--:|--:|
| factor base | 0 | 0% |
| oracle setup | 2,070 | 27.8% |
| **relations** | **5,348** | **71.8%** |
| linear algebra | 0 | 0% (unpriced: 288 row ops) |
| verify | 26 | 0.3% |
| total | 7,444 | 100% |

- `S_ic = 9.93` (min 6.39, max 14.12) against `S_rho = 1.08` (min 0.39, max 2.28):
  **9.2x rho**, on a curve where rho is the right reference.
- Relation phase = 3,853 trials for 28 relations, a **0.73% yield**, at
  **1.39 GAE per trial**. The relation phase is trial count x per-trial cost.
- Every one of `sqrt_solves`, `lookups`, `row_ops`, `legendre_symbols` and
  `target_guard_probes` is **uncharged** in this record, so 9.93 is a lower
  bound in the §5a sense, and the phase shares are shares of a partial bill.

Koblitz m=31, same shape (unadmitted diagnostic):

| arm | S median | S min | S max |
|:--|--:|--:|--:|
| `rho-neg` | 1.247 | 0.301 | 2.800 |
| `ic-frob-m3` | 2,495.9 | 105.4 | 14,638.5 |

The relation phase there is 10^6–10^7 GAE per solve; everything else is noise.
That arm is **~2,000x rho**, with a spread of two orders of magnitude across
targets. It is not a tuning gap.

## What this rules out, and the ranked levers

1. **Not linear algebra, not verification.** LA is priced at zero GAE in the
   record and its 288 row ops are uncharged; verify is 0.3%. Optimising either
   cannot move `S` by more than the accounting error, and that error must be
   measured before it is optimised.
2. **Relations is the only phase that matters** at both sizes. For the 20-bit
   prime arm the two independent sub-levers are per-trial cost (1.39 GAE) and
   yield (0.73%). A 20% cut in either is a 14% cut in `S`.
3. **The factor-base lever is now measured, and it loses at 20 bits.** An
   earlier version of this file claimed XP-20261016 measured `size=64` at
   0.818 / 0.798 / 0.726 of `size=45`; that was wrong. Those ratios were
   `size=45` against `size=32`. XP-20261029-p20-large-n records N=64 against
   N=45 as mixed (lower at w = 1, higher at w = 0.1) and never held it out.
   The holdout ran on 2026-10-30 at w = 0 (below): **N=64 is 8.9% worse**, so a
   bigger base is not the lever at this size.
4. **Setup is 28% at 20 bits and unbounded at 131.** Above 64 bits the certified
   prime path cannot set an instance up at all. Per instruction, primality is
   left alone in this work: that means the 131-bit arm stays unprofileable by
   construction, and no rho-relative claim can be made for it.
5. **The Koblitz arm is not a speed target.** 2,000x rho with a two-order
   spread is an oracle/yield problem that no constant-factor work reaches. It is
   a candidate for `abandoned` with this record as the evidence.

## Result: N=64 loses at 20 bits

Two sessions on the latest-main ecbench (`ce9868bb…`, built from `9edcbe73`,
image `…/isolab/tasks@sha256:3accbd75…`): the frozen spec
[`spec-prime-n45-n64.json`](spec-prime-n45-n64.json), 8 targets x 5 rounds,
192 records per session, **192/192 verified** in both. Summary in
[`prime/session-n64-20261030.json`](prime/session-n64-20261030.json).

| arm | S median | S min | S max |
|:--|--:|--:|--:|
| `rho-neg` | 1.077 | 0.385 | 2.275 |
| `ic-n45` | 9.929 | 6.392 | 14.122 |
| `ic-n64` | 10.794 | 8.891 | 13.361 |
| `ic-n45-aa` | 9.929 | 6.392 | 14.122 |

Paired by workload and round, `S_n64 / S_n45` has median **1.089** over 40
pairs (min 0.744, max 1.437). The mechanism is in the phases: setup rises
2,070 -> 4,160 GAE (+109%) while relations fall 5,348 -> 3,907 (-27%). At one
cold target the base pays for itself in the relation phase and loses more than
it gains in setup.

**Admission.** Wall time is refused at this size: a 20-bit solve is 186.9 us and
L2 requires at least 1 ms, so 176/192 and 181/192 executions grade L1. The
comparison above is in counted GAE, which is deterministic and unchanged from
the 2026-10-19 binary session for `rho-neg` and `ic-n45` (medians 1.077 and
9.929 in both). It is a counts-only comparison: no wall-time claim, and the two
launcher receipts are `failed` on the admission rule, not on correctness or
verification.

**What this leaves.** Setup is the tax. The next lever is not a bigger base but
a cheaper or amortised one: many targets sharing a cold setup (the setup is
per-solve here), or a relation-phase change that does not double the base. The
Koblitz arm still gets one rerun for admission only; if it lands the same order
of magnitude it is retired.

## Lever 1: amortising setup across targets, and its ceiling

The profile's second-largest phase is oracle setup at 2,070 GAE, and ecbench's
cold rule charges it inside every one-target row by design ("each execution is
a fresh process"; nothing survives). So the amortisation question is not a new
session: it is the measured phase split with setup divided by the number of
targets `P` that share it, and it is an **extrapolation from counted phases**,
not a new measurement.

`S(P) = (2070/P + 5374) / sqrt(562427)`, from the medians above and the curve's
`r = 562427` (the 1,024 divisor the record uses is `sqrt(r)` = 749.95):

| P (targets sharing one setup) | setup per target | S | vs cold |
|--:|--:|--:|--:|
| 1 | 2,070 | 9.93 | 1.00 |
| 2 | 1,035 | 8.55 | 0.86 |
| 8 | 259 | 7.51 | 0.76 |
| 64 | 32 | 7.20 | 0.73 |
| infinity | 0 | 7.17 | 0.72 |

Two things follow. First, the whole amortisation lever is worth **at most 28%**
(9.93 -> 7.17): it is real, it is free of new code, and it is bounded above by
the setup share we already measured. Second, it does **not** change the class of
the result: at `P -> infinity` the pipeline is still **6.7x** matched rho
(7.17 against 1.08), and the relation phase alone (5,374 GAE) is 4.7x rho. To
reach parity the relation phase has to fall by about 7x, which setup
amortisation cannot touch.

So the order of levers is: amortise setup for the free 28% if and when a shared
setup method exists (none does today; the cold rule is deliberate), but the only
lever that can move the class of the result is the relation phase's per-trial
cost and yield, and the factor-base direction for that is now measured to lose.

## The factor-base axis is closed

A four-size sweep in one session (8 targets x 5 rounds, 240/240 verified, same
main image and host) settles the local shape of the one lever that was left:

| factor base | S median |
|:--|--:|
| 32 | 11.226 |
| **45** | **9.929** |
| 56 | 10.099 |
| 64 | 10.794 |

N=45 is a local optimum and every neighbour loses. Together with the paired
N=64 result this closes the factor-base direction at 20 bits: the setup cost
grows faster than the relation cost falls past 45. Summary in
[`prime/session-sweep-20261030.json`](prime/session-sweep-20261030.json).

So the cheap levers are exhausted: setup amortisation caps at 28% and 6.7x rho,
and no base size beats the baseline. A real speedup has to come from the
relation phase itself — a different decomposition oracle, or a relation search
that raises the 0.73% yield without doubling setup — not from tuning this one.

