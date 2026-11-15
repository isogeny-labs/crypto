# PROTOCOL: Cheon's DLP with auxiliary inputs as an ecbench method

Written and committed before any measured run of this study (2026-10-22).
Everything below is fixed in advance. Deviations are listed in `REPORT.md` under
"Deviations" and are never edited in here. This study builds on
`research/cheon_p224_shape_20261022/` (PR #2422): same algorithms
(`src/cryptanalysis/cheon_counted.rs`), same 14 curves, same planted-target law.
What is new is that the Cheon algorithms are registered ecbench methods, run by
`ecbench run` in sealed sessions, and that the auxiliary inputs come from a
**static Diffie-Hellman oracle whose queries are counted as a resource of their
own**.

## 1. Question

1. Can Cheon's `n - 1` and `n + 1` algorithms be charged in ecbench's unit
   (`ecbench.gae`, one affine addition or doubling = 1) next to `rho.negation`,
   verified by the runner against the planted logarithm, replayed bit for bit,
   and sealed as bounds?
2. What do they cost in that unit on the ladder of 14 curves, with what
   exponent and interval, and how far are they from the matched rho at every
   price of the unpriced native work (AGENTS.md section 5a) **and** at every
   price of an oracle query?

Scope. This is the DLP with auxiliary inputs, not the plain ECDLP. The attacker
is handed a static-DH oracle `P -> [alpha]P` (Brown and Gallant). The rho arm
cannot use it. No row is an ECDLP advance and none will be described as one;
every gap to rho is the distance remaining under the stated oracle, reported
as a measurement and not as a pass or fail (AGENTS.md, Success criterion).
The index-calculus baseline is not touched by this study.

## 2. The oracle as a resource

* Model: the server holds `alpha` and answers `P -> [alpha]P`. The attacker
  iterates it from the target `Q = [alpha]G`: after `i - 1` queries it holds
  `[alpha^i]G`.
* `cheon.pminus1_*` needs `[alpha^d]G` (`d | n - 1`): **`d - 1` queries**.
  `cheon.pplus1_comb` needs `[alpha^i]G`, `i <= 2d` (`d | n + 1`): **`2d - 1`
  queries**. (Brown and Gallant: `d` queries for the `n - 1` case.)
* The planted scalar lives inside `PlantedStaticDh`
  (`src/cryptanalysis/ecbench/oracle.rs`); a method sees `query`, `queries`
  and nothing else, and a public target has no oracle (the method is refused).
* Recorded per run, outside `total_gae`: `counters.oracle_queries`,
  `counters.oracle_server_group_ops` (the server's additions and doublings,
  one scalar multiplication by `alpha` per query), `detail.oracle`.
* A bound for an arm that queries an oracle must name the oracle in its domain
  (`ecbench bound fit --oracle LABEL`, `Envelope.oracle`): such a bound is never
  set beside a plain-ECDLP bound. The rho reference is fitted in the same
  oracle domain (it queries nothing: `oracle_queries` = 0, known). In an oracle
  domain `oracle_queries` per `sqrt(r)` is an axis of the frontier's dominance.
* Labels: minus `static_dh:n-1:d=shape(0.12943,0.015)`, plus
  `static_dh:n+1:d=shape(0.21106,0.015)`.

## 3. Frozen inputs

* Curves: the 14 curves of `research/cheon_p224_shape_20261022/curves.json`,
  as `prime_explicit` workloads (copied from that study's `ecbench-spec.json`),
  7 sizes `b in {32, 36, 40, 44, 48, 52, 56}` per family, `minus`
  (`n - 1 = d q`, `q` prime, `d` about `n^0.129`) and `plus` (`n + 1 = d q`, `d`
  about `n^0.211`). Registered in `docs/curves/registry.json` in this PR
  (table in section 8).
* Targets: `target_seed = 20261022`, **8 planted targets per curve** (index 0 to
  7; indices 0 to 5 are the workloads of PR #2422, same ids).
* Specs (three sessions, `rounds: 1`, `warmup: 0`, `isolation_required: L0`,
  `timeout_seconds: 1800`; counts need no isolation level and wall time is
  descriptive only and not reported as a result):
  `spec-minus.json` (arms `rho-neg` = `rho.negation`, `cheon-comb` =
  `cheon.pminus1_comb`; 7 minus curves), `spec-minus-naive.json` (arms
  `rho-neg`, `cheon-naive` = `cheon.pminus1_naive`; the minus curves of 32 to
  52 bits) and `spec-plus.json` (arms `rho-neg`, `cheon-comb` =
  `cheon.pplus1_comb`; 7 plus curves). The naive arm is the algorithm of
  `cheon_attack.rs` and costs about 20 times the comb arm; it is left off the
  56-bit curve after a timing probe (see "Timing probe" below), and is
  fitted over the four to five sizes of tier medium it reaches (36 to 52 bits).
* `d` is the method parameter `shape:RATIO:TOL` (`cheon_counted::shape_divisor`:
  the divisor of `n -/+ 1` with prime cofactor closest to `RATIO` in
  `log d / log n`, within `TOL`). RATIO is P-224's: `rho_minus = 28.9913/224 =
  0.12943`, `rho_plus = 47.2784/224 = 0.21106`, `TOL = 0.015`. The resolved `d`
  must equal `curves.json`'s `d` on all 14 curves (checked from
  `counters.d` of the records).
* Unit and accounting: as PR #2422 section 4. Charged: every addition and
  doubling of tables, baby and giant steps, shift, polynomial sums and the final
  verification; identity additions are not counted. Counted and not charged
  (`*_uncharged`): hash inserts and lookups, scalar-field multiplications,
  `F_{n^2}` multiplications for Cheon; canonicalisations for rho.
* The same ecbench instrument runs every arm; the runner verifies every answer
  against the planted logarithm in its own process.

### Timing probe (disclosed)

Before freezing the specs, one probe session (not part of the study, not
committed, binary built from the sources of this branch) ran the three arms
once on target 0 of the 56-bit minus curve to size the work: `rho.negation`
140,289,177, `cheon.pminus1_comb` 93,066,020 and `cheon.pminus1_naive`
1,871,132,659 operations, 8.6 minutes in all on a shared 4-core host. The
numbers are the ones the study will record for that workload (the methods are
deterministic); the probe chose only to leave the naive arm off the 56-bit
curve. Smoke runs on the 32-bit curve (two targets, three arms) checked the
plumbing: `cheon.pminus1_comb` 53,074 and 68,568 operations, equal to PR #2422's
driver records for those targets.

## 4. Pricing (AGENTS.md section 5a, extended to the oracle)

Two prices, both applied to every arm alike:

* `w` in `{0, 0.1, 1}` GAE per unpriced native operation (every `*_uncharged`
  counter of every arm, rho included).
* `q` in `{0, 0.1, 1}` GAE per oracle query, and a fourth point `q = server`:
  one query costs what the server spends (`oracle_server_group_ops / queries`,
  about `1.5 log2 n`). The server point is an additional point; it never
  replaces the band. Rho has no queries.

An order between Cheon and rho is claimed only where it holds at all `3 x 4`
grid points; otherwise the pair is "unresolved" at that size. Bounds are fitted
at the charged cost and at `--weight 0.1` and `--weight 1` (the `w` band); the
`q` band is applied in the tables from the recorded query counts.

## 5. Pre-registered predictions (written before data)

* P1 (replay). Every record replays `identical` under `verify --replay-all`;
  the audit is OK on both sessions.
* P2 (positive control). For targets 0 to 5, the Cheon `ops`, phases and
  uncharged counters equal PR #2422's driver records exactly
  (`research/cheon_p224_shape_20261022/runs/cheon-all.jsonl`, same workload id,
  same arm): the method wrapper changes no count. Any difference is a defect
  and is reported.
* P3 (exponents of `ops` in the bound fit, tier medium, sizes 36 to 56, six
  sizes, `gae = C r^alpha`): `cheon.pminus1_comb` in `[0.40, 0.50]` (PR #2422:
  0.447 +- 0.012 over seven sizes), `cheon.pplus1_comb` in `[0.30, 0.46]`
  (PR #2422: 0.413 +- 0.013), `rho.negation` in `[0.45, 0.55]`.
* P4 (oracle exponent): queries `~ n^rho` up to the discrete choice of `d`:
  `oracle_queries` exponent in `[0.10, 0.17]` (minus) and `[0.17, 0.26]` (plus).
* P5 (distance to rho, a report and not a gate): at 56 bits Cheon-comb is
  cheaper than the matched rho in `ops` (PR #2422: ratios 0.59 minus and 0.21
  plus) and the order survives the whole `w x q` grid, because 4031 queries at
  about 85 server operations each are 3.4e5 against 9.8e7 operations. At 32
  bits (toy) the order is not predicted.
* P6 (frontier). In each oracle domain the rho reference and Cheon-comb are both
  on the frontier (Cheon wins `ops`, rho wins `oracle_queries`).

## 6. Success, failure and stop conditions

* A row is a result only if the runner verified it (`status = verified`).
  Failures, timeouts and wrong answers stay in the tables with their reason and
  are excluded from fits and counted.
* The scaling bound is a result if a tier has at least four sizes with
  verified runs for the arm. The tier `toy` (32-bit field) has one size; it is
  reported and never pooled. Bounds are fitted for tier `medium` (36 to 56
  bits).
* Rho sanity gate (PR #2422's): if the rho `S` is outside `[0.3, 4]` on any
  size, or its exponent outside `[0.40, 0.60]`, nothing is ranked against rho.
* Negative control: a wrong oracle must not return the planted logarithm
  (`tests/ecbench_cheon.rs`).
* Stop: a run that exceeds `timeout_seconds` is recorded as a timeout, listed,
  and not repeated at a longer limit.
* Not allowed after seeing a result: changing curves, `d`, seeds, targets, unit,
  accounting, the band, or dropping a target.

## 7. Analysis plan (fixed)

1. `ecbench verify --replay-all`; `ecbench table` and `ecbench compare --a rho-neg
   --b <arm>` per session (paired ratios `sum S_B / sum S_A` with bootstrap
   intervals, per curve).
2. `ecbench bound fit --tier medium` per arm and session at the charged cost
   and at `--weight 0.1`, `--weight 1`, with `--oracle LABEL` on both the
   Cheon arms and the rho reference (six sizes for the comb arms, five for the
   naive arm); `ecbench bound check`; `ecbench frontier
   build` into `docs/bounds/`.
3. A native analysis program (`examples/cheon_ecbench_analyse.rs`) reads the
   sessions' records only and writes the `w x q` grid per curve (ratio Cheon /
   rho and its resolved or unresolved verdict), the phase shares, the
   gap-to-rho table and the figure data. No new measurement.
4. Figures (SVG with 150 dpi PNG): ops against `r` on log-log axes with the
   fitted lines; ratio to the matched rho per size at `w = 0, 0.1, 1`; oracle
   queries and their priced share against `r`; phase shares. Plotting only
   recorded values.
5. Classification (AGENTS.md section 3): the Cheon arms are DLP-with-oracle
   rows. Their `S` is below rho's where reported, with the queries priced
   apart; this is neither an ECDLP advance nor an index-calculus improvement.

## 8. Curves

Registered ICV1 slugs (`docs/curves/registry.json`):

| family | bits | `n` | `d` | slug |
|:--|--:|--:|--:|:--|
| minus | 32 | 3221255537 | 16 | `icv1-fp32-tm30063-42a4d230` |
| minus | 36 | 51539573731 | 30 | `icv1-fp36-t33869-2ec6c587` |
| minus | 40 | 824632004309 | 28 | `icv1-fp40-t1716529-3d19b225` |
| minus | 44 | 13194139832747 | 74 | `icv1-fp44-tm299397-25170a45` |
| minus | 48 | 211106219695673 | 56 | `icv1-fp48-t12837375-d282330e` |
| minus | 52 | 3377699780661797 | 164 | `icv1-fp52-tm60133899-e028287d` |
| minus | 56 | 54043195377297631 | 210 | `icv1-fp56-t151148327-021eeef8` |
| plus | 32 | 3221175689 | 90 | `icv1-fp32-t49785-c7011f8d` |
| plus | 36 | 51539736719 | 240 | `icv1-fp36-tm129119-ba25e086` |
| plus | 40 | 824633358973 | 398 | `icv1-fp40-t361865-3168d4f0` |
| plus | 44 | 13194137032873 | 574 | `icv1-fp44-t2500477-898d2466` |
| plus | 48 | 211106244944429 | 990 | `icv1-fp48-tm12411381-a81a3368` |
| plus | 52 | 3377699707176467 | 1836 | `icv1-fp52-t13351431-b09ac3d1` |
| plus | 56 | 54043195250336543 | 2016 | `icv1-fp56-t278109415-2d6157a8` |
