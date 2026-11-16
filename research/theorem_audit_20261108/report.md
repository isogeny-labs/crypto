---
title: "Fourteen-target theorem audit: a Lean increment"
date: "2026-11-08"
---

The full short-Weierstrass S3 zero-sum equivalence, including repeated
coordinates, now compiles using Mathlib's actual elliptic point group.
The theorem assumes an algebraically closed field with 2 nonzero,
short coefficients and nonzero discriminant. This is the central general
S3 converse; it is not a binary formula or a resultant-degree theorem.
Signed PCC standing is pending. [pcc:unverified]

![](coverage.pdf){width=100%}

The project contains 33 new declarations and reuses two unchanged historical
modules. The repository axiom gate passes all 68 audited declarations (33 new,
35 reused), with zero forbidden axioms and zero native exceptions. Local builds, production Rust regression tests and axiom outputs
are retained in evidence/. The proof map names every statement and its
limits; the full README retains all fourteen requirements. [pcc:unverified]

| Targets | Component supplied | Still required |
|---|---|---|
| 1 | General S3 existence equivalence | binary case, recursion/degrees, native exhaustive checks |
| 2--3 | Triangular Frobenius core; exact order from trace-zero cardinality | actual torsion/Frobenius and supersingular/Hasse bridges; extension census |
| 4--5 | Cyclic pairing restriction; stable branch-locus obstruction | actual pairings, ramification geometry, factor-base lower bound |
| 6--7 | Asymptotic source review; abstract subgroup confinement | Jacobian definitions, counts and counterexample family |
| 8--9 | Nonreduced multiplicity control; exact hypothesis review | general multihomogeneous bound; msolve system experiments |
| 10--11 | One-step collision mass; walk and NIST source review | mixing/collision analysis, Smart attack and rho measurements |
| 12--13 | Pinned msolve source audit; existing cross-language evidence | implementation refinement and second-machine replay |
| 14 | Known degree-3 proof and actual census-routine controls | independent signed validation |

Every entry is a scoped producer assertion, not a signed scientific
promotion. The original request remains incomplete. [pcc:unverified]

The fixed negative control is
F={x0^2+x0, x1^2+x1, x0*x1+x0+x1} over F2. The existing criterion
rejects degree two; explicit degree-three rows contain the Gröbner basis
{x0,x1}. The actual Rust functions return low dimensions 0 and2, ranks3
and9; all three fixed development tests pass. An independent C++ fixture
agrees on this same host. [pcc:unverified]

Source corrections matter: the actual BK node excludes r dividing q-1,
so the earlier F7 examples do not refute it. Diem Condition2.7 excludes the
base-defined x projection by its Frobenius-stable branch locus; Diem
constructs another projection. NIST AppendixC.1 excludes anomalous curves
and requires large embedding degree. See sources.json for exact hashes and
read sections. [pcc:unverified]

No paid cloud jobs or new research censuses were launched. The complete
resume document could not be uploaded because the required checkpoint token
and AWS profile are absent. No second independent execution environment is
selected. These dependencies block experiments/replay, not the completed
local proof and development checks. [pcc:unverified]

Reproduction: `bash check.sh`; then the repository
`tools/lean/proof-gate.sh research/theorem_audit_20261108/lean`.
Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679
are pinned. The accepted axiom set is propext, Classical.choice and
Quot.sound; no native evaluation or research axiom is permitted.
[pcc:unverified]
