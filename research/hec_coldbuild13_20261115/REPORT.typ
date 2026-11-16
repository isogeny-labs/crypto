#set page(paper: "a4", margin: 18mm)
#set text(font: "Helvetica", size: 10pt)
#text(size: 21pt, weight: "bold")[Public-input-only cold construction]
#v(6pt)
Genus 3 · F127 · fixed admitted primary factor · 15 November 2026

The construction and mapped solver run from only the public curve, prime subgroup
order and two divisors. No earlier kernel certificate, normalized sections,
quotient equation or orientation calibration is read during execution.

#image("CONSTRUCTION.svg", width: 100%)

*Derived setup.* Source point counts and the public prime subgroup determine
L = [1, 1, −43, −1224, −5461, 16129, 2048383]. The first admitted degree-13 field
is defined by z¹³ + z + 10. Attempt 0 yields both 27-point kernels; the configured
kernel-1 normalization succeeds on source multiple 2 without root extraction.

*Geometry.* Sixty-one regular samples among the fixed 64 prove quadratic rank 36.
The rank-7 Gamma00 jet gives the runtime hyperplane and smooth plane quartic.
The existing specialization/descent proof applies without cubic or Coble fitting.
The destination count selects the same-field Serre sign. Actual getters pass the
computed objects into transport; the first decoded point calibrates lambda = 10.

*Solve and checks.* All 81 large-prime columns are eliminated and all 129 point
logs reconstructed. The scalar 183,049 verifies on source and destination.
A separate native control gives the same scalar, 407 trials and 72 relations.
Post-run comparisons match all 54 kernel and 27 normalization records exactly.

*Execution.* Frozen source a6e57ffc2e39 and checkpoint 20261115T184509Z preceded
one cold and one native execution, both exit 0 with empty stderr, in a fresh
directory containing only PUBLIC.txt. There was no scientific rerun. The configured
primary factor is fixed for this admitted instance; no general admission claim.

*Cost boundary.* This gate records correctness, without timing. The separately
frozen coldcost13 experiment prices the regenerated construction and entire solve.
Its failed speed criterion must be read alongside this positive correctness result.

#text(size: 8pt)[Artifacts: PUBLIC.txt, PROTOCOL.md, PROOF.md, RESULTS.md, GATE.txt, RUN.txt, NATIVE.txt
and hashes.sha256 in research/hec_coldbuild13_20261115. Primary Tian, Oort–Ueno and
Fay/Pauly–Previato sources are linked in the protocol and earlier reconstruction proof.]
