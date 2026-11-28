# Guarded exact M31 constructor multiplication

Sources checked (KB): unavailable (wrapper has no executable). Primary review:
J. Solinas, Generalized Mersenne Numbers, CACR99-39, Introduction PDFpage2,
https://cacr.uwaterloo.ca/techreports/1999/corr99-39.pdf . Read rendered page;
embedded text extraction is corrupt. Existing replay acceptance derivation and
Rust consumer obligations remain unchanged. No novelty claim.

For M=2^31-1 and0<=a,b<=M, x=a*b<=M^2<2^62. Write x=2^31*h+l,
0<=l<=M,0<=h<M. Thus s=h+l<2M and x congruent s modulo M.
One conditional subtraction produces the unique residue in[0,M). The u64
product and sum cannot overflow. Guard p==M and(a|b)<=M; every other case
keeps the original u128 multiplication/remainder, including wide primes and
noncanonical operands. This note supplies a derivation, not a formal proof.
[pcc:unverified]

Hypothesis: avoiding general128bit division may lower paid construction and
proof completion cost. No speed is predicted. Do not change the separate
consumer's arithmetic, constructor checks, schema, proofs, file/resource caps,
old binaries, rejected A/A transcript or original reserved fresh seeds.

Before performance: differential boundary/fallback tests plus fixed65536 pairs
against the original u128 expression; all affected crate release tests under
default/scale/counter features and strict clippy; actual-root native worker
and public API tests; full native compact/expanded/sparse certificate audit on
already published n3,n8,n16 fixtures against frozen prior workers and fresh
Singular. Correctness clocks provide no speed result. Freeze/copy/hash the
executables and publish source/evidence before a separately registered paid
comparison with complete original answers and all checking/preparation paid.
