# L12: bielliptic quartics, involution-orbit factor base and elliptic push-forward check (frozen protocol, 2026-11-16)

Status: protocol only. No datum of this study existed when this file was written.
Every number this study will produce is `[pcc:unverified]` (one session, shared
4-core container, Python instrument, no independent replay). Lab numbers quoted
here are `[pcc:unverified]` assertions. This is not a benchmark: no leaderboard
row, no `benchmarks/` row, no speed claim. Cost is exact operation counts, never
wall time.

Implementation contract: [`SPEC.md`](SPEC.md). Freeze: `frozen.sha256` (sha256
of this file and SPEC.md), written before any code or data. A change after the
freeze is an amendment: a new dated section at the end, a new `frozen.sha256`,
and the reason; thresholds in section 9 are never changed after data exist.

## 0. Prior art checked (grep; the research-kb tool is not installed, so no `source_id` exists and no novelty claim is made)

| path | what it shows |
|---|---|
| `crypto:src/cryptanalysis/bielliptic_quartic.rs` (read lines 1-560) and `crypto:research/bielliptic_quartic_20261005/README.md`, `cryptanalysis:experiments/bielliptic-quartic/README.md` | The lab model `C: v^4 = x^3+ax+b -> E: y^2 = x^3+ax+b`, line sections certified by full section reconstruction and an elliptic norm replay (`sum of images = O`). Linear algebra is done "AFTER the elliptic norm projection" and "its columns fold elliptic negation and repeated norm images". `p <= 257`. No cost comparison. |
| `crypto:src/cryptanalysis/chained_transfer/quartic_ic.rs` (lines 1-80, 815-900) | Counted copy for `p <= 2^20` (case O4). `FactorBase::build`/`fold`: one column per abscissa `x` (the whole `x`-fibre of `C`, i.e. sigma-orbits and the elliptic sign folded together), sign `+-1`. This is arm C below. |
| `crypto:research/chained_transfer_next_20261115/{NOTE.md,PROGRESS.md}` | O4 protocol frozen and code written; PROGRESS.md "Not done: The run itself". No results file exists in that directory. O4 compares the quartic source with a direct elliptic source and rho; it never compares an unfolded base with a folded one. |
| `crypto:research/cover_pushforward_20261112/PUSHFORWARD.md` | Genus-2 bielliptic covers: push-forward of relations to `E` checked on 37,866 relations; there the relations are fibres and the IC is a collision search on `E`. Hyperelliptic, not plane quartics. |
| `crypto:research/full_jacobian_ic_20261113/PROTOCOL.md` | O3: genus-2 IC on all of `J_C` with the `r`-part projection `h D`; the cofactor-projection convention reused in section 2. |
| `crypto:research/leads_20261111/EXISTING_LEADS.txt` | `LEAD-ICB-frobenius-orbit-factor-base`, `LEAD-ICB-orbit-linear-algebra`, automorphism rho leads: names only, elliptic/Koblitz framed. |
| `crypto:research/g2plus_leads_20261116/L02_automorphism_orbit_bases/PROTOCOL.md` | Sibling lead in this pass: automorphism-orbit bases on CM hyperelliptic families `y^2 = x^q + a`. Different curves (hyperelliptic, `mu_2q`), no overlap in instances or code. |
| `crypto:research/g2plus_leads_20261116/L07_prym_ic_desk_count/PROTOCOL.md` (head) | Desk count of Prym IC in genus 4-5; no plane-quartic involution base. |
| `crypto:research/genus_ladder_*`, `crypto:research/plane_quartic_program_20261018/` (listing), `crypto:research/notes/index-calculus/` (grep `bielliptic|involution`: hits in COVER_DECOMPOSITION_LEDGER, GLV_INVARIANT_FACTOR_BASES, HYPERELLIPTIC_IC_RHO; none is an orbit-base measurement on a quartic) | No executed unfolded-versus-folded comparison on bielliptic quartics. |

Verdict. Not a duplicate of executed work: the comparison A (unfolded) versus
B (sigma-orbit) has not been run anywhere in either repository, and the lab's
O4 run itself is not done. But the mechanism is not new to the lab: the lab's
existing bielliptic code already uses a base that is folded *more* than the
lead's orbit base (arm C). Consequence, stated before data: a "supported"
verdict for B against A quantifies a symmetry the lab baseline already
exploits; it is **not** an improvement over the lab's index-calculus baseline.
The distance from the lead's arm B to the lab baseline is measured as
`TOTAL_B / TOTAL_C` and reported.

## 1. Mathematics, checked and corrected

Notation. `p` odd prime. Family F1:
`C: y^4 + h(x) y^2 = f(x)`, `h = a1 x + a3`, `f = x^3 + a2 x^2 + a4 x + a6`;
projectively `F = Y^4 + (a1 X Z + a3 Z^2) Y^2 - (X^3 Z + a2 X^2 Z^2 + a4 X Z^3 + a6 Z^4)`.
`sigma(x, y) = (x, -y)`. `pi: C -> E`, `pi(x, y) = (x, y^2)`,
`E: w^2 + (a1 x + a3) w = x^3 + a2 x^2 + a4 x + a6` (long Weierstrass),
`pi(inf_C) = O_E` with `inf_C = (1:0:0)`. The lab model is the subfamily
F0: `a1 = a3 = a2 = 0`. The brief's form `y^4 + h2(x) y^2 + h4(x) = 0` is the
general bielliptic quartic; F1 is the case with a rational ramification point
of `pi` placed at `(1:0:0)` (`deg h2 <= 1`, `deg h4 = 3`). The restriction is a
choice of model for this study, not a claim about all bielliptic quartics.

Checked statements (each re-derived here; checked numerically at `p = 23` in
SPEC.md section 10, otherwise `[pcc:unverified]`):

1. **Smoothness and genus.** `(1:0:0)` lies on `C` and is smooth (`dF/dZ = -1`
   there). The line `Z = 0` meets `C` only there, with multiplicity 4 (a
   hyperflex). An affine singular point needs `y = 0, f = f' = 0` or
   `2y^2 = -h` with `g = f + h^2/4` having a double root. So `C` is smooth iff
   `disc(f) != 0` and `Delta(E) != 0`. The fixed points of `sigma` are
   `(1:0:0)` and the three points `y = 0, f(x) = 0` (over the closure): 4
   branch points, so Riemann-Hurwitz gives `2g - 2 = 2(0) + 4`, `g = 3`. **Correction to the brief:** orbits have size 2 *except* at the ramification
   points (`y = 0`), which are fixed by `sigma` (0 to 3 affine rational ones).
2. **Line relations and the push-forward gate.** For a line `l` not equal to
   `Z = 0`, `div(l/Z) = P1 + P2 + P3 + P4 - 4 inf_C` (the four intersections
   with multiplicity, `inf_C` included when `l` passes through it, i.e. the
   horizontal lines `y = c`). `pi_* div(l/Z) = div(Nm(l/Z))` is principal on
   `E`, and `pi_*(4 inf_C) = 4 O_E`, so `sum_i pi(P_i) = O_E` in the group law,
   for every line section. **Correction to the brief:** the "fixed point" is
   `O_E` exactly in this model (because `Z = 0` is a hyperflex at a
   ramification point); in the general model `y^4 + h2 y^2 + h4 = 0` with
   `deg h4 = 4` it is `S_inf = sum pi(Z = 0 section)`, a fixed but in general
   nonzero point, and the relations are then inhomogeneous.
3. **How sigma acts on the target subgroup, and how to tell from the group order.**
   `J(C)` is isogenous to `E x P` with `P = Prym(C/E) = (ker(1 + sigma))^0`
   (dimension 2). `pi^*: E -> J(C)` is injective because `pi` is ramified, and
   `pi^* pi_* = 1 + sigma`, `pi_* pi^* = [2]`. For an odd prime `r` the
   `r`-primary part splits as `J[r^inf](F_p) = (+1 eigenspace) + (-1 eigenspace)`
   with idempotents `(1 + sigma)/2`, `(1 - sigma)/2`; the `+1` part is
   `pi^*(E(F_p)[r^inf])` and the `-1` part is `P(F_p)[r^inf]`. Isogenous
   abelian varieties have equal point counts, so `#J(F_p) = #E(F_p) * #P(F_p)`.
   Hence: if `r | #E(F_p)` and `r` does not divide `#P(F_p) = #J(F_p)/#E(F_p)`,
   the `r`-part is in the `+1` part and sigma acts as `+1`; if `r | #P(F_p)` and
   `r` does not divide `#E(F_p)`, sigma acts as `-1`; if `r` divides both, the
   `r`-part is not cyclic and contains both eigenspaces (a generic element has
   both components, separated by the idempotents). `#P(F_p) = L_P(1)`, with
   `L_P(T) = 1 - s1 T + s2 T^2 - p s1 T^3 + p^2 T^4`,
   `s1 = t1(C) - t1(E)`, `s1^2 - 2 s2 = t2(C) - t2(E)`,
   `t_k(X) = p^k + 1 - #X(F_{p^k})`: two point counts of `C` and `E`, over `F_p`
   and `F_{p^2}`, decide the case (SPEC.md section 4).
4. **Rewriting a relation in the orbit representative.** Let `e_r` be the
   projection to the `r`-part and `l(P) = log e_r [P - inf_C]`. Because the base
   point `inf_C` is fixed by sigma, `[sigma P - inf_C] = sigma [P - inf_C]`, so
   `l(sigma P) = +l(P)` on the `+1` part and `-l(P)` on the `-1` part. (With a
   base point not fixed by sigma there is an extra constant
   `[sigma P0 - P0]`; the brief omits this.) On the `+1` part,
   `2 e_r[P - inf_C] = pi^*(e_r[pi(P) - O_E])`, so the unknown of the orbit
   `{P, sigma P}` is (half) the elliptic log of the image `pi(P)`: the orbit base
   *is* the elliptic factor base `B = pi(C(F_p)) subset E(F_p)`, and the orbit
   index calculus with the target in the `+1` part is an index calculus on `E`
   with relations from quartic line sections. This is exactly the lab's "norm
   projection". On the `-1` part the push-forward kills everything
   (`pi_* = 0` there), so the gate of item 2 still holds but says nothing about
   the logs; this study does not measure the `-1` case (no Jacobian arithmetic
   for plane quartics in the standard library; out of scope, stated as such).
5. **How the factor base halves.** Plain columns: `N_aff` affine rational points
   (`inf_C` has log 0 and is not a column). Orbit columns:
   `n_B = (N_aff + N_ram)/2`, `N_ram` = affine rational ramification points
   (`0 <= N_ram <= 3`). The elliptic sign fold (arm C) also merges `Q` with
   `-Q` when both are images: in F1, `Q = (x, w)` is an image iff `w` is a
   square, and `-Q = (x, -w - h(x))` independently so, giving `n_C ~ 3 n_B / 4`;
   in F0 with `p = 1 (mod 4)`, the automorphism `tau(x, y) = (x, i y)` lifts the
   sign (`pi tau = [-1] pi`), every `x`-fibre has 4 points, and `n_C ~ n_B / 2`.
6. **Relations, duplicates, linear algebra.** A line `L` and `sigma L` (`y = mx + c`
   and `y = -mx - c`) give the *same* orbit row; in the plain base they give
   two different rows. With random secants and only `~n` relations needed out
   of `~p^2/24` split lines, such pairs are rare, so the relation yield per trial
   is the same in both bases and the relations needed scale with the column
   count: prediction relations `/2`. Sparse Wiedemann costs
   `~3 n` sparse matrix-vector products of weight `~2W + R` with `W ~ 4R` and
   `R ~ n`, i.e. `~ n^2`: prediction counted LA `/4` (dense Gauss would be `/8`).
   The brief's predictions are therefore correct for the sparse solver used
   here, with the two corrections of items 1 and 2.
7. **Why the unfolded base can never finish when `r | #P`.** Line relations hold in
   `J(C)`, so they span at most the kernel of `(Z/r)^n -> J(F_p)/r`. If the
   `r`-part of `J` has rank 2 (r divides both orders), the plain system keeps a
   2-dimensional kernel forever, while the orbit system (which imposes
   `x_P = x_sigmaP`, true on `E`) does not. Instances with `r | #P(F_p)` are
   therefore excluded by the selection gate (section 6), not left to stall.

## 2. Hypothesis and exact claim

H1 (the lead's prediction): on F1 bielliptic quartics, with the same line stream,
the same stop rule and the same counted sparse linear algebra, the sigma-orbit
base (B) reduces the counted total of the factor-base-log precomputation
(`TOTAL = SETUP + COLLECT + LA`, in FPM) by at least 1.5x against the plain base
(A); predicted ratios: columns `~2`, relations `~2`, collection FPM `~2`, LA `~4`,
total `~3` to `4` (LA dominates at every `p` here).

Exact claim the data can support if H1 holds: "On the frozen F1 instances at
`p in {263, 523, 1031, 2063}`, the median ratio `TOTAL_A / TOTAL_B` is X at each
`p` (counted FPM, LA priced at 1 FPM per mod-`r` multiplication), all relations
passed the push-forward gate and all logs were certified on `E`." Nothing
about large `p`, about the `-1` (Prym) case, about the individual-log phase,
or about the lab baseline beyond the reported `TOTAL_B / TOTAL_C`.

## 3. Arms

All arms run the same code; only the column map differs (SPEC.md section 6).

| arm | role | columns |
|---|---|---|
| A | baseline of the lead (plain base) | one per affine rational point of `C` |
| B | treatment (sigma-orbit base) | one per sigma-orbit = one per image `pi(P) in E` |
| C | lab-baseline reference (fold of `bielliptic_quartic.rs` / `quartic_ic.rs`) | one per class `{Q, -Q}` of images, sign `+-1` |
| B' | A/A control | arm B with an independent stream seed (`s + 100`) |
| N | negative control (fake orbits) | arm A columns paired by a random permutation, both members coefficient `+1`; expected: never certifies |
| PC | positive control (dose response) | arms A, B, C on F0 with `p = 1 (mod 4)`: fold sizes 1, 2, 4 |
| G | family control (generic plane quartic, no involution) | collection only: relation yield per trial, row weight |
| R | rho reference (orientation only, no decision) | Pollard rho on `E` for `[k]G = T`, counted FPM |

## 4. Variables held fixed

Field arithmetic code and its counters; the section algorithm (schoolbook
polynomial products, synthetic division, square roots by a precomputed table);
the line stream (pairs `(i, j)` over the sorted affine point list: `i = t mod N_aff`,
`j` uniform from the stream RNG; identical for arms A, B, C, N on the same
instance and seed, checked by a digest); the stop rule (rows `>= ncols + 20` and
every column covered, batches of `ceil(0.05 ncols) + 1` rows on retry, at most
12 LA attempts, at most `40 N_aff` trials); the LA algorithm (scalar Wiedemann
on `A^T D A`, Berlekamp-Massey, Horner solution, exact verification); the
anchor rule (first point in sorted order whose image `Q` has `[c]Q != O`);
the LA seed rule; Python 3 version (recorded); 1 thread.

## 5. Unit of cost

FPM = `MUL + 10 INV` counted by instrumented counters on `F_p` (squarings and
products by curve coefficients count as multiplications; products by the
integer literals 2, 3, 4 are done with additions and are not counted).
Linear algebra over `Z/r` is counted the same way (`MR + 10 IR`, one `MR` per
multiply-accumulate of a matrix entry, whatever its value). `r <= p + 1 + 2 sqrt p`,
so both are one-word operations; `LA_FPM` is reported separately and the decision
uses weight 1. Additions and table lookups are counted and reported, not priced.
Verification work (gates, certification) uses a separate counter and is not charged.

## 6. Instances

Primes. F1: `263, 523, 1031, 2063` (all `= 3 mod 4`; on F1 this choice is
immaterial to the arms and makes the F0 sub-case below disjoint). PC (F0):
`269, 521, 1033, 2069` (all `= 1 mod 4`). G: `263, 523, 1031, 2063`.

Curve generation (SPEC.md section 5). Curve seed `cs = 1000003 p + k` (F1),
`2000003 p + k` (F0), `3000003 p + k` (G), `k = 0, 1, 2, ...`; coefficients from
`random.Random(cs)` in a fixed order. F1 admissible iff `(a1, a3) != (0, 0)`,
`disc(f) != 0`, `Delta(E) != 0`, `c4 != 0`, `c6 != 0`; `n_E = #E(F_p)` has
largest prime factor `r >= 5` with `r^2` not dividing `n_E` and cofactor
`c = n_E / r <= 16`; `N_aff >= 8`; the `F_{p^2}` count gates pass; `s1^2 - T2`
even; `#P(F_p)` inside the Weil interval; `r` does not divide `#P(F_p)`. F0: same
with `a1 = a3 = a2 = 0`, `a4 != 0`, `a6 != 0`. Take the first 4 admissible `k`
per prime for F1, the first 2 for F0, the first 2 for G (G: no rational
singular point, `N_aff >= 8`, coefficient of `y^4` equal to 1). Every rejected
`k` is counted by reason.

Seeds. Algorithm seeds `s in {1, 2}`; stream seed `ss = 7919 cs + s`; LA seed
`ss + 50000 + 1000 * attempt`; N pairing seed `ss + 90000`; A/A uses `s + 100`.
Rho: target seed `cs + 777`, walk seeds `cs + 900 + j`, `j = 0..4`.

Run matrix: F1 `4 p x 4 curves x 2 s x {A, B, C, B'}` = 128 runs; N at
`p in {263, 523}`, 4 curves, `s = 1` = 8 runs; PC `4 p x 2 curves x 2 s x {A, B, C}`
= 48 runs; G `4 p x 2 curves x 1 s` = 8 runs; R 5 seeds per F1 curve = 80 runs.
Each run is limited to 600 CPU-seconds (`ulimit -t 600`) and 2 GB
(`ulimit -v 2000000`); a killed run is recorded as `not_run` and never
extrapolated. Expected largest run: arm A at `p = 2069` (about `1.3e8` counted
`MR`, minutes in Python).

## 7. Metrics (per run)

`ncols`, `N_aff`, `N_ram`; `trials`, `relations` (distinct nonzero rows kept),
`duplicate_rows`, `zero_rows`, `nonsplit`; `rows_used` at the successful solve;
`la_attempts`; matrix `R x N` and weight `W` at the successful solve;
`SETUP_FPM`, `COLLECT_FPM`, `LA_FPM` (with `MUL, INV, MR, IR` separately),
`TOTAL = SETUP + COLLECT + LA`; certification result; for arm A the fraction of
sigma-pairs with equal solved logs; stream digest. Ratios per
`(p, curve, s)`: `TOTAL_A/TOTAL_B`, `COLLECT_A/COLLECT_B`, `LA_A/LA_B`,
`rel_A/rel_B`, `ncols_A/ncols_B`, `TOTAL_B/TOTAL_C`, A/A `TOTAL_B/TOTAL_B'`.
Per `p`: median, min, max over the 8 pairs. Rho: median FPM over 5 seeds; gap
`TOTAL_X / RHO` reported as distance remaining, labelled a lower bound on the IC
gap (the IC arms do factor-base logs only, no individual log).

## 8. Correctness gates (all must pass before any metric is read)

- G0 unit tests: `python3 -m unittest` passes, including the hand values of SPEC.md section 10.
- G1 point counts: point enumeration count equals the character-sum count over
  `F_p`; the `F_{p^2}` count of `E` equals `p^2 + 1 - (t1(E)^2 - 2p)`;
  `s1^2 - T2` even; `#P` in the Weil interval; `r` does not divide `#P`.
- G2 section certificate, every relation of every arm: every point lies on `C`
  and on the line; the product `lead * prod (X - X_i)^{m_i}` equals the section
  polynomial; total multiplicity including `inf_C` is 4.
- G3 push-forward (the brief's gate), every relation: `sum m_i pi(P_i) = O_E`,
  recomputed by the independent verifier. One failure kills the lead (section 9).
- G4 tamper: in each arm-A run at `p = 263`, replace one point of the first
  relation by another rational point; G2 or G3 must reject it.
- G5 common stream: the digest of the first 1000 pairs of the stream (generated by
  the stream rule from the stream seed, SPEC.md 8.2) is equal for arms A, B, C, N on
  the same `(instance, s)`.
- G6 certification: for every column `j`, `[x_j]([c] Q_j0) == [c] Q_j` on `E`
  (`x_j0 = 1`). A, B, C, B', PC runs must certify; arm A must have all
  sigma-pair logs equal (the measured sigma = +1 check).
- G7 negative control: N must certify in 0 of 8 runs.
- G8 determinism: rerun of one arm-B run at `p = 523` gives an identical JSON
  except the `wall_s` field.

If G0, G1, G2, G4, G5, G6 or G8 fails, the study is **invalid** (instrument
error), not killed: fix, amend, rerun everything. G3 failing on a relation that
passes G2 kills the lead as the brief specifies. G7 failing makes the study invalid.

## 9. Decision rule (frozen; thresholds never moved)

Let `R_p` = median over the 8 `(curve, s)` pairs of `TOTAL_A / TOTAL_B` on F1.
Let `AA_p` = the 8 values `TOTAL_B / TOTAL_B'`.

- **Killed** if any G3 failure; or `R_2063 < 1.5`; or arm B fails G6 in any run
  where arm A certified.
- **Supported** if all gates pass, `R_p >= 1.5` at every prime with complete data,
  `R_2063 >= 1.5` with at least 7 of the 8 values at 2063 `>= 1.5`, the positive
  control passes, and `max(AA_2063)/min(AA_2063) <= 1.5`.
- **Inconclusive** otherwise (for example `R_2063 >= 1.5` with fewer than 7 of 8,
  the 2063 cell `not_run`, the positive control failing, or A/A spread above 1.5).

Positive control passes iff at `p = 2069` the median of `TOTAL_A / TOTAL_C` is
`>= 6` and the median of `TOTAL_A / TOTAL_B` is `>= 2` (predicted about 16 and 4).
If it fails, the instrument has not been shown to see a fold, and the verdict
is inconclusive.

Family control (reported, does not change the verdict): "no family effect" if
the pooled relation yield per trial differs between F1 (arm-A stream) and G by
at most 0.05 at every prime; otherwise the difference is reported as a
confounder of the yield, not of the A/B ratio (same family on both arms).

Lab-baseline reading (reported, not a decision): median `TOTAL_B / TOTAL_C` per
prime. Predicted `~1.5` to `2` on F1 (arm B is *worse* than the lab's fold).
Whatever H1's verdict, the report states that a supported H1 is not an
improvement over the lab's IC baseline.

## 10. Confounders and how each is isolated

| confounder | isolation |
|---|---|
| different line streams between arms | one stream definition independent of the arm; digest gate G5 (common random numbers) |
| stop rule favouring smaller bases | same rule (`ncols + 20`, coverage, batch `5%`) for every arm; rows and attempts reported |
| LA randomness (Wiedemann failure, retries) | same seeds rule; every attempt charged; attempts reported; A/A arm |
| bielliptic family special for split probability | generic-quartic control G; same yield definition |
| extra automorphisms (F0 `tau`, `j = 0, 1728`) | F1 excludes `c4 = 0`, `c6 = 0`, uses `h != 0`; the `tau` effect is isolated in PC only |
| `r` dividing `#P` (stalling plain system) | excluded by gate G1 at selection |
| cost unit mismatch between `F_p` and `Z/r` | both one word; LA reported separately; LA_FPM and COLLECT_FPM both in the table so the reader can reprice |
| implementation shortcuts in one arm | one code path; arms differ only in `column_map`; unit tests compare arms on a toy |
| analyst bias | thresholds frozen here; `analyze.py` computes the verdict mechanically from the JSON |
| wall time, host load | not a metric; counts are deterministic (G8) |

## 11. Exact commands (executor runs these from the lead directory, unchanged)

```sh
cd /home/user/crypto/research/g2plus_leads_20261116/L12_bielliptic_quartic_orbits
python3 --version > raw/python_version.txt 2>&1
python3 -m unittest discover -s code -p 'test_*.py' -v > raw/unittest.log 2>&1
python3 code/select_instances.py --out raw/instances.jsonl --rejects raw/rejects.tsv > raw/select.log 2>&1
python3 code/run_matrix.py --instances raw/instances.jsonl --outdir raw/runs --log raw/run_matrix.log
python3 code/analyze.py --instances raw/instances.jsonl --runs raw/runs --summary raw/summary.tsv --decision raw/decision.json > raw/analyze.log 2>&1
sha256sum raw/instances.jsonl raw/summary.tsv raw/decision.json raw/runs/*.json > raw/outputs.sha256
```

`run_matrix.py` launches each run as
`bash -c 'ulimit -t 600 -v 2000000; python3 code/ic_run.py ...'` (and
`rho_run.py`, `generic_run.py`), in the order of SPEC.md section 9, one at a
time. If a repository hook refuses any command (for example the resume
checkpoint or benchmark-record hooks), the executor records the refusal verbatim
in `raw/HOOK_REFUSALS.txt`, does not work around it, and continues with what
remains possible.

## 12. What a result here does not establish

No speed claim, no rho comparison verdict (rho is orientation only), nothing at
cryptographic sizes, nothing on the Prym (`-1`) case, nothing on the
individual-logarithm phase, no improvement over the lab's existing bielliptic
baseline (arm C).
