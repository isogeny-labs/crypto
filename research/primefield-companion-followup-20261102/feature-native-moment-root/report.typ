#set page(paper: "a4", margin: 18mm)
#set text(size: 10.5pt)
= From aggregate moments to shrinking roots
T-1793 • 2026-11-07 • PCC-20261107-ba3553 • unsigned producer evidence

The recovery state now converts an aggregate indicator to its candidate root polynomial once the count falls below a declared threshold. Subsequent signs operate in that smaller quotient. The final scalar-to-point check remains mandatory.

#image("flow.svg", width: 100%)

== Exact conversion
The aggregate's high coefficients encode the candidate power sums. Newton identities reconstruct Q(Z)=product(1−tZ); fixed-length reversal gives the monic root polynomial, including a root at zero. The candidate count m is below r, so denominators 1 through m are invertible. The empty set gives G=1. Successful conversion drops the dense coefficients and inverse array.

== Updates and evidence
A nonzero sign s at offset a updates G by gcd with (X+a)^((r−1)/2) mod G minus s; a zero sign uses X+a. Since G has distinct rational roots, this retains exactly the matching candidates. Its degree gives count and its next coefficient gives first moment.

Three selected native methods pass, with 157 imported methods filtered. There are 324 small transcript-prefix checks comparing exact root coefficients against enumerated linear-factor references and moments against the dense state. All 20 frozen public-point cases recover and verify; 178 updates execute in the root state with threshold 16. A direct-first-constraint gcd control matches every transcript count. Caps, repeated and contradictory observations, zero/empty cases and invalid inputs are checked.

Twenty-eight dependency hashes are pinned. Test/normal builds retain 16 and 26 warnings. No selected test failed. Exact source generation, logs and replay script are included. These checks share arithmetic and are not independent review.

== Costs and interpretation
Conversion performs m(m+1)/2 multiply-accumulate terms plus m scalings. Later modular powers cost O(M(m) log r), plus ordinary Euclidean gcd work. State has m+1 coefficients, with temporary buffers; no RSS measurement. Prior dense work and source-query costs remain.

Direct gcd can start from the first sign and avoid this conversion. The component is a representation bridge for already-available aggregate moments; no speed advantage over direct gcd is established. The next scaling task is to obtain the required moments without materializing the dense indicator. No timing panel, cloud job or original 4,563-case cohort rerun occurred.

== Sources
DERIVATION.md gives the proof, domain and cost details. Standard Newton identities and modular-power/gcd filtering were checked in FLINT primary documentation; the finite-field denominator conditions are proved here. Existing aggregate and polynomial code reused; KB unavailable. No novelty claim.
