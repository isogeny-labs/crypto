# Three supplied ECDLP leads: audit protocol

Date: 2026-10-18. Scope: the three claims supplied in this chat: subfield
summation-polynomial decomposition (odd n=2..5 and binary n=2), PKM prime-field
towers, and quadratic-extension transfer to genus-2 index calculus.

This is a correctness and evidence audit, not a performance improvement.
Historical evidence is read only. No new wall-time or fastest-solver claim is
admitted. Preserve the original numerical claims as unverified until their
exact producer and raw records have been located and independently checked.

## Requirements

| Requirement | Planned evidence |
| --- | --- |
| Trace obstruction, domain, coset, subgroup consequence | Original Kosters–Yeo statement; independent finite-field enumeration |
| Exact 3/8 coverage claim | Distinguish the half-group theorem from an asymptotic counting theorem; test generic and descended coefficients separately |
| Odd-characteristic n=2..5 and random controls | Inspect existing source and primary Gaudry/Diem analysis; separate coverage from solving cost |
| PKM Kummer/Dickson claims and descendants | Read original PKM construction and later prime-field solver papers; locate repository producers and matched-control definitions |
| Genus-2 construction/map | Check Scholten/Bernstein–Lange construction, kernel assumptions, infinity model and existing native support |
| First-cycle extraction and rho control | Check signed cycles versus graph cycles, zero coefficients, target dependence and strongest matching generic reference |
| All quoted counts and pricing cells | Locate exact source; recompute when available; leave unrecovered records explicitly unverified |
| Reviewable output | Source-linked report, diagram, PDF, checks and precise open obligations |

## Independent finite checks

Use a dependency-free Rust program with portable polynomial-basis GF(2^d)
arithmetic. Find and verify irreducible moduli, enumerate all rational curve
points, and enumerate unordered factor-base pairs including repeats. The curve
model is y^2+xy=x^3+a*x^2+b with b != 0; the identity is included in the group
population and excluded from the factor base. Factor-base x coordinates satisfy
x^(2^m)=x. Record all coefficients and the actual population denominators.

At quadratic extension degrees d=6,8,10,12,14 use seven deterministic generic
curves per degree (b outside the subfield). Include descended-b controls and
both absolute traces of a at d=8,12. At extension degree four use base sizes
4 and 8 and compare the fourfold sumset with 2E.

Required assertions: irreducibility; field inverse identities; every enumerated
point on the curve; subfield size and trace; closure of all enumerated pair
sums; trace-map additivity on all factor-base pairs; exactly half the group in
2E; equality of the doubled-point image and trace kernel; no reachable target
outside 2E; every odd-order point lies in 2E. Cross-check the curve enumeration
against exhaustive x,y evaluation at d<=8. Independently check S3 against group
pair sums on the smallest quadratic cases. Negative controls must demonstrate
that 3/8 is not a universal finite-field coverage identity. This census does
not reproduce the original 21-curve experiment or prove an asymptotic limit.

Stop on the first correctness failure; preserve its output. Bounds: d<=14,
seven generic curves per degree, no timing interpretation, local deterministic
correctness work only. No cloud instances or production targets.

## Amendment 1: independent odd-characteristic and prime-tower checks

Before execution, add a separate dependency-free Rust implementation of
GF(p^n), p in {5,7}, n in {2,3,4,5}. Use three deterministic genuinely
extension-defined nonsingular curves per field and 32 matched random bases
per curve. Enumerate full group points and the complete n-fold sumset.
Match sign-orbit count and retain the same rational two-torsion points.
Record coverage, null mean/spread, and raw per-control coverage. Small
fields and three curves are correctness diagnostics, not replacements for
the original study or equivalence tests. Check the smallest fields against
exhaustive x,y enumeration, all inverses, and all emitted sums on-curve.

Also inspect Kummer and split-Dickson fibers over primes {257,769,1153},
depths {2,3,4}, using the curve y^2=x^3+2*x+3 whenever nonsingular.
Enumerate each fiber directly, independently rebuild the Kummer fiber by
the multiplicative group, check recursive Dickson evaluation against
u^d+u^-d on split torus inputs, and compare the complete two-sum coverage
with 32 same-size sign-orbit controls. The fiber, actual usable base size,
and every null result stay in the output. This tests fiber correctness
and coverage; it does not time or validate a PKM equation solver.

Fixed pseudorandom seed: 0x20261018. Stop on any arithmetic, group, fiber,
or sumset verification failure. No early stopping on a desired z score.

## Amendment 2: retain critical Dickson fibers and add generic ones

The first prime-tower diagnostic uses the critical Dickson fiber D_d(x)=2.
Retain that entire first run and source as v1 evidence. A second run uses
the first u>=2 with u^(2d)!=1 and c=u^d+u^-d, verifies the d distinct roots
against u*zeta+1/(u*zeta), and records c explicitly. Repeat all other
deterministic checks unchanged. No performance conclusion is drawn.

## Amendment 3: coefficient-splitting derivation check

After the coverage census, add assertions to the existing small binary
S3-versus-group check. For every rational target with x outside the subfield,
split b in the basis (1,x), verify the two S3 coefficients and the reduced
quadratic in w. Replay the complete CSV unchanged. This is a finite
correctness check of the report's derivation, not a solver implementation,
timing experiment, proof of the 3/8 limit, or identity.certificate/v1 record.
