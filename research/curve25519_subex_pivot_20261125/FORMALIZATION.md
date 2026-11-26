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

## R2 — Uniform precision history

Put `s=M_0`, `lambda=ceil(log2(s))`, and
`u_d=floor(lambda/(2^17 2^d))`.  Under R1's strict shrink and the fixed root
leaf cutoff, an active depth satisfies `8^d < floor(sqrt(lambda))`.

For every `lambda >= 2^26`, establish simultaneously at every possible active
depth:

```text
u_d >= 24,
81 floor(sqrt(lambda)) u_d <= 2^(u_d+1),
8 u_d (a_d+1) <= 2^(a_(d-1)+u_d+1),
p_next u_d <= 34 M_d (L_d/2^i).
```

`audit_uniform_history` checks the complete finite depth schedule and the
negative `lambda=2^23` control, but a formal proof must quantify over all
`lambda >= 2^26`. `[pcc:unverified]`

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

The native suite covers direct one-level controls, two taper coordinates,
creation-precision mutation tests, and a nonvacuous three-level one-taper
control.  An isolated recovery control additionally covers `u=2` and `u=3`,
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
