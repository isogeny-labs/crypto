# Public-input exact reference: pre-code correctness protocol

Question: can a public-only, bounded reference discover original six-member
roots and certify exhaustion for its full declared small base while keeping
distinct raw indices and point aliases? This is fixed native development
validation, not a performance/solver campaign or scaling measurement.

Inputs are Spec, even arity2/4/6 and public target only. Construct the entire
declared raw family internally, record identity exclusions and canonical
signs, and reject before enumeration when raw columns exceed64 or signed
half-tuples exceed131072. No planted labels, point list, scalar or witness
enter the solver. Successful roots must satisfy TargetCircuit and its
independent signed-row replay; exhaustion is scoped to this exact finite
base/arity, not the whole route or Curve25519.

## Mechanism and cost contract

Meet in the middle over signed halves of r distinct raw columns, r<=3.
For each elliptic sum, store an r-representative family against r forbidden
columns in the rank2r uniform matroid. Label column i by x_i=i+1 in the
prime field, requiring p>raw_count. Use its Vandermonde vector
(1,x,...,x^(2r-1)). Encode a half by all r-by-r minors; keep an ordinary
column basis per elliptic sum, retaining original witnesses. Its dimension
is C(2r,r), at most20. A disjoint completion gives a nonzero determinant
functional, so the selected original witnesses preserve its existence.
Query regeneration tests at most20 candidates per half and returns one
original root or complete finite exhaustion. Keep raw aliases separate.

This applies Fomin/Lokshtanov/Panolan/Saurabh arXiv1304.4626v4, section2.5
and Theorem3, rather than asserting a new representative-family algorithm.
The paper was downloaded/read directly; KB backend/MCP unavailable.
Citation: https://arxiv.org/abs/1304.4626

The fixed-arity reference explicitly enumerates Theta(B^r) halves (signs
are constant), with constant-dimensional field work plus dictionary factors.
For six members this is cubic setup/query work and cubic worst-case space,
not the required gamma<2 or a linear-memory candidate. Record operation
events as development-accounting checks only, not an admitted benchmark.
Charge setup in full to every one-target cost statement; diagnostic
multi-target correctness coverage establishes no batch speedup.

## Fixed controls, no sampling

1. Reuse p101/A95/h(17,12)/a17/q3/k8 exactly. Expect104 curve points,
   two recorded identity columns and22 retained raw columns. Enumerate all
   signed half-triples and all13 public cofactor-image targets. Every
   returned root must replay; compare every original half's disjointness
   query at every sum key with the compressed family's answer. Preserve
   complete compressed comparison trace, table/basis data and roots.
2. Actual p/A, h_new=h_old^213156431 of order60824497, q1/k6. This is a
   distinct explicitly bounded six-column fixture, not the huge actual
   many-coset base. Test even arities2/4/6 against independently generated
   known-answer sums; pass only the target to the solver. Also query the
   RFC u9 point and compare arity6 with all64 sign assignments over the six
   raw columns. Preserve either observed outcome; no resampling.
3. Reject oversized actual q213156431/k20633 before base enumeration,
   malformed target, unsupported arity and unsafe Vandermonde label field.
4. A synthetic same-sum family demonstrates why selecting its first witness
   alone can miss disjoint completions; check selected basis preserves them.
   A duplicate-column mutation must reject, not silently deduplicate images.
5. Generate the standard six-column Vandermonde determinant identity record
   with the maintained native identity-certificate API and a corrupted-identity
   refusal. This is evaluated over its declared Mersenne field, not a formal
   compiler or representative-family proof. The primary theorem remains
   the mathematical source; finite native checks remain software evidence.

Acceptance: agreement/replay and representative-family finite coverage,
correct resource/exhaustion distinctions, and source/public-input separation.
Failure rejects the implementation; timeout/OOM is censored, not disproof.
Original consumer ROUTE_CRITERIA_V3.md and private-tag/one-retained-row,
field/map/group/family and full cold-cost requirements remain unchanged.
No paid job, benchmark, sampling, new solver campaign or broad library rerun.
