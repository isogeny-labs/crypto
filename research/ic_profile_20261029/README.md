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
3. **The biggest already-measured lever is the factor base, not the code.**
   XP-20261016 showed `prime-abscissa:size=64` at 0.818 / 0.798 / 0.726 of the
   N=45 pipeline at w = 0 / 0.1 / 1. Re-running that arm in this session shape
   is the cheapest real `S` reduction available, and it needs no new code.
4. **Setup is 28% at 20 bits and unbounded at 131.** Above 64 bits the certified
   prime path cannot set an instance up at all. Per instruction, primality is
   left alone in this work: that means the 131-bit arm stays unprofileable by
   construction, and no rho-relative claim can be made for it.
5. **The Koblitz arm is not a speed target.** 2,000x rho with a two-order
   spread is an oracle/yield problem that no constant-factor work reaches. It is
   a candidate for `abandoned` with this record as the evidence.

## Next discriminating run

One admitted session on the same host, three arms: `ic-n45` (baseline),
`ic-n64` (candidate), `ic-n45-aa` (control), plus the priced §5a band. Success:
the 95% upper bound of `S_n64 / S_n45` below 1 at w = 0, 0.1 and 1. That is the
only change in view that plausibly moves the 20-bit prime `S` by more than noise.
The Koblitz arm gets exactly one rerun purely to clear admission; if it lands
the same order of magnitude, it is retired rather than optimised.
