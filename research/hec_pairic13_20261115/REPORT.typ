#set page(paper: "a4", margin: 18mm)
#set text(font: "Helvetica", size: 10pt)
#text(size: 21pt, weight: "bold")[Pair lines and large-prime elimination]
#v(6pt)
Genus 3 · Q1/F127 · same source target · 15 November 2026

The reduced relation engine recovered 183,049 and verified it on the source and
destination. Its reconstructed point-log functional matches the full reference
up to one scale, with no unresolved coordinates in this execution.

#image("COUNTS.svg", width: 100%)

*Collection.* The small base contains 48 fixed points. Of 1,128 pairs, 250 repeat
a line; 564 unique lines fail the split/distinctness test and 314 pass. Accepted
relations contain 0/1/2 large primes in 14/86/214 cases. Each pair's known roots
are divided out, leaving a residual quadratic or linear polynomial plus infinity.

*Elimination.* Removing 81 large-prime columns leaves 232 rows and 47 columns,
rank 46 and nullity 1. Reverse normalized-pivot tracing recovers all 129 logs.
Rows and diagnostics exactly match the unmodified pinned sparse routine; all
314 admitted-row residuals pass. Unknown dependencies would remain unknown.

*Point generation and descent.* Rational points come from the linear part of
127 affine polynomial restrictions and an infinity polynomial. The ordered list
matches all 129 reference points. Both public maps execute anew; generator and
target descent take 0 and 13 translates. Their log ratio yields 183,049.

*Execution and cost boundary.* Frozen source 7cb3afb0f885, checkpoint
20261115T171249Z precede one release run; exit 0, empty stderr. Full enumeration
and the second eliminator are controls outside the candidate path. Counts are
not timings, and the final factor-base solve remains dense. Sparse diagnostic
inversion charges are conventional, not calibrated field costs. Complete cold
construction, calibration, transport, solving and verification still need pricing.

#text(size: 8pt)[Artifacts: PROTOCOL.md, PROOF.md, RESULTS.md, RUN.txt, extract.sh and hashes.sha256
in research/hec_pairic13_20261115. Reuses pinned diem_quartic.rs LP elimination; primary
Diem JTNB24(2012) §§1.2/3.2. Fixed-base gate does not implement the full staged theorem.
No end-to-end 10× or novelty claim. Standalone science is not automatically run by native CI.]
