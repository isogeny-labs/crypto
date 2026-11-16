#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Stream mode coefficients into recovery
T-1853 · 2026-11-07 · Parameter-only prime-field mode representation

StreamModes stores r, m, zeta and a chunk cap. It generates selected signed-binomial coefficients in descending order, instead of compiling their full vectors. The first constraint is filled directly; later modes feed retained quotient-ring arithmetic, and phase extraction uses streamed scalar Horner. Public source and traced control flow are preserved.

#image("flow.svg",width:100%)

*Recurrence.* For h=(r−1)/2 and c_N=(−1)^(h−N) binom(h,N), descending one selected coefficient multiplies by (−1)^m times the product of N−i, i=0..m−1, divided by the product of h−N+i, i=1..m. All factors lie in 1..h and are nonzero modulo prime r. The highest coefficient starts from the complementary binomial index q smaller than m. Bounded prefix/backward inversion supplies the ratios, retaining only the next coefficient between chunks.

*Integration proof.* The recurrence equals the original coefficient definition. Chunk Horner then produces exactly F_e(tZ) mod G. Quotient-ring projection and gcd invariance preserve both constraints; scalar Horner preserves phase evaluation. Unit and constant-mode cases retain their original meaning. DERIVATION.md gives the invariant and formulas.

#table(columns:(2.5fr,1fr,2fr),inset:4pt,
 [Native check],[Count],[Result],
 [Coefficient sequences],[476],[5,632 equal coefficients],
 [Scaled residue comparisons],[4,176],[Exact equality],
 [Scalar evaluations],[1,392],[Includes zero argument],
 [Sequential state comparisons],[42,324],[Same state after each radius],
 [Streamed public recovery],[617],[5,929 calls; zero unresolved])

*Storage and arithmetic.* The native 64-bit mode object is 32 bytes. First C0 remains dense; later coefficient buffers are bounded by the chunk cap (or retained degree if smaller). Each coefficient transition uses m numerator and m denominator factors plus batch inversion/update work. Full mode generation has O(r) direct factor work and repeats on use. Degree-one residues may require an inversion per transition. This is a storage representation improvement with an explicit arithmetic tradeoff, not a timing result.

*Evidence.* validation01 receipt pins 45 transitive inputs and stdout; overflow checks enabled. Inherited controls pass. No native or report failure. PCC-20261107-d8b908 is an unsigned producer statement. No cloud run, timing measurement or full Cargo rerun; original 4,563-case cohort and trace protocol unchanged. The large r=2^61−1 check constructs only the parameter object, without coefficient generation or public recovery.

*Next construction and sources.* Replace direct stride-factor products with a faster bounded-storage recurrence evaluator. Exact predecessor source, NIST DLMF26.3 factorial/symmetry identities and FLINT modular-polynomial documentation were reviewed; KB unavailable. These are standard algebraic building blocks, with no novelty claim. Prior and canonical graphs remain unchanged.
