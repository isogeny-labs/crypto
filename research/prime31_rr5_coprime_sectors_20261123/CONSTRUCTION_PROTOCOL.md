# Coprime RR5 sectors: construction qualification

Frozen before the first new native build or correctness execution.
Date: November 23, 2026 Pacific. Tracking: PR #5119.
Base: `ceba74bb64854027c4dd16f00428bf676383bfdb`.

## Question and hypothesis

Can the existing admitted M31 selector H=D*Y be presented as five exact
quadratic quartic-divisor sectors, with fewer variables in each system,
while preserving all supported RR5 chart solutions across their union?
This component qualifies source construction only. It does not call the
RR5 extractor, enumerate a factor-base product, measure a solver, recover
an unknown target, or change the frozen B677 extraction protocol.

The coordinate field is p=2147483647. The curve is
y^2=x^3+847514846*x+2087444724, with full prime order n=2147446831 and
G=(625037715,2100627251). Reuse the exact admitted Hybrid677 fixture,
including the frozen Y7 CSV and canonical point orientations.

## Candidate

For each k=0,1,2,3,4, introduce monic U_D of degree k and U_Y of degree
4-k, impose the four nonleading coefficients of U=U_D*U_Y, and separately
impose U_D|D and U_Y|Y. D=D_1024(X,1)-tau and
Y=product_i((X^3+A*X+B)^7-c_i^2). The degree-zero factor is exactly1,
with vacuous divisibility and no degree-zero modular gate.

Use complete monic modular multiplication equations. All residues and
quotients are explicit, each coefficient is constrained, and final gates
pin their required affine remainder. No factor root is selected by the
production builder. All field, term and construction deadline operations
use the existing Rust Builder.

Expected variable/equation inventories, subject to actual native checks:
k0=59/60, k1=53/54, k2=56/57, k3=58/59, k4=59/60. Maximum equation degree
is2. Compare with the unchanged coupled circuit's122 variables/123 equations.
These are per-system construction counts; all five systems remain required
for complete chart extraction. Total solving work is unmeasured.

## Controls and observables

1. Every k: compare terminal graph residuals against independent scalar
   polynomial long division of dense D and Y at fixed factor coefficients,
   including zero, one and p-1 values.
2. Every k: use a deterministic known four-point signed fixture with k
   Dickson points and4-k Y7 points. Construct its public target in test code,
   interpolate RR5 b,c,d, and check the fourth interpolation equation.
   Reject unsupported target charts explicitly. These are construction
   fixtures, not public extraction attempts or relation-yield samples.
3. Form monic factors independently from known fixture abscissae, fill gate
   auxiliaries by scalar monic long division, and check every equation.
4. Perturb each non-b,c,d variable independently by1 on a valid full
   assignment. At least one complete equation must reject every mutation.
5. Cover invalid sector and variable-count inputs, construction deadline
   and symbolic-term limits where the public/internal interface permits.
6. Run the unchanged native library and touched example regression tests;
   record root-library qualification separately. Retain failures before
   corrections and bind every test attempt to its exact source manifest.

Construction passes only if all controls succeed and actual variable,
equation and degree counts match the emitted native inventory. A failure
requires a documented correction and separate attempt, never deletion or
overwrite of the earlier result. No conclusion about solving degree,
Macaulay fill, extraction time, useful relation rank or cold cost follows.

## Development resources and evidence

Rust1.100.0, at most two compiler workers, one test/Rayon worker. Each
development invocation has a600-second process limit; unit binaries retain
their individual construction limits. No cloud build, scientific job,
paid compute, checkpoint retry or authentication change is authorized here.
Record command, exact source/lock hashes, compiler identity, stdout/stderr,
exit and binary hashes in write-once attempt directories under `evidence/`.
The report sits beside the evidence. No captured times are benchmark claims.

The established B677 scientific gate remains a separate future experiment:
the original frozen structured/cofactor sequence, one worker,180-second
outer cap,2GiB and zero retries remain unchanged. Any successor comparison
requires a separately frozen five-sector schedule and a total budget shared
across all sectors; running each sector with a fresh full budget would not
be a matched comparison.

## Sources and interpretation

Primary repository sources: the original RR5 `MATHEMATICS.md`,
`scaling-b677/MATHEMATICS.md` and `PROTOCOL.md`, native selector/fixture,
and PR #5099's explicit source-pin review. External review: Faugere--Mou,
*Sparse FGLM algorithms*, arXiv:1304.1238, sections1--2. Its quotient-basis
input requirement leaves this front-end construction/elimination question
open. The Semaev2004/031 landing page was accessible but its PDF returned403;
no unread paper content is used as evidence.

KB tools and Conductor are unavailable. The bounded fallback repository and
open-PR search identifies no matching active sector lane; no novelty claim.
Lean/lake are absent from PATH and the standard installed-toolchain path;
the central factor-divisibility proof remains written, not kernel-checked.
The repository's identity.certificate/v1 evaluator is fixed to2^61-1 and
is not run for this new M31-only work. Native M31 constructor evidence must
not be relabeled as that certificate or as a formal completeness proof.
