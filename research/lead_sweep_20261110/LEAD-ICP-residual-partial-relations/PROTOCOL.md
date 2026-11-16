# LEAD-ICP-residual-partial-relations

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (leads.json); disposition_20261101: closed_obstructed (measured). Family: ic-prime. Last reviewed 2026-10-29.
- Title: "Crediting residuals as partial relations (single/double large prime) in prime-field IC". Cost: cheap (extra variants).
- Next-step excerpt (truncated): "a double-large-prime or graph-cycle variant is shown to lower the fitted exponent below the gene[ric]..."
- Why it might beat sqrt(p): large-prime pairing lowers charge to ~N^((1+beta)/2), but measured slope equals generic searches (birthday).
- Recorded evidence (atlas_survey.md): 39 sessions at 20-44 bits; slope of log2 S vs log2 r -0.025 [-0.034,-0.017] (B=2), -0.014 (B=8), -0.017 (B=128), same as rho.negation (-0.010), bsgs.negation, kangaroo. F_{p^4} 2LP cut LA exponent 0.495->0.420 but whole solve 2.31x costlier. Session-log desk estimate: 2LP table of size l succeeds with l^2/r, optimum l~sqrt(r).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 22, 98, 141, 241); /home/user/crypto/docs/session-logs/2026-11-01.md (lines ~7871-7877). Others not read.

## 2. Hypothesis
In a generic prime-order group of order N, crediting residuals R - F_i (factor base size B) as single-large-prime partial relations, and the table-restricted large-prime variant with table size l = sqrt(N), cost Theta(sqrt N) group operations to first usable relation: the fitted exponent of mean group-op cost vs N is 0.50 (within [0.45, 0.55]) for B in {2, 8, 128}, identical to the plain birthday collision search (B=0), i.e. no exponent below birthday.

## 3. Preregistered prediction and falsification criterion
- Statistic: slope of log2(mean ops) vs log2 N over n = 12,14,...,22 (200 seeded reps per cell), with 95% bootstrap CI over reps (400 resamples).
- Prediction: every arm has point slope in [0.45, 0.55].
- Falsified: any large-prime arm (A_slp_B*, T_table) has CI upper bound < 0.45 (exponent lowered below birthday) while controls pass. That would make the double-large-prime/graph follow-up worth building.
- Consistent: all point slopes in [0.45, 0.55], controls pass (closes the toy-scale reopening, supports the recorded obstruction). Otherwise inconclusive.
- Negative control / null: B=0 plain birthday on independent samples (CI must cover 0.5).
- Positive control: synthetic cost 3*N^0.4 * Exp(1) noise through the same slope/CI code; detector must give CI upper < 0.45, otherwise verdict inconclusive.
- A/A: even-rep vs odd-rep halves of arm A_slp_B(first); |slope difference| reported (expected <~0.1 at full size; informational, not a gate).

## 4. Method
Generic group simulation: Z_N (N prime nearest 2^n), hidden x, "points" are exponents; each group operation (forming R=u+vx, each residual R - f_i, each table insert) is charged 1 op. Arm A: store all residuals, stop at first collision between different trials with v != v' (solve x, asserted correct). Arm T: table of l=isqrt(N) known random points; B=8 lookups per trial. Arm P: A with B=0. Seeds: ai*1000003 + n*7919 + k*104729 + 20261110. Python stdlib only, one core; estimated < 1 minute total (ops <~ 10^7). The 2LP/graph-cycle variant is represented only by its table-side desk bound (l + N/l), not a full graph solver.

## 5. Classification: runnable_local
A stdlib toy decides the exponent question for the generic-group model; cheap re-derivation of the recorded birthday obstruction.

## 6. What a run cannot establish
- It is a generic-group model with free exponent bookkeeping; it does not test real prime-field curve Semaev decomposition, linear algebra cost, or actual IC pipelines.
- No real double-large-prime graph/cycle solver is run; only the table variant. A consistent result does not exclude structure-dependent gains.
- Toy sizes (2^12..2^22); slopes carry finite-size constants. Results are labelled [pcc:unverified]; relabelling a birthday search is not an improvement.
