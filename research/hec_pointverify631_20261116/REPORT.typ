#set page(paper:"a4",margin:24mm)
#set text(size:11pt)
= F631 point logs: geometric verification
Genus 3 · 15 November 2026

#table(columns:(1fr,1fr),[Check],[Outcome],[Recorded point values],[597],[Direct Jacobian identities],[597 pass],[Generator],[Nonzero; killed by prime59296481],[Corrupted value],[Rejected],[Process],[Exit zero; stderr empty])

== Identity checked
Set G = 4(P1 − P0). For every retained point Pi and recorded relation-kernel value vi, verify 4(Pi − P0) = vi G using the actual plane-quartic Jacobian group arithmetic.

Every identity passes. Normalization v0 = 0 and v1 = 1 is checked. G is nonzero and has the registered prime order. A one-unit corruption at index one is rejected. Each individual point check is retained in the raw output.

== What this establishes
The fresh sparse relation values now have an independent geometric interpretation as point logarithms after cofactor clearing. This does not depend on establishing that the relation kernel has dimension one. Multiplier four removes the full cofactor; unprojected two-primary components are outside this identity.

== Custody and next gate
Source9f5b8ead9f79 and checkpoint20261116T024735Z precede the single execution. No old point logs, expected original images or scalar were supplied.

Next, conic descent must evaluate the recovered original classes while respecting this projection and mixed-primary centering, followed by scalar verification on source and destination. Global correspondence, charged comparison and genus-two/four mechanisms remain open. No timing or tenfold improvement is claimed.
