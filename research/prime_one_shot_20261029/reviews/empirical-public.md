# Independent evidence review of the one-shot prime-field proposal

2026-10-29. Reviewer task: `/root/evidence_review`. This is a source and retained-record audit, not a new solver invocation, performance experiment, or independent cryptographic replay. Public-source findings are separated from the private-repository appendix.

## Main conclusion

The report's recounts and quoted symmetric matrix/counter values are correct. The most consequential omitted qualifications are: (1) the original msolve timeout host was itself badly overloaded, (2) the incomplete m=4 N=20 runs nevertheless establish a genuine engine-specific lower bound of at least degree eight, (3) the symmetric example's generator does not require actual rational subgroup points or a prime-order nonsingular curve, and (4) its plain and symmetric comparators have different repeated-abscissa domains. Consequently the retained symmetric experiment is a reason to investigate an encoding, but not clean evidence that coefficient sharing alone improves an actual prime-order PDP pipeline.

## Independent retained-record checks

Performed with separate jq/awk/cmp commands rather than the parent audit script:

- `research/ic_next10_20261014/03-pkm-degree/results.csv`: 24 rows; 16 have `verdict=no_solution` and exit zero; eight have `verdict=none`; exit counts 124/139/143 are 4/3/1. No verdict/exit inconsistency was observed. Every completed row is a refutation.
- `research/pkm_symmetric_tower_20261026/rows_m3.jsonl`: six overlapping symmetric rows, t=2,3,4 and targets 0,1, match `rows_m3_symmetric.jsonl` after removing the elapsed `ms` field. The latter also contains two t=5 rows, so the overlap is not the entire repeat file.
- At m=3,t=4,target=0, plain-chain/symmetric max columns are 12,314/1,224, a ratio of 10.0604575163. Field-operation counters are 15,884,596,296/124,297,944, a ratio of 127.794521654. These counters exclude system construction and are not complete DLP costs.
- All 90 `rank6_lattes_isogeny_factor_base/raw/run1.json` records have msolve return code zero, `root_check=true`, `singular.plant_not_eliminated=true`, and engine status `ok`. The entire run1/run2 files compare byte-identically, not just their deterministic projection.
- ECBench `evidence/canonical_run1.json` and `canonical_run2.json` compare byte-identically. They are binary control systems, not prime-field evidence.

No experiment output was altered.

## Findings requiring explicit reporting

### A. The original msolve wall stops ran at about one fifth of a core

Public sources:

- `/workspace/crypto/research/ic_next10_20261014/03-pkm-degree/README.md`, sections “Where the wall falls,” 2.2, and Setup.
- Git-tracked raw log `research/ic_next10_20261014/03-pkm-degree/runs/pkm_kummer_p2013265921_m3_t7_x221026520.log`.
- Matching N=18 raw log `runs/pkm_kummer_p2013265921_m3_t6_x1219323407.log`.

The README reports a load average of 22–25 on four cores. It says single-thread jobs received around 19% of a core. The raw N=21 log's first degree-seven round reports **2,041.12 seconds real / 395.68 seconds CPU**, a ratio of 0.1938543545. The N=18 README reports **2,024 real / 384 CPU**, ratio 0.1897233202.

This is more directly relevant to the user's concern than the older overloaded M4 microbenchmark. A rerun of the exact wall-censored m=3 cells on an admitted isolated host is scientifically justified. The justification concerns uncensoring the degree/completed-work result; it does not imply that the solver exponent will improve. Later N=21 attempts already show that a clean host alone is insufficient to obtain a full answer cheaply, so a rerun must preserve those later records and resource limits rather than pretend they never happened.

### B. Some censored runs still supply a real degree lower bound

Public sources:

- `/workspace/crypto/research/ic_next10_20261014/03-pkm-degree/README.md`, opening verdict and section 2.2.
- Raw logs `runs/pkm_kummer_p2013265921_m4_t5_x843615913.log` and `runs/pkm_kummer_p2013265921_m4_t5_x362838726.log`.

The m=4,N=20 runs finish three productive degree-eight rounds. In the checked target-one raw log these add **1,241, 2,507, and 3,578** basis elements at matrices 78,739×134,970, 151,098×236,724, and 213,684×320,433. The next degree-eight round starts but does not finish before allocation failure/exit139 under the five-GB virtual-memory cap.

Thus “eight incomplete rows” is correct, but does not mean all eight have no scientific information about degree. These two runs support the **engine/presentation-specific `D >= 8`** lower bound under the experiment's metric, and refute that instrument's plateau-at-seven hypothesis at m=4,N=20. They do not supply a completed solving degree, useful SAT extraction, or complete work cost. This is material counterevidence against an unqualified bounded-degree hope.

Important source precedence: `logs/trace_m4_t5_x843615913.txt` is truncated after the start of the second degree-eight round; the raw `runs/*.log` contains all three completed rounds. Use raw solver logs, not that stale derived trace, to verify the claim.

### C. The symmetric comparator changes the finite domain

Public sources:

- `/workspace/crypto/examples/pkm_symmetric_tower.rs`, `plain_system`, `symmetric_system`, and main.
- `/workspace/crypto/research/pkm_symmetric_tower_20261026/README.md`, Construction and Controls.

The symmetric system enforces `Q | X^(2^t)-1`, so the monic divisor Q is squarefree in the relevant characteristic and has distinct abscissas. The plain per-root towers permit repeated abscissas. No matching distinctness constraint is inserted in `plain_system`.

The six compared m=3 systems all refute. Matching refutations do not establish that the full model sets or chart coverage match. The width/counter advantage is therefore confounded by elimination of repeated-root components as well as root permutation quotienting and circuit presentation. Future comparisons should use the **same selected distinct-abscissa chart** on both arms, then separately account for coverage and excluded charts. The root report's future protocol already asks for this; its discussion of existing supporting evidence should acknowledge that the earlier experiment did not meet it.

### D. The symmetric generator is an algebraic instrument, not a rational subgroup fixture

Public source: `/workspace/crypto/examples/pkm_symmetric_tower.rs`, main around lines 606–618, `Instance::base`, and random-target loop around line 674.

Current source draws a,b,g from field values, without a discriminant check, curve-order test, or subgroup selection. `Instance::base` returns every abscissa in `g mu_(2^t)` without checking whether f(x) is a quadratic residue. Its measured target is `rng.below(p)`, not a point generated in E(F_p). The planted control chooses x-values and S3 roots and evaluates equations at a constructed assignment; it does not recover a root through the solver or independently verify a sum of rational subgroup points.

Semaev vanishing is fundamentally an algebraic-closure point relation unless field-of-definition conditions are enforced. These omissions therefore matter. This does not invalidate the quoted F4 measurements on those exact algebraic systems. It limits their interpretation as natural prime-order ECDLP decomposition costs. The report should not silently transfer those observations into a prime-order or on-curve target population.

Raw symmetric records also omit a,b,g,zeta,x_R, system hash, binary hash, and source revision; they retain only p,m,t,target ordinal and outcome/counter fields. The current deterministic source explains how targets align across arms, but exact source/input provenance is weaker than in a sealed ecbench record. Repeated counter agreement is not independent replay of a fully bound original input.

### E. Planned successful-root controls are not evidenced by the audited original CSV

Public sources:

- `/workspace/crypto/research/ic_next10_20261014/03-pkm-degree/PROTOCOL.md`, Arms row PL and instrument gate.
- Same study's README, “Positive control” and “Instrument check at p1.”
- Git tree under the study directory, original CSV, and Setup draw flags `--planted 0 --random 2`.

The protocol asks for planted-solution systems whose known point must be among msolve's returned solutions. The retained results instead cite only the 16 refutations agreeing with another instrument/exhaustive search. No planted-named run is present in the tracked study tree and the CSV contains no successful-root result. This audit cannot establish that the PL arm was executed and retained. Phrase the gap as **not evidenced**, rather than proving that no person ever ran a planted test elsewhere.

The symmetric example's equation-evaluation checks are also not a SAT extraction control. Future work needs both a true planted assignment and a successful solver/extractor path verified independently in the group, with natural successful targets separate from planted ones.

### F. Rank-six Lattes/isogeny null has real instrument checks and real sensitivity limits

Public sources:

- `/workspace/ecbench/research/summation_dreg_20261023/exotic/rank6_lattes_isogeny_factor_base/{PROTOCOL.md,RESULT.md,tower.py,one.py}`.
- Its raw run1/run2 JSON.

Unlike the symmetric instrument, rank-six target construction uses actual points from a nonsingular generated curve, forms a target by independent group addition, and checks the planted point is not eliminated. This is useful SAT consistency evidence, though not a complete relation-extraction performance study or prime-order subgroup experiment.

The top membership constraint is the degree-s product over chosen terminal images, and s is 16,24,32. This dominates the observed F4 degree floor. Families match k,d,s and number of variables, **not the number of usable point abscissas**. At p=211,k=2,d=4,s=16, for example, Lattes usable-x counts vary 7,22,59 while generic counts are 20,14,12 over their three seeds. The actual different base densities can confound yield and Singular degree comparisons.

Rank-six's msolve parser reads round degrees and is reasonably tied to the advertised metric. Its Singular parser removes bracketed protocol tokens and takes the largest remaining integer before `DIM`; the code does not itself certify that every remaining integer is a degree. The report should avoid turning those Singular numbers into an intrinsic invariant without a parser calibration. No fresh Singular process was launched here to test the parser.

The msolve wrapper also marks a nonzero return code `ok` if at least one degree was parsed, except for timeout124; in this particular retained run all codes are zero, so this latent status bug did not contaminate the observed 90 rows. Harden it before extending the study.

A rerun with equal usable base size and no high-degree product-polynomial floor is justified to measure a different, more sensitive hypothesis. Repeating the same existing design merely on a less busy host would not resolve those structural confounders.

## Audit script robustness

`audit.sh` produces the correct current counts, but it does not enforce its own source bindings:

1. It never verifies `inputs.sha256` before parsing. A future sibling checkout could drift while the audit still reports successful record comparisons.
2. Empty/equally truncated arrays would compare successfully. Assert expected nonempty sizes, required keys, uniqueness and source identity.
3. AWK chooses CSV columns by header but does not reject missing/duplicate headers or unexpected verdict/exit combinations. Validate them before aggregating; assert completed and incomplete totals reconcile with all exits.
4. `D_msolve` on incomplete CSV rows is the maximum degree selected/seen, not a completed solve degree. Preserve `D_last_productive` and protocol interpretation in any downstream table.

These are integrity fixes for a bounded recount, not new cryptographic tests.

## Priorities for a justified rerun

1. Correctness admission on actual prime-order nonsingular prime-field fixtures, on equal charts, retaining full point/system/source identities and an independent group-law verifier.
2. Successful-root solver/extractor control and natural SAT targets alongside separate UNSAT refutations.
3. Exact original m=3 overloaded-wall cells on an admitted isolated host, if the scientific question is still the original instrument's final degree; report later uncensored/capped attempts too.
4. Existing symmetric and proposed coefficient circuits on a matched squarefree chart before attributing a gain to symmetry or shared membership.
5. Rank-six-like towers only after matching usable cardinality and removing the product-degree floor; compute completed cost rather than degree alone.

None of these tests, even if favorable, proves a growing-arity sub-exponential algorithm.
