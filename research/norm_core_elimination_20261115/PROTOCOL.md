# Core graph elimination: prospective native correctness qualification

Owner objective: complete classical subexponential prime-field ECDLP algorithm,
using a different route from tensors. This step simplifies the shared norm
encoding. It does not claim a solving-degree bound or a performance result.
PCC-20261115-be28ec remains unverified. [pcc:unverified]

Before implementation/calls, freeze this exact graph substitution:

1. In the witness-pruned system, solve A(x_R)=y_R C(x_R) for a0 (unit
   coefficient). On a curve target, A^2-f C^2 then vanishes at x_R.
2. Monic synthetic division by X-x_R writes U=(A^2-f C^2)/(X-x_R).
   Substitute those polynomial coefficient expressions for all m U variables.
3. For membership layers with 2^j<m, monic Euclidean division forces
   Q_j=0 and V_j=X^(2^j). Substitute these public constants.
4. At the first wrapping layer, the previous polynomial is still public.
   Its quotient has degree at most 2^j-m; set only the higher declared
   quotient coefficients to zero. Keep the remaining retained layers and
   target inverse. Do not expand X^(2^t) or enumerate norm coefficients.

The resulting equations have degree at most3. At t4 the planned arities3,5,7
have15,18,22 variables. These are encoding counts, not solving-cost predictions.
Every extracted assignment must lift to an assignment of the complete original
witness-pruned system and satisfy every original equation before point decoding
and group-sum checks. Projection equivalence uses unit-coefficient graph
elimination and uniqueness of monic division, not experimental counts.
[pcc:unverified]

Sources checked: existing norm continuation/reducedness and initial-elimination
notes; prior witness-pruning source/proof map; pinned msolve help. No prior
core-elimination implementation was found in these narrow paths. This is
standard preprocessing, not a novelty claim. KB remains unavailable.

Native software controls, each once in a fresh write-once core-001 tree:

- Strict build and structural checks: t4 variable counts above, maximum input
  degree3, zero-c and off-curve target admission rejection, and exact symbolic
  cancellation of eliminated norm/target equations. Existing baseline native
  selftests and retained28-model check must still pass after a generic oracle
  adapter change; no reference/gate/backend change.
- Same90 F17 m3 public domain/target cases as the parent protocol: exact full
  model sets versus bounded reference, then independently compiled OpenSSL
  enumeration. Stop immediately on a mismatch or a failed m3 solve; preserve it.
- Same three arity controls, E/F17 and R=(0,6): (m,t,c)=(5,3,16),(5,4,1),
  (7,4,1). Compare every completed full set; preserve caps/errors as unknown.
  Stop on disagreement. No adaptive fixture/cap expansion.
- Same18 point-only cold controls with seed1234. Reuse the candidate's existing
  mixed-query, signed relation, left-kernel and scalar recovery path through a
  generic oracle interface. Independent OpenSSL replay must accept all queries,
  relations and scalars. Known target scalars remain only in the outer harness.

Use the pinned msolve0.7.5/FLINT3.1.3 backend, one thread,20s CPU/wall and2GiB
address cap per call, quotient cap128, original reduction limits. Record source
commit and source/binary hashes before calls. Preserve every backend input,
basis, stdout/stderr, exit and unknown result. Fixed independent reference
censuses are correctness oracles outside the candidate, never candidate inputs.
No scientific clock, population inference, speedup, fitted exponent or cloud
job is authorized by this development protocol. No old evidence is overwritten.

Acceptance is complete-model equality and faithful cold replay at this fixed
fixture. Passing leaves the original growing-family algorithm criterion open.
Formalization attempt: use the pinned shared polynomial laws and existing
Mathlib monic division; do not copy them. Record exact missing endpoint laws
and actual build/axiom receipts if any new Lean declaration is reported.

## Amendment D1 — diagnose the preserved arity5 backend crash

After the fixed core arity controls, the dense m5/t4 call exited139 with an
empty basis and no stdout/stderr. Its cause is unknown: do not label it a
memory-cap observation or a mathematical failure. No recovered relation can
be extracted from that empty file.

Permit exactly two diagnostic native backend calls on that preserved public
input, with unchanged20s CPU/wall and2GiB address caps, one thread:
(a) -v2,-l2 (current exact-sparse method with diagnostic output),
(b) -v2,-l1 (the backend's distinct exact sparse/dense method).
Capture the exact input, backend digest, flags, stdout/stderr, basis and exit
in fresh diagnostic-001 paths. No adaptive cap change or extra fixture.
This bounded software crash diagnosis is not a timing/performance experiment.
Preserve the original139 outcome, whichever diagnostic outcome follows. A
successful basis still requires bounded extraction and original/group checks
before it can supply any model claim; those are separate from backend exit.

## Amendment D2 — discriminate allocation failure and bounded pair batching

Both D1 modes exit139. Their logs progress through F4 degree6; the default
method next selects33764 pairs at degree7 and then crashes before a matrix
line is printed. This suggests a resource/large-matrix failure, but does not
establish its cause. Freeze exactly two further diagnostic calls on that same
preserved public input, with the identical20s/2GiB/one-thread limits:

1. Default exact-sparse configuration under strace restricted to memory syscalls
   and SIGSEGV, to record whether an allocation fails before the crash.
2. Exact-sparse configuration with `-m 512`, the backend's explicit maximum
   pair batch, to test a smaller-matrix execution route without raising memory
   or time. This changes the algorithm's batching, not the relation equations.

Retain all flags, backend/input digests, raw trace/stdout/stderr/basis/exit in
new diagnostic-002 paths. No extra fixture or adaptive cap. These are software
failure diagnostics, not scientific timing or comparative performance results.
A backend completion alone is not a point-model claim. Original001 and D1
unknown/crash statuses remain immutable. No other retry is permitted by D2.
