#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#show link: underline
#align(center)[#text(size: 15pt, weight: "bold")[Run report: étale-cover search for $T_n$ (lead L8)]\ run-20261018T2043Z · study `research/ecdlp_leads_20261018` · commit `bd694bf2`]

*Kind.* Exact enumeration with the native Rust crate `etale_search`. Not a timing or statistical
measurement. Host: shared cloud container (Xeon 2.80 GHz, rustc 1.97.0), isolation L0.

*What ran.* Every smooth curve $B: y^2 + h(x) y = f(x)$ over $FF_2$ of genus 2, and every smooth
hyperelliptic curve of genus 3. For each, $J_B(FF_2)$ was computed from relations among places of
degree at most 4, validated by free rank 1 and torsion $= L_B (1)$. Then, for every odd prime
$n | \#J_B(FF_2)$ and every character $chi$ of $J_B(FF_2)\/n$, $L(B, chi, X)$ was built and tested
for the factor $(1 - c pi X)(1 - c overline(pi) X)$, with $c in mu_n$ and $pi = (t plus.minus i sqrt(7))\/2$,
$t = plus.minus 1$.

#table(columns: 3, stroke: 0.4pt + luma(170), inset: 4pt,
  [*quantity*], [*genus 2*], [*genus 3 (hyperelliptic)*],
  [smooth models], [768], [6,144],
  [validated, with an odd prime dividing $\#J$], [520], [4,128],
  [failed validation], [0], [0],
  [failed the RH check], [0], [0],
  [largest $\#J_B(FF_2)$ among them], [19], [56],
  [characters with a $T_n$ factor], [192, all $n = 3$], [576, all $n = 3$],
)

#figure(image("hits-by-n.svg", width: 100%), caption: [Characters tested and characters with a $T_n$ factor, per $n$. Recorded values from `hits-by-n.csv`.])

Hits occur in six isogeny classes of $B$, three in genus 2 and three in genus 3 (`hit-isogeny-classes.csv`).
They cover both $a = 1$ and $a = 0$. No character has a $T_n$ factor for any $n >= 5$.

*Not covered.* Plane-quartic genus-3 curves, which are needed for $\#J >= 57$ and so for
$n = 131, 163$. Genus 4 and above. Ramified covers.

*Records.* `RUN.txt` (commit, host, SHA-256 of outputs), `genus2.jsonl`, `genus3.jsonl`,
`hits-by-n.csv`, `hit-isogeny-classes.csv`, `hits-by-n.plot.awk`.
