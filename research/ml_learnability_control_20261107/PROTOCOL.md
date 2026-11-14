# PROTOCOL: learnability control on toy prime-order groups (slug `mllc`)

Status: **DRAFT FROZEN FOR REVIEW 2026-11-07, before any scientific data.** Freeze = the main session
records the sha256 of this file and `mllc_harness.py` `PROTOCOL` dict in the task card and in
`experiment.protocol/v1`; any later edit is a new version. All numbers below are `[pcc:unverified]`.
Claim class: exploratory-confirmatory control (it can close a lead, it cannot establish an attack).
Lead: `LEAD-EXO-ml-learned-models`. Related: D4 of `leads-round2-20261104/new-leads-memo.md`
(heuristic HD4, "planted homomorphic leak reads chance unless its multiplicative height is small").

## 1. Question and hypotheses
Q. Can a generic learner predict a bit of the discrete logarithm k from the coordinates of Q = kP on a
prime-order curve of order N ~ 2^m, m in {12,16,20,24}, better than chance, and does the instrument have
the power to see a leak of a stated kind?

- H0 (curve): held-out accuracy of every model class on the curve arm is 1/2 (no learnable leak in bit
  encodings of (x,y)).
- H1 (curve): accuracy > 1/2 that survives the relabelling null and replicates on independent curves.
- HD4 (control validity, from the memo): planted leaks of the form label = lsb(c^-1 u mod N), u = c k mod N,
  are learnable only for small multiplicative height of c. This protocol **measures** that height ladder
  instead of assuming the control is valid; so the curve NULL is scoped to what the ladder shows.

## 2. Arms (all share N, k-sample, train/test split within a curve cell)
Labels: `lsb` = k mod 2; `msb` = [k >= (N+1)/2]. k drawn without replacement from [1, N-1] (N prime), balanced
labels by construction (lsb exactly; msb to within 1).

| arm | encoding fed to the model | label | purpose |
|:--|:--|:--|:--|
| `curve` | bits of x, bits of y (2m features) of Q = kP | lsb, msb | the experiment |
| `PC-A` delta in {0.02, 0.05, 0.10, 0.20} | curve features + 1 planted feature equal to lsb(k) with probability 1/2+delta | lsb | pipeline power on the **curve feature distribution** (explicit leak, strength known) |
| `PC-B` c in {1, 2, 3, 5/3, rand0, rand1} | bits of u = c k mod N (m features), Z/N with affine encoding | lsb (all c), msb (c = 1, rand0) | HD4 height ladder: homomorphic leak with multiplicative height h(c); c=1 is the identity encoding |
| `PC-D` | bits of h = g^k in F_p^*, p = 2q+1 safe prime ~2^m, N = p-1 | k mod 2 = Legendre(h) | planted smooth character (r = 2); predicted BLIND (QR is pseudo-random in bit encodings), reported as a labelled boundary, not a gate |
| null | each arm's own encodings, labels randomly permuted (train and test jointly) | permuted | the random-relabelling null; run for the `curve` arms (see section 5) |

Height of c = min over (|num|,|den|) representations a/b = c mod N of max(|a|,|b|) (small-height rational);
`rand*` have height ~sqrt(N).
PC-A is not a claim about learnability of a homomorphic leak. PC-B is. PC-A shows whether the harness can
see a leak of known advantage in the curve feature layout. The decision gate (section 7) uses PC-A; the
scope sentence uses PC-B.

## 3. Sizes, curves, samples
m in {12, 16, 20, 24}; per size 3 independent curves (p ~ 2^m, random (a,b), `#E` prime, `#E != p`, point count
exact by Legendre sum, order checked `N*P = O`); curve seeds `20261107 + 1000 m + i`. Points via exact tables
and one vectorised affine addition, validated against pure-Python double-and-add and on-curve checks
(`mllc_ecgen.validate`).

| m | examples per curve | train / test | test n | MDE per curve | MDE pooled over 3 curves |
|--:|--:|:--|--:|--:|--:|
| 12 | all N-1 (~4.1e3) | 80/20 | ~8.2e2 | 0.081 | 0.047 |
| 16 | all N-1 (~6.5e4) | 80/20 | 1.31e4 | 0.020 | 0.012 |
| 20 | 7.0e5 sampled | 5.0e5 / 2.0e5 | 2.0e5 | 0.0052 | 0.0030 |
| 24 | 7.0e5 sampled (4% of group) | 5.0e5 / 2.0e5 | 2.0e5 | 0.0052 | 0.0030 |

MDE = minimum accuracy advantage over 1/2 detectable with 90% power by the one-sided exact-binomial test at
cell level alpha_cell = 0.01/(4 sizes x 3 models x 2 labels) = 4.2e-4 (normal approximation in
`mllc_harness.mde`). **At N = 2^12 the instrument is blind below advantage ~0.05-0.08 whatever the model; at
2^24 the sample is 4% of the group**, so a leak needing the model to see most of the group (a lookup-style
memoriser) is outside power at 2^20 / 2^24 by design (state it in any NULL).

## 4. Model classes (fixed, no tuning on test)
`logreg` sklearn LogisticRegression(C=1, lbfgs, max_iter 300); `hgb` HistGradientBoostingClassifier(200 iterations,
depth 6, lr 0.1); `mlp` MLPClassifier((128,128), adam, batch 512, 20 epochs, no early stopping, train capped at
5e5). Input = 0/1 bit features. Seeds derive from the curve seed. No hyperparameter search: tuning would
need its own held-out split and a multiplicity correction. Versions are recorded in the output JSON
(dev check used scikit-learn 1.9.1, numpy 2.5.3, Python 3.13).

## 5. Statistic
Primary: held-out correct count c of n_test, accuracy c/n, Wilson 95% CI, one-sided exact binomial p against 1/2
(labels balanced). Pooled over the 3 curves of a size by summing counts. Null distribution: R = 20 random-
relabelling retrains (R = 10 for the mlp) per (curve, size, model, label) of the `curve` arm, giving mean and
max null accuracy; the binomial p is the primary test and the retrain null is a check that the training
pipeline itself (not just the test) is unbiased (a cell whose null mean deviates from 1/2 by more than 3 binomial
sd is flagged `NULL_BIAS` and excluded from NULL calls).

## 6. Variable ledger and execution
Confounders fixed: one container image (digest-pinned), CPU only, same library versions for all arms; arms
and models within a curve cell run in a fixed code order, but every cell is independent of the others, so order
cannot leak information (no shared state; seeds derived, not sequential). Analysis masking: the analysis script
reads cells by hashed arm id (not required for a single-analyst deterministic decision rule; the rule below is
mechanical). Spot preemption: the job writes one JSON per size (`--sizes m`), so a preempted size reruns whole;
results are keyed by seed and deterministic given the library versions (MLP/HGB threading can change the last
digits: recorded as a deviation if the reruns differ).

## 7. Preregistered decision rule (per size, model, label; `mllc_harness.decide`)
Let a = alpha_cell. Gate G = pooled PC-A(delta = 0.10) detected: pooled one-sided binomial p < a for that
model.
- **SIGNAL**: pooled curve p < a AND at least 2 of 3 curves have individual p < 0.05 with accuracy above their
  own null maximum. Then the next step is an independent re-draw of curves (fresh seeds) before any claim; it
  would be a candidate new statistic for `LEAD-EXO-coordinate-statistic-leakage`.
- **NULL**: not SIGNAL AND G holds. Reported with its scope: no learnable leak of advantage >= MDE(size) in
  the bit encoding for this model class; and, from PC-B, no leak of the form "label = bit of c^-1 u" for
  multiplicative heights above h*, where h* is the largest height of PC-B detected by that model at that size
  (a curve-arm NULL says nothing about leaks the PC-B ladder cannot detect).
- **UNDERPOWERED**: not SIGNAL and not G (so the harness cannot see even a 0.10-advantage explicit planted bit
  at that size / model), or the cell is flagged NULL_BIAS.
The lead is closed at a scale only if NULL is reached for all three model classes and both labels at that size
and the PC-B ladder detects c = 1 (identity encoding). Mixed outcomes are reported cell by cell; no
aggregation across sizes or models hides a SIGNAL (family alpha = 0.01 is split over all 24 curve cells).
Gaps to rho: not applicable (no solver is run; this is not a speedup claim); distance to a useful attack is
not measured here.

## 8. Predictions written before data (so they can fail)
1. PC-A is detected for delta >= 0.10 by all models at m >= 16; at m = 12 only delta = 0.20 (power).
2. PC-B: c = 1 detected by all models. c = 2 detected (lsb(k) is a bit-1 function of u up to an N-dependent
   carry), c = 3 only partly (a sawtooth of period N/3 in u). 5/3 and random c: chance at m >= 16 (HD4, ~70%
   prior). At m = 12 a random c can still be learned (every residue has a representative a/b with
   |a|,|b| ~ sqrt(N) ~ 60, which trees/MLPs can fit): this is the expected height-versus-size
   effect, not a failure. If random c reads >= 0.9 at m = 16 the memo's falsifier holds and PC-B is a
   valid control as designed.
3. PC-D (Legendre in bit encoding): chance for all models.
4. Curve: chance for all models and sizes (prior ~97%).
