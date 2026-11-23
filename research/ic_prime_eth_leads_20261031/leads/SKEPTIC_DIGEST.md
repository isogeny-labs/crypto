# Skeptic digest (2026-10-31): verdicts on the 12 lead evaluations and 9 new proposals

Each lead had two independent Sonnet skeptics (numbers lens; mechanism lens), except L03, L09, L10, L11
(one combined). "stands" = 0 refutations, "contested" = 1, "refuted" = 2. Numbers were reproduced from
raw files in every case where a rerun was attempted; refutations are about interpretation and scope.

## Leads

- L01 char-2 Fourier gadget — REFUTED as "undecided"; stands as constant_only/no gain. Raw data now hold
  finished l=7 gadget runs (1.36M/1.55M/1.30M conflicts): gadget l5–7 slope ≈ 2.58 (reopen threshold was
  CI < 2.5), l6→7 step 2.9 vs 3.5 standard, gadget/standard conflicts ≈ 1.2 at l=7 but CPU 3× worse. Gap
  closing fits a constant padding floor (~11k conflicts) + common 3-bit/l growth; gadget padded to N=27
  with n tied to l; GJ off by default and hurt when forced; only 4 of 11 products use the gadget. Census
  constants (0.026–0.074 of 1100n) count XOR-layer wires only and omit ~48k internal ANDs at n=131;
  must be restated. Lean covers index arithmetic only. Close into LEAD-ICB-decomposition-encoding-and-sat.
- L02 ring-linear propagator — NOT RUN (permission denials; agent declined to retry). Design on file.
- L03 sparse-parity bases — STANDS as constant_only, narrowed: weight ≥ 5 random sparse rows match
  Bochum/dense at b ≤ 22, m ≤ 3; weight-3 ~10% deficit is a 1/b finite-size 3-XORSAT consistency
  effect, not constant; reg3 unusable at these sizes (parity artefact). Restates L07 + KN-FIND-007;
  size-matched null makes pairwise-independent hashes match by construction; "above 2/5 for all
  families" false (reg3 0.378); cost table (6600 → 172) is a derived unvalidated gate model and omits
  base-enumeration cost; SAT arm not run. Bochum hash near- but not exactly faithful (per-slot prime).
- L04 backdoor census — CONTESTED → partial. Numbers reproduce; search exhaustive; propagator is
  cap-bounded finite-domain filtering with F_p root-finding, NOT Gaussian elimination with slope/t
  variables; offset = 2·log2(cap) − 1 is enumeration moved inside the propagator; B* identical across
  all 32 targets per curve (effective n = 3 curves/cell); CIs mostly exclude m−1 from ABOVE (finite-size,
  OLS slope not meaningful; state offset saturation instead); at m=4 cap2 the upper bound fails;
  window-only candidates by construction; s ≤ 7. Cannot close L2 or L5. Supportable: "no sub-(m−1)s
  backdoor under cap-bounded window-branching propagation at s ≤ 7".
- L05 policy pilot — REFUTED as "closed" (both skeptics, high confidence). Numbers accurate (350 rows,
  0 FAIL, ref ALL slope 2.17 [1.93,2.38], (3,4) 1.55, SAT-only 1.41, nosym 1.3–4.4× worse, renaming
  inert). But: renaming is a null experiment by construction in kissat; ladder held p fixed (b=8) and
  varied l, which is not e vs s=b/m; l=2 cell degenerate; CIs ignore curve clustering (3 curves);
  hooked CaDiCaL engine (cadical.hpp ExternalPropagator installed) never built. Label: "harness built,
  renaming null recorded, policy question untested". Do not fund an LLM search on renaming; the hooked
  engine question is open. Atlas: LEAD-EXO-ml-learned-models has the only prior learned-branching
  negative (WDSat 16,097 vs 3,092 conflicts, worse).
- L06 contraction width — REFUTED as "closed" (both). Numbers reproduce (MMD+ 15–51, min-fill 146–389,
  model matches ECCp-131 arity 1032 exactly, edges +7.7%). But treewidth of the dense gate graph is the
  wrong proxy for D.4 (black-box vertices, Hermite interpolation) and ignores functional dependence
  (width ~21–25 remains after pinning ALL inputs, a poly-time circuit evaluation); Fourier arm is
  dense-arity only (b=8 exact; larger b arity counts); structured transform-word cut effectively not
  run (k=1,2 at b=8); no counter timings; schoolbook arm has no crate ground truth; b=16 ratio 6.1× not
  5.7×. Supportable: "plain tree-decomposition DP on the school CNF loses to enumeration at b ≤ 16".
- L07 base second moment — CONTESTED → narrow constant_only. Numbers reproduce (skeptic rerun 24 s);
  faithful Bochum threshold arm added by skeptic also matches null. Corrections: interval-control range
  0.0007–0.31 (not 0.001–0.04), slope bounds up to 6e-4/9e-4, positive control not run at b=22, b=22 has
  3 instances (consistency check only), drop "m! confirmed" (ordered tuples include repeats; measured
  ord/unord 2.0/7.2/58.5 vs 2/6/24), fix b=18 reproduce command. Shared negation-closed x-window deficit
  (0.71 at m=4 b=14, closing with b) is an instance of KN-FIND-007 / EXP-FB3-001 symmetric-base
  mechanism; the x-window/shared-base quantification is new. Says nothing about m growing with b or
  per-target variance. Do not claim "base choice free for every solver arm".
- L08 Koblitz/binary transfer — REFUTED as stated (both); constant_only, binary-only, unproven sketch.
  Counts reproduce exactly (2,694,680 / 1,564,506 / free27 787,740 / +524 per slot fold; fit a=440,715;
  hand bound 80d+228√d fails at d=46; 175d and 24.4 d^{3/4} hold; c* 1.0e-4..3.4e-4). But "char-2 not
  worse than integer" compares a freed binary gadget with an unfreed integer one: skeptic built a freed
  integer multiplier (133-bit: 37,030 → 14,618 edges), true margin ≈ 1.4×. "Holds with the same exponent"
  is a sketch: enc/x=0, cofactor h=4 subgroup constraint, char-2 enumeration of F_i, complete-addition
  split unaddressed; folded theorem needs Lemma 10 with m(n−1)/L term (toy MC loss 0.72–0.92 at m=3 is
  the regime L ≥ 10m excludes), Lemma 12 recheck, and an unbuilt O(n log n) canonicaliser (single
  4n-port vertex breaks the degree cap asymptotically). Rho gets the same Frobenius gain, so net gap
  unchanged. No prime-field content.
- L09 BTN census tool — CONTESTED → tooling_done with a downgraded yardstick. Tool and tests reproduce;
  no row D.1-admitted (margin −16.1..−16.3 bits of log2 λ; needs λ ≥ 2^85) stands. But s is for the
  one-constraint-per-vertex partition: an UPPER bound on the best legal D.1 partition; clustering into
  degree ≤ s^0.9 vertices could shrink s toward the interface k ≈ b, where s/√log2 s at constant 1 can
  fall below the enumeration exponent at small b (9.2 vs 12.67 at b=19, m=3). k ≥ b is an injectivity
  count on declared inputs, not a theorem; n19 l6 k=18 is a convention difference. Add a cut-minimising
  clustering column before calling the ratio a yardstick. (Later proposal review: clustering cannot
  rescue anything because no row is admitted and hidden interface bits cost 2^h entry evaluation.)
- L10 Lean filings — REFUTED as fileable (high). All seven LEADS-vs-Lean gaps CONFIRMED (no_crossover
  needs c ≥ 1; E(m) cost model is a hypothesis; crossover_bits is unlinked arithmetic; interface_lower_bound
  is injectivity only; polymethod covers only 1/(2d); items (b),(d) have no Lean receipt; sibling KR-2
  withdrawn, KR-1 upper bound only). Build clean (standard axioms). Rows a10001 (unqualified title and
  forecloses; log2 b > 256c² and attainment are hand algebra; "extends" mislabelled on an open lead),
  a10005 and a10007 ("proven" status for hand-derived / unrebuilt content) overstate; add
  LEAD-ICE-generic-oracle-arity-cost-floor, LEAD-ICE-sat-xor-pdp-oracle,
  LEAD-ICP-digit-presentation-solving-degree; mint KN-LIT for Bochum and Caminata–Gorla and a KN-FIND
  for the Lean receipt; ids are placeholders.
- L11 multiplier encodings — STANDS as constant_only. Rerun exact (school 22046, pc 18072 medians at
  b=9). Paired geometric-mean ratio pc 0.78 [0.67, 0.92] (report 0.70 is ratio of medians); one prime
  at one size, reverses at b=6 (pc 1.15× worse), untested at b ≥ 12 (cap-saturated); Karatsuba tested at
  one recursion level only; pc is per-adder GAC with constant-input fallback. "pc default" is a weak
  provisional preference. No slope measured.
- L12 clause retention — REFUTED (both, high). Conflicts attributed correctly (per-solve differences,
  setup included), runs deterministic and reproduced. But: the retained solver fills a |F|^{m−1} table
  (47 censored then all solved; per-block cost 0.095 → 0.006); setup never charged vs the explicit
  MITM/enumeration reference (8 abscissae at l=3: ~512 sums, microseconds, vs 0.777 CPU-s/relation);
  fresh baseline fully censored at m=3 (0/256 at 20k cap); target lists 48% duplicated (m3 b12: 134
  distinct of 256; "106 relations" = 16 distinct targets); baseline is a generic variable-target CNF,
  not the production constant-target CNF; one seed; N^−0.8 is a 3-point fit of setup+floor. Classify
  as retained-setup/advice (LEAD-ICP-retained-setup-online, closed). Do not make incremental the default.

## New proposals (3 proposers × ≤4; two converged on clustering) — ALL NINE REFUTED as day-long leads

1. CDCL cube-class census (solver as backdoor oracle) — refuted: mostly L04+L05 again; decision
   threshold (slope < m−1) does not match the exponent bar (j* ≤ s/2); "mean conflicts ≤ 4" ignores
   propagation and the uncharged 2^12 cap; grid is 7–140 CPU-h at b=15 alone; b=6 degenerate.
   Residue: a few-hour check at one size (b=9/12) of 3–4 classes with median conflicts+propagations vs
   direct solve, plus a cube-and-conquer vs plain-kissat timing.
2. UNSAT-free harvesting hazard ladder — refuted: γ = e/3 − 1 in disguise (γ = −1 forced; γ ≤ −1.5
   needs e < 0, contradicted by every measurement); estimator censored out at b ≥ 12, l ≥ 4; flat-hazard
   signature (ml-ic-review §2.3) caps cutoff gain at ~2×. Residue: relations-per-conflict at natural SAT
   rate with a fixed cutoff at b ≤ 12 as a harness accounting change.
3. X(R) predictability / solution clustering — refuted: X-dispersion half is L07/L03; feature half is
   the closed coordinate-statistic / dl-character-sum / yield-predictor registry (null to 0.0002 bits at
   2^24–2^26) and ml-ic-review P2; best outcome is a constant lever. Residue: log Legendre/bit features as
   extra columns inside P2 with a pre-declared ≥1.3× lift floor.
4. D.1-legal clustering frontier (two proposers) — refuted: F(Δ) = max(AΔ, s_eff/√log s_eff) misreads
   D.1 (2^{A s^0.9} is a global entry budget, Δ cap separate); no row admitted so s_eff optimisation is
   unrealisable; decision rule normalised by enumeration not the rho budget b/6; L06 widths already give
   the predicted F ≈ 20–30b (r = 30–45, the "close" branch); greedy partitioner gives only an upper bound.
   Corrections: the Lean floor k ≤ s does not apply to clustered graphs (hidden bits), hiding h bits costs
   2^h entry evaluation so s_eff ≥ (h/A)^{1.11}; b < 2^36 closure is m=3-specific (m=4: b < 2^16); m ≥ 4
   is closed by no_crossover not the floor. Free linear merge is legal but known ≈2× in s (L09 XOR rows).
   Residue: one-line closure in LEADS §6 and a one-page rigour note.
5. Edge-constant engineering (transform sharing, fused multiply-add, multiply-shift hash) — refuted:
   blackbox_balance/no_crossover depend on a·c, honest a ≈ 3e3 so no achievable cut changes any decision;
   m=2 never below rho; realistic gain 1.3–1.8×; multiply-shift breaks Lemma 10 pairwise independence;
   asking D.1 authors for c is unactionable (inert below 2^{2^85}). Residue: minutes to re-run L08 fit.py
   with the freed integer gadget.
6. Forced-recursion op counts — refuted (high): forced mode is a radix-2, u=1, array-16 diagnostic
   gadget, not D.4; repo's problem file already states such controls carry no runtime/crossover content;
   a 2-decade fit outside the proof regime bounds nothing; "c_eff as lower bound on c" invalid.
7. Descended-ANF scaling / c_break — refuted (high): binary-only; D.1 not admitted so c_break is an
   uninformative fit; solver_saving_floor already settles it; L09's own n=15..19 slope 1.8 meets the
   proposer's close criterion; required WDSat baselines do not exist beyond Koblitz n=15.
8. Log-domain Fourier-peak homomorphism scan — refuted: the lab already ran exact-dlog FFT scans
   (analysis/o2-sum-compatible-filters: 507 families, 13,528 combos, max 1.10× vs matched null;
   EV-ECDLP-f83a17 at 2^20–2^26) and composed Weil/Fourier bounds into O2_composition_closure; algebraic
   features settled by theorem; a b ≤ 26 scan cannot discriminate ε = 2^{−b^δ} from the Weil floor 2^{−b/2};
   Gumbel null miscalibrated for correlated structured maps; ML stage duplicates approved EXP-ECDLP-9f7772.
   Residue: fold non-algebraic features into the planned ml-learned-models learnability control; the
   four-list lead's own open item (bucket variance W) is the better-scoped step.
9. Planted-leak k-tree calibrator — refuted: a planted log-mod-M character is the Pohlig–Hellman leak,
   not a coordinate-computable map; cost formula double-counts ε^{−2^{t+1}}; ε*(b) follows by algebra;
   interesting ε regime unreachable at b ≤ 36; generic track already ruled √r-bounded. Residue: 2-hour
   desk note + Lean lemma beside ladder_rule, filed under LEAD-EXO-four-list-ktree-join.
