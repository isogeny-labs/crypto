> Red-team lane report (2026-11-14). Desk review only: no experiment run, no evidence edited, no status changed, no lead
> declared supported or refuted. Every statement is an unverified assertion [pcc:unverified].

# Red-team review of the wave-5 Curve25519 lead set

**Evidence reviewed.** `research/curve25519_leads_wave5_20261112/` at `98cd2fd1f8` (text unchanged at `fc4dbd3a54`; only
`sd_law.png` moved to the artifact store); the cells in `research/curve25519_exponent_study_20261108/cells` and
`research/curve25519_exponent_study_ext_20261110/cells`; source at `98cd2fd1f8` (`src/cryptanalysis/ic_boundary.rs`,
`ic_framework/plugins.rs`, `ic_framework/subgroup_semaev.rs`, `f4_fp.rs`) and `src/bin/curve25519_family.rs`. Checks are
stdlib Python over committed JSON, with the formulas inline so anyone can redo them; none is a measurement. research-kb
and ecdb were not reachable from this runtime, so there are no `source_id` receipts. Severity: *blocks* (the headline
cannot stand as written), *weakens* (prediction or framing must change before a task card), *cosmetic*. Kind: *wrong*,
*unsupported*, *unpriced* [pcc:unverified].

## Index, by severity

| id | lead | severity | kind | finding |
|:--|:--|:--|:--|:--|
| R1 | SYN-3 | blocks | wrong | the "8 of 8 preregistered, not read" values are in a table the lane says it read |
| R2 | SYN-3, README §1 | blocks | wrong | "SD ≥ 65,148 in every presentation" holds by construction for chain and flat, fails for other μ_d encodings |
| R3 | SYN-3 | blocks (as a cost law) | unsupported | SD does not predict cost: R² = 0.09 over 60 cells; T differs 8.0e4× at equal SD |
| R4 | PL-1 | blocks (k = 20 sub-claim) | wrong | no k = 20 `inc-lazy` exists; the k = 20 incumbent (wsum box) already stores its set-up |
| R5 | RP-4 | blocks | wrong | the proposed Faugère–Lachartre split is already in `f4_fp` |
| R6 | SYN-3 | weakens | wrong | "SD" is the highest F4 pair degree; plain form gives d + 2; a k = 18 cell breaks the m = 2 band |
| R7 | PL-1 | weakens | unsupported | the WalkTable walks T = x·√r steps unconditionally, so ρ_s ≈ 0.73 at k = 24 by the lead's own model |
| R8 | PL-1 | weakens | unpriced | `tw2cj-lazy` has no owner IC classification; a rho-neg-rec reference may widen the w = 0 distance |
| R9 | PL-3 | weakens | wrong (undercount) | tw-lazy arms run the [d]G = Q check twice; the lead prices one |
| R10 | SYN-1 | weakens | unsupported | improves no existing arm; its sign conflicts with the T₀ study's 2× gain at fixed columns |
| R11 | RP-1 | weakens | unpriced | chain is the reference; msolve not named; degree cap blocks d ≥ 400; d = 73, m = 3 memory unsupported |
| R12 | README §1 | weakens | wrong | prime-d law pre-existed (exponent study, E6); "after all four, k ≥ 28" rests on PL-2 alone |
| R13 | README §5 | cosmetic | — | line citations hold only at `98cd2fd1f8` (`ic_boundary.rs` has gained 670 lines since) |

## 1. W5-SYN-3, solving degree = largest prime factor of d plus a small constant

**Claim.** SD = e_max(d) + c; SD = d + 1 at prime d in 20 of 20 sym cells; eight preregistered k = 22, m = 3 values match;
hence SD ≥ 65,148 for every μ_d base of Curve25519, "in every presentation" (synthesis lane) or "on that route" (README
§1) [pcc:unverified].

**1a. What the quantity is (R6).** `F4Report.solving_degree_max` is "the highest step degree at which the run learned
something new". In all 73 complete cells (sym and plain) it equals `degree_reached`, the highest pair degree processed, on
every target; the step at which 1 entered the ideal (`solving_degree`) is 2 to 6. So "SD" is how high F4's normal strategy
climbs, not a degree of regularity or the degree of the inconsistency certificate. The cap of 400 does not bind: a target
is complete only with `pairs_above_bound == 0` (`curve25519_family.rs` near l. 1684), and the largest recorded SD is 91.
The order (grevlex, chain variables first; PROTOCOL §2) was never varied, so order dependence is untested
[pcc:unverified].

**1b. Construction, not structure.** In every complete sym cell, SD minus the cell's recorded `input_degree` lies in [0,
5]; it is 1 in all 20 prime cells except the two m = 3, d = 3 cells (0). The input degree is e_max by construction (the
generator y^{e_max} − y′), or the Semaev sym degree 2^(m−1) when larger. That is why SD − e_max rises to 4–5 at e_max = 3
(k = 20, m = 3, d = 27: input degree 4, SD 8). A run that handles a degree-d generator reaches pair degree at least d, and
that generator's first pair with a leading monomial y_i·v has degree d + 1, so at prime d the law carries at most one
degree of content. Cheapest check: dump the F4 step trace of one k = 20, m = 2, d = 19 sym target; if the last productive
step at degree 20 is a binomial–quadric pair, the prime-d law is bookkeeping [pcc:unverified].

**1c. Form dependence and an excluded cell (R6).** The plain form gives SD = 5, 9, 21 at m = 2 (d = 3, 7, 19: d + 2) and
12, 15, 27 at m = 3. The lane's mechanism (top system (y1^d, y2^d, y1²y2²), regularity d + 1, Caminata–Gorla) is stated
for the plain form at m = 2, where it caps SD at d + 1; all three plain m = 2 cells exceed it, so the recorded quantity is
not the solving degree that bound governs. The check script globs `k2*_m[23]_sym_*`, excluding k = 18, yet
`k18_m2_sym_d923_g0.json` (923 = 13·71) has SD 75 = e_max + 4, outside README §1's "e_max + 2 to 3 at m = 2", and SYN-3
itself names the k = 18 tables as test data [pcc:unverified].

**1d. The out-of-sample claim (R1).** The main cell table of `research/curve25519_exponent_study_20261108/README.md`
(lines 152–188) has a solving-degree column; for k = 22, m = 3, d = 3, 5, 11, 15, 25, 31, 33, 41 it lists 4, 6, 12, 9,
10, 32, 15, 42, exactly the eight "preregistered" values. The synthesis lane lists this README ("the census table")
among its sources and quotes the table's `columns` (2, 2, 4, …, 278) and `covered (m)` entries, including the empty k =
22, m = 3 cell. The same README (line 252, erratum E6) already states SD = d + 1 at prime d in the sym form, with seven
exceptions. Five of the eight values are prime cells nearly forced by 1b; the three composite bands are the in-sample
spread {3, 4, 5}. The 8 of 8 is an in-sample restatement [pcc:unverified].

**1e. Cost (R3).** Over the 60 complete sym cells (4 are coset repeats), the least-squares R² of ln T on ln SD is 0.09
(0.60 on ln `max_cols`). At m = 3 and SD = 8: d = 7 (6 variables) T = 3.30e6, d = 27 (12 variables) T = 2.63e11, ratio
8.0e4. At m = 2 and SD = 75: k = 18, d = 923, T = 4.01e10 against k = 20, d = 219, T = 6.18e7 (649×). Beyond the variable
count, SD carries no cost information [pcc:unverified].

**1f. The security statement (R2).** (i) 65,148 is 716× the largest measured SD and 163× the engine cap. (ii) For chain
and flat presentations it holds by construction (the input contains a generator of degree ≥ 65,147); no law is needed.
(iii) As "every presentation" it is false. With d = 65,147, μ_d is the image of the exponent-bit map y = ∏_{j<16} (1 +
(ζ^{2^j} − 1)·b_j), b_j² = b_j, because 2^16 = 65,536 ≥ d (exponents of d or more wrap); with auxiliary products z_j =
z_{j−1}·(1 + (ζ^{2^j} − 1)·b_j) every generator has degree ≤ 2. A baby-step/giant-step split y = a·b, a ∈ {ζ^i : i <
256}, b ∈ {ζ^{256j} : j < 255}, has univariate degree 256. The lane's own SYN-4 and RP-5 are further lower-degree
encodings. Under the lane's max-degree reading these sit in the tens or hundreds, not at 65,148; their cost is unmeasured.
(iv) The empty window is real (checked here by trial division to 10^6 plus 40-round Miller–Rabin: p − 1 =
2²·3·65147·C, C a 236-bit probable prime; p + 1 = 2·5²·79·M, M a 244-bit composite), but it is the known PKM smoothness
condition (PKM 2016 §3.3), not a consequence of the SD law [pcc:unverified].

**Severity.** Blocks for 1d, 1e (as a cost law) and 1f; weakens for 1a–1c. **Cheapest falsification.** 1d and 1e are done
above from committed files. Order: rerun the k = 20, m = 2 sym cells at d = 19 and 57 with the e-variables first. 1f: one
k = 20, m = 3, d = 19 cell in the exponent-bit encoding (5 bits, 2^5 ≥ 19), with `solving_degree_max` and T read against
the recorded 20 and 2.20e8. Minutes each; proposals, not runs [pcc:unverified].

**What it should say.** "In the exponent study's sym-form F4 runs (grevlex, chain variables first, normal strategy), the
highest productive pair degree equals the largest generator degree plus 0 to 5; at prime d that is d + 1, as the exponent
study recorded (E6). It does not predict cost and says nothing about encodings with lower generator degree. Chain and flat
μ_d presentations of Curve25519 contain a generator of degree ≥ 65,147 by construction; whether every μ_d encoding is that
expensive is open." Demote it from rank 3 and drop the "security" class [pcc:unverified].

## 2. W5-PL-1 (store set-up points) and W5-PL-3 (final check from the stored ladder)

**PL-1 claim.** ρ_s ∈ [0.80, 1.00]; `tw2cj-lazy` 0.83× and `inc-lazy-rec` 0.80× at k = 20; a strict-IC ratchet at 0.80× of
the box; never worse on tw arms [pcc:unverified]. **2a. Charged and discarded: confirmed.** `WalkTable::build_inner` and
`TargetWalk` (ladders, `cheap_jumps`) charge the [2^i] ladders and jump evaluations to `setup_ops`, merged into
`build_ops`; only walk points are inserted; `plugins.rs` merges `build_ops` into the oracle's charged operations
[pcc:unverified].

**2b. The k = 20 reference does not exist (R4).** The snc-panel README (line 16) gives `inc-lazy` at k = 20 as "n/a (box,
0 collisions)", and `spec-k20.json` (line 20) shows the k = 20 `inc` arm is `wsum:coef=15,free=1`. WSum's build inserts
every multiple and combination, so the box already stores its set-up. "inc-lazy-rec / inc-lazy 0.80 at k = 20" has no
denominator, and "wwalk-lazy-rec below the box" bundles an oracle switch with recycling [pcc:unverified].

**2c. The unconditional prefix (R7).** The WalkTable walks T = round(x·√r) steps before its grow loop (`for _ in 0..t {
step(..) }`, `wwalk_plan_check`), so recycled points can only shorten the tail; `inc` uses x = 0.9 at k = 24 and 28 and x
= 0.6 at k = 32. With rank-2 time √(rX), X ~ Gamma(2, 1) (the lane's finding 2; its truncated walk of 1.371·√r
reproduces), and m ≈ 0.21·√r set-up points at k = 24 (from the code: about 54 doublings plus 16 jumps ×
15.5 additions), the saving E[max(√X, 0.9) − max(√X − m, 0.9)] is about 0.73·m: below the band [0.80, 1.00], near the
lane's 0.70 falsification line. `TargetWalk` stops at full rank (`while tw.rank < need`), so the tw arms keep the
mechanism [pcc:unverified].

**2d. Magnitude unsupported.** The set-up composition of `tw2cj-lazy` at k = 20 was not read (per-run
`tw_setup_adds`/`tw_setup_doubles` are in the artifact store; `snc-panel/sessions/k20` holds only `session.json`). 0.83×
needs about 0.48·√r of distinct recyclable points out of a 0.79·√r non-walk share that also holds probes, linear algebra,
base and two [d]G checks (2g). Distinct points are fewer than set-up GAE: `eval` partial sums share prefixes; at x = 2.0,
T = 724 and w = round(1.2·√724) = 32, so 16 jumps draw α from 32 values (about 12.7 distinct per seed); E_0 = P1 coincides
with the ladder point [2^0]P1 (a zero row); and same-ladder pairs ([2^i]P, [2^j]P) never collide, where the model assumes
1/r [pcc:unverified].

**2e. Collision statistics.** For any pair involving a walk point the 1/r lemma holds over the walk stream (its
G-coefficient is uniform mod r); pairs of set-up points are fixed per instance. A walk point that hits a set-up point is
not inserted (the first writer keeps the slot). If that row is dropped for a zero seed part (K_j or a G/Q-ladder point
against a walk point with equal (u1, u2)), the twin's later row lies outside the recycled span, so "no run costs more" can
fail in rare runs (about 1/r per pair). Low severity; the paired test catches it [pcc:unverified].

**2f. Classification and reference (R8).** The snc-panel README (line 42) says of `tw=2`: "Classification is the owner's
call … a collision walk with superfluous unknowns rather than a decomposition"; the 2026-11-04 ruling (autolab README line
99) covers C = 2 seed arms and wwalk `g=1,grow=2`, not `tw=2`. The pipeline lane writes "(classification pending)"; README
§1 drops it. rho-neg pays its own set-up (0.375·√r at k = 20, engine items §6), and the lane's model puts rho-neg-rec near
S = 1.20 at w = 0, which would move the w = 0 distance from 2.529 / 1.570 = 1.61 to 2.10 / 1.20 = 1.75. Not relabelling:
walk operations fall, provided set-up inserts carry `counted:table_insert_attempts`; without that marker the extra inserts
go unpriced and part of the w > 0 gain is accounting [pcc:unverified].

**PL-1 verdict.** Blocks for the k = 20 strict-IC sub-claim, weakens elsewhere. Cheapest falsification: Stage 0 counting
distinct coefficient vectors (not `setup_ops`) from the stored records; the prefix model is ten lines of stdlib. It should
say: "On tw arms (classification pending), storing the distinct set-up points cuts walk steps by ρ_s × (distinct points);
on WalkTable arms ρ_s is about 0.7–0.8 at x = 0.9 unless T is re-planned (a separate change). A k = 20 strict-IC
comparison waits for a measured wwalk-lazy k = 20 arm. Distance is reported against rho-neg and rho-neg-rec, unresolved
where they swap inside the band" [pcc:unverified].

**2g. PL-3 (−2 % at k = 20, −0.7 % at k = 24 from NAF over the stored [2^i]G ladder): charged in S, but counted once
(R9).** `g.mul(&mut ver.group_ops, generator, d)` runs in the RecoveryCheck phase; rho-neg's check is 0.065·√r of its S at
k = 20. `CountedGroup::mul` is plain double-and-add (at log2 r = 17, about 16 doublings plus 7.5 additions); NAF over the
ladder needs about 4.7 additions, saving about 19 GAE, as the lane says. The lane missed that tw-lazy arms check the same
d twice: inside `TargetWalk` (`g.mul(&mut check_ops, generator, d)`, charged as `tw_lazy_check_*`) and in the pipeline
(`plugins.rs`, `lazy` parameter text: "The pipeline's [d]G = Q check still runs, charged. With tw=1|2 the build also runs
that check itself once"). Converting both gives about −4.1 % at k = 20 and −1.7 % at k = 24 (38 and 47 GAE against S·√r =
916 and 2,773), and would trip the lane's own falsification ("any paired-run difference other than the predicted check
delta"). Dropping one check outright counts as "skipping verification" (rule §4) and needs a Validator ruling. Neither
relabelling nor accounting: the ladder is already charged to set-up. Severity: weakens; the error favours the lead. It
should say: "Both [d]G checks of the tw-lazy arms from the stored ladder, both kept: −4 % (k = 20), −1.7 % (k = 24);
WalkTable arms have one check: −2 %, −0.9 %" [pcc:unverified].

## 3. W5-RP-1, flat y^d = 1 against the chain

**3a. Arithmetic: checks.** The band endpoints reproduce: the two-point L = 1 slope at k = 20, m = 3 between d = 7 and
19 is 4.21, and chain T / 8.5^(L−1) at d = 27 is 3.64e9. From the band, chain/flat is 73–263 at d = 27 and 72–184 at
d = 28 [pcc:unverified].

**3b. Shared premise.** The flat band is the L = 1 prime-d curve (three k = 20 cells, d = 3, 7, 19; local slope falls
6.09 → 4.21). That curve is SYN-3's premise, so RP-1 tests SYN-3's premise rather than giving an independent second view
[pcc:unverified].

**3c. Memory.** Monomials up to degree d + 1 in 6 variables (m = 3 sym): C(34, 6) = 1.34e6 (d = 27), C(43, 6) = 6.10e6
(36), C(80, 6) = 3.01e8 (73). Recorded `max_cols` / C(n + D, n) falls from 0.29 (d = 7) to 0.12 (d = 41); a power fit on
the L = 1, m = 3 cells (slope 4.31) gives about 2.4e5, 2.8e5, 8.2e5 and 1.7e7 columns at d = 27, 28, 36, 73. The k = 22, d
= 41 cell finished with 1.51e6 columns inside the frozen 3.5 GB, so d ≤ 36 is plausible. d = 73 at m = 3 is
11.6× wider, `f4_fp` keeps reduced pair rows dense over the free columns, and its projected T is 0.8e11–1.3e11 field ops:
unsupported at 12 GB. The frozen `max_degree` 400 caps flat cells at d ≤ 399, so the m = 2 chain cells d = 511, 513 have
no flat twin [pcc:unverified].

**3d. Reference (R11).** No ecbench arm uses a μ_d base (every committed Curve25519 spec uses prime-projected-abscissa or
prime-isogeny bases); enumeration on the same base is cheaper at m ≤ 3 (pipeline lane §0); msolve, the open reference F4
under AGENTS §15, is not named for the flat input. **3e. Scope.** "A flat win removes the only reason μ_d needs smooth p −
1" does not touch Curve25519: no divisor of p − 1 lies in [2^19.6, 2^235.4] in any presentation. It should say: "Component
ratio, flat against chain F4 on the same cells (d ≤ 36 at m = 3, d < 400 at m = 2), plus msolve on the identical flat
input; no baseline improvement claimed." Severity: weakens [pcc:unverified].

## 4. W5-SYN-1, the descent-character coset rule

**4a. The engine.** `subgroup_semaev` symmetrises only under S_m (sym form in e_1..e_m) and returns consistency per
target; it neither collects relations nor removes x ↦ 1/x duplicates. No ecbench arm uses a μ_d base, and the lead's cost
band says "otherwise 0", so it improves no existing arm; README's "improvement, algebraic component" should read "yield
census" [pcc:unverified].

**4b. The T₀ study (R10).** `research/curve25519_t0_symmetry_20261102/README.md` measured T₀-closure doubling coverage at
a fixed column count (1.944–2.016, 8 of 8 cells) and recommends T₀-closed bases and T₀-invariant variables for the
algebraic track (its "Next 1"). SYN-1 reports a 2^(m−2) loss at fixed |V|: same counting, opposite normalisation. README
§3 calls them one symmetry that "doubles coverage" and omits the sign change. Hidden assumption: equal F4 cost per system
across gidx, measured only at m = 2 (d = 27, 63: 1.79e6, 2.23e7 at gidx 0, 1, 2) and only in the current presentation.
gidx 0 is the one coset where the T₀ study's invariant rewriting applies (κ = x + 1/x, membership degree (d + 1)/2 per
κ_i). S_{m+1} is invariant only under inverting an even subset of summands, so the invariant ring is larger than the κ_i
and the per-system gain is unknown; if the degree halved, the lab's d^4.2–4.95 slope at m = 3 would give
18–31×, outweighing a 2× clumping loss. Moving off gidx 0 forecloses that route [pcc:unverified].

**Verdict.** Weakens. The Stage 0 census is cheap and sound; add one k = 20, m = 3, d = 19 F4 cell in the invariant
presentation against the recorded 2.20e8. It should say: "On T-stable bases solutions come in orbits of 2^(m−1); at fixed
|V| this costs 2^(m−2) solves to full rank at m ≥ 3, at fixed columns it gains 2× (T₀ study); the invariant presentation
is unmeasured and may reverse the sign" [pcc:unverified].

## 5. README §1: what is overstated

- "One law found in the data": the prime-d part is the exponent study's E6, restated without "sym" or its seven
  exceptions [pcc:unverified].
- "Preregistered eight values it had not read": contradicted by 1d. To make it verifiable next time: (1) the coordinator
  holds the evaluation data, using cells generated after the prediction, or giving the lane only a copy of the tables
  with the target column removed; (2) the lane writes its predictions to a file and the coordinator commits it (or
  registers a pcc claim) with its SHA-256 before any run or data release, so git order proves precedence; (3) the
  lane's file-access log is archived with the result; (4) the predictions cover cells the in-sample law does not force
  (composite d at an untested m or k, plain form, reversed variable order) [pcc:unverified].
- "Four pipeline levers could each give a measured gain at k = 20 and 24": PL-4 is predicted only at k = 32. "After all
  four … 1.7 to 1.8× above rho-neg at k ≥ 28": PL-1 and PL-3 give nothing at k ≥ 28 by their own numbers, and PL-4 is
  moot if PL-2 lands, so the figure rests on PL-2 alone (prior 0.45) [pcc:unverified].
- "Solving degree at least 65,148 on that route": see 1f. "The owner ruled … these arms count as index calculus": not
  for tw=2 (2f). "#7 is the instrument the exponent question is waiting for": see 6a [pcc:unverified].

## 6. Other findings

**6a. W5-RP-4 (R5).** `f4_fp` already reduces pair rows by Faugère–Lachartre: reducers A | B are brought to B′ = A⁻¹B,
pair rows C | D become D − C·B′ (comment at the `reduce_pairs_fl` call; `F4_FP_FL` switch; B′ capped at `FL_MAX_ENTRIES` =
2^28 entries), and the module header says only the residual is dense. RP-4's "only B|D densified" is the current design,
so its predicted ≥ 4× RSS reduction has no mechanism; the 12 GB wall lies elsewhere (dense reduced pair rows, basis, pair
storage). Cheapest check: one d = 28 chain replay with `F4_FP_FL=0` and its peak RSS, plus free columns × pair rows at the
widest step. Severity: blocks [pcc:unverified].

**6b. Citations (R13).** `ic_boundary.rs` has gained 670 lines since `98cd2fd1f8`; cite functions, not lines. **6c.** Not
reviewed in depth: PL-2, PL-4, SYN-2, SYN-4, RP-2, RP-3, RP-5; no finding about them is implied [pcc:unverified].

## 7. What survives

- **As stated:** none without at least a change of wording [pcc:unverified].
- **Minor rewording, experiment unchanged:** (1) W5-PL-3, counting both [d]G checks on tw-lazy arms (about −4 % at
  k = 20), both kept; (2) W5-RP-1, as a component ratio at d ≤ 36 (m = 3) and d < 400 (m = 2), with msolve added and
  stated as a test of SYN-3's premise [pcc:unverified].
- **Rewording needed before any task card:** (3) W5-PL-1: ρ_s ∈ [0.8, 1] for tw arms only, ≈ 0.7–0.8 on the WalkTable;
  no k = 20 strict-IC ratchet until wwalk-lazy is measured at k = 20; tw2 classification pending; both rho references;
  Stage 0 counts distinct vectors. (4) W5-SYN-1: a census, not an improvement; reconciled with the T₀ study; plus the
  invariant-presentation cell. (5) W5-SYN-3: a descriptive regularity of the sym-form F4 step degree, not a cost law
  and not out-of-sample; demoted [pcc:unverified].
- **Drop as stated:** (6) the SYN-3 / README §1 security statement ("SD ≥ 65,148" in every presentation, or on that
  route); (7) W5-RP-4 as written (mechanism already in `f4_fp`), replaced by a memory-profile measurement; (8) README
  §1's "eight preregistered values it had not read" (say "in-sample") and "after all four … k ≥ 28" [pcc:unverified].

