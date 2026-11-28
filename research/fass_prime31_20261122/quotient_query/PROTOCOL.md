# Quotient query development protocol

Frozen before native execution, 2026-11-23 UTC (2026-11-22 Pacific).
Tracking: https://github.com/isogeny-labs/crypto/pull/4966 .
Parent: merged #4952. Initial source base: ea58aed121d2efc268537c348b0bd3b49c525a2d.

## Question and scope

Construct a specialized, certificate-producing triangular solver for
`<F(x), G(t), H(x,t)>` over a prime field. F and G are monic, squarefree,
fully split; H is reduced in both variables. Dynamic Euclidean elimination
must split a t modulus when a pivot is a nonunit, instead of treating its
coefficient ring as a field. Each terminal component carries polynomial
division and Bezout identities. The exact affine support count is the sum
of the products of the two triangular degrees.

This is implementation correctness work, including fixed finite controls.
It is not a timing study, admitted performance experiment, scalar-recovery
cohort, or comparison with a general Groebner implementation. Actual complete
relation solving, useful independent rows and cold solver cost remain separate
requirements. Empty affine grids do not cover missing identity-prefix charts.

## Prior sources and ownership

- `../target_quotient/{THEORY,REPORT,PROOF_MAP}.md`: full coefficient rank can
  coexist with empty zero support; actual radical target quotient matters.
- `../coupled/REPORT.md` and `src/cryptanalysis/semaev_prime_poly.rs`: existing
  first norm, S3 and field/polynomial arithmetic.
- `research/prime31_summation_compression_20261122/iteration9/PAIR_SPECTRUM_RESULTANT.md`:
  existing resultant/gcd/Bezout miss-certificate semantics.
- Dahan, Moreno Maza, Schost, Xie, *On the complexity of the D5 principle*:
  https://cs.uwaterloo.ca/~eschost/publications/DMSWX-05b-TC-2006.pdf .
- Li, Moreno Maza, Pan, *GCD computations modulo regular chains*:
  https://www.csd.uwo.ca/~mmorenom/Publications/Li.Moreno-Maza.Pan-JSC-format-2009.pdf .
- Petit, Kosters, Messeng, *Algebraic approaches for the elliptic curve discrete
  logarithm problem over prime fields*:
  https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf .

This applies classical dynamic evaluation to the existing quotient input;
neither the D5 principle nor the homogeneous norm recurrence is new here.
Sources checked (KB): no research-KB MCP is exposed; the repository wrapper
reports `research-kb: no executable found`. Scoped primary sources above are
the fallback, not an exhaustive absence or novelty claim. Conductor is not
installed; the parent reserves this new directory and shared tracking appends.
The concurrent signed-extraction and pair-spectrum lanes remain separate.

## Exact input and output contract

Use Rust and the published field/polynomial module. Canonical coefficients
are in `[0,p)` and low-degree-first; zero is `[0]`. Reject invalid/nonmonic
selectors, repeated roots and nonsplit selectors. Check splitting by
divisibility into `T^p-T`; no field-root scan is a producer primitive.

For each output block q, record monic D in x and A, B, U, V such that,
modulo q(t), `A F+B H=D`, `F=D U`, and `H=D V`. All block moduli are
nonconstant, pairwise coprime and their product is G. D=1 marks an empty
block; positive x degree routes extraction. Handle F=1 or G=1 explicitly.
Record exact support count, degrees, splitting diagnostics and proof size.
Never interpret a cap or invalid input as an empty answer.

The candidate construction of H uses the homogeneous quadratic norm in the
tensor quotient. For `S3(x,z,t)=a z^2+b z+c`, put
`M=[[-b,-a c],[1,0]]`; evaluate `a^d+c^d-trace(M^d)` by binary powering.
An equivalent existing Lucas doubling recurrence is acceptable. This is
division-free and does not assume a nonzero quadratic leading coefficient.
It retains B*e dense coefficient slots. The independent construction is the
previous direct selector product reduced modulo F and G.

## Fixed correctness panel

No case is selected using a new output. Curve cases are run for both the full
selector polynomial `x^d-1` and the polynomial of its liftable abscissae.
The latter includes rational 2-torsion. Target coordinates never come with
discrete-log scalars.

1. F97, A=2, B=3, d in {1,2,3,4,6,8,12,16}: the first three non-2-torsion
   rational points in increasing x order (smaller ordinate), plus (30,0).
   Eight degrees times four targets times two left selectors: 64 cases.
2. The existing Prime31 fixture, p=2147483647, A=847514846, B=2087444724,
   d in {7,21,63}, targets x=0,1,2 with smaller rational ordinate, both
   left selectors: 18 cases.
3. Existing rank-one control F97, A=96, B=0, d=2, target x=0,1,2,
   both left selectors: six cases.
4. Seven synthetic cases: F=(x-1)(x-2)(x-3), G=t(t-1)(t-2), with
   H equal to 1, 0, t*x-1, t*(t-1)*x+(t-1)*(t-2), and x-t; plus F=1
   and G=1 cases. These force zero divisors, degree drops and empty domains.

Total: 95 cases, with category duplicates retained. The direct tensor norm
is checked on every F97 curve case, every Prime31 d7/d21 case and the single
Prime31 d63/x0 liftable-left case: 83 constructions. The other five d63
cases use the prior dense H construction and record the tensor construction
as not attempted. This selection predates all calls and does not change
after observing runtime or results.

## Independent checks and retained objects

A separately written Rust checker uses its own u128 finite-field arithmetic
and polynomial operations. It checks field and input admission, every
partition, cofactor and Bezout identity, the empty verdict and support count.
A direct finite-grid oracle compares the complete roots and per-block gcd
support on these fixed cases. Candidate construction never uses this oracle
to decide emptiness. Check nonsingular curve admission and the exact right
polynomial against existing S3 factors. Rational coordinates are not signed
point relations; no useful row count is emitted.

Reject mutated partitions (omitted, duplicated, overlapping), nonmonic or
altered gcds, altered cofactor/Bezout identities, false empty claims and
incorrect support counts. Also reject noncanonical, repeated-root and
nonsplit inputs. Record actual mutation classes exercised.

Every case emits a compact record with full-certificate SHA256 and byte size,
or an explicit reason if that binding cannot be produced. Complete proof
objects are persisted for the following predetermined cases only: all seven
synthetics, F97 ordinary d2 (eight), the six rank-one controls and Prime31 d7
(six), 27 objects. Other cases are checked in memory and their certificates
are reconstructible from the frozen source; their records explicitly mark
`full_certificate_persisted=false`. This is not a claim that all proof bytes
are published. Individual case artifacts are natural records, not chunks to
evade storage thresholds. Any oversized artifact follows the repository's
existing store policy or remains an explicitly reported publication blocker.

## Source custody, resources and attempts

Before building, copy every transitive compiled source, manifest and lockfile
into a fresh immutable snapshot and retain hashes. Build from that snapshot,
copy the executable to a distinct immutable path, and write a new pre-call
manifest before any execution. Never replace a build, pre-call or post-call
manifest. Record source, binary, protocol and command identities, host/toolchain,
exit status, stdout/stderr and stable post-call hashes. Preserve failures.

- Standalone build: one process/job, 120-second deadline, 2 GiB virtual memory.
- Fixed correctness qualifier: one process/thread, 180-second deadline,
  2 GiB virtual memory. Capture output incrementally; timeout remains incomplete.
- At most one corrected-source retry following a diagnosed implementation
  defect, with a new snapshot and receipt. No resource increase or altered
  panel after outcomes. A second failure ends this qualification attempt.
- Separate-role replay uses the same frozen executable and resource bounds;
  checker mutations are deterministic. This is same-host, unsigned checking.
- A full repository release-library test is a distinct gate. Materialize
  tracked path dependencies first; permit one 600-second, one-job, 6 GiB
  development call and retain any setup or compilation failure. It must not
  overlap the standalone native call. Its failure does not promote or alter
  the standalone result.

No performance experiment or paid job is authorized by this protocol. Future
timing/competitive-cost work requires the existing EKS/checkpoint/admission
route, a frozen matched comparison and complete phase accounting. This local
development record cannot substitute for those requirements.

## Acceptance and deliverables

Accept the component's finite qualification only if every planned case and
required mutation finishes and agrees with the separate checker; retain each
partial result if it does not. Written arguments remain separate from native
checks and from Lean/kernel verification. Record all scientific claims with
unverified PCC standing until the existing signed verifier changes it.

Publish Rust source, immutable evidence, the independent review, a theorem
and proof map, a source-linked report, explanatory diagram and inspected PDF
in the tracking PR. Document why global performance dashboards stay unchanged.
Next integration obligation: positive-root isolation, signed lift and group
replay, missing identity charts, independent relation rank and cold cost.
