#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Nonlinear coefficient-domain construction
2026-11-05 · T-1616

A native constructor now builds cubic character features as polynomials modulo X^r-X and solves directly for the coefficient vector of X. At r=479, it produces a checked 478-term packet using 16 source characters.

#image("pipeline.svg", width: 100%)

== Identity and representation
Euler's criterion represents each character by (X+a)^((r-1)/2). Products reduced modulo X^r-X are exact polynomial functions. The constructor recomposes its solution to X coefficient by coefficient. The quotient is a split function ring, not an extension field. An invertible evaluation matrix relates this system to the existing public value-domain system: the basis change alone does not compress it.

== Checks and cost
All 47 optimized native tests pass with overflow checks. The 15-shift control returns outside-span with observed rank 477; the 16-shift system has observed rank 479. Its augmented matrix contains 334342 field entries and records 56196286 inner elimination updates, excluding all other construction and source work. The original columns are retained separately. No runtime gain is asserted.

An independent executable imports no producer code. It reads the saved packet, expands ordinary polynomials, reduces only the final sum, and checks the exact coefficient vector X. It also checks all 479 field values and rejects a saved one-coefficient corruption. Eight public-point source integrations pass. Eighteen producer and three checker/input hashes bind the runs; four compiler warnings are preserved.

== Derived next construction
For a degree-at-most-three polynomial g with nonzero constant and h=(r-1)/2, the standard Miller recurrence from gF'=h g'F computes coefficients of F=g^h except at index r. The identity gF²=g(X^r) supplies that coefficient from earlier ones, then the recurrence continues. Zero shifts factor out first. This gives a proposed linear-work column builder; it is derived, not yet implemented or timed. Dense elimination remains separate.

== Evidence
DERIVATION.md, NEXT_RECURRENCE.md, lib.rs, check.rs, program.tsv, receipts and logs. No cloud experiment or timing panel ran; original4563-case cohort unchanged. Background: #link("https://doc.sagemath.org/html/en/tutorial/tour_linalg.html")[SageMath exact linear systems] and #link("https://fredrikj.net/fdoc/html/fmpz_poly/_fmpz_poly_pow_multinomial.html")[FLINT Miller recurrence documentation]. The derivation handles characteristic-r division explicitly.
