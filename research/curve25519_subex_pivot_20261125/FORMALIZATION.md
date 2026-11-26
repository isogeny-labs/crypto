# Formalization map for the conditional prime-field ECDL result

This file freezes the statement assembled by the native audits.  It is a map
of proof obligations, not a kernel-checked theorem and not a practical
Curve25519 attack.  Every theorem below remains `[pcc:unverified]` until its
listed dependencies are independently discharged.

## Top-level statement

Let `E/F_p` be an ordinary elliptic curve, let `G` generate a prime-order
subgroup of order `r`, let the cofactor be fixed, and assume
`log2(r) = log2(p) + O(1)`.  For

```text
b = ceil(log2(p)),
m = ceil((log2(b))^(1/4)),
L = max(ceil(r^(1/m)), 10m),
```

the typed hashed-factor-base reduction plus the Appendix D.4 contraction
routine has expected total bit work

```text
2^O(b/(log b)^(1/4)) = r^o(1),
```

including base construction, relation attempts, prefix self-reduction, dense
linear algebra, and target recovery.  This statement is conditional on the
contraction-correctness and implementation-refinement obligations below.
`[pcc:unverified]`

The finite Curve25519 audit is not an instantiation of the asymptotic speedup:
it reports a 149-bit expected contraction-call bound and a 412-bit dense
elimination bound before the contraction theorem's hidden constants.
`[pcc:unverified]`

## Dependency graph

```text
R1 object refinement ─┬─> C1 contraction correctness ─┐
R2 uniform history ───┤                               │
R3 active-call cost ──┼─> C2 complete source cost ────┼─> T total ECDL
R4 arithmetic widths ─┤                               │
R5 recursion tree ────┘                               │
E1 reduction semantics ─> E2 success/cost balance ────┘
```

An independent review may replace any node with a stronger theorem.  A
counterexample to any node blocks only its descendants.

## R1 — Rust object refinement

For every active call produced by
`crates/hashed-ecdl-tensor/src/contraction/recursive.rs`, bind the live objects
to mathematical values

```text
(M, K, a, L=2^a, u, i, D=L/2^i, p, degree)
```

and establish all of the following:

1. `K L <= 3M`, `1 <= u <= a`, `i < u`, and `u <= M`;
2. the audited packet count includes empty slots and all retained packets;
3. every old vertex, packet vertex, isolate, edge incidence, and creation
   precision in the live child is represented in the mathematical child;
4. the strict branch gives `8 M_child <= M` and child maximum degree at most
   the root `delta`; and
5. packet descriptions contain data only, with no callback to a parent
   evaluator.

The runtime guards and canonical serialization tests are evidence for R1 but
do not constitute a refinement proof. `[pcc:unverified]`

`IMPLEMENTATION_BOUNDARY.md` records that the positive production recursion is
unreachable for a host-representable `usize` graph. The first symbolic active
row has `lambda=131072,u=1`; at `lambda=2^26` all source guards pass but the
theorem's `u>=lambda^(4/5)` invariant does not. The first passing power-of-two
scan row is `lambda=2^480`, with 80 active depths. These facts make R1 a
symbolic/refinement obligation rather than a runnable finite attack path.
`[pcc:unverified]`

The forced source-object control covers 2,080 `(radix,a,M,family,schedule,i)`
contexts, 3,388 retained tuple states and 25,490 group-child calls. It checks
every old vertex, isolate, endpoint/port identity, packet ownership, creation
precision and total endpoint mass, plus 7,351 generated modes whose packet,
target-row digit, source-column digit, nonzero coefficients, and weight bound
all match their descriptions. A balanced radix-64 fixture additionally keeps
all 262,144 packet vertices, including 254,016 isolates. This substantially
tests items 2–5, but deliberately bypasses `Setup` admission and therefore
does not close the live-object refinement. `[pcc:unverified]`

## R2 — Uniform precision history

Put `s=M_0`, `lambda=ceil(log2(s))`, and
`u_d=floor(lambda/(2^17 2^d))`.  Under R1's strict shrink and the fixed root
leaf cutoff, an active depth satisfies `8^d < floor(sqrt(lambda))`.

For every `lambda >= 2^26`, establish simultaneously at every possible active
depth:

```text
u_d >= 32,
81 floor(sqrt(lambda)) u_d <= 2^(u_d+1),
8 u_d (a_d+1) <= 2^(a_(d-1)+u_d+1),
p_next u_d <= 34 M_d (L_d/2^i).
```

`audit_uniform_history` checks the complete finite depth schedule and the
negative `lambda=2^23` control.  The following all-lambda derivation is the
independent proof-obligation sketch, not a kernel check. `[pcc:unverified]`
For `d<=4`, the threshold gives `u_d>=32`; for `d>=5`,
activity gives `lambda>2^(6d)` and therefore
`u_d>=2^(5d-17)>=2^d>=32`.  Consequently
`lambda<2^(17+d)(u_d+1)<2^18*u_d^2`, so
`floor(sqrt(lambda))<512u_d`.  The inherited absorption follows from its
`u=32` base case because the exponential right side doubles while its
quadratic left bound grows by less than two.  For evaluation,
`(A+2u+1)/2^A` decreases for every integer `A>=0`, so `A=0` is worst; its
`u=32` base case and the same induction prove the second absorption.
`[pcc:unverified]`

## R3 — One active-call cost

Assuming R1 and R2, prove that one active call with `M` edges and scale `u`
invokes children at most `[pcc:unverified]`

```text
2^(96 M/sqrt(u))
```

up to a fixed-degree polynomial in the input and description lengths.  The
proof must include zero-filtered mode construction, weighted tuple
enumeration, all active Hermite nodes, every interpolation sample, all 64
radix groups, and packet-entry coefficient choices.

The executable ingredients are `audit_node_dimensions`, the exact small
weighted-tuple enumerations, and the packet-capacity runtime guard.
`[pcc:unverified]`

## R4 — Arithmetic and representation widths

For the direct algorithms actually called by `active` and `recover`, prove
that the following bounds cover every stored value and temporary.
`[pcc:unverified]`

```text
retained jet coefficients      J <= 8 M L
maximum multiplicity           q_max <= 4 M
ring multiplication temporary  <= 2p+1 bits
rational multiplier            <= (J+q_max)(a+1)+1 bits
rational accumulator           <= J(J+q_max)(a+1)+J+p+ceil(log2(J+1))+1 bits.
```

Also refine the direct interpolation and recovery operation counts returned by
`audit_arithmetic_widths`.  Canonical coefficient reduction and reduced
`Ratio<BigInt>` representations may lower these widths but cannot be assumed
without a representation lemma. `[pcc:unverified]`

## R5 — Complete recursion tree

Assuming R1–R4, prove `[pcc:unverified]`

```text
M_d <= s/8^d,
u_0 <= 4^(d+1) u_d,
sum_d M_d/sqrt(u_d) <= 3s/sqrt(u_0).
```

The active branching exponent is then at most `288s/sqrt(u_0)`.  Direct leaves
and packet-entry traversal add one unit each, giving

```text
(s+description)^O(1) * 2^(290s/sqrt(u_0))
 <= (s+description)^O(1) * 2^(148480s/sqrt(lambda)).
```

`audit_recursion_composition` checks the finite source schedule at the
threshold.  The missing step is a quantified theorem linked to R1's live
recursion tree. `[pcc:unverified]`

## C1 — Contraction correctness

For every admitted bipartite tensor graph and every creation-precision
history, prove that `active` returns the same ring element as direct tensor
contraction. `[pcc:unverified]` This decomposes into:

1. the Cauchy kernel and mode expansion equal the original packet entries;
2. weighted tuple truncation drops only terms divisible by the working
   precision;
3. every constructed child represents its intended coefficient product;
4. interpolation recovers each required Taylor coefficient over the extension
   of `Z/2^p Z`;
5. tapered Hermite recovery returns the constant term and is 2-adically
   integral; and
6. creation-time representatives are lifted canonically at later precisions.

`HERMITE_IDENTITY.md` states item 5 as an explicit fraction-field identity,
maps every term to `recover`, and derives an all-parameter recovery loss below
the existing `8h(L/2^i)` node allowance. Independent checking and the live
Rust refinement link remain open. `[pcc:unverified]`

For item 4, the sample points are binary-coordinate lifts of distinct elements
of `GF(2^d)`. Every pairwise difference is therefore nonzero modulo two and
is a unit at every coefficient precision. Lagrange interpolation over the
extension of `Z/2^p Z` is exact whenever the guarded sample count is at most
`2^d`. The native control covers precisions 1/3/17, degrees 1/3/4/8,
extension-valued coefficients, all retained lengths through nine, and a
duplicate-node failure. Linking the values returned by `evaluate_at` to the
claimed deformation polynomial remains in C1 items 1–3. `[pcc:unverified]`

For item 1's scalar kernel step, `mode_coefficient` is the geometric expansion
of `Cauchy::weight` along a tile `y=y0+2^(i+1)j`. Its weight-`b+1`
coefficient is divisible by `2^(b+1)`, and the coefficient at `b=p-1` is zero
modulo `2^p`, justifying the finite series. The native control now checks all
active nodes for `u=1,a=2,L=4,p=5` and `u=2,a=4,L=16,p=7`, at scalar and
nontrivial extension sample values. Factoring those scalar identities through
`modes`, `child`, and all packet ports remains in items 2–3.
`[pcc:unverified]`

For the packet-local part of items 2–3, an independent exhaustive oracle now
enumerates every constant/incoming-mode choice per row and filters only by the
requested exponent vector. It agrees with the production depth-first
coefficient extractor on all 64 port assignments for four rows, two columns,
two simultaneous incoming modes of total degree three, local affine terms,
and two outgoing forms including a square. A separate direct deformation
oracle sums over independent left/right incidence bits and applies the full
Cauchy packet kernel without calling `modes`, `child`, `next_tuple`, packet
entry, or `evaluate_at`. It agrees with the extracted modes-to-child product
in 1,536 configurations: all 256 endpoint-table pairs for three parallel-port
label/sample cases and all 256 behavior pairs for each other two-port vertex
partition (split-left, split-right, and split-both). The cases cover local and
crossing groups, zero/unit/extension samples, correlated ports, and vertices
assigned to both groups. Removing a generated mode or flipping its target
group is also detected against that direct oracle. The remaining link is the
quantified refinement for every admitted graph, not an untested finite
semantic target. `[pcc:unverified]`

The direct oracle also exhausts all 25 left/right set-partition pairs of three
labeled edges, all 64 products of per-vertex delta-table bases, and four label
schedules, for 6,400 further configurations. Since both sides of the identity
are separately linear in every old tensor table, this basis determines the
factorization map for arbitrary tensor entries on each tested structural
shape. This strengthens the independent finite refinement without claiming
all packet counts, edge counts, or labelings. `[pcc:unverified]`

At four edges, the control quotients all 225 partition pairs to 33 graph-shape
orbits under simultaneous edge relabeling, then checks all 256 delta-basis
coefficients under four label schedules: 33,792 comparisons. A test-only
symbolic child map batches the coefficients by old-incidence mask while still
calling production mode construction, tuple enumeration, child construction,
and packet entries. It agrees with a separately computed direct packet-kernel
map. The measured Rust 1.78 debug runtime fell from 536.50 seconds for repeated
delta contractions to 3.96 seconds for the same coefficient set.
`[pcc:unverified]`

At five edges, the same symbolic kernel covers all 91 graph-shape orbits of
the 2,704 left/right partition pairs, four label schedules, and all 1,024
delta-basis coefficients: 372,736 comparisons. A third independent map now
expands same-group kernel weights and generated mode row/source forms directly,
without calling child construction, tuple enumeration, or packet entry. All
three maps agree in 76.17 seconds under Rust 1.78 debug. This meets the
90-second scaling target but remains a bounded refinement rather than a
quantified proof. `[pcc:unverified]`

`COEFFICIENT_MAP_REFINEMENT.md` states the reusable incidence-basis lemma
behind those maps, proves the producer implication `D=E=H`, maps each object
to Rust, and lists the six checks required from an independent kernel.
`[pcc:unverified]`

A separate public native certificate verifier now implements those six checks
over plain certificate data without invoking production child construction
during verification. Duplicate packet coverage, duplicated mode slots,
mode-target reassignment, tuple removal, and packet-table mutation are rejected.
The schema-2 positive certificate round-trips through a stable 2,172-byte JSON
representation; schema mismatch, noncanonical ring-element, invalid tile
period, and overflowing structural-count mutations fail closed.
`[pcc:unverified]`

The serialized checker also covers all 100 combinations of the 25 three-edge
left/right set-partition pairs and four label schedules. Their total wire size
is 318,734 bytes, the largest certificate is 10,146 bytes, and the warm Rust
1.78 debug verification sweep takes 0.34 seconds. `[pcc:unverified]`

For four edges, the same boundary covers 33 graph-shape orbits and four label
schedules: 132 certificates, 792,556 total bytes, and a 20,777-byte maximum.
Measured checker time is about 0.60 seconds; production and JSON bring the
complete warm debug gate to 1.12--1.26 seconds. `[pcc:unverified]`

For five edges, 91 relabeling orbits under four schedules give 364 serialized
certificates, 3,652,442 total bytes, and a 43,460-byte maximum. The measured
verifier phase is 2.06 seconds and the complete production/JSON/verification
gate is 4.93 seconds, below the three-second and six-second targets.
`[pcc:unverified]`

`CERTIFICATE_DESCRIPTION_ENVELOPE.md` then bounds the complete certificate
description from `M,K,L,tau,R,Q,T,d,p`. The entry-value term follows from
`sum_g 2^a_g <= 2^(2L)+R-1`; explicit JSON and canonical-binary ceilings are
checked on every serialized fixture through five edges. The verifier also
reconstructs the tile keys and exact weighted tuple set, enforcing
`Q<=K tau(p-1)` and `log2 T<=5 ceil(sqrt(K tau p))`. This binds `Q` and `T` to
the existing active-call source ledger while leaving recursive history and
precision refinement open.
`[pcc:unverified]`

The producer control constructs `NodeDimensions` from every bounded
certificate. Of the 596 three- through five-edge partition certificates, 285
satisfy `K L<=3M`; their packet length, tile period, per-weight and total mode
ceilings, and tuple-log2 ceiling equal the independent BigUint audit exactly.
The other 311, plus the correlated two-edge fixture, are retained only as
algebraic controls and are explicitly rejected by the source packing premise.
`[pcc:unverified]`

The verifier is now a public module separate from recursive contraction and a
native `factorization-certificate-check` executable. It imports exact ring
arithmetic but no contraction, deformation, packing, child, or packet-tensor
code. The command caps JSON input at 64 MiB and applies explicit work, vector,
width, and mode-depth limits. This is an executable separation boundary, not a
theorem-assistant or independently reimplemented arithmetic kernel.
`[pcc:unverified]`

`GROUP_FACTORIZATION.md` states that remaining theorem as a packet-polynomial
coefficient identity, maps its constants, incoming coefficients, exponent
tuple, and outgoing forms to Rust, and isolates the all-graph refinement gate.
It now also gives the quantified producer proof from the direct deformation
sum to the product of child contractions: whole-vertex port correlation,
packet ownership, isolated vertices, target/source placement, finite Fubini,
and weighted truncation are explicit. The remaining gap is independent or
kernel-checked Rust refinement rather than an unstated composition step.
`[pcc:unverified]`

Production `next_tuple` also matches an independent Cartesian enumeration in
83,970 exhaustive small-state configurations, plus two multi-packet fixtures.
`WEIGHTED_TUPLE_ITERATOR.md` states the general positive-weight cached-state,
arithmetic-safety, termination, and restricted-odometer argument. This closes
the producer-side iterator subargument without replacing independent or
kernel-checked refinement. `[pcc:unverified]`

The native suite covers direct one-level controls, two taper coordinates,
creation-precision mutation tests, and a nonvacuous three-level one-taper
control. That callback tree now carries 433 independently verified serialized
node certificates through active depths zero, one, and two, with exact counts
`1,20,412` and working precisions `3,67,131`. It remains a non-strict,
non-shrinking diagnostic and therefore does not establish a source-admitted
history. An isolated recovery control additionally covers `u=2` and `u=3`,
all their taper valuations, root precisions 3/5/7 at `u=2`, and scalar plus
nontrivial extension-coordinate polynomials.  A combined two-taper/deep
diagnostic remained live past 90 seconds, so these bounded tests do not close
C1. `[pcc:unverified]`

## C2 — Complete source cost

C2 is R1–R5 plus the assertion that all code executed by an active call is
included: allocation, graph construction, packing, interpolation, recovery,
packet entry, recursion bookkeeping, and integer/rational arithmetic.  No
unpriced oracle, parent callback, or depth-dependent hidden description may
remain. `[pcc:unverified]`

## E1 — Prime-field reduction semantics

For one fixed public seed, establish that:

1. base enumeration is exact and deterministic;
2. the contraction oracle sees only the public bases and target group element,
   not its hidden scalar coefficients;
3. prefix self-reduction is deterministic and returns a replayable witness;
4. relation dependency selection is deterministic;
5. duplicated points in different slots remain distinct columns; and
6. target recovery accepts only after exact scalar replay.

These properties are exercised by the typed network and reduction tests but
remain part of the end-to-end refinement boundary. `[pcc:unverified]`

## E2 — Success probability and total cost

Using the pinned Bochum v0.2 Theorem 4 and Lemmas 9–13, independently establish
the fixed-target seed average `>=2/5`, good-seed probability `>=1/4`, enough-row
probability `>=3/4`, and final nonzero coefficient probability `1-1/r`.  The
per-trial lower bound is

```text
3(r-1)/(16r) > 1/6,
```

so fresh trials have expected count at most six.  Charge factor-base
enumeration, `O(mb)` prefix contractions per target attempt, an `(N+1)`-row
dense elimination, and exact target replay. `[pcc:unverified]`

## Acceptance rule

The top-level statement may be promoted only when C1, C2, E1, and E2 have
independent proofs or a kernel-checked composition, with R1–R5 discharged as
their named dependencies.  Until then, the result is a conditional producer
certificate and every finite Curve25519 number is a cost ledger rather than a
security estimate. `[pcc:unverified]`
