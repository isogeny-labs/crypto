# Three-instance 14-bit PKM degree-32 panel — 2026-11-24

## Frozen question

Does the previously replicated degree-32 PKM relation-extraction component lead persist on three independently generated 14-bit prime-field instances under equal factor-base size, mirrored execution order, complete-row verification and an explicit timing-stability gate?

## Fixed construction and comparator

- Binary SHA-256: `06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59`.
- Candidate: `prime-isogeny:t=5,seed=5,aux=ordinary`, five degree-2 Vélu steps composed to degree 32.
- Comparator: `prime-abscissa:below=32` with `pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain`.
- Candidate oracle: `pkm-semaev:m=2,engine=f4,degree=8,mode=chain`.
- Every preflight and timed call uses `--bits 14 --family generic --repeats 16 --targets walk`.

## Size-only instance selection

Scan CLI seeds 20261200 through 20261599 in ascending order. For each seed, run the candidate and comparator with `subtract`, one target trial and zero rho runs. Retain every preflight. Select the first three distinct instances whose candidate and comparator each report one constant, exactly equal lifting-abscissa count across all 16 rows. Exit code 1 is accepted only for this deliberately truncated size preflight when valid JSON, instance identity and all 16 sizes are present.

Write the ordered three-instance selection record to the append-only store before starting any timed arm. No relation, solver, wall or CPU result may select an instance. If fewer than three matches exist in the frozen range, stop without timing and report the censored selection window.

`--repeats` participates in generic-instance generation in this CLI. It therefore remains exactly 16 in preflight and timing. CLI seed selects the curve and planted-scalar stream; auxiliary seed 5 remains fixed and is a separate factor-base parameter. The planted stream is deterministic and is not described as independently uniform.

## Timed panel

For each selected instance, in selection order, run whole 16-repeat invocations in the mirrored order interval, candidate, candidate, interval. Each invocation uses 1,000 maximum target trials, a five-second solver-call budget, one counted rho context run and a 600-second outer wall cap. Preserve command, complete stdout JSON, stderr tail, exit, external wall and child CPU. Do not retry or replace a failed arm in this store.

Every qualifying instance requires all four invocations to exit successfully, retain the selected instance and base size, contain 16 rows, verify every recovered scalar, recover the same scalar in all four corresponding rows and use 16 distinct scalars within the instance. Failed and censored rows remain in the store and fail that instance's gate.

## Frozen gates and reporting

For each instance, pair interval-1 with candidate-1 and interval-2 with candidate-2. The component transfer gate requires:

1. both mirrored charged-GAE ratios below 0.9;
2. both mirrored external-wall ratios below 1;
3. both mirrored child-CPU ratios below 1;
4. combined candidate/interval charged GAE below 0.9; and
5. timing stability: within each arm, the larger of the two whole-invocation charged-GAE totals divided by the smaller is at most 1.5, and the corresponding external-wall ratio is at most 1.5.

The overall 14-bit gate passes only if all three instances are complete and pass their individual gates, and aggregate candidate/interval charged GAE, wall and child CPU are each below 1.

Report, without pooling away failures: factor-base size, scalar verification, relations, target trials, relation probability per trial, useful relations per charged GAE, F4 multiply-adds, solver time, measured nanoseconds per operation, system dimensions, wall, child CPU, charged GAE and median `s_over_rho`. This is a relation-extraction component experiment. It does not establish a complete cold same-target advantage, source-exact replay, asymptotic behavior or transfer to s59, P-192 or NIST curves.
