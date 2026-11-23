# Scratch: moonshot proposals for prime-field ECDLP (Idea Generator, 2026-11-10)

Status: **proposals only.** Nothing here was run. None of these records is a result, a
hypothesis status change, or a task assignment. The main session will move or delete this
file. Every number quoted from the repository is `[pcc:unverified]`: none of the sources
below carries a pcc claim id (the atlas survey says the same of the lead atlas).

## 0. Grounding

### 0.1 What I read (exact paths)

- `AGENTS.md`: "Preserve scope and report evidence" (l. 392), "Name the obstruction"
  (l. 436), "Mathematical results should include Lean proofs" (l. 716), "Implementation
  language: no Python" (l. 756), "The rule: boundary, table, ratio" (l. 983), "Knowledge
  base first" (l. 70).
- `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §0–§2 and §4, including §4.6.
- `research/prime_s5_small_root_leads_20261028/README.md` §0 and §5 (killed table K1–K16).
- `research/split_prime_ecdlp_20261107/README.md` §3 (scalar-only theorem) and §6/§6a.
- `research/prime_field_lead_assessment_20261107/atlas_survey.md` §A, §B and §D. This is
  the lab's own "do not re-propose" list.
- `research/prime_ic_lead_review_20261031/REPORT.md`. It covers where SAT belongs, the
  arity gates `e < m/2 − 1` and `a < m/2`, and the record that cvc5 returned unknown at
  120 s on a 20-bit case.
- `research/prime_shared_coefficient_sat_20261031/README.md`. Its native emitter lowers a
  quadratic F_p IR to CNF, CNF-XOR, cvc5 `QF_FF` and msolve.
- `research/pdp_ml_sat_ceiling_20261027/README.md`. Binary CDCL+XOR behaves as an
  enumerator: 0.963 and 0.998 conflicts per sorted triple, and the learned triage AUC is
  at chance.
- `research/ml_learnability_control_20261107/RESULTS.md`. It found no SIGNAL in 24 cells.
  At m=20 the pooled accuracy is 0.4991 [0.4978, 0.5004] with n=600,000.
- `research/hashed_ecdl_tensor_audit_20261107/AUDIT.md`, which covers the Bochum
  tensor-contraction claim.
- `research/p2_lift_linear_complexity_20261110/PROTOCOL.md`, today's F2 run.
- `research/curve25519_autolab_20261104/ideas/NONCOLLISION-20261108.md` §1.

I confirmed that these tools exist on disk:

- `src/bin/ecbench.rs`
- `tools/task_card.py`
- `examples/prime_ecdlp_breakthrough_probe.rs` (Probe B/C lattices)
- `research/prime_shared_coefficient_sat_20261031/native/` (Rust emitter and hash-pinned
  CaDiCaL/CryptoMiniSat runners)
- `research/p2_lift_linear_complexity_20261110/tool/` (std-only Rust)
- `research/split_prime_ecdlp_20261107/tools/splitcount.rs`

### 0.2 Knowledge base and atlas

- **research-kb MCP: not available to this subagent.** No `kb_status` or `search_knowledge`
  tool was exposed. The repository-path citations and the web searches below replace it.
  The main session should run the KB protocol before any record is promoted to a lead.
- **Lead atlas:** the checkout path named in the survey
  (`/home/user/isogeny-labs/crypto-autoresearcher`) does not exist on this machine. I used
  the survey of it instead (`research/prime_field_lead_assessment_20261107/atlas_survey.md`,
  2026-11-07).

### 0.3 Literature searched (web; abstracts unless stated)

- Trimoska, Ionica, Dequen, ePrint 2019/313: SAT/WDSat for binary-field PDP.
- Béjar, Fernández, Guitart: SAT encodings for factoring and discrete log. They are worse
  than specialized algorithms, and elliptic curves are left as future work.
- Galbraith and Gebregiyorgis, ePrint 2014/806.
- Ozdemir et al.: "Satisfiability Modulo Finite Fields" (ePrint 2023/091) and "Split Gröbner
  Bases" (ePrint 2024/572, CAV'24, in cvc5).
- Wroński et al.: QUBO/annealing for ECDLP. The direct method reached F_7; Semaev QUBO with
  D-Wave hybrid reached F_251 (ICCS 2021); twisted Edwards reached F_1021.
- Corrigan-Gibbs and Kogan, ePrint 2017/1113: `S·T² = Ω̃(εN)` for generic algorithms.
- **Corrigan-Gibbs, Henzinger, Wu, ePrint 2026/384, "The Structured Generic-Group Model"**
  (abstract fetched). An algorithm that exploits structure on at most a δ fraction of
  elements needs `Ω(min(√q, 1/δ))` time. The paper claims analogous bounds for
  elliptic-curve points. I did not read the body.
- Cremona and Thongjunthug: complex AGM and elliptic logarithms.
- Silverman, "Lifting and Elliptic Curve Discrete Logarithms" (SAC 2009; abstract only).
- Kaliski and Boneh–Shparlinski: hardcore bits.
- Cited from memory, not re-fetched (marked `[recalled]` where used):
  - Akavia–Goldwasser–Safra on hardcore predicates via list decoding;
  - Kushilevitz–Mansour sparse Fourier;
  - Kohel–Shparlinski exponential sums on elliptic curves;
  - Merel, Oesterlé, Parent and Derickx–Kamienny–Stein–Stoll on torsion over degree-d fields;
  - Green–Tao U³ inverse theorem;
  - Ercsey-Ravasz–Toroczkai analog SAT;
  - Bryant ROBDDs.

### 0.4 The three generic barriers every record must name its escape from

- **G-Shoup.** A generic algorithm sees only label equalities, so its view is simulable by
  linear polynomials in k, which gives `T ≳ √n`. It is escaped only by branching on the
  *encoding*: bits, size, characters or algebraic relations of coordinates.
- **G-CGK.** Generic algorithms with target-independent advice obey `S·T² ≳ n`. A
  non-generic algorithm escapes it the same way.
- **G-CHW.** This is the newest barrier and the sharpest test. If a method's advantage
  comes from structure on a δ-fraction of elements, it costs `≥ min(√q, 1/δ)`.
  - For index calculus, δ is the fraction of *decomposable* elements, ≈ `B^m/(m!·q)`.
  - Every record must therefore exploit structure on a set of density `δ ≫ q^{−1/2}` with
    a fast membership/decomposition test, or sit outside the model.
  - Whether CHW's elliptic-curve-point model covers x-coordinate decompositions is
    unverified until the body is read.

A fourth barrier recurs below. I call it **W (Weil/Bombieri)**: every function of x of
algebraic degree d has all Fourier coefficients along `⟨G⟩` bounded by `≈ 2d√p`.
Character and statistic levers therefore need either `d ≳ N^{1/4}` (derivation in D1) or
non-algebraic structure.

## 1. Levers I did not turn into records (killed on paper here; one line each)

| lever | why it dies (derived here unless cited) |
|---|---|
| SAT/SMT on k's bits where the theory evaluates the ladder exactly on partial scalars (no bit-blasting) | Its only theory is group operations on partially fixed scalars, so it is generic and G-Shoup applies verbatim. It is BSGS/MITM in CDCL clothing. Non-genericity can only enter through bit-level field clauses, which is why A1 bit-blasts. |
| McCormick/LP relaxation of the integer lift `S(x) − p·q = 0` | The LP prunes a box X only if the real range `S(X)` misses `pZ`. With `x_R ≈ p`, `∂S/∂x_i ≳ p` on S₃, so any box holding two integer points per axis has a range above p. Pruning therefore happens only at single-point boxes, which is enumeration. |
| Lagrangian/Fourier dual bound for branch-and-bound | The exact Lagrangian dual of "S ≡ 0 mod p" over a box is `#roots = (1/p)Σ_t Σ_{x∈box} e(tS(x)/p)`. Deligne/Weil bounds the incomplete sums by `≈ C p^{m/2} log^m p`, so the dual estimate is informative only when `vol(box) > p^{1+m/2}`. That is larger than the whole space for m ≥ 2. The bound never prunes. |
| Tensor-network contraction in the Fourier/character bond basis | The S₃ incidence tensor `1[S₃(x₁,x₂,x₃)=0]` has `T̂(0)=p²` and `|T̂(t)| ≈ p` for all `p³−1` nonzero t (Deligne for surfaces): a flat spectrum. The Fourier basis is an invertible basis change, which the atlas says cannot lower ranks (`LEAD-EXO-holant-decomposition-count-network`, closed_dominated; QTT rank 1024 vs random 1023–1024 at p=1021). |
| Heath-Brown/Salberger determinant method with an auxiliary prime ℓ (ℓ-adic clustering of small roots) | Fixing the residue class mod ℓ shrinks the box volume by `ℓ^m` at an enumeration cost of `ℓ^m`. That is neutral by box-volume invariance (F4, `prime_s5_small_root_leads_20261028` §3.3), and the determinant method is Coppersmith's method (F1/F2). |
| Non-torsion global lift + archimedean decoding | If G̃ has infinite order, the lift `[k]G̃` has canonical height `k²ĥ(G̃)`, so its coordinates have `≈ k²ĥ` digits. LLL cannot find them. This is the xedni height wall (program §4.6; JKSST). C1 takes the only other branch, torsion lifts. |
| Sparse-FFT hidden shift with an algebraic g of bounded degree | W bounds every coefficient by `2d√p`, so no heavy coefficient exists. D1 takes the escape branch (`d ≳ N^{1/4}`). |
| Regev-style multidimensional exponent lattices | There is no small-norm generating set in `E(F_p)`, and the quantum part is essential. This is not a classical lever. |

## 2. Records

Each record names the obstruction it attacks, its escape from G-Shoup, G-CGK and G-CHW,
its Lean obligation, and §5a pricing. The pricing rule is the same everywhere: unpriced
native work in every arm (conflicts, propagations, sweeps, BDD node operations, FFT
butterflies, hash/DP work in rho) is priced at w ∈ {0, 0.1, 1} GAE per operation. Two arms
are ranked only where the order holds across that band. The reference is always ecbench
`rho.negation` on the same curves. Its toy S is ≈ 1.29–1.45 (atlas
`LEAD-RHO-matched-rho-calibration`).

---

```yaml
id: MOON-A1-ladder-cnf-clause-reuse
title: "Bit-blasted Montgomery-ladder SAT for x([k]G) = x_Q with k's bits free, and learned-clause reuse across targets"
family: (a) SAT/SMT/CDCL
obstruction_attacked: >
  G-Shoup/G-CGK. A generic algorithm cannot read coordinate bits. A CDCL solver on the
  bit-blasted F_p circuit of the ladder learns clauses about the bits of intermediate field
  elements. Those clauses are not label equalities, so the GGM simulation does not cover
  them. Clauses learned without target assumptions are target-independent advice.
claim: >
  On prime-order toy curves with b = log2 r ∈ {10,12,14,16,18,20}, the median CDCL conflict
  count to solve x([k]G) = x_Q for uniform targets grows as 2^{α b} with α < 0.5. That is,
  the solver beats rho's exponent. Separately, retaining learned clauses across 32
  sequential targets on one curve cuts the median conflicts of target 32 by ≥ 2x against
  target 1.
mechanism: >
  The ladder is a fixed straight-line program of ~b steps. Each step has ~6 F_p
  multiplications, which bit-blast to O(b²) gates, so the circuit has O(b³) gates. Its only
  free inputs are k's bits. If the multiplication circuits mod p have bit-level regularities
  that resolution can exploit (carry patterns of reduction mod a fixed p), learned clauses
  would prune scalar space faster than 2^{b/2}. Over F_p there is no subfield; the
  "integers mod p" structure enters only through the carry/reduction gates. This test
  measures whether that structure is resolution-visible at all.
prediction: >
  α ∈ [0.9, 1.4] (an enumerator or worse; the binary PDP precedent is 0.96–1.0 conflicts per
  candidate). The reuse ratio (target 32 / target 1 median conflicts) is in [0.8, 1.1].
  QF_FF (cvc5 split Gröbner) times out at b ≥ 14 at 600 s, consistent with the recorded
  unknown at 120 s on a 20-bit Semaev case. P(α < 0.5) = 0.001. P(reuse ≥ 2x sustained to
  b = 20) = 0.02.
discriminating_test: >
  1. Write a Rust generator (≈ 200–300 lines) that emits the x-only Montgomery ladder for
     (curve, G, x_Q) as the quadratic F_p IR of
     `research/prime_shared_coefficient_sat_20261031/native`. The scalar bits are Boolean
     variables, b_i² = b_i.
  2. Reuse that crate's deterministic Boolean lowering and its hash-pinned runners:
     - CaDiCaL on CNF, incremental with assumptions on x_Q's bits for the reuse arm;
     - CryptoMiniSat on CNF-XOR (Gauss–Jordan over F_2);
     - cvc5 QF_FF on the unlowered IR.
  3. Run 32 targets × 6 sizes × 3 encodings on curves drawn by ecbench (prime order,
     cofactor 1).
  4. Check every SAT model by scalar multiplication. Check UNSAT on a planted wrong x_Q
     with a DRAT proof.
  5. Fit log2(conflicts) against b by OLS and report a bootstrap CI.
  Budget: about 1.5 days of code and ≤ 6 CPU-hours; b = 20 dominates at ~10^6 conflicts.
falsification: "Lower 95% CI of α ≥ 0.6 and reuse ratio ≥ 0.5 at b = 18 and 20 → close the lever (bit-blasted ladder CDCL is at or above enumeration)."
cost_band:
  candidate: "conflicts + propagations priced at 0 / 0.1 / 1 GAE each; the circuit build is charged once per curve; one GAE ≈ 1 group addition from ecbench's counter"
  reference: "ecbench rho.negation on the same 6 curves, 32 targets, DP/hash work priced at 0 / 0.1 / 1 GAE"
  baseline: "exhaustive scalar enumeration (r/2 GAE with the negation fold)"
controls: >
  - Positive: plant k in [0, 2^{b/2}) by fixing its top bits. The solver must then finish
    in ~2^{b/2}·c conflicts; this calibrates the per-bit conflict rate.
  - Negative: a random F_p straight-line program of the same shape and gate count with no
    group law. It must give the same α if the ladder carries no exploitable structure.
  - Encoding control: the CNF-XOR and CNF arms must agree on the model set.
risks: >
  - Circuit size makes b > 24 impractical.
  - Solver nondeterminism: fix seeds and report the spread.
  - "Fewer conflicts" can be bought with costlier propagations, so the band pricing is
    mandatory.
  - The CNF lowering uses a Θ((log p)²) schoolbook multiplier. This is not a lower bound on
    Boolean multiplication (lead review §Result).
closest_baseline: >
  - Literature: Béjar–Fernández–Guitart (SAT for DL in cyclic groups; worse than specialized
    algorithms; EC left open) and Trimoska–Ionica–Dequen 2019/313 (binary PDP, not the
    ladder).
  - In repository: pdp_ml_sat_ceiling (binary CDCL = enumerator) and the lead review's
    recorded cvc5 unknown at 120 s.
  - Difference: the scalar-multiplication circuit itself over F_p, with the scalar as the
    only unknown, and a cross-target clause-reuse arm. I found no prior instance on disk or
    in the search.
escape: >
  - G-Shoup/G-CGK: escaped by construction (bit-level reads).
  - G-CHW: δ = 1, since every ladder point is read, so CHW does not bind.
  - The real barrier is the resolution proof complexity of a pseudorandom circuit. The
    expectation is exponential, and that is what α measures.
lean_obligation: >
  Lean statement LadderIR.sound_complete: for a curve with a·b ≠ 0 and odd prime-order G,
  the IR's solution set projects onto {k ∈ [0, 2^b) : x([k]G) = x_Q}. Correctness also has
  a per-instance certificate (model replay). Run search_theorems for an existing
  x-only-ladder invariant in crypto-lean before writing one.
prior: "0.001 (exponent); 0.02 (reuse ≥ 2x as a labelled multi-target component)"
```

---

```yaml
id: MOON-A2-cdcl-macaulay-gauss-mod-p
title: "CDCL(T) with a Gaussian-elimination-mod-p propagator on the degree-D Macaulay matrix of the m = 3 chained-S3 system"
family: (a) algebraic SAT (linear-over-F_p-aware)
obstruction_attacked: >
  - Binary CDCL+XOR decomposition solvers are enumerators (0.963–0.998 conflicts per sorted
    triple; pdp_ml_sat_ceiling).
  - The prime-field `F_p`-trail theory in the lead review propagates only evaluated
    constraints.
  - This propagator instead runs XL-style elimination: after each decision on digit bits it
    computes the F_p-linear span of all degree-≤ D multiples of the residual equations and
    reads off forced digits and conflicts. It is the F_p analogue of WDSat's Gauss–Jordan
    module, lifted from XOR to the Macaulay span.
claim: >
  On the one-hot, disjoint-window chained-S3 m = 3 predicate (lead review, rank 2), with
  p ∈ {2^12, 2^14, 2^16, 2^18, 2^20} and B ≈ p^{1/3}, the Macaulay propagator at D = d_ff
  brings conflicts per sorted factor-base triple below 0.3. Its total charged field
  operations per triple, priced at the band, give a cost exponent e < 0.5 in s = log2(p)/3.
  That is the lead review's m = 3 gate.
mechanism: >
  Unit propagation sees one equation at a time. Over F_2, Gauss–Jordan on XORs is what lets
  WDSat refute large blocks. Over F_p with Boolean digit variables, linear combinations of
  degree-D multiples can expose a forced digit long before any single equation does. The
  known digit-presentation first-fall degree is
  d_ff = m·2^{m−1} + ⌊(s − 2^{m−1})/2⌋ + 1 (atlas `LEAD-ICP-digit-presentation-solving-degree`).
  At D ≥ d_ff the span contains degree-falls, so partial assignments could trigger them.
prediction: >
  Conflicts per triple ∈ [0.3, 1.2]. Field operations per triple ≥ 10^3, because elimination
  dominates. The cost exponent e ∈ [0.9, 1.15], i.e. enumeration with a large constant.
  P(e < 0.5) = 0.005. P(conflicts per triple < 0.3 at any size) = 0.1; that outcome would be
  a pruning signal even if cost-dominated.
discriminating_test: >
  - Add the propagator as a fourth arm of the lead review's frozen m = 3 tournament, not as
    a new study. Reuse the E1 IR and DRAT/proof-replay runners from
    `prime_shared_coefficient_sat_20261031/native`, with CaDiCaL's external-propagator
    (IPASIR-UP) interface.
  - Write a dense F_p row-reduction in Rust; Macaulay matrices at D ≤ 6 and s ≤ 7 fit in
    memory.
  - Run 64 planted and 64 random targets per size and report conflicts per sorted triple
    and field operations per triple.
  - Budget: about 3 days of code and ≤ 10 CPU-hours.
falsification: "Lower CI of e ≥ 0.7, or charged field operations per triple ≥ 100x exhaustive enumeration at every size → close the Macaulay-propagator arm."
cost_band:
  candidate: "conflicts, propagations and F_p row operations priced at 0 / 0.1 / 1 GAE; one F_p multiply-add = 1/63 GAE is NOT assumed; use the measured field-mult per GAE ratio on the same host"
  reference: "rho.negation on the same curves at band; m = 3 pair-table IC row (Table B) as baseline"
  baseline: "exhaustive sorted-triple enumeration with the 2-sum table"
controls: >
  - Same propagator on the curve-free null system: same monomial support, random
    coefficients (as in EV-PFDR-1394a4).
  - D = 1 arm, which reduces to the plain linear theory.
  - Plain CaDiCaL CNF arm.
risks: >
  - This duplicates part of the planned CDCL(T)/F_p work. Coordinate with the owner of
    cryptanalysis PR #1347 and run it as an arm only.
  - Macaulay cost explodes with D.
  - A "lower conflicts" result can be an accounting artefact if propagator time is unpriced.
closest_baseline: >
  - Literature: WDSat (Trimoska et al. 2019/313; Gauss–Jordan XOR over F_2) and Ozdemir et
    al. split Gröbner bases in cvc5 (2024/572; full GB per theory call).
  - In repository: the lead review's CDCL(T)/F_p draft.
  - Difference: a degree-bounded Macaulay span as the propagator. It is cheaper than a full
    Gröbner basis and stronger than single-constraint evaluation.
escape: >
  - G-Shoup/G-CGK: escaped (algebraic reads).
  - G-CHW: δ is the decomposable fraction, ≈ B³/(6p) ≈ 1/6. The binding requirement is the
    lead review's e < 0.5 together with a < 1.5 setup.
lean_obligation: "MacaulayProp.sound: any literal implied by an F_p-linear combination of degree-≤ D ideal members holds on every common zero (with b² = b). This is short but must match the exact row semantics."
prior: "0.005"
```

---

```yaml
id: MOON-B1-milp-beyond-howgrave-graham
title: "Interval/MILP branch-and-bound on the integer-lifted reduced Coppersmith polynomials, beyond the Howgrave-Graham bound"
family: (b) constraint/MILP relaxation
obstruction_attacked: >
  - The lattice reach shortfall: 1/70.8 for S5 against 1/5 needed, and δ ≈ 1/313 at
    feasible dimension (prime_s5_small_root_leads §0).
  - Today, pushing past the bound means guessing high bits, which costs one LLL per guess.
  - Below the bound, each reduced polynomial h_j satisfies h_j(x0) = 0 over Z.
  - Above it, h_j(x0) = p^ℓ·q_j with a small integer quotient |q_j| ≤ K_j. That is a
    mixed-integer system over (x, q) whose real relaxation is non-vacuous, unlike the raw
    modular one (§1). B&B can share one reduced basis across all guesses and prune jointly
    over r polynomials.
claim: >
  For S3 (m = 2) at p = 2^40, with Probe C's level-2/3 symmetric lattices, interval B&B over
  (x1, x2, q_1..q_r) on the r shortest reduced polynomials finds every planted root in boxes
  X = X_HG·2^j, j = 1..8. Its node count grows as 2^{2γj} with γ < 0.9; naive bit-guessing
  is γ = 1.
mechanism: >
  The r reduced polynomials are different integer combinations of (S, p)-shifts. Their real
  zero sets in a sub-box are different curves, so jointly constraining all r of them could
  prune sub-boxes that no single polynomial prunes. If joint pruning outruns the growth of
  each K_j ~ 2^{j·deg h_j}, the effective reach rises without a larger lattice.
prediction: >
  γ ∈ [0.95, 1.05] (derived expectation). A sub-box larger than the HG cell gives each h_j a
  range above p^ℓ, so the relaxation is non-pruning there. The constant-factor gain over
  guess-and-LLL is in [10, 1000]x, from one LLL instead of 2^{2j} LLLs. That is a
  component gain on a lattice oracle that is not part of the IC baseline. P(γ < 0.9) = 0.02.
  P(the effect moves S5 materially toward δ = 1/5) < 0.001.
discriminating_test: >
  - Extend examples/prime_ecdlp_breakthrough_probe.rs (Probe C lattices) with a native
    interval-arithmetic B&B over i128/rug integers.
  - Use 40-bit prime-order curves and 64 planted roots per j for j = 1..8.
  - Report nodes, solved fraction, wall-clock time, and a guess-and-LLL control at the same j.
  - Budget: about 1 day of code and ≤ 4 CPU-hours.
falsification: "γ lower CI ≥ 0.9 on j = 4..8 → close (beyond-bound B&B is bit-guessing with a cheaper constant)."
cost_band:
  candidate: "B&B nodes and interval evaluations priced at 0 / 0.1 / 1 GAE; the one LLL charged in measured bit-ops converted at the measured ratio"
  reference: "guess-and-LLL at the same j (component reference); rho.negation listed for the ratio column, not as a pass/fail gate"
controls: >
  - Random-polynomial lattice with the same Newton polytope, as in L4-C.
  - j = 0 (within bound), where B&B must match one-shot root finding.
  - Planted roots at box corners versus centres.
risks: >
  - A component result on an oracle that is ≥ 63x short; it cannot change the program's
    conclusion on its own.
  - Interval overestimation can mimic non-pruning; use affine arithmetic as a check.
closest_baseline: >
  - Literature: May's "exhaustive search on the bits beyond Coppersmith" (recalled) and
    Jochemsz–May.
  - In repository: Probe B/C; smallroot_optimal_reach_20261026 (strategy-independent bound
    on (p, S)^t lattices).
  - Difference: the bound covers lattices, not lattice-plus-B&B over quotients. This record
    tests whether the extra integer layer buys exponent.
escape: >
  - Outside G-Shoup/G-CGK (algebraic).
  - G-CHW does not bind a single decomposition oracle, but the §2.4 cost bridge does: the
    gain must reach T_dec ≪ p^{1/4} at m = 4 to matter.
lean_obligation: "BeyondHG.complete: if h ∈ (S, p)^ℓ and |h(x)| < p^ℓ(K+1) on the box, then every root x0 of S mod p in the box has h(x0) = p^ℓ·q with |q| ≤ K. This is a short arithmetic lemma that makes the B&B complete."
prior: "0.02 (γ < 0.9); <0.001 (program-relevant)"
```

---

```yaml
id: MOON-C1-archimedean-torsion-lift-decoder
title: "Archimedean decoding through a global torsion lift: an LLL-short lift of x(Q) plus the complex elliptic logarithm, and the Merel–Oesterlé boundary it sits on"
family: (c) real/complex analytic side (℘, heights) — also a (g)-grade wild idea
obstruction_attacked: >
  - Every lift-based route found so far either loses the homomorphism or carries height
    k²ĥ (xedni).
  - The only height-zero lifts are torsion lifts. If G lifts to a torsion point G̃ of order
    N on Ẽ over a number field L of degree d, then:
    - the reduction is injective on ⟨G̃⟩;
    - x([k]G̃) are algebraic integers of bounded house X0;
    - ℘ / C/Λ make k ↦ x([k]G̃) explicit over C.
  - So x(Q) mod 𝔓 can be lifted by LLL in O_L ≅ Z^d to its unique short representative,
    and k can be read off from the complex elliptic logarithm.
claim: >
  (A, a theorem to prove.) Suppose 𝔓 | p has degree 1, Ẽ/L has good reduction at 𝔓 with
  Ẽ mod 𝔓 ≅ E, G̃ has order N and reduces to G, every x([k]G̃) is integral with house
  ≤ X0, and (2X0+1)^d < p. Then ECDLP in ⟨G⟩ is solvable in poly(d, log p, log N).
  (B, the moonshot.) For prime-order E with N ≈ p ≥ 2^20 such data exist with d ≤ log p.
mechanism: >
  - The competitor count is ≈ (2X0+1)^d/p: the number of points of x(Q) + 𝔓 in the
    house-X0 box.
  - Merel–Oesterlé `[recalled]` gives prime torsion N ≤ (3^{d/2}+1)², so d ≥ ≈ log_3 N.
  - With X0 ≥ 1, uniqueness needs 3^d ≤ (2X0+1)^d < p ≈ N ≤ 3^d + 2·3^{d/2} + 1.
  - So the decoder can work only in a relative window of width O(3^{−d/2}) and only with
    X0 = 1 exactly. By Kronecker that means root-of-unity or zero x-coordinates.
  - Known maximal prime torsion grows far below 3^d `[recalled, DKSS]`:
    d=1: 7, 2: 13, 3: 13, 4: 17, 5: 19, 6: 37, against 3^d = 3, 9, 27, 81, 243, 729.
  - Independently, the lift must hit the given point (E, G) of X_1(N)(F_p). X_1(N) has
    genus ~N²/24, so that is impossible below its gonality except at finitely many sporadic
    points.
prediction: >
  (B) is false at every d where the torsion list is known. The decoder applies only to
  p ≲ 40 at d ≤ 6. (A) holds, and the toy decoder recovers k mod N in 100% of trials for
  N ∈ {5, 7} over Q and N = 13 over a quadratic field, with p ≈ 2^30 (N-part only).
discriminating_test: >
  1. Zero compute: tabulate maxprime(d) against ⌊log(p)/log(2X0+1)⌋ from the published
     torsion lists. Have a second reader re-fetch them; my numbers are recalled.
  2. Native Rust toy (≈ 1 day; the period lattice comes from the complex AGM of Cremona and
     Thongjunthug):
     - LLL lift of x(Q) in Z (d=1) and Z[√D] (d=2);
     - complex elliptic log;
     - read off k mod N;
     - 1,000 targets per (N, p).
     This checks (A)'s correctness and calibrates X0 for real torsion families.
falsification: "(B) dies if every published (d, maxprime(d)) has (2·X0_min+1)^d ≥ maxprime(d)+2√maxprime(d)+1 with X0_min the smallest measured house. The predicted outcome is yes at all d ≥ 2. (A) dies if the toy decoder fails on any valid instance."
cost_band:
  candidate: "toy decoder: LLL bit-ops and AGM iterations priced at 0 / 0.1 / 1 GAE; boundary diagnostic, no pipeline S"
  reference: "rho.negation on the N-part (trivial at N ≤ 13); the comparison is the uniqueness inequality, not a speed ratio"
controls: >
  - Wrong-lift control: a random integral lift with the same house distribution must fail
    uniqueness.
  - Competitor-count check: count the coset points in the box by enumeration at d = 2.
risks: >
  - Torsion bounds recalled from memory (flagged).
  - The "window" argument assumes X0 ≥ 1, which holds for nonzero algebraic integers.
  - (A) is a reformulation of reduction-injectivity plus archimedean uniqueness. Its
    novelty is in the framing and the boundary, not the pieces.
closest_baseline: >
  - Silverman's lifting survey (SAC 2009): global lifts are infeasible.
  - Smart's attack: the p-adic counterpart, which needs N = p.
  - Repository: xedni/JKSST (program §4.6, K10/K11) and split-prime §3 (the ray-class-field
    labelling ambiguity).
  - Difference: this record isolates the one height-zero archimedean route and states its
    death as an explicit inequality between Merel–Oesterlé and lattice-coset uniqueness.
escape: >
  - Reads algebraic lifts, so it is outside G-Shoup and G-CGK.
  - G-CHW: δ = 1.
  - The barrier it must escape is arithmetic: uniform boundedness of torsion together with
    the gonality of X_1(N).
lean_obligation: >
  - TorsionLiftDecoder.correct: under the hypotheses of (A), the decoded k satisfies
    [k]G = Q.
  - CosetCount: the number of α ∈ x + 𝔓 with every |σ(α)| ≤ X0 is ≤ (2X0+1)^d/N(𝔓) + O(boundary).
  - Merel–Oesterlé enters as an explicit hypothesis, not a Lean proof.
prior: "<0.001 for (B); (A) is near-certain and is a theorem, not a lever"
```

---

```yaml
id: MOON-D1-weil-escape-dickson-spectrum
title: "Cheap maps of degree ≥ N^{1/4} (Dickson D_n, n huge) escape the Weil bound: does χ(D_n(x([k]G))) have a heavy Fourier coefficient in k?"
family: (d) multiplicative structure of F_p (characters) as an oracle
obstruction_attacked: >
  W (Weil/Bombieri). Suppose a_k = g([k]G) has one η-heavy coefficient, |A(t0)| ≥ ηN, for a
  cheaply computable g. Then:
  - Kushilevitz–Mansour finds t0 with poly(log N, 1/η) queries `[recalled]`;
  - the target's shifted sequence g(Q + [j]G) carries the phase e(t0·k0/N);
  - so k0 follows in O(log N/η²) group operations. This is the AGS hardcore-predicate
    framework, run as an attack.
  Sub-rho needs η ≫ N^{−1/4}. W gives η ≤ 2d·p^{1/2}/N ≈ 2d·N^{−1/2}, so any g of degree
  d < N^{1/4}/2 is provably useless. Escape requires polylog-time maps of degree ≥ N^{1/4}.
  Dickson polynomials D_n(u + 1/u) = u^n + u^{−n} with n ≥ p^{0.3} are the cheapest such
  maps that carry multiplicative (torus) structure. Unlike χ(x^n) = χ^n(x), they do not
  collapse under characters.
claim: >
  For at least one (curve, n, ℓ) cell at p ≈ 2^20, the sequence a_k = χ_ℓ(D_n(x([k]G)))
  (χ_ℓ of order ℓ | p−1) has max_{t≠0}|A(t)| ≥ 2x the maximum of the random-sequence null.
  The claim also requires replication on fresh curves.
mechanism: >
  - D_n is the n-th power map on the norm-one torus T₂(F_p) (or F_p^*) seen through the
    x-line.
  - E(F_p) and T₂ are unrelated groups, so square-root cancellation is the default (Katz
    equidistribution heuristics).
  - The hope is a hidden correlation when n is tied to the curve: n ≡ ±1 mod N, n ≡ t
    (trace), or n | p ± 1 ± N.
  - Such n make the composition degree large yet arithmetically structured. These are
    exactly the maps W cannot rule out.
prediction: >
  Every cell lies in [0.8, 1.25]x the null maximum (≈ √(N ln N) for unimodular sequences).
  The MOV positive control peaks at exactly |A(±1)| = N, ≈ 280x the null at N ≈ 2^20.
  P(any replicated heavy cell) = 0.003.
discriminating_test: >
  - Write a native Rust tool (≈ 1 day):
    - walk k → k+1 over all of ⟨G⟩ (N additions);
    - evaluate D_n with a Lucas-sequence ladder (log n);
    - take χ_ℓ by exponentiation;
    - run a Bluestein FFT of prime length N in f64;
    - report max_{t≠0}|A(t)|/√N together with its argmax.
  - Cells: 10 prime-order curves × n ∈ {random ≥ p^{0.6} (×3), N±1, t, p+1−N±1} × ℓ ∈ {2, 3, 4}.
  - Null: 50 random unimodular sequences per N, which give the Gumbel maximum.
  - Budget: minutes per cell; ≤ 2 CPU-hours total.
falsification: "All cells inside the null band, with both positive controls detected → close Dickson/torus maps as W-escapes at toy size, and record the n classes tested."
cost_band:
  candidate: "boundary diagnostic, no pipeline S; FFT butterflies and Lucas-ladder steps priced at 0 / 0.1 / 1 GAE for the record"
  reference: "none for speed (no solver runs); the decision quantity is the heavy-coefficient ratio against the null and the MOV control"
controls: >
  - MOV positive control: supersingular y² = x³ + x, p ≡ 3 mod 4, prime N | p+1 ≈ 2^20.
    Use g(P) = e(log_ζ e_N(P, ψ(G))/N), with μ_N logs from a toy table, which gives a pure
    character.
  - Smart positive control: an anomalous toy curve, with g from the formal log of [p]P̃.
  - These two controls answer the atlas caveat that the structure instruments are blind to
    known weak classes (`LEAD-EXO-weak-class-instrument-calibration`).
  - Negative: random n coprime to p²−1 with no arithmetic tie.
risks: >
  - f64 FFT precision at N ~ 2^20 is adequate; at 2^24 use an exact NTT modulo a 62-bit
    prime and compare.
  - The prior is low because no mechanism links T₂ to E.
  - Overlap with `LEAD-EXO-dl-character-sum-pseudorandom`: that record tested the small-x
    indicator, a degree-1 interval, which is inside W. This record tests degree ≥ N^{1/4},
    which is outside W.
closest_baseline: >
  - Literature: Kohel–Shparlinski exponential sums along EC subgroups `[recalled]`;
    AGS hardcore predicates `[recalled]`; Boneh–Shparlinski coordinate bits.
  - In repository: character-sum leakage measured null (R 1.040 [0.999, 1.092] at 2^20;
    0.993 [0.972, 1.017] at 2^22) and the coordinate-statistic census.
  - Difference: the explicit W-escape threshold d ≥ N^{1/4}, with maps chosen to cross it.
escape: >
  - Outside G-Shoup and G-CGK (character of a coordinate).
  - G-CHW: δ = 1, so it does not bind.
  - The only binding barrier is W, which D_n escapes by degree.
lean_obligation: "HiddenShift.recover: on ZMod N, if a has |â(t0)| ≥ ηN, then from samples of a(· + k0) the phase e(t0·k0/N) is estimable and k0 is determined (Mathlib ZMod DFT/shift lemmas). W itself is cited, not proved."
prior: "0.003"
```

---

```yaml
id: MOON-D2-kummer-cover-decision-oracle
title: "Gauss/Jacobi-sum decision oracle: decomposition counts over a multiplicative-coset factor base are point counts of a Kummer cover of E"
family: (d) multiplicative structure (Jacobi sums) as an oracle
obstruction_attacked: >
  - A decomposition search must find a witness.
  - Search reduces to decision by halving the factor base, at a cost of log B decisions.
  - So a fast count N_H(R) = #{(P1, P2) : x(P_i) ∈ H, P1 + P2 = R} for a coset or subgroup
    H ⊂ F_p^* would give witnesses.
  - Character orthogonality writes N_H(R) as a sum of "elliptic Jacobi sums"
    J_R(χ1, χ2) = Σ_P χ1(x(P))·χ2(x(R − P)).
  - Those sums are, up to bounded corrections, the χ-isotypic Frobenius traces of the
    (Z/ℓ)²-Kummer cover C_R: {u^ℓ = x(P), v^ℓ = x(R − P)} of E, with ℓ = [F_p^* : H].
claim: >
  (A, an identity.) N_H(R) = ℓ^{−2}·#C_R(F_p) − (explicit ramification terms).
  (B, the lever.) For B = |H| ≈ p^{1/3} (ℓ ≈ p^{2/3}), N_H(R) > 0 is decidable in time
  ≪ p^{1/3}.
mechanism: >
  Point counting is polylog in p for fixed genus (Schoof–Pila) and can exploit the group
  action through the isotypic pieces of Jac(C_R). If the χ-pieces had small dimension and
  were computable separately, decision would be fast.
prediction: >
  (A) holds exactly. (B) is false by the following derivation.
  - The pieces are indexed by Galois orbits of characters and have dimension ≈ φ(ℓ)·O(1).
  - All known point-count methods are exponential in dimension (Schoof–Pila) or cost
    ≥ p^{1/2}·poly(g) (p-adic). So decision cost ≥ min(exp(Ω(ℓ)), p^{1/2}).
  - Polytime therefore needs ℓ = O(log p), i.e. a factor base of density ≥ 1/log p.
  P((B)) = 0.0005.
discriminating_test: >
  - Native Rust exact check of (A) at p ∈ {2^12, 2^16, 2^20} with ℓ ∈ {2, 3, 4}:
    - compute N_H(R) by enumeration;
    - compute #C_R(F_p) by enumeration over (P, u, v);
    - compare for all R.
  - Budget: ≤ 1 CPU-hour.
  - Then a written cost argument for (B) with the dimension count. The paper derivation is
    the real kill; the run only guards the identity.
falsification: "(A) fails on any R → the oracle is mis-specified. (B) is killed on paper unless a point-count algorithm for χ-isotypic pieces of dimension ~ℓ runs in poly(log p, log ℓ). That would itself be a point-counting breakthrough; it is the named escape."
cost_band:
  candidate: "identity check only; enumeration priced in GAE; boundary diagnostic"
  reference: "the m = 2 pair-table decision (O(1) lookup per target with B-sized table)"
controls: "ℓ = 1, where H = F_p^* and N_H(R) = #E − (ramified terms); random-coset H versus subgroup H."
risks: "Ramification bookkeeping at x = 0 and x(R − P) = 0. Low value apart from closing the (d) Jacobi-sum family cleanly."
closest_baseline: >
  - Program §4.6: multiplicative-subgroup bases via cyclic-norm folding cost Õ(B) per
    target. Atlas: the algebraic-membership bases' yield ratio is 1.000000.
  - Difference: decision by geometric point counting instead of folding, with a crisp
    density threshold (≥ 1/log p).
escape: >
  - Outside G-Shoup.
  - G-CHW: the useful δ (decomposable fraction) would be fine, but the oracle cost is the
    obstruction, not density.
lean_obligation: "KummerCount.identity: orthogonality of multiplicative characters on F_p^* turns N_H(R) into Σ_{χ1,χ2 trivial on H} J_R(χ1, χ2)/[F_p^*:H]². Mathlib MulChar orthogonality suffices."
prior: "0.0005"
```

---

```yaml
id: MOON-E1-compiled-advice-bdd-compressibility
title: "What S·T² does not forbid: is the decomposable set X_2 = {x(F_i ± F_j)} (or X_3) compressible into fast-query advice (ROBDD) below a random set of equal size?"
family: (e) many-target / amortised under the S·T² bound
obstruction_attacked: >
  - CGK is an incompressibility argument for generic advice.
  - The bound does not forbid non-generic advice that answers "is z decomposable?" in
    polylog time with S ≪ |X_{m}|.
  - If X_3 (density ~1/6 at B ≈ p^{1/3}) had a ROBDD of size p^{β} with β < 1, then advice
    S = p^β and online T = O(b) per randomised target would put S·T² ≈ p^β·b² < p.
  - That would be the first non-generic point under the CGK curve. It would apply to every
    target after one precomputation, which is the multi-target setting.
claim: >
  For the small-x base with B = ⌈p^{1/3}⌉ at p ∈ {2^16, 2^18, 2^20, 2^22, 2^24}, the
  minimum ROBDD size (CUDD sifting, best of 3 orders) of 1[z ∈ X_2] or 1[z ∈ X_3] over z's
  b bits is ≤ 0.5x that of a random subset of equal cardinality, and the ratio falls with p.
mechanism: >
  - The base X_1 = [1, B] has a ROBDD of size O(b), which is maximal compressibility.
  - Under the additive law, X_2^{add} = [2, 2B] is still O(b).
  - Under the elliptic law the question is whether any interval structure survives in
    x(F_i ± F_j). The ladder of controls (additive, multiplicative, elliptic) shows where
    compressibility dies.
prediction: >
  - X_2^{ell} and X_3^{ell} ratios to random are in [0.95, 1.05].
  - Size growth exponent β in [0.95, 1.0] in |X|.
  - X_2^{mul} = {x1·x2 mod p}: ratio in [0.8, 1.0].
  - Additive control: O(b).
  P(ratio ≤ 0.5 and falling) = 0.003.
discriminating_test: >
  - Native Rust:
    - enumerate X_2 and X_3 exactly (B³/6 ≈ p/6 sums at most);
    - build the ROBDD with a native level-wise unique-table builder from the sorted member
      list;
    - cross-check node counts on two sizes against CUDD (a C backend via FFI is allowed).
  - Report node counts for X_1, X_2^{add}, X_2^{mul}, X_2^{ell}, X_3^{ell}, with 10 random
    equal-size sets per cell.
  - Budget: about 1 day of code and ≤ 3 CPU-hours.
falsification: "Ratio ≥ 0.9 for X_2^{ell} and X_3^{ell} at all five sizes and every order tried → close BDD-compiled decomposition advice (random-like at fast query)."
cost_band:
  candidate: "advice S = node count (words); online T = b node visits per randomised target, priced at 0 / 0.1 / 1 GAE per visit, plus 1 GAE per randomisation (R + aG)"
  reference: "Bernstein–Lange preprocessing rho, S = n^{1/3}, T ≈ 2n^{1/3}, S·T² ≈ 4n (program §1, Probe A); pair-table IC row S·T² ≥ 0.56p"
controls: >
  - Positive: X_1 and X_2^{add}, which must compress to O(b).
  - Random sets of equal size as the null.
  - Variable order: natural bits, reversed, and sifted.
risks: >
  - ROBDDs are one representation. Incompressibility under BDD does not rule out other
    fast-query encodings; F_p^* smooth numbers are a structure BDDs also miss.
  - A positive result must still be priced end to end: the precomputation cost P is not
    bounded by CGK, but it is part of the cold S.
closest_baseline: >
  - CGK 2017/1113 (incompressibility for generic advice); Bernstein–Lange non-uniform rho.
  - Lead review rank 4: target-parametric clause/tensor reuse, labelled multi-target.
  - Difference: this measures compressibility with fast query directly. It is the exact
    quantity CGK's proof says a generic algorithm cannot have.
escape: >
  - G-CGK: escaped only if the advice is non-generic and compresses.
  - G-Shoup: membership reads z's bits.
  - G-CHW: δ(X_3) ≈ 1/6 ≫ q^{−1/2}, so CHW would not bind.
lean_obligation: "None for the measurement. A promoted claim needs Advice.ST2: a correct ROBDD of size S for 1_{X_3} plus the randomised self-reduction solves ECDLP with T = O(b·p/|X_3|) expected queries (short probability lemma)."
prior: "0.003"
```

---

```yaml
id: MOON-F1-unsupervised-u2-character-learning
title: "Label-free learning of a group character: maximise the U²-energy of a learned feature f(P) using group-law samples only, against a relabelled-group null"
family: (f) learning/statistical with the right null
obstruction_attacked: >
  - The lab's ML closures (mllc: 22 NULL cells; learned triage at chance) are supervised.
    They need DL labels, so they stop at 2^24.
  - They test lsb/msb only. Under a random generator those are the same Fourier family.
  - The relevant property is label-free: f has a heavy Fourier coefficient along ⟨G⟩ if and
    only if its Gowers norm ||f||_{U²}^4 = Σ_t |f̂(t)|^4 = E_{P,Q,R} f(P) f(Q) f̄(R) f̄(P+Q−R)
    is large.
  - That quantity needs only group operations, never DLs, so the search scales to
    p = 2^48 and beyond.
  - A heavy coefficient then gives the DL through D1's hidden-shift step.
claim: >
  In a pre-registered span of D = 64 cheap features of (x, y) (χ_ℓ(x + c), χ_ℓ(y), bits of x
  and y, Dickson-characters from D1, parity and size indicators), the maximum of
  ||f||_{U²}^4/||f||_2^4 over unit-norm f exceeds the relabelled-group null maximum by ≥ 3x
  on some curve. The test runs at p = 2^20 (labelled validation), 2^32 and 2^48.
mechanism: >
  - A feature span that contains an approximate character would show up as a quartic-form
    eigenvalue.
  - The relabelled-group null is the right null. It uses the same features evaluated
    through a random bijection of the group, which keeps every feature marginal and removes
    the group law.
  - Every NULL the lab has recorded then becomes a scale-free bound on "learnable
    characters in this span".
prediction: >
  - Every cell is within [0.8, 1.25]x the null maximum.
  - The MOV and Smart features (D1's controls) give U⁴ ≈ 1 against a null of ~D/N.
  - MDE: with M = 10^9 quadruple samples, detectable η ≈ M^{−1/8} ≈ 0.075.
  P(non-null) = 0.002.
  Derived note: the mllc NULL at 24 bits already bounds advantage by ≈ 0.0013. The
  heuristic HNP threshold for sub-rho (bias ε with ε^{−2} ≪ N^{1/2}) is ε ≫ N^{−1/4}
  = 0.0156 at 2^24. Supervised ML is therefore already below the exploitable bias at that
  size. This test can see only gross leaks (η ≥ 0.07) and cannot replace that bound.
discriminating_test: >
  - Native Rust (no Python; the mllc harness is Python and cannot be extended under the
    no-Python rule):
    - sample M quadruples (P, Q, R, P+Q−R) via ecbench curve arithmetic;
    - accumulate the D⁴ quartic moment tensor as a symmetric sketch;
    - maximise the quartic form by power iteration from 256 random starts;
    - repeat on the relabelled-group null.
  - At 2^20, validate any hit by exact DFT against true DLs.
  - Budget: about 2 days of code and ≤ 8 CPU-hours (M = 10^9 at three sizes).
falsification: "All cells inside the null band, with MOV/Smart controls detected at U⁴ ≥ 0.5 and the planted noisy-character ladder recovering its MDE → close the span at the stated MDE."
cost_band:
  candidate: "boundary diagnostic, no pipeline S; group operations counted in GAE and feature evaluations priced at 0 / 0.1 / 1 GAE"
  reference: "relabelled-group null (statistical reference); rho.negation listed only if a hit is promoted to a hidden-shift solver"
controls: >
  - Positive: MOV, Smart, and a planted character with SNR ladder (η ∈ {0.5, 0.2, 0.1, 0.05}).
  - Negative: relabelled group; constant and balanced-trivial features excluded by
    |E f| ≤ 0.1.
risks: >
  - The MDE floor is ≫ the exploitable η at large N, so a NULL is weak evidence in the
    sub-rho regime. State that in the scope.
  - The quartic optimum can be degenerate; use multiple restarts and report the spectrum.
closest_baseline: >
  - Literature: the AGS hardcore framework and the Gowers U²/U³ inverse theorems
    `[recalled]`.
  - In repository: ml_learnability_control (supervised; lsb/msb; m ≤ 24) and
    `LEAD-EXO-coordinate-statistic-leakage`.
  - Difference: label-free, scale-free, with a group-law-ablated null.
escape: >
  - Outside G-Shoup (features of coordinates).
  - G-CHW: δ = 1.
  - The barrier is W for algebraic features. It escapes only through non-algebraic or
    high-degree features in the span.
lean_obligation: "U2.heavy: on ZMod N, ||f||_{U²}^4 = Σ|f̂|^4 ≤ max|f̂|²·||f||_2², so ||f||_{U²}^4 ≥ c·||f||_2^4 implies a coefficient ≥ √c·||f||_2 (Parseval; Mathlib ZMod DFT)."
prior: "0.002"
```

---

```yaml
id: MOON-G1-physics-solvers-semaev-qubo
title: "Physics-inspired solvers (parallel tempering, analog CTDS, survey-propagation decimation) on the Semaev decomposition QUBO: time-to-solution exponent against enumeration"
family: (g) admittedly wild (≤ 1%)
obstruction_attacked: >
  - Enumeration-shaped decomposition cost. Physics solvers exploit landscape correlations
    rather than resolution or algebra.
  - If the energy (S(x) − p·q)² over the bit encodings of x_i ∈ [0, B] and quotient bits q
    had a funnel toward roots, local dynamics could reach roots in fewer evaluations than
    enumerating candidate tuples.
claim: >
  For m = 2 (S3, B ≈ √p, about one root per target) at p ∈ {2^12, 2^14, 2^16, 2^18, 2^20},
  the best of PT, SA, CTDS and SP-decimation reaches 99% time-to-solution in
  TTS ∝ (B²/2)^{e} spin-flip evaluations with e < 0.9, i.e. sub-enumeration.
mechanism: >
  - None known; that is why this record is wild.
  - Annealing literature exists only at F_7 to F_1021 (Wroński et al.), with no scaling
    exponent.
  - A positive e < 0.9 would mean the mod-p residual is locally correlated in the bit
    encoding.
prediction: >
  e ∈ [1.0, 1.6]: a golf-course landscape, because flipping a low bit of x_i changes
  S mod p by ~∂S ≈ p, which decorrelates neighbours. PT is worst once the quotient bits add
  search space. P(e < 0.9) = 0.01. P(e < 0.5) = 0.0005.
discriminating_test: >
  - Native Rust:
    - QUBO builder (binary digits of x1, x2, the quotient q, auxiliary products);
    - PT with 32 replicas;
    - SA;
    - an Euler-integrated CTDS on the CNF;
    - SP on the factor graph.
  - Run 64 planted targets per size, tune hyperparameters on a held-out size, and freeze
    them before the panel.
  - TTS uses the standard 99% formula; fit e against log2(B²/2).
  - Budget: about 2 days of code and ≤ 10 CPU-hours (< 100 spins at 2^20).
falsification: "Lower 95% CI of e ≥ 0.9 for every solver → close physics-inspired local search on the Semaev QUBO."
cost_band:
  candidate: "spin-flip evaluations / ODE steps / SP messages priced at 0 / 0.1 / 1 GAE each, with the QUBO build charged"
  reference: "exhaustive pair enumeration with table (component reference, B per target) and rho.negation on the same curves"
controls: >
  - Planted-solution QUBO with a smooth (additive-law) analogue, where annealing must find
    the funnel. This is the positive control for the solver.
  - Random-coefficient QUBO of equal size as the negative control.
risks: >
  - Hyperparameter overfitting; the held-out size guards against it.
  - TTS fits on ≤ 5 sizes are fragile.
  - Annealing records claimed in the literature (16-bit, ICN 2026) are single-source and
    unverified.
closest_baseline: >
  - Wroński et al. (ICCS 2021; F_251 via D-Wave hybrid; F_1021 Edwards) and Dzierzkowski
    2024 (generalized QUBO).
  - Difference: classical physics solvers with a pre-registered scaling exponent against
    enumeration on the same QUBO. Prior work reports no exponent.
escape: >
  - Reads bits, so it is outside G-Shoup.
  - G-CHW: δ for m = 2 is the decomposable fraction ≈ 1/2.
  - The binding barrier is the §2.4 arity bridge: no m = 2 oracle can beat rho even at
    T_dec = 1. A positive e therefore matters only if it carries over to m ≥ 4.
lean_obligation: "QUBO.ground_states: minimum energy 0 is attained exactly at decompositions (x1, x2) with x_i ≤ B and S3(x1, x2, x_R) ≡ 0 mod p."
prior: "0.01 (e < 0.9 at toy); 0.0005 (anything rho-relevant)"
```

## 3. Ranking by information per CPU-hour (my ordering; the Coordinator decides)

1. **D1**: minutes of CPU. It gives the lab a calibrated instrument with MOV/Smart positive
   controls, which no current statistic instrument has. It cleanly tests the only
   W-escape class.
2. **E1**: hours. It measures the exact quantity CGK forbids for generic algorithms.
3. **A1**: about 1.5 days. This is the SAT direction the user named, on the ladder rather
   than Semaev. It reuses the existing emitter and runners.
4. **F1**: label-free and scale-free. It reuses D1's controls.
5. **A2**: only as an arm of the existing frozen m = 3 tournament.
6. **G1**: wild, hours.
7. **B1**: component only.
8. **C1** and **D2**: mostly closed on paper. Their runs are correctness guards.

None of these records changes the program's §5 decision.

## 4. Summary table

| id | one-line mechanism | prior | cheapest test (tool reused) | kill line |
|---|---|---|---|---|
| A1 ladder-CNF | Bit-blasted ladder SAT with k's bits free; learned clauses as target-independent advice | 0.001 (α<0.5); 0.02 (reuse ≥2x) | Rust IR generator into the `prime_shared_coefficient_sat_20261031/native` lowering; CaDiCaL / CMS-XOR / cvc5 QF_FF; b = 10–20; ≤ 6 CPU-h | α lower CI ≥ 0.6 and reuse ratio ≥ 0.5 |
| A2 Macaulay-CDCL(T) | Gauss-mod-p elimination on the degree-D Macaulay span as the CDCL propagator | 0.005 | Extra arm in the lead review's m = 3 tournament (IPASIR-UP); ≤ 10 CPU-h | e lower CI ≥ 0.7, or ≥ 100x enumeration field ops per triple |
| B1 MILP beyond HG | B&B over (x, q) on reduced Coppersmith polynomials with small integer quotients | 0.02 / <0.001 | Extend `examples/prime_ecdlp_breakthrough_probe.rs`; 40-bit S3; ≤ 4 CPU-h | γ ≥ 0.9 (only bit-guessing with a cheaper constant) |
| C1 torsion-lift decoder | Height-zero global torsion lift, LLL-short lift of x(Q), complex elliptic log | <0.001 | Literature torsion table plus a d ≤ 2 native toy decoder | (2X0+1)^d ≥ maxprime(d) + O(√·) at every known d |
| D1 Weil-escape spectrum | Dickson D_n with n ≥ N^{1/4} escapes Weil; a heavy coefficient gives DL by hidden shift | 0.003 | Native FFT over ⟨G⟩ at 2^20 with MOV/Smart positive controls; ≤ 2 CPU-h | All cells within [0.8, 1.25]x null, with the controls detected |
| D2 Kummer-cover decision | Jacobi-sum decomposition counts are point counts of (Z/ℓ)² covers | 0.0005 | Exact identity check at p ≤ 2^20; ≤ 1 CPU-h | Killed on paper unless χ-piece point counting is poly(log p, log ℓ) |
| E1 compiled advice | ROBDD compressibility of the decomposable set X_2/X_3 (non-generic advice under CGK) | 0.003 | Native ROBDD builder (CUDD cross-check); p = 2^16–2^24; ≤ 3 CPU-h | Ratio to random ≥ 0.9 at all sizes and orders |
| F1 U² learning | Label-free maximisation of U²-energy in a feature span against a relabelled-group null | 0.002 | Native quartic-moment sketch plus power iteration; 2^20/2^32/2^48; ≤ 8 CPU-h | All cells within null band, with controls detected at the stated MDE |
| G1 physics solvers | PT / CTDS / SP on the Semaev QUBO, TTS exponent against enumeration | 0.01 / 0.0005 | Native QUBO plus solvers; m = 2; 2^12–2^20; ≤ 10 CPU-h | e lower CI ≥ 0.9 for every solver |

## 5. Sources checked

- research-kb: **not available** to this subagent (no tool exposed). No `source_id` was
  recorded.
- Repository: the paths in §0.1.
- Web pages:
  - https://eprint.iacr.org/2019/313
  - https://eprint.iacr.org/2014/806
  - https://eprint.iacr.org/2023/091
  - https://eprint.iacr.org/2024/572
  - https://www.iccs-meeting.org/archive/iccs2021/papers/127470144.pdf
  - https://reference-global.com/article/10.2478/tmmp-2024-0021
  - https://eprint.iacr.org/2017/1113
  - https://eprint.iacr.org/2026/384 (abstract fetched)
  - https://arxiv.org/pdf/1011.0914
  - https://link.springer.com/chapter/10.1007/978-3-642-04159-4_6
  - https://www.academia.edu/92951884/Encoding_Basic_Arithmetic_Operations_for_SAT_Solvers
