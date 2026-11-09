# Prospective native rational recovery correctness census, v1

No census data exists at filing. The actual mathematical construction is pinned
by sourcebd8cbfe23 and the Hermite recovery report. This is a separate study
from the completed common-scalar mode panel; it does not reopen that panel.
Primary sources and KB unavailability are recorded in the linked report.

## Question and prediction

Does the native finite quotient/recovery implementation return the known target
residue using the original source child precision? Prediction: all180 positive
fixtures pass; each of3 deliberately defective controls is rejected as specified.
This is exact finite computation, with no speed or population inference.
A failure preserves partial stdout, stderr, dense polynomial and node charts,
invalidates the census, and stops before another invocation. No retries.

## Fixed census

Parameter triples(a,u,M): (3,3,3),(3,3,4),(3,3,5),(4,3,4),(4,4,4).
The a4 cells are held out relative to the a3 development domain. Every triple
uses all active centered-grid nodes and the actual order/precision formulas.
Weights: delta A0=B0=1 (target1); A0=2,Afull=-1,B0=-3,Bfull=5
(target-11); A1=1,A2=-2,B4=3,Bfull=1 (target0), full=2^M-1.
Two injective placements: one packet with slot i; two packets with packet i%2
and slot floor(i/2). No packing-balance or fast packing claim.
Output bits1,4,9. Canonical lifts and offset lifts adding
2^p_t*((t+2j+2L)%5-2), with nonnegative remainder, at each node/coefficient.
Total5*3*2*3*2=180 positive records per physical invocation.

Controls at(3,3,3), delta weights, one packet, p4, canonical lift:
zero inverse/quotient vector must miss target1; a +1 perturbation of node1's
quotient constant must yield a nonintegral recovery; affine arithmetic D=3,N=2
must distinguish D-N=1 from1-N=-1. These test distinct implementation defects;
they are not statistical null populations or counterexamples to the theorem.
The full second invocation is the A/A control and must be byte-identical.
Reference target is the independently known sparse contraction. The analyzer
recomputes congruence via numerator-target*denominator divisibility rather
than the harness's modular denominator inversion. Neither is signed independent
verification; both are producer artifacts.

## Instrument and variables

check.cpp: exact dense GMP rational polynomial construction, actual affine
clearing, integral source charts, modular finite inverse/convolution and
rational recovery sum. Dense input construction is intentionally direct; no
fast arbitrary-tensor evaluator is asserted. analyze.cpp: complete unique-key
coverage, rational normalization/integrality/congruence, controls and A/A bytes.
Native compilation is allowed before freeze; no census preflight is run.
Source and binary hashes, compiler identity and analysis source are frozen
before data. All inputs deterministic, seed0; no warmups, sampling, blinded
labels, statistical interval or power calculation. Output record is TSV with
13 frozen columns shown by check.cpp;183 data rows plus header per invocation.

Same immutable binary, flags, weights, grid, arithmetic library, precision
formula, one thread and environment in both invocations. Record CPU/kernel,
SMT/NUMA/virtualization and co-tenancy as host observations. They can affect
completion under the cap, not exact equality; no clock is admissible. Order is
fixed because exact arithmetic and no timing observation are used. Census
parameters/lift modes are crossed; host variation measured; analyst fixed.

## Admission, limits and stopping

Standing user-approved local placement:1logicalCPU,2GiB,600s hard ceiling per
invocation, at most2invocations,32MiB output each, no paid/cloud work, no retries.
Require scripts/checkpoint.sh gate success before launch; upload transport
must exist, or a specific owner override must cover that gate. A local-backend
override alone does not assert a successful S3 upload. No data until protocol,
source, binary hash and native analysis are frozen and committed.

Freeze JSON using the canonical schema and maintained native metadata utility.
Run0 then run1 only after complete run0 success. On timeout, signal, output cap,
OOM, arithmetic/control failure or A/A mismatch: retain all artifacts and mark
invalid/incomplete; do not select successful cases. A source change after data
needs a new dated amendment. No mid-run restore: counted partial artifacts
remain diagnostic, clocks invalid, resume/retry prohibited. Complete analysis
requires183 unique records per invocation and all tests, otherwise invalid.

Planned raw paths: runs/0/panel.tsv, runs/0/stderr.log, runs/1/panel.tsv,
runs/1/stderr.log; metadata and export beside runs, not invented physical rows.
Verdicts: exact_pass, invalid, incomplete, not_run. No measured benchmark row,
leaderboard promotion, asymptotic exponent or novelty claim.
