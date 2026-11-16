#set page(paper: "a4", margin: 18mm)
#set text(font: "Helvetica", size: 10pt)
#text(size: 21pt, weight: "bold")[Actual source jets recover divisor pairs]
#v(6pt)
Genus 3 · F127 · source quotient functions · 15 November 2026

Five actual source section zeros decoded to oriented pairs on Q1. Their endpoint
differential scales agreed up to sign, and each negative source input reversed
the oriented pair. This supplies the derivative input missing from the earlier
destination-only decoder gate.

#image("CASES.svg", width: 100%)

*Checks.* All 16 fixed CRT/Cantor translation controls passed. The curve equation
holds through jet degree 4, and translated constant Mumford coefficients match
scalar Cantor. All 27 function constants and eight section constants match scalar
evaluation; section coefficients lie in F127. Existing 32 multiplication/inverse
jet controls and the determinant control passed.

*Decoded inputs.* Multiples 853, 938, 943, 1831 and 1998 yielded unique pairs.
The first case fixed global differential scale 10; later ratios were 10 or 117,
with reversed pairs at negative inputs. Nonzero target classes passed negation
and order 2,057,786 annihilation. Complete jets and encodings are in RUN.txt.

*Coverage and origin.* The full 2,048-trial cap produced five complete cases,
passing the registered minimum three (maximum six). The section is centered at
ψ(T), so source zero x maps to the difference ψ(x − T). This two-torsion shift
is retained. Zero-Hessian controls admit all six chord pairs and are ambiguous.

*Execution.* Frozen source 2bafe6f049a9 and checkpoint 20261115T162927Z preceded
one release run; exit 0, empty stderr. No rerun or independent implementation.
Counts are exact diagnostics, without a runtime comparison.

*Successor.* For a public prime-subgroup input D, a successful zero x = [n]D
can give its image by doubling the decoded difference and dividing by known 2n
modulo the subgroup order. A separate publicmap13 protocol executes that extension.

#text(size: 8pt)[Source files: PROTOCOL.md, source.rs, RESULTS.md, RUN.txt and hashes.sha256 in
research/hec_sourcejet13_20261115. Prior work: geometry13, difference13, transport13;
Tian arXiv:2007.03172 §§4.2–5. Standalone scientific runner is not automatically run by native CI.]
