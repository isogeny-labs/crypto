# Independent fixture admission — draft before implementation

This is validation of an existing Hasse/order-witness method. It proposes
no new cryptanalytic mechanism. No new native/admission/solver run has taken
place. Sources and exact prior implementation are recorded in README.
[pcc:unverified]

## Question, predictions and comparison

Can a separately implemented native check confirm the six frozen curve domains
and twelve public targets without trusting the Rust constructor's output or
the advertised group order? The constructor already checks these properties.
Predict that an independent replay will confirm its six full prime orders;
record every contrary result with its exact cause. Exceptional-class flags
and small embedding-degree screening are a descriptive census, not a
prespecified claim that every fixture passes. No runtime comparison is planned.
[pcc:unverified]

Read all six literal domain tuples from frozen scale.rs. Obtain the twelve
public target headers from the first warmup certificate a-case-seed-0 for
every case0..5 and seed101/202, without supplying recovered scalars. Do not
select targets by observed performance. Preserve their original input bytes,
identities and hashes. This is a post-run admission replay, not twelve new
solver observations or a repair of the original protocol/export pairing.
[pcc:unverified]

## Independent native method

Use C and OpenSSL curve/point addition primitives, with deterministic trial
division for p and r in the bounded domain. Do not reuse the Rust primality
routine or accept a probable-prime assertion as the result. Check canonical
coordinates, nonsingularity and nonidentity/on-curve P and Q. Compute rP and
rQ by explicit addition/doubling without configuring a claimed generator
order; avoid order-reduction assumptions in the verifier itself.
[pcc:unverified]

For integer radius=floor(sqrt(4p)), established Hasse/Lagrange facts give
r divides N=#E(Fp), N>0, and N<=p+1+radius. If the upper bound is below2r,
the existing positive-multiple theorem gives N=r. Record the exact integer
root certificate and bounds. If uniqueness is insufficient, return unknown;
do not infer an invalid curve. Record anomaly/supersingularity and low
embedding-degree flags only at the scope justified by a certified order.
Do not call a conditional Nat interval lemma a full Lean proof of the
elliptic-curve or C implementation bridge. [pcc:unverified]

## Controls before a real census

- Keep real-target rows separate from all controls.
- For each domain, test a definitely composite field9p, composite order9r,
  wrong prime order3, off-curve G=(0,0), off-curve Q=(0,0), and noncanonical
  generator coordinate x+p. All six source b values are nonzero; validate
  that control precondition before using the zero point. Require exact causes.
- Include a real resource-bound case, checked before arithmetic, which must
  return unknown/resource rather than a mathematical rejection.
- Use the known small anomalous curve p7,a0,b5,r7,P=(3,2), and a singular
  curve control. Distinguish a valid order certificate from eligibility for
  an ordinary non-anomalous benchmark.
- Cross-check against an independently enumerated point count on the fixed
  144 parameter cells: p in5,7,11,13,17,19,23,29,31,37,41,43;
  a in0,1,p-1; b in0,1,2,3. Count infinity explicitly. Use a fixed small-prime
  table for the enumeration oracle. Composite/too-small orders and ambiguous
  Hasse intervals are separately classified, not forced into acceptance.
- Test known composite/pseudoprime inputs with explicit small divisors;
  preserve every compiler/assertion/error outcome.

These controls must be fully specified in the frozen executable/source before
its census run. Any changed control or criterion gets an additive pre-run
note and a distinct attempt. No accepting an arbitrary Err as the intended
cause, no overwriting raw output, and no turning a timeout into a theorem.
[pcc:unverified]

## Acceptance, resources and evidence

Require all programmed control expectations, all twelve public-point checks,
exact per-domain arithmetic certificates and source/binary pre/post hashes.
Any unexpected result fails the attempt; retain completed and unattempted
records. Resource-limited cases remain unknown. A failed fixture/admission
check is not by itself a conclusion about the cryptanalytic method.
[pcc:unverified]

Proposed limits: p,r<2^48; one build/native worker;600s phase ceiling,60s
native ceiling; unchanged sampled whole-cgroup stop15032385536bytes. Native
Rust/C only. Node may extract/serialize public metadata and hash files; no
Python scientific computation. Freeze source, flags, input manifest, qualified
binary and exact output path before execution. Preserve every failure. Local
raw output/checkpoint, source-linked report/diagram/PDF and separate signed
standing status accompany results; no S3. [pcc:unverified]

## Subsequent larger cohort

The current worker, collector and point auditor must all be changed and
qualified prospectively before admitting different target seeds/curves.
The future-only draft remains unadopted. Test the actual instance/seed/round
mapping and corruptions using explicitly synthetic metadata before new data.
New curves also require actual field/order-bit reporting rather than the
worker's current positional20+4*case label. Distinct fixed targets must not
be described as independent uniform random samples without a matching sampler.
Use the user's standing local authorization to finish the prospective
controls and execute the qualified new cohort after its protocol is frozen. [pcc:unverified]
