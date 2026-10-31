#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)

= curve_traits: the last 17 open End(E) levels, by descending isogeny walks

2026-10-28. Code: `src/cryptanalysis/curve_traits/walk.rs` (`height_above_floor`) and `mod.rs`
(`settle_by_descent`). Output: `docs/curves/traits.json`.

== The open levels

17 registry curves over 47- and 53-bit prime fields, added after the earlier level work, had one open
volcano level each:

- the 2-volcano for $d_K = -3, -4, -7$, depth 20 or 24;
- the 3-volcano for $d_K = -3$, depth 12.

The prime-power scalar test and the horizontal-isogeny cycles had bounded each level at 1 or 3.

These bounds could not be closed by the cycles. A cycle of two 11-isogenies fits both level 2
($h(cal(O)_4) = 2$) and level 3 ($h(cal(O)_8) = 4$), since the class above 11 has order 2 in both.

== Method

With $ell | v$, every vertex of the $ell$-volcano above the floor has $ell + 1$ rational
$ell$-isogenies, and a floor vertex has exactly one. Below the crater one edge goes up and $ell$ go down.
So a walk on $j$-invariants that never steps back along the edge it arrived by keeps descending once it
has.

From each rational neighbour of $j(E)$ (the roots of $Phi_ell(j, Y)$ in $"GF"(p)$), the walk runs until it
reaches a vertex with a single rational $ell$-isogeny. The shortest such walk has length exactly the height
$h$ above the floor, and the level is $"depth" - h$ (Kohel; Fouquet–Morain). $Phi_2$ was added to the
committed modular polynomials. A level above the bound the other tests reached stops the build.

== Checks

- *Unit test.* Heights match a breadth-first search of the $Phi_ell$ graph in PARI/GP (`bfs.gp`,
  `gen.gp`, `vectors.txt`) on 40 random curves over 17- to 20-bit prime fields, covering $ell = 2, 3, 5$
  and levels 0 to 6.
- *The 17 curves.* An independent PARI/GP implementation of the walks (`walk.gp`, `pari_levels.txt`)
  gives the same 17 levels (`levels.json`).
- *Consistency with the cycles.* Every level is within the earlier bound. The class order of the prime
  above 11 in $"Cl"(cal(O)_f)$ agrees with each closed 11-isogeny cycle.

== Result

#table(columns: 3, [], [before], [after],
  [End(E) proved], [19,876], [19,893],
  [End(E) bounded (open levels)], [17], [0])

Only the 17 records change, with 0 regressions. No volcano level is open in the catalogue.

#figure(image("levels.svg", width: 100%), caption: [Bound before and level after, per open level.])
