# Reciprocal arithmetic candidate

After the sparse-pivot change, reuse a modulus context in dependency
elimination, modular exponentiation and cached square roots. For canonical
residues modulo 2 <= p < 2^32, cache R=ceil(2^64/p). For x=a*b, the high word
of x*R estimates floor(x/p) exactly or one too large: its real approximation
error is nonnegative and strictly below one because x<2^64. Thus the wrapped
residual is in [0,p) or [2^64-p,2^64), and one conditional wrapping addition of
p restores the canonical remainder. Larger moduli use the original u128
remainder. No Montgomery-domain conversions or input restrictions are added.

Literature reviewed: [HAC Chapter 14, section 14.3.3](https://cacr.uwaterloo.ca/hac/about/chap14.pdf),
Algorithm 14.42 and Notes 14.43–14.45. The textbook uses a floor reciprocal
and up to two corrections; this code uses the separately bounded ceiling
variant above. This is standard reciprocal arithmetic, with no novelty claim.
Precomputation remains inside the charged solve/replay interval.

Validation: 26 standalone release tests and locked all-target strict Clippy
pass. Products are checked against independent u128 remainder for every
residue pair of every modulus 2 through 255, boundary operands at ten larger
moduli (including composite/even moduli and u64::MAX), and 10,000 random pairs
per larger modulus. Existing dense reference dependency, wide-field curve,
root and archived certificate tests also pass. An initial compile identified
two overly broad textual replacements; these were corrected before the
successful full validation. No new timing result exists for this candidate.

Next measurement must retain baseline 2bd4588fba60, all three fixtures, full
setup/replay/serialization intervals and the existing A/A and batch-1 controls.
Sparse tails and reciprocal arithmetic form a bundled candidate; do not
attribute a panel-level difference to either change alone.
