# Results addendum (2026-10-26): corrections to the orbit-aware double-large-prime results

This file is additive. `RESULTS.md`, `PROTOCOL.md`, `receipts/`, `results/` and
`pilot/` are exactly as merged in PR #2591 (merge `f24efb19d`); the only edit to
anything that produced them is `pipefail` in `run_all.sh` (section 7). The
corrections below come from an independent review of that PR. Each is verified
here against the code or the data, or refuted.

* Protocol, committed before any run: [`PROTOCOL_ADDENDUM_20261026.md`](PROTOCOL_ADDENDUM_20261026.md)
  (commit `2dba818643`). Code: commit `0f5ce556d6`.
* New evidence: `receipts_addendum/` (255 cells, `ENVIRONMENT.txt` has the
  toolchain, host and binary hashes), `results_addendum/` (`tables.md`,
  `summary.json`, `chart.svg`, `structure.md`, `structure.json`).
* Reproduce: `run_addendum.sh primary|trajectory|realisation`, then
  `orbit_addendum_summary --in receipts_addendum --baseline receipts --out results_addendum`,
  `orbit_window_structure report --out results_addendum --receipts receipts`.
* Toy scale throughout (r between 2^15 and 2^21). No ECDLP speedup claim.

## 0. Summary

| finding | verdict | one line |
|---|---|---|
| A golden test environment dependent | confirmed and fixed | an `ic` built with `GITHUB_SHA` set differs from the golden in `software.git_commit` and `git_commit_source`; the old comparison fails, the fixed one passes (section 7) |
| B tests never exercise merges/absorptions | confirmed and fixed | the n = 11 fixture has 4 large points and no merge; new tests on the n = 17 K_1 cell (64 merged, 19 absorbable) recount the table independently and corrupt a factor (section 7) |
| C arms did not see an identical stream | confirmed; headline survives | streams differ in **9 of 25** pairs (not 6; all of n = 31). On the identical stream the gains are +3.0 / +0.6 / 0.0 / +32.4 / +14.8 independent relations at n = 17 / 19 / 23 / 29 / 31 (merged: +3.0 / +0.6 / 0.0 / +32.4 / +16.0) |
| D n = 31 excess "unexplained", birthday reading | confirmed; explained | 30 of the 34 merges at n = 31 are exact Frobenius images of raw abscissae of the low-degree window. With a random modulus the merges fall to 2 and the n = 31 gain to +0.7 +/- 2.9 (no gain). n = 29 stays birthday-driven (+35.3 +/- 12.5 on the random modulus) |
| E (i) replication unit | confirmed | one base per n; a second base moves the plain arm's yield by up to 1.9x, more than the seed spread |
| E (ii) caps at the knee | confirmed | cap-free: the orbit arms reach a given rank 8-25% sooner at n = 29, 31 and not at all sooner at n = 19, 23; the scalar is recovered no sooner at n = 29 |
| E (iii) CPU per relation, set-up | confirmed | at n = 31 collect is 9.5 s, filter + replay 2 ms |
| F smaller fixes | confirmed and fixed | `pipefail`, fail-closed summarizer, CLI help, `r < 2^63` guard |

## 1. Statements of RESULTS.md / PROTOCOL.md that this addendum corrects

| statement | status | evidence |
|---|---|---|
| "from the *same* relation stream", "identical relation stream" (RESULTS headline and section 4, PROTOCOL sections 1 and 3) | **false as written** | section 2: the accepted decompositions of arms (ii) and (iii) differ in 9 of 25 `(n, seed)` pairs |
| "counter differences are exact properties of the arm, not noise" (RESULTS section 4) | half true | the counters are deterministic (50 of 50 reproduced here, repeats agree), but a part of the (iii) - (ii) difference is the larger accepted set of an orbit-aware collector, not the graph (`D_acc`, section 2); the n = 31 headline moves from +16.0 to +14.8 on the identical stream |
| "Each factor is checked against real curve arithmetic when the table is built (`[factor]rep == Q`)" (PROTOCOL section 2) and "building the table checks `[factor]rep == Q` ... for each large point" (RESULTS section 5) | true only for the factors of the *large* points | the small-column factors used by the absorb arm, and the combination `f_Q f_S^-1`, were checked only by the later replays. Now checked at build time too (section 7) |
| "Setup is identical across arms" (RESULTS section 7) | **false as written** | the orbit arms certify `LambdaPowers`, canonicalise and run one self-check scalar multiplication per large point, none of it in `phase_group_ops` (identical at 1053 scalar multiplications). The measured set-up CPU does not resolve the difference (section 6) |
| "far above the model at n = 31 for a reason not found" / "This is not explained" (RESULTS headline and section 5) | **explained** | section 4 |
| the "birthday effect" reading of the gains, "consistent with random pairing at n = 17, 19 and 29" | holds at n = 17, 19 and 29; **fails at n = 31**, and at n = 17 the 64 merges are 41 raw-abscissa images + 23 projected-only | section 4 |
| "Untested hypothesis: ... a prefix base is partly Frobenius-structured ... the small-to-large relation is where I would look first" | the structure is in the **large-to-large** relation (n = 31, and a little at n = 29); the small-to-large relation (absorption) was not studied here | section 4 |
| the 95% t intervals over 5 seeds | cover one base realisation per n | section 5 |
| caps "chosen from the pilot", "gain in independent relations" at a cap | the caps sit at the knee; the yield at a cap is a windowed metric | section 3 |
| "Online CPU is not distinguishable ... CPU per verified relation falls only because the yield rises"; the CPU-per-relation verdicts | correct but redundant | CPU per relation is `collect / indep`; section 6 |
| 2c "orbit mode off reproduces the existing fixture byte for byte ... only the manifest path, the executable's own hash and wall time are normalised" | the test was environment dependent | section 7 |

## 2. Identical-stream re-measurement (finding C)

**What was wrong.** `Decomposer::find` accepts the first decomposition whose
number of distinct nonzero large *columns* is at most 2. In orbit mode the
column map is coarser, so arm (iii) accepts a superset of what arm (ii) accepts
and its first hit for a trial can be a different decomposition.

**How it is measured now.** Arm (iii-post), `LargePrimeGraph::FilterOrbitPostCollect`,
accepts on the plain column count (the stream of (ii)) and only then
re-expresses each accepted decomposition in orbit columns before the batch
filter. A BLAKE3 digest over `(trial, a, b, atom list)` of the accepted
decompositions proves the identity per cell. Everything else is as in
`PROTOCOL.md`: same curves, factor bases, caps, seeds 1-5, `max_states`.

**Checks** (`results_addendum/tables.md`, Table 1):

* (ii) and (iii-post): digest, trials, decompositions, accepted count, relation
  histogram and plain classes identical in **25 of 25** pairs; invalid identity pairs: none.
* (ii) and (iii): **9 of 25** pairs have a different accepted stream. They are
  n = 29 seeds 1, 2, 4, 5 and **all five seeds at n = 31**. In only 2 of the 9
  does the accepted count differ (n = 31 seed 1: 371 vs 375; seed 4: 377 vs 379);
  in the others the same trial gets a different first-hit decomposition.
  The class histograms differ in 6 pairs (the reviewer's count, checked on the
  merged receipts with `jq`): the 4 of these that are among the 9, and two
  (n = 23 seed 4, n = 29 seed 3) where the decompositions are identical and the
  orbit map moves one relation from double to single. A differing histogram
  therefore under-counts differing streams.
* Reproduction of the merged receipts: the deterministic counters of arms (ii)
  and (iii) (trials, plain classes, complete, verified, independent, rank,
  large points, orbits, merged) equal the merged receipts in **50 of 50** cells.
  Repeats agree on every counter and on the digest (60 repeat groups, 0 disagreements).

**Result** (independent relations at the primary cap, mean +/- 95% t half-width
over 5 seeds; signs = seeds above / equal / below):

| n | cap | `D_post` = (iii-post) - (ii) | signs | `D_orig` = (iii) - (ii), merged | `D_acc` = (iii) - (iii-post) | verdict `D_post` |
|---|---|---|---|---|---|---|
| 17 | 200 | +3.0 +/- 0.9 | 5+/0=/0- | +3.0 +/- 0.9 | +0.0 +/- 0.0 | survives |
| 19 | 400 | +0.6 +/- 0.7 | 3+/2=/0- | +0.6 +/- 0.7 | +0.0 +/- 0.0 | no gain |
| 23 | 450 | +0.0 +/- 0.0 | 0+/5=/0- | +0.0 +/- 0.0 | +0.0 +/- 0.0 | no gain |
| 29 | 300 | +32.4 +/- 6.7 | 5+/0=/0- | +32.4 +/- 6.3 | +0.0 +/- 1.5 | survives |
| 31 | 450 | +14.8 +/- 3.2 | 5+/0=/0- | +16.0 +/- 3.2 | +1.2 +/- 1.0 | survives |

**The headline gains survive the identical stream.** The prediction recorded
before the run (survival at n = 17, 29, 31 with `|D_orig - D_post| <= 4`, n = 19
indistinguishable from 0, n = 23 exactly 0) held. The acceptance-set effect is
at most +1.2 independent relations (n = 31), and it is a genuine effect of an
orbit-aware *collector*, not an artefact: it is not part of the quotient on a
fixed stream. The sentence "more verified relations ... at n = 17, 29 and 31
(every seed)" is **confirmed on the identical stream**, with the n = 31 figure
14.8 instead of 16.0; n = 19 remains "small and not clearly nonzero" (+0.6 +/- 0.7)
and n = 23 zero. The same holds for complete relations
(`results_addendum/tables.md`, Table 3). Arm (iv) was not re-run; it carries the
same acceptance-set caveat.

## 3. Cap-free yield (finding E ii)

The pilot (`pilot/p3_cap_sweep`, seed 9004; checked) shows the chosen caps at the
knee of the rank curve: n = 29 cap 300 gives 22 -> 57 independent relations for
(ii) -> (iii) and cap 600 gives 62 -> 62; n = 31: 5 -> 11 and 66 -> 66; n = 17:
cap 200 gives 8 -> 13, cap 300 gives 14 -> 14. In the merged dose series the gap
at twice the cap is 0 at n = 19, 23, 29, 31 and +0.3 (one seed +1) at n = 17.
So "+169% at n = 29" is a statement about where the cap falls.

The trajectory series removes the cap: for each cell the pipeline is re-run on
prefixes of the stored stream and the first prefix reaching each rank target is
recorded (run length 4x the cap, 3x at n = 31; `monotone` and
`consistent_with_pipeline` hold in 75 of 75 cells; the trajectory cell at trial
cutoff = primary cap equals the primary cell in 75 of 75). Trials to reach the
target, median over 5 seeds (`results_addendum/tables.md`, Table 4, has every
target and seed; no cell is censored):

| n | S+1 | target | (ii) | (iii) | (iii-post) | seeds (iii-post) earlier / equal / later than (ii) | paired mean (ii) - (iii-post) |
|---|---|---|---|---|---|---|---|
| 17 | 15 | rank >= 14 | 328 | 302 | 302 | 4 / 1 / 0 | +24 +/- 29 |
| 17 | 15 | full rank | 397 | 362 | 362 | 3 / 2 / 0 | +23 +/- 40 |
| 17 | 15 | scalar recovered | 181 | 147 | 147 | 1 / 4 / 0 | +19 +/- 53 |
| 19 | 32 | rank >= 29 | 459 | 459 | 459 | 2 / 3 / 0 | +4 +/- 9 |
| 19 | 32 | full rank | 640 | 640 | 640 | 1 / 4 / 0 | +23 +/- 65 |
| 19 | 32 | scalar recovered | 289 | 289 | 289 | 1 / 4 / 0 | +5 +/- 15 |
| 23 | 54 | rank >= 49 | 406 | 406 | 406 | 0 / 5 / 0 | 0 |
| 23 | 54 | full rank | 952 | 952 | 952 | 0 / 5 / 0 | 0 |
| 23 | 54 | scalar recovered | 349 | 344 | 344 | 3 / 2 / 0 | +3 +/- 4 |
| 29 | 62 | rank >= 56 | 410 | 310 | 310 | 5 / 0 / 0 | +97 +/- 20 |
| 29 | 62 | full rank | 423 | 346 | 386 | 5 / 0 / 0 | +55 +/- 24 |
| 29 | 62 | scalar recovered | 191 | 191 | 191 | 0 / 5 / 0 | 0 |
| 31 | 66 | rank >= 60 | 481 | 443 | 444 | 5 / 0 / 0 | +38 +/- 16 |
| 31 | 66 | full rank | 493 | 451 | 452 | 5 / 0 / 0 | +34 +/- 25 |
| 31 | 66 | scalar recovered | 493 | 447 | 452 | 4 / 1 / 0 | +32 +/- 29 |

Reading: the gain is real but is a **time shift**, not a different regime. At
n = 29 and 31 the orbit arms reach any rank target in every seed, 8-25% fewer
trials than the plain arm (n = 29, rank >= 56: 410 -> 310; full rank 423 -> 386
for (iii-post) and 346 for (iii); n = 31 full rank 493 -> 452). At n = 29 the
plain arm goes from 19.2 of 62 at trial 300 to 61.8 of 62 at trial 450 (means over
seeds); a cap of 300 sits in the steep part, which is why the yield at the cap
shows +169%. At n = 17 the shift is 8-12% with wide intervals, at n = 19 and 23
it is zero or within the spread. **Trials to recover the scalar**: no change at
n = 19 and 29 (the same trial in every seed at n = 29), 5 trials at n = 23, one
seed (221 -> 126) at n = 17, and 8% at n = 31 (4 of 5 seeds earlier); the scalar is pinned long
before full rank in several cells (e.g. n = 29 seed 5 at trial 26), so recovery
is a poor measure of the relation yield and a poor place to look for a
speedup at this scale. (iii) and (iii-post) differ by 0-6 trials for every
target except full rank at n = 29 (346 against 386), which is one seed (316
against 387).

## 4. Structure of the merges (finding D)

`orbit_window_structure report` (Rust; `results_addendum/structure.md`). The
large-point table is recomputed from its specification, independently of
`orbit::canonicalise` (brute-force pairwise Frobenius test on the curve,
union-find); the recomputed `L`, orbits and merged points equal the merged
receipts for all five curves (474/410/64, 490/478/12, 444/442/2, 444/340/104,
460/426/34).

**Mechanism.** For `x^31 + x^3 + 1`, `alpha^(2^5) = alpha^32 = x*x^31 = x^4 + x`;
for `x^29 + x^2 + 1`, `alpha^(2^5) = x^3(x^2 + 1) = x^5 + x^3` (both printed by the
example). The squaring map pi^5 therefore sends the low-degree prefix window
`x < 2^10` almost into itself, and so do its powers.

**Pair counts** (unordered pairs `{x0, x1}` in the large part of the window,
`x1 = x0^(2^m)`, `1 <= m < n`; 8 seeded random subspaces of the same dimensions
per n; `E` is the random-window expectation):

| n | modulus | prefix window | random windows (8) | E |
|---|---|---|---|---|
| 17 | `x^17+x^3+1` | 76 | 51-79 | 60.1 |
| 19 | `x^19+x^5+x^2+x+1` | 14 | 12-19 | 15.8 |
| 23 | `x^23+x^5+1` | 0 | 0-3 | 1.05 |
| 29 | `x^29+x^2+1` | 20 | 0 in all 8 | 0.02 |
| 31 | `x^31+x^3+1` | 112 | 0 in all 8 | 0.01 |

These are exactly the reviewer's prefix counts (76, 14, 0, 20, 112) and spot
checks (`256^(2^26) = 258`, `300^(2^12) = 558`); the random counts are consistent
with theirs (n = 17: 76 is inside the range of 8 random windows). n = 17, 19, 23
are at the random-window rate; n = 29 and 31 are not.

**Merges by cause** (bench modulus; `raw` = members that are Frobenius images of
each other on the raw abscissae, `projected only` = conjugate only after the
cofactor projection):

| n | merged | raw | projected only | E (random model, projected only) | largest orbit |
|---|---|---|---|---|---|
| 17 | 64 | 41 | 23 | 54.7 | 3 |
| 19 | 12 | 6 | 6 | 16.4 | 2 |
| 23 | 2 | 0 | 2 | 1.0 | 2 |
| 29 | 104 | 10 | 94 | 129.7 | 4 |
| 31 | 34 | **30** | **4** | 4.4 | **8** |

At n = 31 the 8-member orbit is `x = 256 + {0, 2, 4, 6, 16, 18, 20, 22}` (reviewer's
orbit, reproduced). The 4 projected-only merges match the model's 4.4; the 30
raw merges are structure. At n = 29 the structural share is 10 of 104 (a lower
bound on the structure, as the reviewer said), the other 94 are birthday
collisions of the projected points. At n = 17 the field is about twice the
subgroup (cofactor 2), so raw-level coincidences are as frequent as projected
ones (41 raw, 23 projected-only), at the random-window rate; the RESULTS model
`C(L,2)(2n-2)/(r-1)` coincides with the total there by size, not because it
describes the raw ones.

**Dense modulus** (same abstract curve, same `r`, over the seeded random
modulus of protocol section 6, `x^31 + x^27 + x^26 + x^24 + ... + x + 1`, 15 terms,
at n = 31): prefix pairs 0, large points 448, orbits 446, **merged 2** (0 raw, 2
projected-only; model 4.2). The structural merges vanish. For all n:

| n | merged, bench modulus | merged, random modulus | raw (random modulus) | prefix pairs (random modulus) |
|---|---|---|---|---|
| 17 | 64 | 49 | 22 | 50 |
| 19 | 12 | 11 | 7 | 12 |
| 23 | 2 | 0 | 0 | 0 |
| 29 | 104 | 103 | 0 | 0 |
| 31 | 34 | 2 | 0 | 0 |

**Consequence for the gains** (realisation series, section 5): on the random
modulus the n = 31 gain disappears (+0.7 +/- 2.9, S = 3) while the n = 29 gain
stays (+35.3 +/- 12.5). Corrected reading of the orbit gain: at n = 29 it is a
birthday effect of the projected points (cofactor 12646, L/r = 1%) and does not
depend on the representation; at n = 31 it is produced by the sparse modulus
`x^31 + x^3 + 1` (the alpha^32 = x^4 + x coincidence), i.e. by the choice of a
prefix window, not by a birthday effect; at n = 17 it is a mixture.

## 5. Replication unit (finding E i), realisation series

The five seeds of an n share one factor base and one target (`merged_points` is
identical across seeds), so the t intervals of `RESULTS.md` and section 2 cover
residual-stream variability for one base realisation; the replication unit
across bases is the five curves. A second realisation per n (same abstract
curve over a seeded random irreducible modulus, seeds 1-3, exploratory, one
repeat):

| n | merged | indep (ii) | indep (iii) | indep (iii-post) | `D_post` (S = 3) | verdict |
|---|---|---|---|---|---|---|
| 17 | 49 | 11.0 | 12.0 | 12.0 | +1.0 +/- 2.5 | no gain (unresolved) |
| 19 | 11 | 24.7 | 25.3 | 25.3 | +0.7 +/- 1.4 | no gain |
| 23 | 0 | 60.7 | 60.7 | 60.7 | +0.0 +/- 0.0 | no gain |
| 29 | 103 | 18.0 | 53.7 | 53.3 | +35.3 +/- 12.5 | survives |
| 31 | 2 | 55.3 | 56.0 | 56.0 | +0.7 +/- 2.9 | no gain |

The plain arm itself depends on the base: on seeds 1-3 of the bench modulus it
yields 5.7, 25.7, 51, 18, 44.3 independent relations at n = 17..31, on the random
modulus 11.0, 24.7, 60.7, 18.0, 55.3, a factor 1.9 at n = 17 and +19% and +25% at
n = 23 and 31, larger than the seed-to-seed standard deviation of one base (2.0
at n = 23 and 4.6 at n = 31 over seeds 1-3). A claim about
"this n" needs more than one base; this addendum has two per n and S = 3 for the
second, which is a caveat on the section 4 consequence and not a replacement for
a design with many bases.

## 6. CPU per relation and set-up (finding E iii)

From `results_addendum/tables.md`, Table 2 (primary series, process CPU, ms,
shared VM): at n = 31 collect is 9510 / 9351 / 9519 for (ii) / (iii) / (iii-post),
filter 0.59 / 0.75 / 0.68 and replay 1.06 / 1.54 / 1.42. CPU per independent
relation is therefore `collect / indep`, and its "outside noise" verdicts in
`RESULTS.md` repeat the independent-relation counts. This addendum ranks nothing
by CPU per relation.

Set-up CPU (factor base, orbit table, combination table) in the primary series
is 180 / 173 / 175 ms at n = 17, 265 / 244 / 255 at n = 19, 263 / 263 / 261 at
n = 23, 372 / 389 / 382 at n = 29 and 441 / 430 / 440 at n = 31 for (ii) / (iii) /
(iii-post); in the merged receipts (all cells) the (iii) - (ii) means are +0.4,
+12.1, -10.4, -10.6, -0.9 ms. The orbit work (`LambdaPowers` certification, one
canonicalisation and one self-check scalar multiplication per large point, and
now the small-column and absorb self-checks) is not in `phase_group_ops`
(`factor_base`: 1053 scalar multiplications for every arm, as the review says);
its cost is below the +/-5-8% spread of the set-up measurement, so the effect on
set-up is small at this scale, and "identical" should read "not resolved".
(iii-post builds a second, plain base for acceptance and counts it under
`factor_base_plain`.)

## 7. Tests and tooling (findings A, B, F)

**A. Golden test.** Verified by reading `src/build_provenance.rs` (`option_env!`
of `CRYPTO_BUILD_GIT_COMMIT` / `GITHUB_SHA` at compile time) and by running it:
an `ic` built with `GITHUB_SHA=deadbeefdeadbeefdeadbeefdeadbeefdeadbeef`
reports `"git_commit":"deadbeef...","git_commit_source":"GITHUB_SHA"`, and the
merged comparison (`jq -S -c 'del(.manifest) | del(.software.binary_blake3) | .run.report.solve_wall_ns = 0'`
against the merged golden file) differs on exactly those two lines. The fixed
test drops the whole `software` object (and `manifest`, and the wall time) and
compares everything else; the fixture lost its `software` object. With the
env-built binary the fixed test passes (`cargo test --release --test
ecbench_large_prime orbit_mode_off_reproduces_the_recorded_fixture_report_byte_for_byte`
with `GITHUB_SHA` exported at build time, 1 passed), and the same `jq` normalisation
with `del(.software)` equals the new golden byte for byte. The reproduction
before the fix was done on the binary output with the old normalisation, not by
running the old test source.

**B. Merges and absorptions.** `tests/ecbench_large_prime.rs`, on the n = 17 K_1
cell (small 5, envelope 10, 3 summands, cap 200, seed 1; 474 large points):
the library's table (`large_points`, `large_orbits`, `merged_points`,
`absorbed_into_small`) equals an independent brute-force recount
(`tests/common/orbit_support.rs`: canonical projected points from the
specification, every pair tested with `frobenius_k` and `neg`, union-find) with
**merged 64 > 0** and **absorbable 19 > 0** (absorb arm: 474 = 398 orbits + 57
merged + 19 absorbed); the identity `large = orbits + merged + absorbed` is no
longer the only check. Every emitted relation of every arm passes both replays
and every input row passes its audit. On the identical stream (post-collection
vs plain, same digest) the quotient strictly adds complete relations (8 vs 5 at
this cell). Negative tests: with `SolveOptions::table_fault` (a `#[doc(hidden)]`
hook that damages the finished table after its self-checks) multiplying an
absorb factor by 2, or a merged member's factor by lambda, leaves the linear
replay passing and makes the audit and the curve replay fail, while a recovered
scalar stays correct. The factor damaged for *every* member of an orbit by the
same lambda is not detected by complete relations, which is correct (it is a
rescaling of one unknown); the test damages one member (`Option` index chosen
by exploration: 19 absorbed points and 61 merged orbits, most of which this
short stream never uses). The small-column factors and the combination
`f_Q f_S^-1` are now curve-checked when the table is built, so the
documentation is corrected rather than weakened.

**F.** `run_all.sh`: without `pipefail` the `|| echo FAILED` after `... | tail -1`
never fired (shown with `false | tail -1 || echo FAILED`, which prints nothing);
`set -o pipefail` added, no recorded result is affected (all 250 cells present).
The summarizer's integrity fields are required: `orbit_large_prime_bench summarize`
reproduces `results/` byte for byte on the merged receipts and exits with
`missing or non-integer field` when `replay_failed_curve` is removed from a
receipt. `ic large-prime --help` now says that the filter modes run every trial
and do not stop at recovery (`--max-trials` default 100000). `LambdaPowers::new`
rejects `r >= 2^63` with a clear error (dead in practice since n <= 62; tested by
setting `r` on a built instance).

## 8. What this addendum verified, and how

* `cargo test --release --locked --test ecbench_large_prime`: 14 passed (the 8
  merged tests, with the golden test fixed, plus 6 new).
* rustfmt clean on every touched file; clippy: see the PR description.
* 255 cells, 75301 verified complete relations, 0 replay failures, 0 audit
  failures; 232 recovered scalars equal the planted log, 0 disagree.
* Compute: primary 3 min 30 s, trajectory 8 min 9 s, realisation 2 min 38 s of
  process CPU.

## 9. Not measured, not established

* **Frobenius-closed factor bases, on purpose.** Every base here is a prefix
  subspace. A base closed under Frobenius (each large prime with n orbit mates)
  is a different regime and is not measured; neither is a base chosen to avoid
  the structure of section 4.
* **Matched rho.** No Pollard rho (or `rho.signed_frobenius`) run, so ledger
  hypothesis H-BINSTD-555991 is still untested. Nothing here is a speedup
  claim, and the scalar is recovered no sooner at n = 29 (section 3).
* One extra base per n, three seeds (section 5); a design with many bases would
  be needed for a statement about a distribution of bases.
* Arms (i) and (iv) were not re-run. The cap-free metric is limited to a run
  length of 3-4 caps. Timing is diagnostic single-threaded process CPU on a
  shared 4-core VM.
* No `report.pdf`: no PDF engine was found for the merged PR either; the chart
  is `results_addendum/chart.svg`.
