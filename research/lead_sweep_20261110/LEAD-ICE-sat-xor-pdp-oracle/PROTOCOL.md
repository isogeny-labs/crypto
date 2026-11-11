# LEAD-ICE-sat-xor-pdp-oracle

## 1. Lead as recorded
- Disposition: open_next_step (reviewed 2026-10-24; 2026-11-01 label open_next_step). Family: ic-engine.
- Title: SAT / native-XOR (CryptoMiniSat, WDSat) point-decomposition oracle with honest pricing.
- Next-step excerpt (truncated): "Calibrate ns per conflict and per propagation against ns per group addition on the count-qualifi..."
- Files read: research/lead_sweep_20261110/leads.json (entry);
  research/prime_field_lead_assessment_20261107/atlas_survey.md (line 47: native-XOR SAT S/rho 0.854 lower bound, conflicts unpriced, n=17 Koblitz; line 225);
  docs/session-logs/2026-10-30.md (line ~6469-6475: z3 probe, no SAT/SMT formulation beat the pair-table oracle; LQ-CDCL gain <= p^{1/6}, break-even ~p>=2^100).
  The research-kb MCP was not queried in this subagent; no novelty claim is made.

## 2. Hypothesis
When SAT search work (propagations + conflicts) is priced at w GAE each, a CDCL/DPLL point-decomposition oracle for m=2 (R = P1+P2, P_i in a signed factor base of size F) costs more than the hash-table oracle (F group additions) at w = 1 even if the instance encoding is free, and at every w in {0,0.1,1} if the R-dependent encoding is charged (>= F additions). I.e. the unpriced-conflict lower bound S/rho 0.854 does not survive pricing.

## 3. Preregistered prediction and falsification criterion
- Statistic: S = SAT_GAE / table_adds, median over targets, per factor-base size, for free-encoding and naive-encoding arms, w in {0, 0.1, 1}.
- Prediction: S_free(w=1) > 1 for every F, and S_naive(w) > 1 for all w and F.
- Falsified: S_free(w=1) <= 1 at any F. Inconclusive: a control fails. Consistent: otherwise. A free-encoding S at w=0 is 0 by construction and S at w=0.1 may cross 1; the order is reported across the 0/0.1/1 band (AGENTS.md 5a) and arms are ranked only where it holds across the whole band, so no ranking is claimed for free-encoding SAT vs table.
- Null/negative control: random curve points as targets (about half undecomposable); table oracle decides truth. Positive control: planted targets R = P1+P2; SAT must return a valid pair. Pass criterion: 0 disagreements SAT vs table, and each returned pair re-added to R.
- A/A: ns per group addition measured twice; relative difference must be < 0.25.

## 4. Method
- Python 3 stdlib only. p = 16381, curve y^2 = x^3 + x + 3, affine arithmetic with a counted add. Factor base: points with the B smallest x values, signed, |FB| = F <= 2B, B in {8,16,24,32}. Seed 20261110 (random.Random). 24 targets per B alternating planted/random.
- SAT encoding: one-hot variables a_i (P1 = FB_i), b_j (P2); ALO and pairwise AMO per block; binary clause (-a_i or -b_j) whenever P_i + P_j != R. Solver: own recursive DPLL with unit propagation, first-unassigned branching, true first; counts propagations, conflicts, decisions. XOR-native propagation is not implemented (no CryptoMiniSat).
- Pricing: GAE = group additions; table oracle = F additions plus hash lookups (lookups free, favouring the table's rivals not at all). Naive arm adds the F encoding additions (the R-dependent clause build, counted exactly).
- Also reports measured ns/GA and wall-ns per SAT work unit (upper bound, includes encoding) as calibration.
- Cost: seconds to about a minute, < 50 MB.

## 5. Classification
runnable_local. A stdlib DPLL on one-hot decomposition decides the pricing inequality at toy sizes.

## 6. What a run cannot establish
- Not CryptoMiniSat/WDSat, not native XOR, not a bit-blasted Semaev polynomial: the one-hot encoding is a weak stand-in, so a falsification would show only that this encoding wins, and a confirmation says nothing about industrial CDCL with learning.
- Python ns per propagation is not a C solver's; the pricing uses the abstract weights 0/0.1/1, not the measured ratio.
- Toy sizes, one curve and one seed; no asymptotics, no S vs matched rho, no ecbench claim. Results labelled [pcc:unverified].
