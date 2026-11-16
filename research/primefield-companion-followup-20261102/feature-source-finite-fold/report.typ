#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= A compact projected-source interval
Phase-free finite-subgroup folding • T-1786 • 2026-11-07

The new constructor describes the projected source using one interval and a quasiperiod correction. It does not enumerate geometry or build an exponent map. For h=(r−1)/2, S=h(h+1)(2h+1)/6 and β=ψ(r+1)²ψ₂/ψ(r+2), the result is the projection of β to power −S times the product of ψ₂ through ψₕ. Reduce the β exponent modulo the projection order q=2 or4.

#image("flow.svg", width: 100%)

== Why the reduction preserves the sign
Ward quasiperiodicity folds n=kr+v to a signed half-range remainder. The implementation retains the reflection sign and β exponent; it does not simply replace n by its residue. The relation αʳ=β² eliminates α inside the projected image. The existing block pairs cover1 through h exactly once, yielding the direct interval constructor. No derivative seed, radical or field extension is needed by the new evaluator.

== Native checks
Four selected methods pass;61 imported methods are filtered out. Folded and compact values match26 original projected-source cases. Single-term controls cover1001 indices, including multiples of the field characteristic. Flipping the sign fails all26 source cases; omitting the β correction fails at least one. The direct descriptor matches expanded folding in96 symbolic partition cases. Fourteen source/fixture hashes are pinned. The prior passing validation and exact source snapshot are retained; unused-code warnings remain in logs.

== Exact remaining work
Compact construction is complete at this component's scope: five integer fields, no term vector. Its reference evaluator still visits h−1 division values and grows a cache. A faster interval-product evaluator preserving the odd-weight sign remains the next construction. The ambient divisor obstruction from the previous round remains valid; this is an equality on the finite subgroup. No production integration, experiment panel or speed measurement is claimed.

== Evidence and sources
PCC-20261107-01ee86 is unsigned producer evidence; shared arithmetic means no independent verification. DERIVATION.md contains the formula, pairing proof, domains and replay details. Sources: Lauter–Stange2008, Theorem4 and section3, https://arxiv.org/pdf/0803.0728; actual source-divisor compiler, seed cancellation and original half-product construction. KB unavailable. The eprint mirror returned403; the primary arXiv paper was read.
