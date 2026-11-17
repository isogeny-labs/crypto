#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)

= curve_traits factor hints: factordb and a CADO-NFS C112

2026-10-28. Inputs: `docs/curves/factor-hints.txt`; output `docs/curves/traits.json`.

== What changed

18 primes were added to the factor hints. 16 come from factordb's factorisations of composites that
`curve_traits` leaves unsplit. The other 2 are the factors of a 112-digit cofactor of a Frobenius
discriminant, which CADO-NFS split here (25,084 CPU-seconds). Those factors were reported to factordb,
which now lists the number as fully factored. PARI's `isprime` proves every added prime.

A hint is only a trial divisor that the build re-checks, so it can complete a factorisation but
cannot change what is proved.

== Effect

Against `main` at the same inputs except the hints:

- 8 curves go from `End(E)` unknown to probable: over GF(2^431) and GF(2^571), and prime fields of
  384, 414, 510, 511 (two) and 521 bits.
- Curves whose `Δ` stayed unfactored: 14 → 6.
- No volcano level and no bound got worse (0 regressions).

== Still open

- 6 curves (5 distinct discriminants) whose `Δ` has an unsplit 125-, 146-, 151-, 152- or 153-digit
  composite. factordb has none of them split. They need a large NFS run or a lucky ECM factor.
- 17 newer curves over 47- and 53-bit prime fields, added to the registry after the last level work,
  have open volcano levels at prime powers. This PR does not touch them.

#figure(image("composites.svg", width: 100%), caption: [The 34 composites checked against factordb.])
