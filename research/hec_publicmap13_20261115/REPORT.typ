#set page(paper: "a4", margin: 18mm)
#set text(font: "Helvetica", size: 10pt)
#text(size: 21pt, weight: "bold")[Explicit public-divisor transport]
#v(6pt)
Genus 3 · same-field quotient to Jac(Q1) · 15 November 2026

All six public prime-subgroup inputs mapped to nonzero target divisor classes.
Addition, doubling, negation, identity and a withheld scalar-control identity
passed. The mapper receives the public divisor and logging label; no input
logarithm, expected image or label-dependent branch is used.

#image("TRIALS.svg", width: 100%)

*Construction.* Find a supported section zero x = [n]D. Actual source jets decode
Q = εψ([n]D − T), with one fixed global sign ε. Since 2T = 0,
M(D) = [(2n)⁻¹ mod ℓ]([2]Q) = εψ(D). Here ℓ = 1,028,893 is coprime to the
isogeny degree 27. The global differential scale 10 was fixed by sourcejet13.

*Retained failure.* V1 stopped before mapping because its chosen B = 2(0,44)
was exactly D1. Original stdout/stderr are retained. A separately committed V2
protocol changed B to twice the second canonical place, here (3,21); no retry
or alternate fixture was allowed. V2 passed in one run, exit 0, empty stderr.

*Evidence boundary.* Trials are exact scan indices, not timings. Inputs are
related, and B/2B share a selected difference point. The separate sum and scalar
controls use different points. No universal scan-success bound or independent
implementation is asserted. This is concrete transport on six executed inputs,
with an algebraic identity for every successful prime-subgroup input.

*Next cost case.* Integrate destination index calculus; charge setup, calibration,
scan exclusions, source jets, destination arithmetic, solve and verification.
Complete-cost 10× evaluation and genus 2/4 constructions remain open.

#text(size: 8pt)[V2 source 9ef29556c435; pre-run checkpoint 20261115T163443Z. Raw RUN_V2.txt;
V1 raw RUN.txt/RUN.stderr. PROOF.md and both protocols retained. Standalone scientific
runner is not automatically executed by native CI.]
