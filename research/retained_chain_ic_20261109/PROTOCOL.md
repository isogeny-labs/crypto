# Retained Kummer and elliptic-addition chains

Status: preregistered implementation and bounded experiment; no new run claimed.

## Request and acceptance criteria

Retain the factor-base power chains and elliptic-addition intermediates, compare
elimination/variable orders and coefficient growth as the summand count grows,
then exercise complete relation collection, independence, linear algebra, and
recovery of a public known-answer toy ECDLP. Publish the code and evidence in a
regular PR to `isogeny-labs/crypto`.

| Requirement | Acceptance evidence |
|---|---|
| Both structures retained | Actual S3 equations plus Kummer square rules, intermediate-variable maps, no S4/S5 expansion on retained arms |
| Ordering comparison | Identical curve/base/targets under summand-major vs level-major tower layouts; left vs balanced four-summand trees |
| Exceptional sums | Explicit projective infinity charts; returned witnesses checked by curve arithmetic |
| Growth | Input/echelon term counts, matrix width/nonzeros, productive degree, counted F4 multiply-adds, bound/timeout status |
| Complete pipeline | Attempts, successes, dependent/independent rows, rank, linear-algebra work, recovered scalar checked by multiplication |
| Reproducibility | Fixed inputs/seeds, native source commit, command/spec hashes, retained raw outputs and resource failures |

## Sources checked

KB unavailable: this runtime has no research-kb MCP tools and
`scripts/research-kb status` fails with exit 127 (no executable/catalog).
Fallback GitHub code queries in this repository: `PKM tower`, `Semaev retained
chain`, `quadratic norm resultant`, `relation rank prime`.

- `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`: prior degree
  ladders and their negative/limited results; reuse the native tower engine.
- `src/cryptanalysis/ic_framework/pkm_semaev.rs`: existing chain/full systems,
  extraction, counted solver work, and complete `ic.pipeline` integration.
- `docs/ml-dataset/PRIME-F4-CHAIN-CONTRACT.md`: affine-infinity scope gap;
  include infinity charts rather than interpreting affine failure as no PDP.
- Petit, Kosters, Messeng, PKC 2016, sections 3–4:
  <https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>.
- Semaev 2015, sections 2–4: <https://arxiv.org/html/1504.01175>.
  The construction is reused; no first-fall-degree or novelty assumption.

## Frozen arms

Baseline: existing native `pkm-semaev`, standard summand-major Kummer layout.
For m=3 retain the existing full-S4 arm as an expanded-system reference.
Candidate: retained S3 chains with all projective intermediate charts, under
both summand-major and level-major square-tower layouts. At m=4 compare left
and balanced addition trees. Only Kummer membership is extended; old behavior
and the other membership families remain unchanged unless explicitly selected.

Infinity chart: each intermediate projective abscissa is either `(u:1)` or
`(1:0)`. Pin unused affine auxiliary variables to zero. Homogenize S3 to degree
two in each abscissa. Iterate all charts; refutation is accepted only when every
chart refutes. Grid extraction and sign lifting remain separately counted.

## Bounded execution plan

Development checks: exact finite-field fixtures; all small subgroup targets;
forced cancellation/infinity examples; invalid parameter rejection; unchanged
legacy defaults; complete toy scalar recovery with rank and relation counts.
These are unit tests, not performance measurements.

Experiment A: p=769 (a=2,b=5, order=751) and p=12289 (a=2,b=28,
order=12373), t=1,2,3; m=2,3,4; two public target seeds 20261109,20261110.
Keep all applicable frozen arms. Maximum productive degree 5, per-chart solver
deadline 250ms, max_nnz 1,000,000. Bound failures are retained, not negatives.

Experiment B: existing exact pipeline fixture p=2393,a=2390,b=1614,
order=r=2377,G=(0,80), t=2, m=3,4, one target per workload, target seed
20261109, algorithm seed 5. Include F4, the algebraic grid reference and A/A.
One warmup and five measured rounds, complete phase accounting, exact scalar
replay. No lower-bound sum of priced group work is called a full solver cost.

Backend: EKS Spot per isolab-eks, frozen source image; at most four CPUs, 4GiB,
one job concurrently, 1,800-second original job deadline, no automatic retries.
Finite spending ceiling USD 5 including build/failure/idle; submit only after
checking live shared daily/monthly remainder and reservations. If either the
checkpoint gate or remaining-budget check is unavailable, preserve the protocol,
complete feasible development work, and retain the experiment as blocked.

## Interpretation and verification

Primary observations are exact operation/shape counters with field/group units
kept distinct. Wall times are diagnostic unless the required isolation, A/A,
and replay gates admit them. Compare matched single-target intervals. Never
assign a completion cost to a timed-out case. Treat producer checks as
unverified until the repository's independent PCC mechanism supplies standing.

An ordering benefit is a component result at the recorded scope. A pipeline
result requires recovered scalar, row/rank evidence, every failed attempt, and
all phases. No promotion to a leaderboard occurs without its validation gate.
