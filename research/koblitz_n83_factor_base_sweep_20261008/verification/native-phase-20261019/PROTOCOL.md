# Native N83 retained-base phase diagnostic, 19 October 2026

User request: upload the exact previously prepared raw evidence ZIP to
`setec-astronomy-artifacts` and continue the N83 factor-base sweep. The ZIP
publication is independently completed in `../raw-approved-publication-20261019`.
This new round addresses the missing native K4096 phase boundary; the complete
finite grid and full cold-runtime objective remain open.

## Freeze and isolation

Freeze this protocol and native sources in Git before execution. Native task
cards pin the clean HEAD, executable SHA256s, specs, exact inputs and wrapper.
Native specs independently freeze worker/runner BLAKE3, source commit and all
panel/fixture file BLAKE3s before launch; observed hashes must match those pins.
The Rust supervisor/auditor and strict native frozen-command card executor
replace Python for this diagnostic. Historical Python helpers are not run.
The canonical card format is retained; unsupported runbooks/retries/environment
settings are refused. The director compiles cards, the parent relays the native
prompt verbatim, and a mechanical executor returns the receipt-derived output.

Primary curve `icv1-f2m83-tm6151469093347-debefd74`: a=0, b=1,
GF(2^83), modulus z^83+z^45+z^2+z+1, subgroup order
2417851639230796216685689, cofactor4. Public fixture0 only; known-answer scalar
is not supplied to relation generation. Policy public_x_sequential,
seed2026100801, retained K2048 then K4096. Archive SHA256s respectively
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a` and
`ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad`.
All current storage publication uses setec-astronomy-artifacts; old-bucket
sidecars remain historical provenance. Exact input paths/digests are in cards.

Docker image
`sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63`:
network none, 1 CPU quota, 4GiB memory, total memory+swap4GiB (zero swap),
64 PIDs, all capabilities dropped, no-new-privileges, read-only root and
inputs/source/worker, task-user output mount. Live/final inspections exclude
process environments. Both native controller and an in-container GNU timeout
apply the wall bound, so controller loss cannot leave indefinite computation.
Cleanup is checked and raw failures retained. Timing isolation is not earned.

## Controls, then bounded retained arms

1. Run the exact native library test
`cryptanalysis::koblitz_index_calculus::tests::chained_s3_frobenius_domain_lifts_and_rejects_incomplete_orbits`
with phase tracing and one test thread. This checks original-group lifts on
m5/m6 and ordered/unordered branches, plus incomplete-orbit/cap rejection.
A native trace checker requires valid phase prefixes and at least four SAT
returns; coverage is supplied by the pinned exact test, not inferred from
four anonymous records. Command allowance120s.
2. Run the native N83 fixed-witness encoding control, existing pinned
`known-witness-ordered-sat-20261014/domains.json` plus fixture SHA256
`fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538`.
5000 conflicts, 180-second wall, same Docker resources. Require
SAT_verified_witness_x, model verification, and independent native polynomial-
word group addition of the five public fixture points to the target.
This is a known-witness positive encoding control, not natural yield.
3. Only after the control card completes, execute independently frozen
K2048 and K4096 cards: m5 orbit-phase chained S3, one trial, 32 models,
150000 variables, 2000000 **domain** clauses, 1000 conflicts, **600 seconds
per arm**, with tracing enabled. These are new diagnostics, not replacements
for the historical 180-second controls or 100000-conflict searches.

Separate cards stop on their own failure; no automatic retries. Controls gate
both arms. Once controls pass, K4096 remains an independently requested arm
even if K2048 produces an unsupported outcome; parent records that result
and may relay the separately frozen K4096 card without modifying either card.
Command allowances: 240s for N83 control; 660s per retained arm, covering Docker
operations, custody and audit. Overshoot is recorded, not charged as solver
work. Declared container compute envelopes180+600+600=1380s, plus native
small-field control <=120s. Build, storage, inspection, verification, tests and
rendering are separate overhead; no complete cold total is inferred.

## Evidence and admission

Flush actual a,b,target before blocking decomposition. Emit closure_begin,
closure_complete, index_complete, encoding_begin, circuit_complete,
domain_complete, solver_enter and solver_return. Preserve marker ordering,
raw streams, immutable source/worker/input identities, resource inspections,
outer wall, final state and cleanup. A wall limit with no markers establishes
zero phase observations, not a phase location. Solver entry does not identify
internal preprocessing versus search. Final counters absent at a wall limit
remain null, never zero. Native original-group arithmetic independently
replays observed public queries and verifies terminal query equality.

This diagnostic auditor admits only a fully checked zero-relation solver-cap
receipt or a censored phase observation. It rejects scalar/relation claims,
OOM and unsupported outcomes; raw observations remain preserved for a future
full native relation/matrix verifier. That verifier is not claimed complete.
All scalar/rank/natural-yield/performance promotions remain gated. No A/A,
matched external performance baseline, complete cold pipeline, runtime winner,
UNSAT conclusion from caps, or sub-rho claim is established by this round.

Native forged-query/counter/isolation/order/binding/deadline/card-mutation
controls must pass. Each run gets an evidence-linked visual report/PDF outside
its audited directory; later results are addenda, not rewrites of frozen data.
