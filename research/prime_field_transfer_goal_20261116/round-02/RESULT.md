# Input-only local torsion lifting on the compact model

The native evaluator computes the source-local prime-to-p torsion lift from
encoded point coordinates, without a scalar argument or subgroup lookup. All
planned correctness cases now complete, including the four hash-derived public
secp256k1 points at64 p-adic digits. This implements a classical local step
from Silverman's lifting framework on the round-one compact model.
PCC-20261116-391259 is unsigned. [pcc:unverified]

| Source | Public inputs per precision | Precisions in p-adic digits | Recurrence nodes |
| --- | ---: | --- | ---: |
| y²=x³+2x+2 over F17, r19 | all18 nonidentity subgroup points | 1,2,4,8,16,32,64 | 13 |
| y²=x³+23x+11 over F257, r83 | all82 nonidentity subgroup points | 1,2,4,8,16,32,64 | 25 |
| secp256k1 | four hash-derived public points | 1,2,4,8,16,32,64 | 2019 |

These are deterministic coverage/node counts, not runtimes, benchmarks or
estimated attack costs. The local evaluator works on the input coordinates;
the small-subgroup scalar enumeration is confined to its separate auditor.
The secp256k1 points come from the literal hash strings in PROTOCOL.md, without
a supplied generating scalar. SHA256 here is public fixture generation, not
the standardized hash-to-curve construction. [pcc:unverified]

## Exact interface and independent arithmetic path

`lift(model, input, precision)` receives the fixed model, Q=(x,y) over Fp and
N. It projects each symbolic global coefficient modulo p^N using modular
exponentiation; it never expands2^r for secp256k1. It evaluates the classical
normalized division polynomial f_r and its derivative by a memoized recurrence,
Newton-lifts x, and chooses y by its original source residue. Identity maps to
identity. The function contains no P, k, scalar search or lift table.

The validation path separately transforms the generalized model to short
Weierstrass form and uses Jacobian addition/doubling over Z/p^N. It checks
[r]lift(Q)=O, source reduction, four small multiplication identities, rejected
coordinate/ordinate controls and malformed inputs. All small-fixture scalar
relations pass; at64 digits the four secp256k1 inputs also pass negation and
addition of the fixed generator. OpenSSL separately handles source membership,
order and public-point addition. The producer wrote the GMP paths; neither
the second arithmetic path nor OpenSSL makes the execution a signed independent
replay. [pcc:unverified]

The classical mathematical reason for scalar preservation is uniqueness:
good reduction's formal kernel has no r-torsion when r is prime to p.
Consequently two r-torsion lifts reducing to the same source point coincide,
so lift([k]P)=[k]lift(P). The original source equation is nonsingular and
r≠p on all three fixtures. This uses Silverman's stated theorem; the native
code and its all-input implementation are not formally proved here.
[pcc:unverified]

## Lean endpoints and their limits

Using Mathlib4 v4.24.0 laws as stated, the actual integral model at r19 has
normalized division-polynomial degree180, and the r83 model has degree3444.
Claims PCC-20261116-75ba98 and PCC-20261116-25851b are unsigned.
The implementation evaluates these polynomials without expanding them.
This illustrates why dense polynomial degree is not itself a lower bound
on compressed local evaluation. It is not an irreducibility certificate.
[pcc:unverified]

Mathlib's characteristic-divisibility law also gives no unital ring hom
ZMod(17^64)→ZMod2 or ZMod(257^64)→ZMod2. Claims
PCC-20261116-ec6bcb and PCC-20261116-fe3335 are unsigned. This rules out
treating the least integer representative of a finite p-adic lift as a
compatible binary residue map. It does not forbid a set map, a group map,
a cover construction or a genuine globally selected algebraic lift.
[pcc:unverified]

The unchanged proof gate passes20 theorems with zero trusted native
exceptions. Four explicit axiom audits list only propext, Classical.choice
and Quot.sound. The first Lean endpoint attempt failed because broad numeric
simplification consumed an already simplified hypothesis; the source and log
are retained. The final endpoints apply existing Mathlib degree and
characteristic laws directly. See PROOF-MAP.md. [pcc:unverified]

## Preserved failures and provenance

Launch01 had no execute permission and ran no arithmetic. Aggregate call02
and the isolated secp256k1 call03 each hit the frozen60-second limit. The
latter completed through32 digits; neither is called a full pass. The same
suite completes when partitioned by the originally specified public point,
with unchanged core evaluator and unchanged60-second per-call limit. See
AMENDMENT-01.md and AMENDMENT-02.md. [pcc:unverified]

Distinct build/pre-call/post-call manifests, read-only qualifier hashes,
source hashes, raw outputs, exit statuses and failed attempts are in
../receipts/round-02/. These are development provenance, not an independently
committed preregistration or an admitted performance run. No EKS job or cloud
spend, timing comparison, benchmark or leaderboard promotion occurred. Cargo,
Conductor and research-KB remain unavailable; native C++/GMP/OpenSSL and primary
sources were used. The full Rust workspace test was not run.

## Transfer obligation still open

The result is in E(Qp)[r], represented modulo p^N. The easy component decoder
expects the compatible point at the pinned2-adic place. Selecting a root at
that other place using only the public input remains unimplemented. Two local
root equations or changing the embedding do not identify the same global
torsion point (CT124). CT95 already rules out fixed low rational templates;
it does not exclude compressed global selection algorithms.

Next test a compressed representation that includes a genuine common-field
branch certificate and supports extracting the oriented component label at
the fixed second place. It must take arbitrary Q, survive the source relation
and CT124 chimera controls, and include its construction/selection costs.
Passing this local suite does not discharge that obligation or establish a
novel map or a demonstrated total target advantage. The original GOAL.md
acceptance criteria and active research goal are unchanged. [pcc:unverified]

## Sources and publication state

Sources checked before code: CT85, CT95, CT97, CT123 and CT124; Silverman,
*Four Faces of Lifting*, pp.11–14; Cremona, *Computing in Component Groups*,
section2; Sage general Weierstrass/division-polynomial documentation; Mathlib
DivisionPolynomial/Degree and CharP/Algebra. PROTOCOL.md has their links.
KB unavailable: MCP not loaded, wrapper exits127; no novelty/absence claim.

The local snapshot at02:46 found a divergent frozen ledger against main;
that receipt remains historical. A subsequent API read establishes PR4378
had already merged at02:30:36Z, merge66d1727b4ab2086a2855578fd5b0942954d5664a,
via an external head3f50ddffe7504818c1fd44dfbf362d54e3f3348c. Both parents'
complete normalized journal blocks survive that merge. Its canonical ledger
preserves the1184-row incoming-main prefix, but does not preserve the old
published head's1182-row prefix (common1179). This session did not perform
or repair that external resolution; the existing frozen-history problem
remains reportable. See round01-external-merge-audit.json. [pcc:unverified]

Publish this new round from fresh main, preserving the unpublished local
ledger bytes and registering the unchanged new claim files with the existing
PCC append tool. No published history is re-chained by this session. The old
monitor is retired and the round-one autonomy outcome is recorded. A fresh
main prefix check is required for the follow-up; historical merge issues are
not erased by that check. New events are in WORKLOG.md and the shared log.
