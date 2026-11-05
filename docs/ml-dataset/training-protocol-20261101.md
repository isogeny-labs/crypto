# ECDLP dataset: prospective ML training and evaluation contract

Status: **design preregistration, not a frozen experiment or trained model**.
Written 2026-11-01 before any model fit on the native dataset. A machine-checked
`experiment.protocol/v1` record, its hash, source and analysis hashes, held-out
cohort manifest, and budget admission must be committed **before** a decisive
run. This document fixes the scientific decisions that record must encode.
Changing them after inspecting held-out outcomes requires a dated amendment and
a new holdout. The existing 252-cell local matrix is development-only.

This contract refines [DESIGN.md](DESIGN.md) and [OPERATIONS.md](OPERATIONS.md),
which remain authoritative for curve identity, export, replay, and custody.
Its purpose is to test whether models discover transferable algebraic structure
and improve verified solver decisions, while separating prediction, finite
counterexample, and theorem claims.

## Evidence available at registration

The local all-arity compatibility matrix contains 252 cells: n in {23,47,61},
three families (Koblitz, other binary, prime), both twist positions, and every
m from 2 through 15, with one seed/generator/key per cell. All 252 known
answers and receipts were verified. The PDP `enumerate` run yielded 68 verified
witnesses and 184 `budget_exhausted` outcomes at work limit 500. A PDP witness
is not an independently recovered ECDLP scalar. These cells were generated
with an explicitly overridden build identity; they are not a frozen campaign.
They are useful for schema, leakage, ingestion, and end-to-end smoke tests only.
The sanitized aggregate receipt is
`s3://setec-astronomy-artifacts/store/sha256/991271fe05f137dd3a9b7e8ba1020dfa5cb8303e516e84a7bbffde4b90d688ff`.
The private 252-row candidate-test join remains under restricted local custody;
its sanitized count/hash receipt is
`s3://setec-astronomy-artifacts/store/sha256/096b3475e88fafa4cccfb22fdd5362538179cb5135a432282e2e7b461e66e504`.
Neither receipt is a training export or a model result. Current prefix export
policy is `strict-structural-allowlist-v5`; older identities must be regenerated
before mixing.

## Claims and task-specific labels

| Task | Admissible input at decision time | Label and eligibility | Decisive check |
|---|---|---|---|
| Structural factor-base/routing model | public curve and field representation, current verified factor-base domain, target point, current graph, caps | verified witness/event and charged work for each **actually attempted** arm; unattempted alternatives are missing, not failures | held-out, paired, verified completion and cost under a fixed fallback |
| Prefix action model | `prefix-features.jsonl` state before the choice, legal-action mask, prior certified events | observed legal branch/action and checked suffix outcome; an observed choice is not optimal | exact replay of every proposed action and held-out charged cost |
| Resource/success model | same public prefix or static features | work-to-event, event observed versus right-censored, terminal status | calibration and censored risk on held-out groups |
| Algebraic-structure conjecture miner | exact public equation graph and declared ring/ordering | independently certified orbit, relation, rank, symmetry, or finite counterexample | independent symbolic checker on a fresh complete domain |
| First/last fall-degree predictor | exact ring, equations, quotient, field equations, variable/monomial order | **only** an exact certified value or explicit interval/lower bound with censoring; ordinary degree drops and maximum visited degree are distinct observations | complete lower-degree exclusion/closure certificate or formal proof for a mathematical claim |
| SAT/F4/F5/F6 policy | certified pre-decision trail/matrix/signature/queue and legal actions | proof-checked transition and total charged outcome | held-out solver with identical verifier, caps, fallback, and source snapshot |

No SAT decision policy is trained from a final SAT outcome alone. No F4 pivot
policy is trained from a static graph lacking pre-pivot rows. The current public
export has static prime-chain graphs and bounded recursive branch prefixes;
complete intra-F4, SAT trail, and F5/F6 policy streams are still separate
instrumentation gates. F6 geometric closure and degree-filtered engines get
distinct capability names. PKM audit scores do not become PKM solve labels.
Prime-field p-power Frobenius on F_p-rational coordinates is trivial and is
never counted as a nontrivial augmentation.

For exact fall-degree work, freeze the **statement** as well as the model:
coefficient field, polynomial ideal, field equations, term order if relevant,
the quantified family, and the meaning of first/last fall degree. A single
checked counterexample can refute that precise universal statement. Finite
positive samples cannot prove it; a learned formula is a conjecture until an
independent proof. Store witness polynomials and checker digests, not only a
predicted number.

## Input boundary and custody

The model loader accepts only `features.jsonl`, `prefix-features.jsonl`, and
validated `algebraic-features.jsonl` where the task requires its exact static
algebraic graph, after pinned-manifest verification and the same public
allowlist checks. Restricted `targets.jsonl`, `index.jsonl`,
and `prefix-records.jsonl` are joined by a separate evaluator **after** feature
construction. The learner never reads raw cells, source paths, known scalar,
generator multiplier, recovered log, future branch/pivot/conflict, solver name,
certificate path, status, receipt, elapsed time, or a hash computed from them.
Public identity hashes are checked to depend on the strictly projected
instance, actual factor base, and PDP domain. PKM Kummer, ordinary cyclic
subgroup, and signed Frobenius domains are not mixed as equivalent arms.

Before fitting, the evaluator writes an allowlist report with field paths and
types, scans public JSON recursively for forbidden keys and provenance
strings, checks that a synthetic target/label swap leaves public bytes
unchanged, and checks that changing a pre-decision public state changes its
identity. It verifies immutable input digests, replay certificates, group
membership, split disjointness, and expected row/terminal-status counts.
Private files have mode 0600 or stronger and do not enter a public model
checkpoint, training log, S3 source packet, or prompt. Predictions carry only
public identity plus evaluator-side join references.

## Population, splits, and sampling

The full requested domain remains n=23..31 and 47..61, all three curve
families and twists, m=2..15, multiple independently sampled curves,
generators, and uniformly sampled keys, including p=2^61-1. Every parameter
cell appears in the coverage manifest even if unsupported or censored. The
pre-run protocol names exact per-cell quotas, generator/key sampling, rejection
probabilities, caps, instrument hashes, and immutable source shards. The
252-cell compatibility matrix may tune parsers and debug a model but may not
select a winner, estimate generalization, or provide the final test set.

Build connected split components **before** looking at outcomes. An edge joins
keys and generators on one curve, both twist positions, isomorphic models,
verified isogeny-linked models, Frobenius-equivalent presentations, and
same-source equation/target variants. Merge components further for shared
construction seeds, factor bases, polynomial systems, or public graph hashes.
An unknown link is conservatively grouped, never assumed independent. Hash
the resulting component with a committed split seed into train/validation/test
at 70/15/15 by component, then report actual row and stratum counts. If a
stratum has too few components to populate each split, retain it as
`insufficient_independent_groups` rather than moving individual rows.

Two **separate** transfer panels exclude complete field sizes and complete
curve families from training, respectively. The size panel must include at
least one n from 23..31 and one from 47..61; exact sizes are frozen before
their outcomes are read. The family panel rotates the held-out family across
Koblitz, other binary, and prime in three independent fits. The p=2^61-1
prime cohort is reported explicitly and not silently pooled into a broader
prime label. Additional field-polynomial and encoding holdouts are named in
the machine protocol. No row-wise random split or reuse of a held-out curve
under a different generator, twist, sign, orbit, or equation presentation.

Natural unconditioned targets and planted-solvable PDP targets are separate
populations, with fixed sampling/rejection rules and selection weights.
Synthetic short-key, known-log, and trivial-symmetry controls stay separately
tagged. The primary analysis is on natural, whole-subgroup targets. Planted
cases test whether the pipeline can detect a known signal; they do not set its
estimated natural success rate.

## Arms, controls, and outcome accounting

For a future policy comparison, the reference is the fastest applicable
verified classical policy for the **same computation** and frozen inputs,
selected by a prespecified rule or a training-only pilot and committed before
any held-out outcome is inspected;
the baseline is the unmodified native policy. The candidate is a learned
proposal with exact legality checks and fallback to baseline. The A/A arm runs
the baseline twice with different nuisance order/placement. Positive controls
are planted decompositions and known legal actions passed through the same
export, model, and verifier. Negative controls include within-stratum label
permutation and orbit/coordinate presentation changes that must preserve a
declared invariant. A failed positive control invalidates that endpoint; a
negative-control signal requires leakage investigation before reading efficacy.

Every attempted arm retains construction, feature extraction, inference,
rejected proposals, fallback, solver work, failed branches, verification,
checkpoint/restore, and publication costs. Keep reusable setup and per-target
online costs separate. Capped, crashed, unsupported, and preempted cases stay
in the denominator/flow with explicit status. `budget_exhausted` is
right-censored at its charged limit, not UNSAT or a zero. A SAT UNSAT label
requires a checked proof under its exact formula and assumptions. PMU
instruction counts are null until the whole worker boundary and cgroup
containment are validated; no surrogate instruction-to-solve label.

Primary policy endpoint: independently verified completion probability at the
same fixed cap, paired by target and split component, followed by total
charged counted work per verified completion with failures charged to cap.
Report unconditional total work and success separately to expose selection.
Instruction counts are an additional primary counted metric only in a cohort
with validated PMU coverage. Wall time is descriptive until its own isolated
L2/tier-B protocol, paired A/A interval, host/cgroup and restore rules are
frozen. For a one-target ECDLP cost comparison, require complete scalar
recovery and replay, all IC phases and a matched one-target rho reference;
PDP witness counts do not enter that table.

For resource prediction, use censored likelihood or survival calibration
with event/censor indicators, not ordinary regression on timeout caps. Record
integrated Brier score, calibration by family/size/twist/m, and prediction
interval coverage at prespecified work limits. Structural conjectures use
exact checker pass/fail and the number of distinct complete held-out domains;
no accuracy score substitutes for a proof.

## Frozen run mechanics and analysis

The machine protocol binds the source commit, toolchain, binary/model hashes,
public/restricted snapshot digests, split seed, all hyperparameter grids,
training seed list, feature allowlist, analysis code hash, caps, host class,
threads, checkpoint interval, and preemption policy. Training runs use Spot
throughput capacity with immutable checkpoints; each resumed run records
attempts and restores. No timing comparison uses a clock spanning a restore.
Train/validation selection may inspect only their respective groups; the test
and transfer panels are opened once for the chosen checkpoint and analysis
script. No new features, threshold, architecture, or stopping rule may be
chosen after that opening. Retain all seeds and failed fits.

Use five prespecified training seeds per arm, with paired target instances and
identical caps. Freeze the actual independent component counts and a pilot-based
power calculation before execution; the existing 252 rows cannot justify a
power claim because they contain only one generator/key per cell. The primary
comparison uses a component-cluster bootstrap (10,000 draws, fixed seed), a
two-sided 95% interval for the paired difference in verified completion and
for the log ratio of charged work. The candidate is a measured improvement
only if the completion-difference lower bound is nonnegative, the work-ratio
upper bound is below 1, all correctness checks pass, and no prespecified
family/twist/size stratum has an unreported regression. Other endpoints use
Holm correction across their frozen hypothesis family and report effect sizes
regardless of significance. If the power check fails, report an estimate and
interval without a positive decision. The analysis outputs planned, attempted,
completed, censored, excluded-by-predeclared-rule, and replay-failed counts
for every arm and stratum, plus public model and private evaluation hashes.

The `experiment.protocol/v1` checker and frozen `experiment.export/v1` flow
are required before any comparative claim. The source-to-export adapter must
map every run attempt, warmup, resume, counted metric, wall-valid flag,
verification status, and artifact digest. A later reviewer independently
recomputes headline numbers and samples replay certificates. No model is
approved for a live solver from offline accuracy alone: proposed actions pass
the native exact legality checker, and the fallback is charged.

## Literature and claim boundaries checked

- Semaev, [*Summation Polynomials and the Discrete Logarithm Problem on
  Elliptic Curves*](https://eprint.iacr.org/2004/031): algebraic relation
  construction motivates exact equation and lift replay.
- Huang, Kosters, Yang and Yeo, [*On the last fall degree of zero-dimensional
  Weil descent systems*](https://arxiv.org/abs/1505.02532): its hypotheses
  require explicit checking before transferring a bound to a dataset family.
- [*On Generalized First Fall Degree Assumptions*](https://eprint.iacr.org/2015/358):
  motivates separating observed drops from a universally quantified claim.
- Selsam et al., [*Learning a SAT Solver from Single-Bit Supervision*](https://arxiv.org/abs/1802.03685):
  motivates a learned SAT baseline, but outcome classification does not by
  itself certify decisions or replace proof checking.

The proposed neural tasks are hypotheses about transferable structure.
Verification of one component, a checked counterexample, a measured policy
gain, and a general theorem are distinct result types and must be reported
with their own evidence.
