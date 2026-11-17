#set page(paper: "a4", margin: 18mm)
#set text(font: "Helvetica", size: 10pt)
#text(size: 21pt, weight: "bold")[Mapped index calculus verifies source scalar]
#v(6pt)
Genus 3 · F127 · same public source pair · 15 November 2026

The explicit quotient transport and plane-quartic relation solver recovered
k = 183,049 for B = [k]D1. Scalar multiplication verified the result on both
the original hyperelliptic source and Jac(Q1); k + 1 was rejected on both.

#image("STAGES.svg", width: 100%)

*Relations and descent.* All 664 original relation rows annihilate the recorded
129-entry kernel; one corrupted row is rejected and eight principal-divisor class
identities pass. The generator's split divisor has point indices 89/91/94 and
scaled log 47,376. The target splits after 13 translates, at indices 5/18/47,
with scaled log 619,220. All preceding descent exclusions are retained.

*Same-pair control.* Native hyperelliptic index calculus, seed 20260916 and trial
cap 200,000, independently returned the same scalar after 407 trials and 72
relations. This is algorithmic agreement using shared arithmetic, not an
independent implementation or a runtime comparison.

*Execution.* Source 8b074fa80042 and checkpoint 20261115T164952Z preceded one
release run; exit 0, empty stderr. Both source maps executed afresh. No supplied
input scalar, substituted target, scientific rerun or fallback.

*Cost boundary.* Kernel certificate, quotient equation, normalized sections and
orientation calibration come from prior executed construction. This run joins
the correctness path after that construction; cold complete-cost evaluation must
charge it. Full projective-line enumeration and dense full-base elimination are
reference implementations. Efficient point enumeration, large-prime relations
and sparse solving are the next changes needed for the speed objective.

#text(size: 8pt)[Artifacts: PROTOCOL.md, PROOF.md, RESULTS.md, RUN.txt and hashes.sha256 in
research/hec_mappedic13_20261115. Prior art: native tian_quartic_km.rs line sieve and descent;
Tian arXiv:2007.03172. Standalone scientific gate is not automatically executed by native CI.
No timing ratio or 10× result is asserted; genus 2/4 work remains open.]
