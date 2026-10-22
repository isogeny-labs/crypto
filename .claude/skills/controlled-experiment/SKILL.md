---
name: controlled-experiment
description: Design, preregister, run and report a controlled experiment before making any empirical or comparative claim about curves, primes, groups, algorithms or their structure ("X is unusual", "X has no effect", "X is as random as", "method A beats B"). Requires classifying each claim, a negative-control population drawn from the right null distribution, a positive control proving the detector can see the effect, an A/A arm for the noise floor, the same instrument on every arm, a variable ledger that isolates every confounder (instrument, host, layout, order, seeds, spot preemption, analyst), interleaved arm order and derived seeds, masked arm labels during analysis, a frozen protocol (experiment.protocol/v1) before data, a unified experiment.export/v1 of every row, and reporting by rank/effect size with uncertainty. Use whenever a research note, PR or answer would state something an exact computation does not by itself establish.
---

# Controlled experiment

Read `AGENTS.md` and the `report-evidence` skill first. This skill decides
*whether a claim needs an experiment*, and if so, *what makes the experiment
able to fail*. `ecbench-measure` remains the harness for ECDLP cost
comparisons; use this skill to design what goes into it, and use this skill
alone for structural or statistical claims ecbench does not cover. The
record of the design is the cross-repository protocol
(`experiment-protocol` skill; schema and checker in
aburan28/crypto-autoresearcher, `docs/experiment-protocol.md` there), and
the record of the data is the unified export; this skill says what goes in
them for a crypto study.

## 1. Classify every claim before writing it

Put each sentence you intend to publish in exactly one class, and label it
in the note and in `protocol.json` `claims[].class`.

| class | what establishes it | needs a control? |
|:--|:--|:--|
| **Theorem / identity** | proof, or a certificate (`identity.certificate/v1`) | no |
| **Exact computation** | a deterministic, verified computation (a factorisation checked by product and primality, an exact order) | no, but it must be reproducible and verified |
| **Comparative / statistical** | "unusual", "typical", "no effect", "as random as", "faster", "smaller", "independent" | **yes — this skill** |
| **Heuristic estimate** | a model (Dickman ρ, a square-probability argument, a cost formula) | it is a prediction; state it as one, and test it if the conclusion rests on it |
| **Out of scope** | a security or impossibility statement no experiment here can reach | say "not established" |

A comparative claim made from an exact computation alone ("P-256's n − 1 is
unusually smooth" from one factorisation) is not established. One number has
no reference. Do not write it until §2–§5 are done.

## 2. Build the control arms

Every comparative claim needs all three, or a written reason each is not
applicable (`controls.*.not_applicable_reason`).

- **Negative control (the null population).** A sample from the distribution
  the claim says the target is *not* different from, or *is* different
  from. Draw it from the closest valid null, and write down why it is the
  right one. For example: group orders of curves over a fixed `p` are
  well modelled by random primes in the Hasse interval `[p+1−2√p, p+1+2√p]`.
  Random 256-bit integers are a worse null, because they are not prime and
  not in the interval. Match everything except the property under test
  (`null_population.matched_on`): size, primality, field and curve shape.
- **Positive control (detector power).** An input where the effect is known to
  be present, run through the *same* code path. A "found nothing" result is
  only evidence if the detector demonstrably finds the thing when it is
  there. Plant it: a curve with a constructed small point, a prime built to
  be smooth, an instance with a known answer. Write the expected detection
  down so a miss is unambiguous.
- **A/A control (the noise floor).** Two arms that differ only in a nuisance
  factor (seed, variable order, a re-encoding, a second build of the same
  input). Its spread is what every difference is read against; a difference
  inside it is not a difference. An A/A arm that shares seeds with the
  reference measures determinism, not variance: say which one you want.
- **Same instrument on every arm.** Same code, same budgets, same seeds policy,
  same metric, same stopping rule (`same_instrument_as` makes the checker
  compare commit, binary and threads). If the instrument is incomplete (a
  factoring budget that leaves cofactors), that is acceptable *only* if it
  is identical across arms and the metric is defined on what it returns.
  Never compare a PARI factorisation of the target against a budgeted rho
  of the controls.

## 2a. Isolate the confounders: the variable ledger

List every factor that could move a primary metric, with its treatment
(`fixed`, `randomized`, `crossed`, `measured`), and every comparison with
its design (`one_factor`, `factorial`, `absolute`, `aa`, `confounded`). A
comparison that differs in two factors is confounded and decides nothing;
cross them or fix one. The usual factors here:

- **Instrument**: commit, `rustc -Vv`, build flags, binary SHA-256,
  allocator, `RAYON_NUM_THREADS`. Fixed, unless it is the variable.
- **Host**: CPU model and features, governor and turbo, SMT, NUMA, kernel,
  hypervisor, co-tenants, IRQs, memory pressure. Measured per row by ecbench
  (levels L0–L3) and isolab (tiers A–D); a clock is evidence only at the
  protocol's level, and only against the session's A/A interval (AGENTS.md
  §10, §12).
- **Memory layout and environment**: ASLR, environment size, link order,
  page cache, THP. Fresh processes per replicate sample the layouts; isolab
  records the settings.
- **Order and warm-up**: one-time costs (thread pools, lazy statics, cold
  caches) land on the arm that runs first. Interleave arms within every
  round (`measurement.order: alternate`), discard warmups, and never run a
  `fixed` order without a written reason.
- **Inputs and seeds**: instances sampled independently of arm; seeds from
  the protocol hash, an explicit list written before any data, a nonce drawn
  after the freeze, or the harness's frozen rule. Never a seed chosen after a
  look, never favourable seeds (AGENTS.md §4).
- **Spot preemption**: a preempted and restored row keeps its counts and
  loses its clock (`wall_valid: false`, `resumed > 0`). Declare the
  checkpoint policy and this consequence before the run
  (`queue-experiment`).
- **The analyst**: mask arm labels (`experiment_protocol.py blind`) until the
  analysis script is frozen and has run; the key stays closed and its hash is
  in the export.

## 3. Freeze the protocol before running

Write `PROTOCOL.md` (the narrative) and `protocol.json`
(`experiment.protocol/v1`) in the study directory and commit them before
the first data point; `freeze` records the canonical hash and commit. The
protocol states:

1. the question and the exact claim each outcome would license;
2. the target(s), the null population and how it is sampled (generator,
   seed, sample size `N` and why `N` is enough for the resolution you need);
3. the positive control and the expected detection; the A/A arm;
4. the metric(s), defined precisely from event to event, with unit,
   resolution and uncertainty method, and the instrument with its budgets;
   counted metrics first, a clock only with its justification and level;
5. the decision rule, for example: "the target is *unusual* if its empirical
   rank is below the 2.5th or above the 97.5th percentile of the control,
   two-sided, with a Clopper–Pearson interval on the tail fraction";
6. how many metrics and targets are tested, and the multiple-comparison
   correction (Bonferroni over the family is fine);
7. the stopping rule (fixed `N` or a preregistered sequential rule; never
   "until it looks right"), the execution backend and checkpoint policy,
   and the analysis script by path and SHA-256;
8. what does *not* count: changing `N`, metric, null or threshold after
   seeing results. Any such change is a new, labelled protocol version
   (an amendment that says whether it preceded the data it affects), and
   both versions are reported.

**Power check before freezing (do the arithmetic).** An empirical rank test
cannot reject below its resolution. With the add-one estimate the smallest
attainable two-sided p is `2/(N + 1)`, so `N` must satisfy
`2/(N + 1) < α/m` for `m` corrected tests, with margin: `N + 1 ≥ 4m/α`
lets a target beyond every control reach `p = α/(2m)`. Check this for every
test family in the protocol and write the numbers into it
(`experiment_protocol.py power --n N --alpha α --m m`; `validate` refuses a
`passes` that disagrees). A protocol that fails this check can only ever
report "within the control distribution", so its nulls are vacuous. (The
first protocol written with this skill,
`research/nist_cyclotomic_controls_20261011/PROTOCOL.md`, failed it in all
three comparative families. The fix was a dated amendment, not an edit.)
**Pre-flight runtime estimate (write it into the protocol).** Before
freezing, time one instrument call per arm and per size class, and
separate the *per-sample* cost (scales with `N`) from the *fixed* cost
(target and positive controls, paid once per curve or target). Then
estimate `fixed + N · per_sample / threads` for every family. Fixed costs
are easy to miss. In the NIST re-test the dependent-pair positive control
(heights of `[3]G̃` and `[4]G̃`, about 16 times the generator's height and
super-linear in it) took longer than 100 random pairs did. Choose `N`
from the power check first. If the estimate is infeasible, change the
design (a cheaper positive control of equal strength, fewer families),
never the power. Record the estimate next to the actual run time, so the
next protocol calibrates against it.

Also do a dry run of the full analysis at small `N` on scratch output. It
catches power failures, runtime blow-ups and broken positive controls
before the real run. Disclose in the amendment anything that dry run
revealed about the target.

## 4. Run

- Native code per `AGENTS.md` (Rust), with the seed for every row derived as
  the protocol says and recorded in the output.
- Emit raw per-sample rows (JSON lines or CSV), not just summaries, so the
  rank and intervals can be recomputed; keep timeouts, failures and refuted
  certificates as rows with their status.
- Run the positive control first. If it fails, stop and fix the instrument.
  A negative result from a blind detector is reported as *invalid*, not as
  "no effect".
- A run that needs an isolated host or cheap long capacity goes through the
  queue (`queue-experiment`): measurement jobs to an isolated pool,
  throughput jobs to spot capacity with checkpoints.
- Under Claude Code, a frozen protocol's run commands may go through the
  model tiers (AGENTS.md "Model tiers"): `experiment-director` writes the
  command card, the main session relays it verbatim to `executor-mechanical`,
  and `experiment-director` assesses the run. The report (§5) stays with the
  reasoning tier.
- Write `export.json` (`experiment.export/v1`) beside the rows:
  `experiment_protocol.py from-rows rows.jsonl --protocol protocol.json
  --out export.json` (or `from-ecbench` for a session), then
  `check-export --verify-files` until clean. Its `flow` block (planned,
  attempted, completed, excluded by rule) goes in the report.

## 5. Report

Lead with the table. For each target × metric:

| target | metric | target value | control median [2.5%, 97.5%] | target rank / N | tail p (two-sided) | A/A spread | positive control detected? | verdict |

- **Verdict wording is fixed by the decision rule.** Use "within the control
  distribution (rank r/N)", "more extreme than q% of controls", "positive
  control failed — inconclusive". Never write "random", "as large as
  possible", "no effect" or "never" unless the claim class allows it.
- **Effect size before significance.** Give the difference in the metric's
  units next to the p-value, and the A/A spread it exceeds.
- **Scope sentence.** State what the experiment does *not* show. "Within the
  null on these metrics" is not "secure", and "no small point below height H in
  this census" is not "no small point exists".
- Keep every earlier claim that this experiment weakens visible, with a
  dated correction: the original sentence, what was wrong, and the replacement.
- Invite the second reader: another runner reviews the export with
  `review-run` and files an `experiment.review/v1` record under `reviews/`;
  cite it in the PR. A result PR without one says so.

## 6. Checklist (paste into the PR)

- [ ] every published sentence is classified (§1), in the note and in `protocol.json`
- [ ] comparative claims have a negative-control population with a stated null, matched on everything but the property under test
- [ ] every "nothing found" has a passing positive control on the same path
- [ ] an A/A arm with independent nuisance draws sets the noise floor
- [ ] one instrument, identical budgets across arms; `same_instrument_as` checked
- [ ] variable ledger lists every confounder; no decisive comparison is confounded
- [ ] arm order interleaved; seeds derived before data; arm labels masked during analysis
- [ ] power check passes: `2/(N + 1) < α/m` for every test family (§3)
- [ ] pre-flight runtime estimate (fixed and per-sample) in the protocol (§3)
- [ ] `PROTOCOL.md` and `protocol.json` frozen (hash recorded) before results; dry run on scratch output
- [ ] raw rows committed; `export.json` passes `check-export --verify-files`; summary recomputable
- [ ] decision rule and multiple-comparison correction applied as written
- [ ] verdicts use the fixed wording; scope sentence present; A/A spread reported
- [ ] superseded claims corrected in place, with date
- [ ] a `review-run` record by another runner cited, or its absence stated
