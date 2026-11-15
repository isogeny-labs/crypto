# Red-team review of PROTOCOL.md (panel P, presentation dependence, m = 3..5), before freeze

2026-11-15. Adversarial pre-freeze review of the draft. **Reviewed:** `research/pkm_present_panel_20261115/PROTOCOL.md`
as committed in `1f98e4a0a6` (HEAD `add6960ac0`), sha256 `88ae251a5191ce0ac382d80b9778387017bcbe6e058ee28601477e31cf4f067b`.
The protocol is a draft by its own §9, so this review records challenges. It is not evidence, and it decides no status.
Code facts below are read from committed sources (`src/cryptanalysis/f4_fp.rs`, `examples/pkm_m8_panel/*`,
`src/cryptanalysis/semaev_higher.rs`). The working-tree-only `examples/pkm_present_panel/` and `smoke/` (seed 777)
are cited only as corroboration, never as evidence, and their data was not read. Every number here is desk arithmetic,
`[pcc:unverified]` (script: `/tmp/claude-0/-home-user/c2970330-965d-56e2-8425-0faf587c0f40/scratchpad/rtp/desk.py`,
stdlib, not committed). Nothing else was edited and nothing was run.

**Bottom line:** the protocol is not ready to freeze as written. B1 makes `supported_within_scope` impossible in every
cell, whatever the data. Each of B2–B5 makes some C2 verdict either unreachable or confounded with something other
than presentation. All fixes are text or plan changes, and none needs new spend.

## Desk facts used below

- **Sign-flip resolution.** The smallest exact two-sided p-value with n pairs, all of one sign, is 2/2^n: 0.125 at
  n = 4, 0.03125 at 6, 0.0078 at 8, 0.00195 at 10. The Bonferroni thresholds are 0.05/4 = 0.0125 at m = 3 and
  0.05/5 = 0.01 at m ≥ 4.
- **Repeated abscissae.** `targets.rs` enumerates signed multisets *with repetition* (`RealEnum::rec(depth+1, s, …)`).
  The planted target draws m symbols *with replacement* (`targets.rs:419–422`). P(planted has a repeated abscissa) is
  1.00 (B_real = 2, m = 3), 0.63 (4, 3), 0.34 (8, 3), 0.91 (4, 4), 0.59 (8, 4) and 1.00 (4, 5). The number of
  distinct-abscissa m-subsets of V is C(B, m): 4, 1 and 0 at B = 4 for m = 3, 4, 5.
- **Degrees.** S_n has degree 2^(n−2) in each variable. Its weighted total degree is (n−1)·2^(n−2): 12 for S_4, 32 for
  S_5 and 80 for S_6. The 2026-10-26 study measured "D = 12 is S₄'s own degree"
  (`research/pkm_symmetric_tower_20261026/README.md`). The header table of `semaev_higher.rs`, which gives 8/16/32,
  undercounts. That header also says "Symbolic full expansion of S_6 is infeasible" (`semaev_higher.rs:46`, ~10^8
  terms). `f4_fp` files every pair with lcm degree > `max_degree` under `above` (`f4_fp.rs:3174`), and a run is
  `complete` only when `pairs_above_bound == 0` (`native_solve.rs:87`).
- **Counters.** `field_ops_total` counts row-reduction multiplications only (`f4_fp.rs:116–121`). `native_ops_total`
  is described as "a diagnostic for AGENTS.md section 5a; incomplete by design (pair selection, matrix assembly and
  sorting are not counted)" (`f4_fp.rs:123–129`).
- **Extraction.** Native extraction is a *second* F4 run plus a substitution tree (`native_solve.rs:122–129`; the
  draft instrument calls it "unrestricted domains (a second F4 run, charged here)"). A node whose basis has no
  univariate element branches over all of F_p (`f4_fp.rs:5104`: `(0, Box::new(0..self.p))`, counted as
  `full_field_branches`).
- **Cell rules.** `--min-prime` *replaces* 2^(T·R) (`cell.rs:244`). `t = 3, ratio = 8` silently reuses the L1 cell,
  p = 16801793, with "no eligibility rule applied", unless `--no-l1` is given (`cell.rs:248`). Target and planted RNG
  streams are seeded by the cell slug `t{t}-r{ratio}-s{seed}` (`targets.rs:346, 419`).
- **Plan size.** m = 3: 3 cells × 3 primes × 7 instances × 6 arm rows × 2 engines = 756 calls. m = 4:
  3 × 2 × 7 × 7 × 2 = 588. m = 5: 392. Total 1,736. The m4t4 and m5 blocks alone are 588 calls: 98 h worst case at
  600 s each.
- **Arity-matched gate.** Replacing B^7 by B^(m−1) in the panel A model `W = max(1, p/B^(m−1))·B^γ + B^2` gives the
  gate γ < min((m−1)/2, m−3): 0, 1 and 2 at m = 3, 4, 5, and 3.5 only at m = 8. This is my own desk derivation and is
  labelled heuristic.

## Blocking

**B1. `supported_within_scope` cannot fire in any cell (resolution failure).** *Wrong.*
- *Evidence:* §3 l.110 (3 SAT + 3 UNSAT natural instances per prime); §5 l.141–148 (sign-flip p ≤ 0.05/k at every
  prime); l.151–152 ("fewer than 4 valid instances").
- *Fault:* with n = 6 the smallest attainable p is 0.03125 (0.0156 one-sided). That is above 0.0125 at k = 4 and above
  0.01 at k = 5, so every cell ends `inconclusive` or `within_control_distribution` whatever the data. The
  label-split readings (n = 3) have p_min = 0.25, and a "valid" prime with 4 instances has 0.125. This fails the
  resolution check in §3 of the controlled-experiment skill. A positive control (S13a) or the dry run the skill
  requires would have exposed it.
- *Test:* `2/2**6 > 0.05/5`.
- *Fix (§3, §5):*
  - §3: "5 natural real-SAT and 5 natural real-UNSAT per (cell, prime)."
  - §5: "The exact sign-flip test is computed per stratum on the paired log-ratios pooled over the cell's primes
    (n ≥ 10, p_min 0.00195). The per-prime clause requires each prime's median to have the pooled sign."
  - Replace "a prime has fewer than 4 valid instances" with "a stratum has fewer than 8 evaluable pairs →
    inconclusive (underpowered)".
  - Write the resolution line into `protocol.json` `sample_size.power`.

**B2. `sym-direct` solves a smaller problem than the protocol states, and C1 and the positive control then fail by
construction.** *Wrong.*
- *Evidence:* §1 l.53 and l.55–58 ("identifies permutations"); C1 l.62–63; §6 positive row.
- *Fault:* the condition P_e | X^(2^t) − c forces P_e to be squarefree, because X^(2^t) − c is squarefree for c ≠ 0.
  So decompositions with a repeated abscissa (2P_i, or P_i + (−P_i)) are not solutions of `sym-direct`. The classifier
  counts them, and the planted target contains one with probability 0.34–1.00 (desk facts). Consequences:
  - At **m5t2**, C(4,5) = 0. `sym-direct` returns {1} on every SAT and planted instance. C1 fails, so by §5 the whole
    cell is `invalid_measurement` for *every* arm.
  - At m4t2 only one 4-subset of V exists.
  - The planted control "fails", so `sym-direct` is declared invalid in most cells.
  - Where repeated-abscissa algebraic tuples exist, `sym-direct` gets a discount beyond the m! symmetry.

  The draft instrument already knows this (`present.rs:26–30`, `:434` "exact_on … pairwise distinct abscissae"), as do
  the 2026-10-26 study ("doublings x_i = x_j are not represented") and PKM_GOALS G1 ("Repeated abscissae … explicit
  exclusions"). The protocol text does not.
- *Fix (§1, §3, §6):*
  - §1: "sym-direct is exact only on decompositions with pairwise distinct abscissae. C1 for sym-direct compares
    against the classifier's distinct-abscissa subset."
  - §3: "Planted targets use pairwise distinct abscissae (all arms). sym-direct runs only in cells with B_real ≥ m."
    This drops it from m5t2, and from m4t2 when B_real < 4.
  - §5: "sym-direct's C2 pairs use only instances whose classifier algebraic set has no repeated abscissa; the others
    are reported separately."

**B3. The censoring rule biases C2 toward arms that censor, and lets one censored call invalidate a whole cell.**
*Wrong.*
- *Evidence:* §4 l.133–134 ("Censored calls … are excluded from medians"); §5 l.151 (C1 failure → invalid cell); §3
  l.120–121.
- *Fault:* dropping a censored candidate call removes exactly the instances where that arm was most expensive. The
  surviving median is then biased toward "A cheaper", which is the pre-registered expectation (§5 l.158–160). A
  censored arm also reports no decompositions, which reads as "misses a verified decomposition that another arm
  finds". So any censoring in any arm turns the cell into `invalid_measurement` (and B4 guarantees censoring).
- *Fix (§4, §5):*
  - "A censored call is right-censored at its stop and enters ranks as +∞ for its arm. The pair's sign is + if A
    censored and chain completed, and − if chain censored and A completed. Pairs where both censored are dropped and
    counted."
  - "A median that falls on a censored pair is reported as a bound. An arm–cell with ≥ n/2 censored pairs has no
    point estimate."
  - "C1 is evaluated on completed calls only. A censored call is 'not evaluable' for C1, never a C1 failure."

**B4. The native `max-degree 16` censors the high-degree presentations by construction, not by work.** *Wrong for the
plan; unsupported as a cap.*
- *Evidence:* §3 l.118–121 (cap 16); the arms table l.93–100; §5 C3 l.154–156.
- *Fault:*
  - **mono-\* at m = 4:** the specialised S_5 has total degree ≥ 16 (32 by the weight count), so it starts at or above
    the cap, and every non-coprime S-pair exceeds it.
  - **mono-\* at m = 5:** S_6 (total degree up to 80) cannot even be expanded (`semaev_higher.rs:46`).
  - **\*-direct at t = 4:** LM(x_i^16 − c) = x_i^16, so any non-coprime pair has lcm degree ≥ 17.
  - **sym-direct at m = 5:** S_6 in e has degree up to 16.

  These are the arms the expectations favour. Their degree-cap "lower bounds" are near-zero work (a negative γ "lower
  bound" is possible), so C2 for mono at m ≥ 4 and for direct arms at t = 4 is unreachable on the primary observable.
  The producer's own seed-777 smoke driver uses `MAXDEG=64` (`smoke.sh`), not 16.
- *Test:* run one SAT and one UNSAT per (arm, cell) at `--max-degree 16` and `64` on seed 777, and list every
  `pairs_above_bound > 0`.
- *Fix (§3):*
  - "Native `--max-degree 96` (non-binding; the wall and memory caps censor)."
  - "mono-tower and mono-direct are not run at m = 5 (S_6 not constructible); recorded `not_run: construction`, not
    censored."
  - Add a reachability table (arm × cell → can C2/C3 be reached), as panel A's B4 did.

**B5. "Work" omits input construction, so presentation is confounded with precomputation moved offline.**
*Unpriced.*
- *Evidence:* §4 l.132 (`w_A` = F4 solve + extraction only); §1 table l.50–53.
- *Fault:*
  - mono-\* build S_{m+1} by iterated Sylvester resultants. The target x_R enters, so the cost is per target unless it
    is split (draft `mpoly.rs` header). PKM_GOALS G4 records S5 "rebuil[t] … 9^4 evaluation/interpolation grid for
    each target" and "Construction is outside the reported solver timer".
  - sym-direct adds the leading-term e-rewrite (`algebra.rs:470`) and the per-cell remainder X^(2^t) mod P_e, whose
    coefficient polynomials grow with 2^t.
  - The chain and tower inputs are O(m·t) small polynomials.

  As written, a contracted arm can "win" because its expensive step is outside the meter. For direct arms, the size of
  the degree-2^t membership polynomial is not the issue (it is a binomial); its degree is (B4).
- *Fix (§4):*
  - "Count construction in field multiplications, split into per cell (target-independent: generic S_{m+1}, e-rewrite,
    remainder) and per target (specialisation at x_R or per-target resultants)."
  - "Record input terms and `.ms` bytes per arm. w_A^full = w_A + per-target construction."
  - "C2 supported requires the same sign on w_A and w_A^full."
  - "Per-cell construction is reported once per cell with the declared reuse count" (PKM_GOALS G2 wording).

## Serious

**S1. Extraction is not neutral across presentations, is p-dependent and is partly uncounted.** *Unpriced/wrong.*
- *Evidence:* §4 l.132; `native_solve.rs:122–129`; `f4_fp.rs:5104, 5114`.
- *Fault:*
  - It re-runs the whole F4. That doubles SAT work relative to UNSAT, where extraction is skipped.
  - With unrestricted domains, a node with no univariate element branches over all p values (2^18–2^22 sub-F4 runs,
    counted). In a reduced grevlex basis, three general points in two variables already have no univariate element.
  - Root finding is counted separately (`root_field_multiplications`, log p dependence). Lifting and sign search
    (`GroupOps`) and the factorisation of P_e for sym-direct are separate counters, and the protocol does not say
    which of them `w` includes.
  - The old path's `--extract-max-q 256` makes *whether* extraction runs depend on the arm's quotient dimension
    (chain ≥ mono ≥ sym/m!).

  The obstruction is already recorded in `problems/aps-single-relation-complete-cost.md` and
  `problems/symmetric-pkm-complete-cost.md` (open).
- *Fix:*
  - "Primary w = F4 to the reduced basis (`field_ops`). The extraction reading (every listed counter, GroupOps at a
    stated multiplications-per-addition) is reported, and C2 needs the same sign on it."
  - "Extraction uses finite domains (V for x_i, μ_{2^t} for y_{i,j}). Any `full_field_branches > 0` flags the call and
    removes it from the extraction reading."
  - "Pin `--extract-max-q` above every arm's predicted quotient dimension."
  - Cite and update the two problem records.

**S2. §5a is misapplied.** *Wrong.*
- *Evidence:* l.72–73: "§5a is satisfied by making no cross-method ranking at all".
- *Fault:* C2 *is* a ranking, and §5a covers "every ranking". Presentations move work between counted row operations
  and uncounted symbolic preprocessing (huge S_{m+1} inputs). The engine exposes `native_ops_total` for exactly this.
- *Fix:* "Every call records `native_ops_total`. Uncounted classes (pair selection, matrix assembly, sorting,
  construction) are listed per arm. C2's sign must hold on w + λ·native_ops for λ ∈ {0, 0.1, 1}
  field-multiplication equivalents."

**S3. The permutation control is not an adequate floor.** *Unsupported.*
- *Evidence:* §5 l.144–146; §6 row "permutation".
- *Fault:*
  - It is defined for chain-tower only but used as the floor for every arm. mono-\* and \*-direct are symmetric in the
    leaves, so their own permutation is a byte-identical no-op, while sym-direct's e-variable order is order-sensitive.
  - At m = 3, 2 of the 6 leaf permutations ({id, (01)}: S_3 is symmetric in its first two arguments and the tower
    blocks are identical) give a byte-identical chain system. A third of the rows therefore measure determinism, not
    sensitivity.
  - D_perm is a max over 6 single draws. That is the predecessor's failure mode: one row set the p = 1009 floor
    (REV-…c184b3, C3).
  - The effect of a censored permutation call on the floor is unspecified.
- *Test:* export chain-tower with σ = (01) and `cmp` it against the identity export.
- *Fix (§5/§6):*
  - "Per arm and instance, two seeded nuisance draws: equation-order shuffle plus a variable-order permutation drawn
    outside the system's automorphism group."
  - "Floor for A vs chain = max(D_nuis(A), D_nuis(chain))."
  - "A literal A/A (same file twice, native and msolve) on one instance per cell checks determinism."
  - "Censored nuisance call → floor not evaluable → that arm–cell inconclusive."

**S4. The msolve co-gate measures msolve's genericity choices.** *Unsupported.*
- *Evidence:* §2 l.81–84; §5 l.149.
- *Fault:*
  - `-c` is not pinned. The default `-c 2` reruns F4 with reversed order, then with a random-form variable A, when
    the staircase is non-generic (`MSOLVE_FORMAT.md` §0.2). The leaf-symmetric arms trigger this far more often than
    sym-direct.
  - `work_proxy` sums all passes and omits the parametrization, which is where the m!-smaller ideal pays off.
  - Determinism of the random form is unverified (there is no msolve A/A).
  - A sign-only gate hides magnitude conflicts, and "msolve censored" has no rule.
- *Fix:*
  - "Co-gate on `msolve -t 1 -v 2 -g 2` (single pass, GB only, the native endpoint). `-P 1` is kept only for the
    variety count, and its `passes` are recorded."
  - "Native vs msolve reduced-GB equality on SAT1/UNSAT1 per arm per cell" (panel A's second-instrument gate).
  - "Co-gate not evaluable → inconclusive."

**S5. No variety-size gate.** *Unsupported.*
- *Evidence:* §6 (C1 compares only verified real decompositions).
- *Fault:* an exporter that over-constrains an arm drops algebraic solutions, makes the arm cheaper and still passes
  C1. Panel A adopted this gate (its S1(b)). Here each arm has its own count: chain and tree are ordered with
  identity-prefix exclusions in that arm's order (`present.rs:21`); mono is ordered x-tuples including
  identity-partial-sum tuples; sym is unordered distinct-abscissa subsets. Because the classifier enumerates points
  directly, this gate is independent of the exporter, unlike the predecessor's replay, which shared `descent.py`.
- *Fix (row in §6):* "variety size | every completed call | msolve `-P 1` distinct count and native assignment count
  equal the arm's frozen formula applied to `targets.rs enumerate_alg` | exporter fault: arm invalid in that cell."

**S6. The §0 "not tried" claim is wrong, and prior data are undisclosed in the pre-registration.** *Wrong.*
- *Evidence:* l.35–37.
- *Fault:*
  - `research/pkm_symmetric_tower_20261026/README.md` measured plain chain, plain one-S_4 and symmetric presentations
    of the PKM Kummer system over F_p (p = 786433, `f4_fp`, m = 2, 3). Symmetric was one degree lower with matrices
    ~10× narrower at m = 3. One-S_4 had D = 12, "the worst arm", and aborted at 10 GB at N = 15.
  - PKM_GOALS records PR2650 m = 4 symmetric and plain-S5 stage runs (field work 4.01e5 → 1.08e10 at N = 12 → 16).
    Its G3 is this panel's design ("same distinct-abscissa domain … deterministic A/A … shape-matched nulls. Do not
    pool planted and unconditional targets").
  - The expectation "`mono-*` below `chain-*`" (l.159) contradicts the in-repository prime-field prior.
- *Fix:* rewrite the bullet as "Unfrozen stage measurements exist (list). This panel is their first frozen, paired,
  two-engine replication and extension." Add a `data_seen` provenance block to each expectation, as the predecessor
  did for its P1.

**S7. C3 quotes the m = 8 gate for m ≤ 5.** *Wrong scope.*
- *Evidence:* l.68–70 and l.155.
- *Fault:* 3.5 is the γ gate of the m = 8 model. The same model at arity m gives 0, 1, 2 (desk facts). A secant of 3
  at m = 4 reads "0.5 below 3.5" but is 2 above its own gate. Calling this a "distance" also borrows the
  success-criterion wording ("gap to the matched rho"), and no matched rho exists here.
- *Fix:* "C3 reports γ_A as a number with its spread. It is a descriptive measurement and is compared with no gate. If
  a gate is ever quoted, it is the same model's arity-m gate with its derivation, labelled heuristic, never 3.5."

**S8. The scope text lets m ≤ 5 decide about m = 8, and does not exclude an "improvement" reading.** *Wrong scope.*
- *Evidence:* l.160–162: "…and no effect is the reason not to".
- *Fault:* a null at m ≤ 5 is used to justify keeping the m = 8 presentation, contradicting the sentence before it.
  The effects measured earlier grew with size: 10× narrower at m = 3 in 2026-10-26, and P-256 2×–654× growing with
  chain length.
- *Fix:* "No outcome here decides anything at m = 8. A supported effect is a reason to include that presentation as an
  arm of any future m = 8 protocol. A null at m ≤ 5 is not evidence that presentation is irrelevant at m = 8." Add:
  "No result here is an improvement under the success criterion: chain-tower is not a verified IC baseline pipeline,
  construction and extraction are not fully priced, and no row goes to the leaderboard or `benchmarks/README.md`."

**S9. Pooling SAT with UNSAT mixes two different computations.** *Unsupported.*
- *Evidence:* §5 l.138–139.
- *Fault:* at these (B, m, p), natural real-UNSAT targets are almost always algebraically empty. A non-real algebraic
  decomposition needs a vanishing twist sub-sum, for example a cancelling pair plus a real (m−2)-decomposition. So
  UNSAT rows measure "derive 1" with no extraction, while SAT rows measure "basis with solutions plus a second F4". The
  median of 3 + 3 is the mean of the 3rd and 4th order statistics and describes neither population. UNSAT calls
  dominate a relation search (SAT rate ≈ C(2B_real+m−1, m)/p, about 10^-4 to 10^-6 here).
- *Fix:* "Strata = classifier algebraic count 0 vs > 0 (panel A S2). C2 is decided per stratum (B1 sizes): the
  algebraically empty stratum is the per-call cost, the other the per-relation cost. The pooled reading is descriptive
  only."

**S10. §3 freeze hygiene: the cell rules are not what the text says.** *Wrong/unpinned.*
- *Evidence:* l.102–108; `cell.rs:13–20, 244–261`; `targets.rs:346, 419`.
- *Fault:*
  - t = 3 ratio 7 gives p ≥ 2^21, not "≥ 2^20".
  - t = 3 ratio 8 (the m = 3 third prime) triggers the L1 exception: p = 16801793, reconstructed a, b, g, no
    eligibility rule, status `reconstructed_unverified`.
  - `--min-prime` replaces the ratio floor, so "since 2^(4·5) = 2^20 already" is false and the ratio is only a slug
    label.
  - Both t = 4 cells share slug `t4-r5-s20261115`, which gives identical target and planted RNG streams on two curves.
  - The m = 3 third prime at t = 4 is unspecified.
  - Not pinned: the solver flags (`--max-degree`, `--deadline`, `--extract`, `--extract-deadline`,
    `--extract-max-q`), msolve `-c`, and the 14 `F4_*` environment variables `f4_fp` reads (e.g. `F4_FP_GM`,
    `F4_FP_LINEAR_INTERREDUCE`, `F4_STAIR`, `F4_FP_NARROW`).
  - The analysis script is missing from the §9 freeze hashes. The predecessor froze `analyze.py` by sha256.
- *Fix:*
  - Replace the ratio prose with a literal table of every `cell` command, each with an explicit `--min-prime`,
    `--no-l1` and a distinct ratio label. State that m3t2, m4t2 and m5t2 share one cell.
  - Run every call under `env -i` plus the listed variables, and record `env` per call.
  - "FREEZE.json also hashes `analysis/*.py`, after a dry run on seed-777 scratch output" (skill §3).

**S11. The C3 secant is not paired, and it mixes B growth with a change of instance regime.** *Unsupported.*
- *Evidence:* l.154–156 ("matched instances (same prime rule, same draw index)").
- *Fault:* each (t, prime) has its own p and curve, so "same draw index" pairs unrelated instances. At a fixed 2^18
  floor, log_B p runs 9 → 6 → 4.5, which changes the rate of multi-decomposition and repeated-abscissa targets. The
  F4 counts themselves are p-independent for generic coefficients; the p-dependence enters through extraction (S1)
  and the instance mix.
- *Fix:* "Per prime band, one cell with 16 | p−1 and nested bases g·μ_4 ⊂ g·μ_8 ⊂ g·μ_16 on the same curve with a g
  valid for all t. The same targets are run at t = 2, 3, 4, so γ is a true paired secant." Otherwise report γ as a
  ratio of medians, labelled unpaired.

**S12. The runtime estimate and the cap mechanics do not hold.** *Unsupported.*
- *Evidence:* §7 l.184–187 (2–8 h); §3 l.118.
- *Fault:*
  - The estimate omits m4t4 and m5 (588 calls, up to 98 h). There is no single-thread probe: 266 CPU-s on several
    threads at m4t3.
  - `f4_fp` checks its budget only between steps: a 900 s budget ran 55 min, and an allocation failure exited 134
    with no record (2026-10-26 study). So the 600 s cap must be external, and the end-of-run JSON (the only counted
    record) is then lost. "Censored calls carry lower bounds" is false unless progress is streamed.
  - There is no lost-row rule for a container or session loss.
- *Fix:*
  - Single-thread re-probe of chain-tower plus one candidate at m4t3, m4t4 and m5t2. Write a per-cell fixed +
    per-call estimate.
  - "`timeout 600` external; F4Progress (`field_multiplications`) streamed to the call log; the last record is the
    lower bound."
  - Panel A's lost-row rule.
  - Optionally run 3 counted calls in parallel on pinned cores (counts are host-independent; CPU stays secondary).

**S13. Controls the skill requires are missing.** *Unsupported.*
- *Evidence:* §6.
- *Fault:*
  - (a) There is no positive control for the C2 detector. The planted row tests solution finding, not the statistic.
    Precedent: the generator-count ladder of `RESEARCH_PKM_TOWER_ORACLE.md` §5.3.
  - (b) There is no null separating the presentation effect from a Kummer-structure effect. The same table's "random
    set" control (V a random 2^t-subset, membership interpolated) is the shape-matched null for this question.
  - (c) There is no literal A/A (see S3).
  - (d) There is no reference row. The exhaustive enumeration solves the same decomposition problem, and in-house F4
    needs 10^3–10^7 more operations than a brute grid on PKM systems (`research/ic_sweeper_20261024/README.md` §2.5,
    via the LEAD-ICP record). Magma F4 is not named as missing (§15).
- *Fix (rows in §6):*
  - "positive | chain-tower + k redundant generators (random multiples of inputs, ideal unchanged), m = 3 cells |
    supported, positive sign, every prime | statistic blind: no C2 verdict."
  - "null | chain-dense and mono-dense (membership ∏_{w∈W}(x−w), W a seeded random B-set) on planted and
    natural-UNSAT targets at m = 3, 4, one prime | d(mono-dense) vs d(mono-direct) | equal → the effect is generic
    contraction, not Kummer structure."
  - "reference | classifier per-target enumeration cost in field multiplications | ratio column."
  - Name Magma as the missing external tool.

## Minor

- **M1. msolve crash band misstated** (l.83–84). *Wrong.* Crashes were seen at 65537, 65557 and 131101, i.e. in
  [2^16, 2^18). `MSOLVE_FORMAT.md` §0.4 lists 10007, 32771 and 65521 as fine, and the predecessor ran 1009, 4099 and
  32749. *Fix:* "crashed on every tried prime in [2^16, 2^18)". Optionally add an m = 3 prime ≥ 2^15 (with
  `--min-prime`, under the crash rule) for a real p spread and a bridge to the predecessor's band.
- **M2. Familywise error.** About 38 (cell, arm) verdicts plus readings, each family corrected only within its cell.
  *Fix:* "each cell is its own family; the count of supported cells is not a test."
- **M3. Bootstrap and ties.** At n ≤ 10 with near-constant ratios (predecessor spread 0.0005) the percentile interval
  degenerates to the extreme order statistics or to zero width. *Fix:* "the interval is descriptive; the sign-flip
  test decides; zero differences are dropped and counted."
- **M4. Fixed arm order** (chain-tower always first). *Fix:* add the reason (counts are order-invariant; CPU is
  secondary) or randomise per instance.
- **M5. "So that the prime-field rows are comparable"** (l.31). The metric (Σ rows·cols·density vs dense rows·cols)
  and the primary engine both differ. *Fix:* "the verdict vocabulary is the same; the metrics are not."
- **M6. Checkpoint gate.** The predecessor ran with `gate_rc=1` and an unverifiable waiver (REV-…c184b3), and this
  container class has no S3 token (2026-10-26 study). *Fix:* "the S3 push succeeds or a committed owner waiver is in
  the study directory before the first row."
- **M7. ICV1.** Name and register every derived curve by its ICV1 slug in RESULTS and in the pcc claims (AGENTS §11).
- **M8. Draw cap.** m3t2 at 2^22 with B_real = 2 has P(< 3 SAT in 10^6 draws) ≈ 0.15, and P(B_real = 2 | B = 4)
  = 6/11. *Fix:* give the rule (next eligible prime, recorded).
- **M9. Route.** Write why this runs locally rather than on the isolab-eks default (budget rule, zero spend, L0 CPU),
  and attach the `bench.record/v1`.
- **M10. Freeze record.** FREEZE.json must name the commit that holds the frozen bytes (predecessor correction-1,
  item 2). A §9a-style list must disclose everything seen before the freeze, including the seed-777 smoke and its
  `MAXDEG=64`.

## The dispatch questions, answered by finding

| question | findings |
|---|---|
| (1) fair work across semantics; what to charge | B2 (sym's distinct domain), B5 (construction per cell and per target, input size), S1 (extraction), S2 (native_ops band), S5 (variety gate) |
| (2) permutation floor; msolve co-gate | S3, S4 |
| (3) power, ties, bootstrap, pooling | B1, S9, M2, M3 |
| (4) p-dependence at ≥ 2^18 | S1 (full-field branches, log p root finding), S11 (log_B p regime), M1 (crash band) |
| (5) caps and censoring | B3, B4, S12 |
| (6) scope, ADVANCE, C3 wording | S7, S8 |
| (7) predecessor flags repeated | S3 (max-row floor), M6 (checkpoint), M10 (freeze commit), M5 |
| (8) missing controls | S13, S3 (A/A), S5 |
| (9) freeze hygiene | S10, B4, S12, M10 |
