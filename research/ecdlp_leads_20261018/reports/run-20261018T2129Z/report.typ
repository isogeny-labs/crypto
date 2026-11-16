#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#show link: underline
#align(center)[#text(size: 15pt, weight: "bold")[Run report: étale-cover search for $T_n$ over every genus-2 and genus-3 curve over $FF_2$ (lead L8)]\ run-20261018T2129Z · study `research/ecdlp_leads_20261018` · commit `483bda1b`]

*Kind.* Exact enumeration with the native Rust crate `etale_search`. It is not a timing or
statistical measurement. Host: shared cloud container (Xeon 2.80 GHz, 4 cores, rustc 1.97.0),
isolation L0. This run builds on run-20261018T2043Z, which covered genus 2 and hyperelliptic
genus 3, and adds every smooth plane quartic. With these, every genus-3 curve over $FF_2$ is
covered.

*Method.* For each curve $B$, $J_B (FF_2)$ is computed from relations: function divisors for
hyperelliptic curves; for quartics, line sections, transversal conic and cubic sections, and
quartic sections as a fallback. Accepting a conic, cubic or quartic section needs $4 deg G$ distinct
zeros in the factor base, so by Bézout every intersection is transversal and none is missed. The
curve is accepted only with free rank 1 and torsion $= L_B (1)$. Places of degree $<= 3$ generate
$"Pic"$, so this is a proof of the group (derived).

For every odd prime $n | \#J_B (FF_2)$ and every character $chi$ of $J_B (FF_2)\/n$, $L(B, chi, X)$ is
built and checked. The checks are the RH, the full functional equation, and for quartics an
out-of-sample $N_6$ prediction. $L(B, chi, X)$ is then tested for the factor
$(1 - c pi X)(1 - c overline(pi) X)$, $c in mu_n$, $pi = (t plus.minus sqrt(-7))\/2$, both in floating
point and exactly in $ZZ[zeta_n]$.

#table(columns: 4, stroke: 0.4pt + luma(170), inset: 4pt,
  [*quantity*], [*genus 2*], [*genus 3 hyperelliptic*], [*genus 3 plane quartic*],
  [smooth models], [768], [6,144], [10,920],
  [validated, odd prime dividing $\#J$], [520], [4,128], [9,152],
  [failed validation], [0], [0], [0],
  [RH / functional-equation failures], [0 / 0], [0 / 0], [0 / 0],
  [$N_6$ predictions checked / failed], [–], [–], [76,440 / 0],
  [exact vs float disagreements], [0], [0], [0],
  [without an $FF_2$-point], [0], [0], [0],
  [largest odd prime $| L(1)$; $| L(-1)$], [19; 19], [37; 37], [71; 29],
  [characters with a $T_n$ factor], [192, all $n = 3$], [576, all $n = 3$], [*672, all $n = 5$*],
)

*Enumeration check (sourced).* The model counts equal the known stack point counts times the
order of the group of coordinate changes. Genus 2: $768 = 96 dot 8$, with $\#M_2(FF_q) = q^3$.
Hyperelliptic genus 3: $6144 = 192 dot 32$, with $\#H_3(FF_q) = q^5$. Plane quartics:
$10920 = 168 dot 65$, with $\#M_3(FF_q) - \#H_3(FF_q) = q^6 + 1$ (Bergström). The quartic models
fall into 78 isomorphism classes. (Counts sourced from Bergström, J. reine angew. Math. 622 (2008) and Doc. Math. 14 (2009); not re-fetched this session.)

#figure(image("hits-by-n.svg", width: 100%), caption: [Characters tested and characters with a $T_n$ factor, per $n$ and family. Recorded values from `hits-by-n.csv`.])

*The $n = 5$ hit (measured).* All 168 hit models form one $"PGL"_3(FF_2)$-orbit. That is one
curve up to isomorphism, with trivial automorphism group:
$ B: x^3 y + x^2 z^2 + x y^2 z + x z^3 + y^3 z = 0, $
with $N = (4, 8, 19)$, $L_B (X) = 1 + X + 2X^2 + 5X^3 + 4X^4 + 4X^5 + 8X^6$ and $J_B (FF_2) tilde.equiv ZZ\/25$.

All four characters of $J_B\/5$ give the factor with $t = -1$. That is the $a = 0$ Koblitz curve
$y^2 + x y = x^3 + 1$. So, for a suitable extension of $chi$ to $"Pic"(B)$, the unramified
$ZZ\/5$ cover $D -> B$ over $FF_2$ has genus 11 and $T_5$ inside its Prym, up to isogeny (derived
from the measured factor). These are the first characters with a $T_n$ factor for any $n >= 5$ in
this search.

*Twisted covers.* If Frobenius acts on the Galois group by $lambda$ of order $k$, then $T_n$ in the
Prym forces $k <= g_0 - 1$ (derived). Such covers exist only if $n | L_B (lambda)$. So genus 2 needs
nothing beyond this search, and genus 3 adds $n | L_B (-1)$, recorded above.

#figure(image("reach.svg", width: 100%), caption: [Odd primes $n$ reachable by étale $ZZ\/n$ covers of genus-2 and genus-3 curves over $FF_2$, against the deployed Koblitz sizes and the base genus $g_0$ the L(1/2) budget allows (`cover-genus-budget.csv`, derived). Recorded values from `reach.csv`.])

*What this closes.*
- ECC2K-130 needs $g_0 <= 2$, and the Weil bound $(1 + sqrt(2))^4 approx 34 < 131$ rules that out
  (derived).
- K-163 needs $g_0 <= 3$, and no genus $<= 3$ curve over $FF_2$ has $163 | L(plus.minus 1)$
  (measured, exhaustive).
- K-233 needs $g_0 <= 3$, and $(1 + sqrt(2))^6 approx 198 < 233$ rules that out (derived).

So the étale route is closed at these three sizes. K-283, K-409 and K-571 stay open: they need
genus-4 or larger bases, with twists of order up to $g_0 - 1$.

*Not covered.* Genus $>= 4$ bases, ramified covers (L7), and non-cyclic covers. The novelty audit
is owed.

*Records.* `RUN.txt` (commit, host, SHA-256 of outputs), `genus2.jsonl`, `genus3.jsonl`,
`quartic.jsonl` (hits, classes, summaries), `hits-by-n.csv` and `reach.csv` (via `tables.awk`),
`hits-by-n.plot.awk`, `reach.plot.awk`.
