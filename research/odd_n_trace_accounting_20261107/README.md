# Odd-n trace accounting: where the extra factor 2 on 2E targets comes from

2026-11-07. Follow-up to
[`subgroup_trap_20261107`](../subgroup_trap_20261107/README.md) (PR #3646).
That study derived, without measuring it, that for odd n the twisted/standard
yield ratio on 2E targets is 2^{n+1}, not 2^n. It called the extra 2
accounting. This study measures the claim with exact counts at toy sizes and
asks who owns the factor.

- Protocol frozen before any study data at `3516d8a885`
  ([`PROTOCOL.md`](PROTOCOL.md), hashes in `FREEZE.sha256`).
- Run: [`reports/run-20261107T1/`](reports/run-20261107T1/). The registered
  verdicts are in `summary.txt`; the post hoc pooled analysis is in
  `posthoc.txt`.
- Report: [`REPORT.pdf`](REPORT.pdf). Figure: `fig_odd_n_trace.svg` / `.png`.
- Instrument: std-only Rust, with its own GF(2^N) arithmetic (N ≤ 25).
- Execution: local, exact counting. The AGENTS.md §9a isolab-eks
  confirmatory run is open, as for the parent studies.

Every statement below is an unverified assertion of this session
[pcc:unverified]. The claim record is listed at the end.

## Answer

The extra factor 2 exists. The registered rule did not decide who owns it,
because of a design flaw explained below. The post hoc pooled analysis
indicates that **it belongs to trace-homogeneous bases in general, not to
the twisted line**.

- **The mechanism is exact.** The exact checks all held:
  - The 2E criterion agreed with [#E/2]R = O on every target (0 mismatches).
  - MITM counts agreed with brute force on 2,293,041 (arm, target) pairs
    (0 mismatches).
  - The coset identity Σ_{R∈2E} N(R) = ((m0+m1)^n + (m0−m1)^n)/2 held
    exactly in all 120 census arm-units.
  - For odd n, the bases `tz`, `tw`, `twh` and `hz` had count 0 on all
    469,256 off-2E (arm, target) pairs.
- **Odd n gives about 2× at equal base size; even n does not.** On 2E
  targets, κ_tw/κ_std pooled to 1.96 over the 18 odd-n curves and 0.996
  over the 6 n = 4 curves (κ = mean N·#E/m^n). This is the discriminating
  part of registered P1. Its point estimate is inside the frozen band, but
  only 11 of 18 per-curve intervals clear 1.25.
- **An ordinary trace-homogeneous line gets the same 2 (post hoc).**
  - `hz` is a line u·F_q with Tr_{N/d}(u) = 0 that is not all-lift. It
    lifts about half its abscissae, so it has the standard base's size and
    the same system shape (x = u·t).
  - On 2E targets, pooled over all odd-n curves: H = κ_hz/κ_std = 2.08,
    99 % CI [1.62, 2.72], and L = κ_tw/κ_hz = 0.90 [0.71, 1.11].
  - On odd-order (cryptographic) targets, over the 17 curves whose F_q base
    is trace-mixed: H = 2.11 [1.55, 2.80] and L = 1.00 [0.80, 1.26].
  - The n = 4 control gives H = 1.00 [0.80, 1.26]. Its mirror image holds
    too: a trace-mixed line there has κ_mix/κ_std = 0.51 [0.40, 0.66].
  - Algebraically, a base of size m has 2E mean
    ((m0+m1)^n + (m0−m1)^n)/#E. That is at most 2m^n/#E, with equality
    exactly when the base lies in one coset. Every `hz` and `tw` base did
    (m1 = 0 on all 18 curves).
- **Shrinking F_q to its trace-zero half does not recover the 2.** On 2E
  targets, the `tz` relations per trial per column, relative to `std`, came
  out at 0.91 [0.70, 1.13] of the composition prediction. That prediction is
  about 2^{2−n}: 0.5 at n = 3 and 0.125 at n = 5. Per column, `tz` is
  therefore worse than F_q. The trace-zero comparator #3646 has in mind must
  be a full trace-homogeneous line of matched size, not the trace-zero part
  of F_q.

**Accounting or real advantage.** Both readings are partly right, for
different comparators.
- Against the plain F_q base, the 2 is a real per-trial gain for odd n:
  about twice the relations per trial per column on cryptographic targets.
- An ordinary index calculus gets it too, by replacing F_q with any
  trace-homogeneous line of the same size. That change gives up no lift
  density and keeps the system shape (the solver cost of `hz` was not
  measured).
- So the 2 belongs to the baseline, not to the twisted line. Crediting
  2^{n+1} to the twisted line is accounting: it uses the wrong comparator.

**Oakley n = 5 lane.** These are per-trial yield ratios on cryptographic
targets, for large q, where the line sizes concentrate
(m_tw/m_std, m_tw/m_hz → 2):

| comparator | yield per trial | trials to fill the columns |
|:--|:-:|:-:|
| plain F_q (trace-mixed for odd n) | 2^{n+1} = **64×** | 2^n = 32× |
| trace-homogeneous ordinary line `hz` | 2^n = **32×** | 2^{n−1} = 16× |

The twisted line's attributable gain is 32× per trial (16× in trials, since it
has twice the columns). Quoting 64× is correct only against the plain F_q
base, and that baseline is improvable by 2× without lift density. This
supports #3646's "accounting" call, provided the trace-zero comparator is
read as a full line (`hz`), not the `tz` sub-base [pcc:unverified].

## Registered verdicts

`reports/run-20261107T1/summary.txt` (scored by `odd_n_trace score`):

| prediction | verdict | why |
|:--|:--|:--|
| P1: 2E tw/std = composition prediction; odd n has the extra 2 | **FAIL** | ratio part ok (misses 1/18, pooled obs/pred 1.015; even 0/6, 0.996); pooled D = 1.96 inside [1.7, 2.3], but only 11 of 18 units have interval low > 1.25 (17 needed) |
| P2: all targets tw/std = (m_tw/m_std)^n | **PASS** | misses 0/24, pooled 0.957 |
| P3: odd n coset split | **FAIL** | the exact part holds (0 of 469,256 off-2E counts nonzero); the std on/off ratio is undefined on n=5 d=3 B3, whose 6-point F_q base is trace-homogeneous by chance; κ_tz has zero means on 3 sampled n = 5 curves |
| P4: n = 4 control | **PASS** | std ⊂ 2E, 0 of 12,134 off-2E nonzero; κ_std, κ_mix as predicted |
| P5: odd-order targets tw/std = P1 prediction | **FAIL** | misses 3/24; n=5 d=3 B2's odd subgroup (257 points) has std mean 0, so the ratio is infinite |
| P6: H, L, T, mix on 2E and odd-order targets | **FAIL** | H_2E: misses 0, but pooled obs/pred 1.11 (> 1.1); the odd-class and T/mix pooled values are 0, ∞ or NaN from zero means at n = 5 |
| P7: Tr(a) = 1 controls | **PASS** | tw m = 0 on all 8 cells; hz entirely off 2E with the predicted zeros |
| P8: instrument | **PASS** | see "Answer" |
| Interpretation rule | **inconclusive** | pooled H = ∞ (one unit with std mean 0 on odd-order targets); L = 0.985; H interval low > 1 in 19 of 36 |

Every miss is an n = 5 unit except one: n=3 d=4 B2, `tz` on its 65-point
odd subgroup. All 9 n = 3 curves pass every statistic on 2E targets.

## What went wrong in the design (post hoc)

- At n = 5 a hit is worth up to n! = 120 ordered tuples, so counts are extremely
  clustered. 97–100 % of targets have count 0 for `std`, `tz` and `hz`
  (`summary.txt`, "zero frac"). With 2,000 sampled targets, the n = 5 means
  of the small arms rest on a few dozen hits, and some are exactly 0.
- The registered pooled statistic was a geometric mean of per-unit ratios,
  with no rule for zero or infinite ratios. One empty unit makes it 0, ∞ or
  NaN.
- At q = 8 the bases are tiny (3 to 5 abscissae). One curve's F_q base was
  trace-homogeneous by chance, and the odd subgroups of two curves have
  only 65 and 257 points.
- At toy q the raw per-trial yield ratio hz/std moves with base-size noise.
  Its predicted value ranges from 0.16 to 5.5 (`posthoc.txt`,
  "Composition"). Only size-normalised κ compares the bases. At
  cryptographic q the sizes of half-lifting lines concentrate, so κ and the
  raw ratio agree.

The post hoc analysis (`src/bin/posthoc.rs`, written after scoring) pools
units first: K = mean of κ over units, ratios of pooled K, and a bootstrap
over targets within units. It is a new statistic chosen after the data, and
it is labelled as such everywhere.

## Not claimed

- No ECDLP cost and no leaderboard, benchmark or rho row.
- No per-solve cost of `hz` (only the shape argument).
- Nothing at cryptographic sizes beyond the exact composition algebra and
  the toy measurements above.
- The registered interpretation outcome is "inconclusive"; outcome A is a
  post hoc reading.

## Sources checked

KB unavailable: `scripts/research-kb` reports no executable, and the
research-kb MCP server is not connected in this session. I used the lead atlas
and `rg` instead:

- `crypto-autoresearcher:knowledge/leads/ecdlp/ic-binary/LEAD-ICB-cofactor-torsion-quotients.yaml`
  (closed_obstructed): the E/2E cofactor mechanism. The factor here is that
  mechanism expressed through the choice of line.
- `crypto-autoresearcher:knowledge/leads/ecdlp/ic-engine/LEAD-ICE-semaev-yield-model-coset-typing.yaml`
  and `ledger/evidence/EV-SEMBIN-0c8bf4.yaml`: trace-parity confinement for
  V ⊆ ker(Tr) subspace bases, including odd-n per-target counts (preliminary).
  This study is the F_q-line version, with matched-size comparators.
- `crypto:research/lift_dense_torsion_pairing_20261106/README.md`: the
  T-stable-line rule (lines u ∈ b^{1/4}·F_q are excluded here).
- `crypto:research/subgroup_trap_20261107/README.md`: the derivation under
  test.

## Reproduce

```sh
cargo build --release
./run_all.sh reports/run-20261107T1              # selftest, 8 cells, score, hashes (about 8 min on 4 cores)
./target/release/posthoc reports/run-20261107T1   # post hoc pooled analysis
python3 plot.py reports/run-20261107T1            # layout only
```

## Claim record

- `PCC-PENDING` (measurement). Its proof is
  `reports/run-20261107T1/validator-report.json`: `odd_n_trace validate`
  recomputes the registered verdicts and headline numbers from the
  sha256-pinned cell rows. No signed verdict covers it, so every statement
  here is [pcc:unverified].
