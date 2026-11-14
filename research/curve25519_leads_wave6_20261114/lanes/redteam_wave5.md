> Red-team lane report (2026-11-14). Desk review only: no experiment was run, no evidence was edited, no status was
> changed, and nothing here declares a lead supported or refuted. Every statement is an unverified assertion
> [pcc:unverified].

# Red-team review of the wave-5 Curve25519 lead set

**Evidence reviewed.** `research/curve25519_leads_wave5_20261112/` at commit `98cd2fd1f8` (text unchanged at `fc4dbd3a54`;
only `sd_law.png` moved to the artifact store); the cells in `research/curve25519_exponent_study_20261108/cells` and
`research/curve25519_exponent_study_ext_20261110/cells`; source at `98cd2fd1f8` (`src/cryptanalysis/ic_boundary.rs`,
`ic_framework/plugins.rs`, `ic_framework/subgroup_semaev.rs`, `f4_fp.rs`) and `src/bin/curve25519_family.rs`. The checks
are stdlib Python over committed JSON, with the formulas given inline so anyone can redo them; none is a measurement.
research-kb and ecdb were not reachable from this runtime, so no `source_id` receipts exist [pcc:unverified].

**Labels.** Severity: *blocks* (the headline cannot stand as written), *weakens* (the prediction or its framing must
change before a task card), *cosmetic*. Kind: *wrong*, *unsupported*, *unpriced* [pcc:unverified].

## Index, by severity

| id | lead | severity | kind | finding |
|:--|:--|:--|:--|:--|
| R1 | SYN-3 | blocks | wrong | the "8 of 8 preregistered, not read" values are in a table the lane says it read |
| R2 | SYN-3, README §1 | blocks | wrong | "SD ≥ 65,148 in every presentation" is true by construction for chain and flat, and false for other encodings of μ_d |
| R3 | SYN-3 | blocks (as a cost law) | unsupported | SD does not predict cost: R² = 0.09 over 60 cells; T differs 8.0e4× at equal SD |
| R4 | PL-1 | blocks (k = 20 sub-claim) | wrong | no k = 20 `inc-lazy` exists, and the k = 20 incumbent (the wsum box) already stores its set-up |
| R5 | RP-4 | blocks | wrong | the proposed Faugère–Lachartre split is already in `f4_fp` |
| R6 | SYN-3 | weakens | wrong | "SD" is the highest F4 pair degree; the law depends on the form (plain gives d + 2); one k = 18 cell breaks the m = 2 band |
| R7 | PL-1 | weakens | unsupported | the WalkTable walks T = x·√r steps unconditionally, so ρ_s ≈ 0.73 at k = 24 under the lead's own model |
| R8 | PL-1 | weakens | unpriced | the headline arm `tw2cj-lazy` has no IC classification from the owner; a rho-neg-rec reference may widen the w = 0 distance |
| R9 | PL-3 | weakens | wrong (undercount) | tw-lazy arms run the [d]G = Q check twice; the lead prices one |
| R10 | SYN-1 | weakens | unsupported | no existing arm improves; its sign conflicts with the T₀ study's 2× gain at fixed columns |
| R11 | RP-1 | weakens | unpriced | the reference is the chain; msolve is not named; the degree cap blocks d ≥ 400; memory at d = 73, m = 3 is unsupported |
| R12 | README §1 | weakens | wrong | the prime-d law already existed (exponent study, E6); "after all four, k ≥ 28" rests on PL-2 alone |
| R13 | README §5 | cosmetic | — | line-number citations hold only at `98cd2fd1f8` (`ic_boundary.rs` has gained 670 lines since) |

## 1. W5-SYN-3, solving degree = largest prime factor of d plus a small constant

**Claim.** SD = e_max(d) + c. SD = d + 1 at prime d in 20 of 20 sym cells. Eight preregistered k = 22, m = 3 values
match. Hence SD ≥ 65,148 for every μ_d base of Curve25519, "in every presentation" (synthesis lane) or "on that route"
(README §1) [pcc:unverified].

**1a. What the quantity is (R6).** `F4Report.solving_degree_max` is "the highest step degree at which the run learned
something new". In all 73 complete cells (sym and plain) it equals `degree_reached`, the highest pair degree processed,
on every target. The step at which 1 entered the ideal (`solving_degree`) is 2 to 6. So "SD" measures how high F4's
normal strategy climbs. It is neither a degree of regularity nor the degree of the inconsistency certificate. The degree
cap of 400 does not bind: a target counts as complete only with `pairs_above_bound == 0` (`curve25519_family.rs` near
l. 1684), and the largest recorded SD is 91. The variable order (grevlex, chain variables first; PROTOCOL §2) was never
varied, so whether SD depends on the order is untested [pcc:unverified].

**1b. Construction, not structure.** In every complete sym cell, SD minus the cell's recorded `input_degree` lies in
[0, 5]. It is 1 in all 20 prime cells, except the two m = 3, d = 3 cells where it is 0. By construction the input degree
is e_max (from the generator y^{e_max} − y′) or, when larger, the Semaev sym degree 2^(m−1). That is why SD − e_max
rises to 4 or 5 when e_max = 3 (k = 20, m = 3, d = 27: input degree 4, SD 8). A run that handles a generator of degree d
reaches pair degree d or more, and that generator's first pair with a leading monomial y_i·v has degree d + 1. So at
prime d the law carries at most one degree of content. Cheapest check: dump the F4 step trace of one k = 20, m = 2,
d = 19 sym target. If the last productive step at degree 20 comes from a binomial–quadric pair, the prime-d law is
bookkeeping [pcc:unverified].

**1c. Form dependence and an excluded cell (R6).** The plain form gives SD = 5, 9 and 21 at m = 2 (d = 3, 7, 19, so
d + 2) and 12, 15 and 27 at m = 3. The lane's mechanism is a top system (y1^d, y2^d, y1²y2²) with regularity d + 1,
bounded by Caminata–Gorla. The lane states it for the plain form at m = 2, where it caps SD at d + 1. All three plain
m = 2 cells exceed that cap, so the recorded quantity is not the solving degree that bound governs. The check script
globs `k2*_m[23]_sym_*`, which excludes k = 18. Yet `k18_m2_sym_d923_g0.json` (923 = 13·71) has SD 75 = e_max + 4,
outside README §1's "e_max + 2 to 3 at m = 2", and SYN-3 itself names the k = 18 tables as test data [pcc:unverified].

**1d. The out-of-sample claim (R1).** The main cell table of `research/curve25519_exponent_study_20261108/README.md`
(lines 152–188) has a solving-degree column. For k = 22, m = 3 and d = 3, 5, 11, 15, 25, 31, 33, 41 it lists 4, 6, 12,
9, 10, 32, 15, 42: exactly the eight "preregistered" values. The synthesis lane names this README ("the census table")
among the sources it read. It quotes the table's `columns` entries (2, 2, 4, …, 278) and its `covered (m)` entries,
including the empty k = 22, m = 3 cell. The same README (line 252, erratum E6) already states SD = d + 1 at prime d in
the sym form and lists seven exceptions. Five of the eight values are prime cells, which 1b nearly forces. The three
composite bands are the in-sample spread {3, 4, 5}. So the 8 of 8 match is an in-sample restatement [pcc:unverified].

**1e. Cost (R3).** Across the 60 complete sym cells (4 of them coset repeats), the least-squares R² of ln T on ln SD is
0.09; on ln `max_cols` it is 0.60. At m = 3 and SD = 8, d = 7 (6 variables) has T = 3.30e6 and d = 27 (12 variables)
has T = 2.63e11, a ratio of 8.0e4. At m = 2 and SD = 75, k = 18, d = 923 has T = 4.01e10 and k = 20, d = 219 has
T = 6.18e7 (649×). Beyond the variable count, SD carries no cost information [pcc:unverified].

**1f. The security statement (R2).**

- 65,148 is 716× the largest measured SD and 163× the engine's cap [pcc:unverified].
- For chain and flat presentations the bound holds by construction, because the input contains a generator of degree at
  least 65,147. No law is needed [pcc:unverified].
- As "every presentation" it is false. With d = 65,147, the same set μ_d is the image of the exponent-bit map
  y = ∏_{j<16} (1 + (ζ^{2^j} − 1)·b_j) with b_j² = b_j, because 2^16 = 65,536 ≥ d (exponents of d or more wrap
  around). With auxiliary products z_j = z_{j−1}·(1 + (ζ^{2^j} − 1)·b_j), every generator has degree at most 2. A
  baby-step/giant-step split y = a·b, with a ∈ {ζ^i : i < 256} and b ∈ {ζ^{256j} : j < 255}, has univariate degree 256.
  The lane's own SYN-4 and RP-5 are further lower-degree encodings. Read by the lane's max-degree logic, these would sit
  in the tens or hundreds, not at 65,148. Their cost is unmeasured [pcc:unverified].
- The empty window is real, and checked here by trial division to 10^6 plus a 40-round Miller–Rabin test:
  p − 1 = 2²·3·65147·C with C a 236-bit probable prime, and p + 1 = 2·5²·79·M with M a 244-bit composite. But it is the
  known PKM smoothness condition (PKM 2016 §3.3), not a consequence of the SD law [pcc:unverified].

**Severity.** Blocks for 1d, for 1e as a cost law, and for 1f. Weakens for 1a to 1c [pcc:unverified].

**Cheapest falsifications.** 1d and 1e are done above, from committed files. For the variable order: rerun the k = 20,
m = 2 sym cells at d = 19 and d = 57 with the e-variables first. For 1f: one k = 20, m = 3, d = 19 cell in the
exponent-bit encoding (5 bits, since 2^5 ≥ 19), with `solving_degree_max` and T compared against the recorded 20 and
2.20e8. Each takes minutes. These are proposals, not runs [pcc:unverified].

**What it should say.** "In the exponent study's sym-form F4 runs (grevlex, chain variables first, normal strategy), the
highest productive pair degree equals the largest generator degree plus 0 to 5. At prime d that is d + 1, as the
exponent study recorded (E6). It does not predict cost, and it says nothing about encodings with lower generator
degree. Chain and flat μ_d presentations of Curve25519 contain a generator of degree at least 65,147 by construction.
Whether every μ_d encoding is that expensive is open." Demote it from rank 3 and drop the "security" class
[pcc:unverified].

## 2. W5-PL-1 (store the set-up points) and W5-PL-3 (final check from the stored ladder)

**PL-1 claim.** ρ_s ∈ [0.80, 1.00]; `tw2cj-lazy` 0.83× at k = 20; `inc-lazy-rec` 0.80× at k = 20; a strict-IC ratchet
at 0.80× of the box; never worse on tw arms [pcc:unverified].

**2a. Charged and discarded: confirmed.** `WalkTable::build_inner` and `TargetWalk` (`cheap_jumps`, the ladders) charge
the [2^i] ladders and the jump evaluations to `setup_ops`, which are merged into `build_ops`. They insert walk points
only. `plugins.rs` merges `build_ops` into the oracle's charged operations [pcc:unverified].

**2b. The k = 20 reference does not exist (R4).** The snc-panel README (line 16) gives `inc-lazy` at k = 20 as "n/a
(box, 0 collisions)". `spec-k20.json` (line 20) shows that the k = 20 `inc` arm is `wsum:coef=15,free=1`. WSum's build
inserts every multiple and every combination, so the box already stores its set-up. "inc-lazy-rec / inc-lazy 0.80 at
k = 20" therefore has no denominator, and "wwalk-lazy-rec below the box" bundles an oracle switch with recycling
[pcc:unverified].

**2c. The unconditional prefix (R7).** The WalkTable walks T = round(x·√r) steps before its grow loop
(`for _ in 0..t { step(..) }`, `wwalk_plan_check`), so recycled points can shorten only the tail. `inc` uses x = 0.9 at
k = 24 and 28, and x = 0.6 at k = 32. Take the rank-2 time as √(rX) with X ~ Gamma(2, 1), the lane's finding 2; its
truncated walk of 1.371·√r is reproduced. With m ≈ 0.21·√r set-up points at k = 24 (estimated from the code: about
54 doublings plus 16 jumps × 15.5 additions), the saving E[max(√X, 0.9) − max(√X − m, 0.9)] is about 0.73·m. That is
below the band [0.80, 1.00] and near the lane's 0.70 falsification line. `TargetWalk` stops at full rank
(`while tw.rank < need`), so the mechanism stands on the tw arms [pcc:unverified].

**2d. Magnitude unsupported.** The set-up composition of `tw2cj-lazy` at k = 20 was not read. Its per-run
`tw_setup_adds` and `tw_setup_doubles` are in the artifact store; `snc-panel/sessions/k20` holds only `session.json`. A
ratio of 0.83× needs about 0.48·√r of distinct recyclable points. The arm's non-walk share is 0.79·√r, and that share
also holds the probes, the linear algebra, the base and two [d]G checks (2g). Distinct points number fewer than the
set-up GAE:

- the partial sums of `eval` share prefixes [pcc:unverified];
- at x = 2.0 the walk length is T = 724 and w = round(1.2·√724) = 32, so 16 jumps draw α from 32 values and give about
  12.7 distinct values per seed [pcc:unverified];
- E_0 = P1 coincides with the ladder point [2^0]P1, which gives a zero row [pcc:unverified];
- pairs within one ladder ([2^i]P against [2^j]P) never collide, where the model assumes probability 1/r
  [pcc:unverified].

**2e. Collision statistics.** For any pair that involves a walk point, the 1/r lemma holds over the walk stream, since
the walk point's G-coefficient is uniform mod r. Pairs of two set-up points are fixed for a given instance. A walk point
that hits a set-up point is not inserted, because the first writer keeps the slot. If that row is dropped for a zero
seed part (K_j or a G/Q-ladder point against a walk point with equal (u1, u2)), the twin's later row lies outside the
recycled span. Then "no run costs more" fails, in rare runs (about 1/r per pair). Low severity: the paired test catches
it [pcc:unverified].

**2f. Classification and reference (R8).**

- No IC classification for the headline arm. The snc-panel README (line 42) says "Classification is the owner's call …
  a collision walk with superfluous unknowns rather than a decomposition". The ruling of 2026-11-04 (autolab README
  line 99) covers the C = 2 seed arms and wwalk `g=1,grow=2`, not `tw=2`. The pipeline lane writes "(classification
  pending)"; README §1 drops that qualifier [pcc:unverified].
- The reference improves too. rho-neg pays its own set-up, 0.375·√r at k = 20 (engine items §6), and the lane's model
  puts rho-neg-rec near S = 1.20 at w = 0. The w = 0 distance would then go from 2.529 / 1.570 = 1.61 to
  2.10 / 1.20 = 1.75 [pcc:unverified].
- Not relabelling. Walk operations fall, provided the set-up inserts carry `counted:table_insert_attempts`. Without that
  marker the extra inserts go unpriced, and part of the gain at w > 0 is accounting [pcc:unverified].

**PL-1: severity, falsification, wording.**

- **Severity.** Blocks for the k = 20 strict-IC sub-claim; weakens elsewhere [pcc:unverified].
- **Cheapest falsification.** Stage 0 counts distinct coefficient vectors, not `setup_ops`, from the stored records.
  The prefix model takes ten lines of stdlib [pcc:unverified].
- **What it should say.** "On tw arms (classification pending), storing the distinct set-up points cuts walk steps by
  ρ_s × (distinct points). On WalkTable arms ρ_s is about 0.7 to 0.8 at x = 0.9, unless T is re-planned, which is a
  separate change. A k = 20 strict-IC comparison waits for a measured wwalk-lazy arm at k = 20. Distance is reported
  against rho-neg and rho-neg-rec, and is unresolved where the two swap within the band" [pcc:unverified].

**PL-3 claim.** −2 % at k = 20 and −0.7 % at k = 24, from replacing the pipeline's `g.mul` with NAF over the stored
[2^i]G ladder [pcc:unverified].

**2g. The check is charged in S.** `g.mul(&mut ver.group_ops, generator, d)` runs in the RecoveryCheck phase, and
rho-neg's check is 0.065·√r of its S at k = 20. `CountedGroup::mul` is plain double-and-add: at log2 r = 17, about 16
doublings plus 7.5 additions. NAF over the ladder needs about 4.7 additions, which saves about 19 GAE, as the lane says.
The lane missed that tw-lazy arms check the same d twice:

1. inside `TargetWalk` (`g.mul(&mut check_ops, generator, d)`, charged as `tw_lazy_check_*`) [pcc:unverified];
2. in the pipeline. The text of the `lazy` parameter in `plugins.rs` reads: "The pipeline's [d]G = Q check still runs,
   charged. With tw=1|2 the build also runs that check itself once" [pcc:unverified].

Converting both checks gives about −4.1 % at k = 20 and −1.7 % at k = 24 (38 and 47 GAE against S·√r = 916 and 2,773).
It would also trip the lane's own falsification rule, "any paired-run difference other than the predicted check delta".
Dropping one check outright counts as "skipping verification" (rule §4) and needs a ruling from the Validator. This is
neither relabelling nor an accounting change, because the ladder is already charged to set-up. Severity: weakens; the
error runs in the lead's favour [pcc:unverified].

**PL-3: what it should say.** "Both [d]G checks of the tw-lazy arms come from the stored ladder, and both are kept: −4 %
at k = 20, −1.7 % at k = 24. WalkTable arms have one check: −2 %, −0.9 %" [pcc:unverified].

## 3. W5-RP-1, flat y^d = 1 against the chain

**3a. The arithmetic checks.** The band endpoints reproduce. The two-point L = 1 slope at k = 20, m = 3 between d = 7
and d = 19 is 4.21, and the chain T / 8.5^(L−1) at d = 27 is 3.64e9. From the band, chain/flat is 73 to 263 at d = 27
and 72 to 184 at d = 28 [pcc:unverified].

**3b. A shared premise.** The flat band is the L = 1 prime-d curve, built on three k = 20 cells (d = 3, 7, 19) whose
local slope falls from 6.09 to 4.21. That curve is SYN-3's premise, so RP-1 tests SYN-3's premise rather than giving an
independent second view [pcc:unverified].

**3c. Memory.** The number of monomials of degree up to d + 1 in 6 variables (m = 3, sym) is C(34, 6) = 1.34e6 at
d = 27, C(43, 6) = 6.10e6 at d = 36, and C(80, 6) = 3.01e8 at d = 73.

- Recorded `max_cols` / C(n + D, n) falls from 0.29 at d = 7 to 0.12 at d = 41. A power fit on the L = 1, m = 3 cells
  (slope 4.31) gives about 2.4e5, 2.8e5, 8.2e5 and 1.7e7 columns at d = 27, 28, 36 and 73 [pcc:unverified].
- d ≤ 36 is plausibly feasible: the k = 22, d = 41 cell finished with 1.51e6 columns inside the frozen 3.5 GB
  [pcc:unverified].
- d = 73 at m = 3 is unsupported at 12 GB. It is 11.6× wider than d = 41, and `f4_fp` stores each reduced pair row
  densely over the free columns. Its projected T is 0.8e11 to 1.3e11 field operations [pcc:unverified].
- The frozen `max_degree` of 400 caps flat cells at d ≤ 399, so the chain cells at m = 2 with d = 511 and 513 have no
  flat twin [pcc:unverified].

**3d. Reference (R11).** No ecbench arm uses a μ_d base: every committed Curve25519 spec uses prime-projected-abscissa
or prime-isogeny bases. At m ≤ 3, enumeration on the same base is cheaper (pipeline lane §0). msolve, the open reference
F4 under AGENTS §15, is not named for the flat input [pcc:unverified].

**3e. Scope.** "A flat win removes the only reason μ_d needs smooth p − 1" does not touch Curve25519: no divisor of
p − 1 lies in [2^19.6, 2^235.4] in any presentation [pcc:unverified].

**What it should say.** "A component ratio: flat against chain F4 on the same cells (d ≤ 36 at m = 3, d < 400 at m = 2),
plus msolve on the identical flat input. No baseline improvement is claimed." Severity: weakens [pcc:unverified].

## 4. W5-SYN-1, the descent-character coset rule

**4a. The engine.** `subgroup_semaev` symmetrises only under S_m (the sym form in e_1, …, e_m) and returns consistency
per target. It neither collects relations nor removes x ↦ 1/x duplicates. No ecbench arm uses a μ_d base, and the
lead's own cost band says "otherwise 0", so it improves no existing arm. The README's label "improvement, algebraic
component" should read "yield census" [pcc:unverified].

**4b. The T₀ study (R10).** `research/curve25519_t0_symmetry_20261102/README.md` measured that T₀-closure doubles
coverage at a fixed column count (1.944 to 2.016, in 8 of 8 cells). It recommends T₀-closed bases and T₀-invariant
variables for the algebraic track (its "Next 1"). SYN-1 reports a loss of 2^(m−2) at fixed |V|. The counting is the
same; the normalisation is opposite. README §3 calls them one symmetry that "doubles coverage" and omits the change of
sign [pcc:unverified].

- **Hidden assumption: equal F4 cost per system across gidx.** This was measured only at m = 2 (d = 27 and 63: 1.79e6
  and 2.23e7 at gidx 0, 1 and 2), and only in the current presentation [pcc:unverified].
- **Rewriting is available only on gidx 0.** It is the one coset where the T₀ study's invariant rewriting applies:
  κ = x + 1/x, with a membership degree of (d + 1)/2 per κ_i. S_{m+1} is invariant only under inverting an even subset
  of the summands, so the invariant ring is larger than the κ_i alone and the per-system gain is unknown. If the degree
  halved, the lab's d^4.2–4.95 slope at m = 3 would give 18× to 31×, which would outweigh a 2× clumping loss. Moving off
  gidx 0 forecloses that route [pcc:unverified].

**Severity and checks.** Weakens. The Stage 0 census is cheap and sound. Add one k = 20, m = 3, d = 19 F4 cell in the
invariant presentation, compared against the recorded 2.20e8 [pcc:unverified].

**What it should say.** "On T-stable bases the solutions come in orbits of 2^(m−1). At fixed |V| this costs 2^(m−2)
solves to full rank at m ≥ 3; at a fixed column count it gains 2× (T₀ study). The invariant presentation is unmeasured
and may reverse the sign" [pcc:unverified].

## 5. README §1: what is overstated

- "One law found in the data": the prime-d part is the exponent study's E6, restated without "sym" and without its seven
  exceptions [pcc:unverified].
- "Preregistered eight values it had not read": contradicted by 1d. To make such a claim verifiable next time:
  1. The coordinator holds the evaluation data. Either use cells generated after the prediction, or give the lane, as
     its only readable copy, the tables with the target column removed [pcc:unverified].
  2. The lane writes its predictions to a file, and the coordinator commits it (or registers a pcc claim) with its
     SHA-256 before any run or data release, so git order proves precedence [pcc:unverified].
  3. The lane's file-access log is archived with the result [pcc:unverified].
  4. The predictions cover cells the in-sample law does not force: composite d at an untested m or k, the plain form, a
     reversed variable order [pcc:unverified].
- "Solving degree at least 65,148 on that route" and "security statement": see 1f [pcc:unverified].
- "Four pipeline levers could each give a measured gain at k = 20 and 24": PL-4 is predicted only at k = 32. "After all
  four … 1.7 to 1.8× above rho-neg at k ≥ 28": by their own numbers PL-1 and PL-3 give nothing at k ≥ 28, and PL-4 is
  moot if PL-2 lands, so this figure rests on PL-2 alone (prior 0.45) [pcc:unverified].
- "The owner ruled … these arms count as index calculus": the ruling does not cover tw=2 (2f) [pcc:unverified].
- "#7 is the instrument the exponent question is waiting for": see 6a [pcc:unverified].

## 6. Other findings

**6a. W5-RP-4 (R5).** `f4_fp` already reduces pair rows by the Faugère–Lachartre scheme. The reducers A | B are brought
to B′ = A⁻¹B, and the pair rows C | D become D − C·B′ (see the comment at the `reduce_pairs_fl` call, the `F4_FP_FL`
switch, and B′ capped at `FL_MAX_ENTRIES` = 2^28 entries). The module header says only the residual is dense. RP-4's
"only B|D densified" is therefore the current design, and its predicted RSS reduction of 4× or more has no mechanism.
The 12 GB wall lies elsewhere: in the dense reduced pair rows, the basis, or the pair storage. Cheapest check: one
d = 28 chain replay with `F4_FP_FL=0` and its peak RSS, plus the count of free columns × pair rows at the widest step.
Severity: blocks [pcc:unverified].

**6b. Citations (R13).** README §5 says the code citations exist as cited. `ic_boundary.rs` has gained 670 lines since
`98cd2fd1f8`, so cite functions instead: `WalkTable::build_inner`, `TargetWalk::cheap_jumps`, the RecoveryCheck block
[pcc:unverified].

**6c. Not reviewed in depth.** PL-2, PL-4, SYN-2, SYN-4, RP-2, RP-3 and RP-5. No finding about them is implied
[pcc:unverified].

## 7. What survives

**Survive as stated:** none without at least a change of wording [pcc:unverified].

**Survive after minor rewording; the experiment stays as proposed:**

1. W5-PL-3: count both [d]G checks on the tw-lazy arms (about −4 % at k = 20) and keep both [pcc:unverified].
2. W5-RP-1: call it a component ratio; limit it to d ≤ 36 (m = 3) and d < 400 (m = 2); add msolve; state that it tests
   SYN-3's premise [pcc:unverified].

**Need rewording before any task card:**

3. W5-PL-1: claim ρ_s ∈ [0.8, 1] only for the tw arms, and ρ_s ≈ 0.7 to 0.8 on the WalkTable. Drop the k = 20
   strict-IC ratchet until wwalk-lazy is measured at k = 20. Mark tw2 as classification pending. Report against both
   rho references. Stage 0 counts distinct vectors [pcc:unverified].
4. W5-SYN-1: restate it as a census, not an improvement; reconcile it with the T₀ study; add the invariant-presentation
   cell [pcc:unverified].
5. W5-SYN-3: restate it as a descriptive regularity of the F4 step degree in the sym form, not a cost law and not
   out-of-sample evidence; demote it [pcc:unverified].

**Drop as stated:**

6. The security statement of SYN-3 and README §1 ("SD ≥ 65,148" in every presentation, or on that route)
   [pcc:unverified].
7. W5-RP-4 as written: its mechanism is already in `f4_fp`. Replace it with a memory-profile measurement
   [pcc:unverified].
8. README §1's "eight preregistered values it had not read" (say "in-sample" instead) and "after all four … k ≥ 28"
   [pcc:unverified].
