#set page(paper: "a4", margin: 18mm)
#set text(font: "Helvetica", size: 10pt)
#text(size: 21pt, weight: "bold")[Destination Jacobian arithmetic]
#v(6pt)
Genus 3 · F127 · frozen correctness gate · 15 November 2026

The reconstructed destination quartics both passed the reused section-space
Jacobian arithmetic gate. On Q1, two explicit doubled divisor classes are nonzero
and have prime order 1,028,893. Their encodings are retained in RUN.txt.

#image("COUNTS.svg", width: 100%)

*Checks.* Each curve passed two identity, inverse, doubling and registered-order
annihilator checks, plus one commutativity and one associativity check. One release
run; exit 0; empty stderr. Doubling is a wrapper control, not independent validation.

*Order inference.* With L(T) = 1 + a₁T + ⋯, Q1 has a₁ = 129 − 128 = 1 and a₂ = −43.
The Weil interval for a₃ has width below 57,250, smaller than the certified prime
subgroup order. Hence the measured counts and prime-order classes uniquely
identify the order of Jac(Q1) = 2,057,786, conditional on the reused arithmetic being correct.
The derived a₃ = −1,224 matches the earlier geometric prediction.

*Protocol correction.* The frozen prose wrote the opposite sign for a₁. The correct
formula is a₁ = N₁ − 128. Expected counts, executable checks and annihilators were
correct and remain unchanged; RESULTS.md records this explicit erratum.

*Reuse and scope.* Khuri-Makdisi's divisor-subspace method, via the pinned repository
plane-quartic implementation. Fixed rational-point triples, no retries or replacement.
No third-extension enumeration or timing comparison. Q0's registered annihilator
passed; this run alone does not independently establish its full group order.

*Next construction.* Align the source quotient's theta Kummer coordinates with the
destination Jacobian representation, then lift public inputs to divisor classes.
Matching subgroup orders alone does not supply that map. Tian §4.2 describes this
alignment/lifting dependency. Costs will include setup, exceptional charts and verification.

#text(size: 8pt)[Source: f72b5fb5f004 + lockfile 6c82945ab581; pre-run S3 checkpoint 20261115T155620Z.
Raw records and exact coefficient normalization: research/hec_transport13_20261115.
Local gate; standalone scientific runner is not automatically executed by native CI.]
