# Conductor-gap study: status and open leads (2026-10-19)

This page records the state of the conductor-gap / fiber-aware index-calculus
study after six rounds, and the leads round 7 tests
([`PROTOCOL.md`](PROTOCOL.md)).

## Rounds

| round | question | headline | PR | state |
|:--|:--|:--|:--|:--|
| 1 | Does a large conductor gap change what IC measures? | No. Fiber-closed tables cut relation cost but not whole cost. Best cell 2.0× rho (F4, F_2^23). | merged | merged |
| 2 | Partial fibers, crater automorphisms | `[d]E[h]` beats the whole fiber by up to 79×. At matched subgroup size, crater and floor agree within 1–3 %. | merged | merged |
| 3 | Two-torsion summation polynomials; combined descent | Quotient systems cost what their degree predicts. The combined descent costs 234–529× blind F4. | #1673 | merged |
| 4 | Fiber-closed IC against rho across 18–30 bits | IC/rho grows as #E^(1/6). No crossover. | #1685 | merged |
| 5 | The cofactor penalty | Clearing the cofactor beats fiber closure by about h^(1/3). Best cell 7.29× rho (study best). Trace-kernel binary base. | #2185 | open: CI red only on checks red on main (`replay`, `Cover transfer assessment`, `certificates`; CodeQL timeouts) |
| 6 | Early-stopping F4; trace coset on every Koblitz curve | Early stop gains −37 % to +43 %, bounded by the solvable share. Coset yield = 1/f₂. Coset MITM wins at F_2^19 and F_2^23. Best cell 12.7× rho. | #2205 | open: same CI picture as #2185; contains #2185 |

**Distance remaining.**
- The best IC cell in the study is 7.29× its matched rho: prime field,
  22 bits, h = 72, cofactor-cleared MITM.
- On binary curves the best is 12.7× (coset MITM, F_2^23).
- Every measured exponent leaves IC/rho rising with size.

**Repository state found along the way.**
- **Registry.** Main's committed curve registry (2,572 curves) is stale
  against its own tree.  `build_curve_registry.py` yields 8,031, adding
  the volcano census and volcano-cost studies.  #2185 and #2205 carry the
  regenerated registry and the files derived from it.
- **CI checks red on main**, unrelated to this study:
  - `replay` fails on `cargo build --locked` (the lock file is out of
    date).
  - `Cover transfer assessment` and `certificates` hard-code 2,100 rows
    and 134 generators.
  - CodeQL's Rust warm-up times out.

## Leads tested in round 7

Ranked in round 6 ([README](../isogeny_conductor_gap_round6_20261018/README.md)):

1. **Early-stop threshold.**  `f4-f2-early` saves up to the solvable share
   of the work but loses where few systems are solvable, because the 2⁸
   enumeration outweighs letting F4 reach 1.  A smaller `free` (0, 2, 4)
   should remove the loss.
2. **A 2E base that keeps the default's solving degree.**  The trace coset
   at dimension d spans `⟨1, …, z^d⟩ ∩ {Tr(x) = Tr(a)}`, one degree above
   the default's `⟨1, …, z^{d−1}⟩`.  The 2E half of the default span is
   the coset at d − 1: same top degree as the default at d, half the
   columns, all points in 2E.
3. **Round 5's F_2^19 yield, 1.48× against 1/f₂ = 2.19.**  `ecbench`
   counts relations per trial (at most one per target).  E8 counted every
   lifted decomposition (several per target).  The hit rate, not the
   decomposition count, may be what saturates.
