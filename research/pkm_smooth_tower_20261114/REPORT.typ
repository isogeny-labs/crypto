#set page(paper: "a4", margin: 1.8cm)
#set text(size: 10pt)
#set heading(numbering: "1.")

= PKM factor bases from smooth maps: `pkm-smooth-tower`

2026-11-14 · isogeny-labs/crypto PR \#4091 · code at commit `a894bbdaa9` ·
status: construction and checking tooling; tests pass, nothing benchmarked
[pcc:unverified].

== What was built

Petit–Kosters–Messeng (PKC 2016, slides 12–17) take a factor base
$cal(F) = {(x, y) in E(FF_p) : x in V}$ with $V = {x : L(x) = c}$, where
$L = L_(n') compose dots.c compose L_1$ is a composition of low-degree maps and
$|V| approx product deg L_j approx p^(1\/m)$. Until now the repository built
$V$ from degree-2 maps only (`ic_framework::prime_tower`). The module
`src/cryptanalysis/pkm_smooth_tower.rs` and the CLI `pkm-smooth-tower` accept
any ordered list of small primes $q_1, dots, q_(n')$, with $S = product q_j$:

- *Kummer form* (slide 14): $L_j (x) = x^(q_j)$ when $S | p - 1$; $V = g dot mu_S$.
- *Isogeny form* (slide 15): an auxiliary $E'$ with cyclic $G = ⟨ Q ⟩$
  of order $S$. Each $L_j$ is the x-map $N_j \/ D_j$ of a Vélu isogeny of
  degree $q_j$, of either parity. $V = x(R' + G)$ with $2R' ∉ G$, and $c = x(Phi(R'))$.
- *Method 1* (slide 16): random curves until $\#E'$ has a $B$-smooth cyclic
  part $S$; the number of curves tried is recorded.
- *Method 2, CM* (slide 16), for the *given* $p$: the 13 class-number-1
  orders, Cornacchia for $4p = t^2 + |D| v^2$, every twist, orders certified
  on random points. Choosing $p$ itself ("kind of trapdoor") is out of scope.
- *Checks*: $|V| = S$ distinct values, $L(v) = c$ on $V$, and below $2^22$
  no other root in $FF_p$. *System* (slide 17): msolve export with
  $S_(m+1)$ written as an $S_3$ chain; an exact $m = 2$ finder checks every
  relation by point addition.

#figure(image("figures/pipeline.svg", width: 100%),
  caption: [The construction as implemented, with the recorded CM example
  (`data/cm_p1000003.json`): a chain of seven 2-isogenies and one
  11-isogeny on a $D = -28$ curve, $S = 1408$.])

== Recorded examples (measured on this container, no isolation)

#table(columns: 4, align: left,
  [command], [route], [result], [record],
  [`kummer --p 20071 --degrees 3,2,5,3`], [Kummer], [$S = 90$, verified], [`data/kummer_p20071.json`],
  [`isogeny --p 1000003 --bound 13 --min-size 500 --seed 2`], [Method 1], [$S = 720 = 2^4 dot 3^2 dot 5$, 39 curves tried, verified], [`data/isogeny_p1000003.json`],
  [`cm --p 1000003 --bound 13 --min-size 300`], [Method 2], [$D = -28$, $S = 1408 = 2^7 dot 11$, verified], [`data/cm_p1000003.json`],
  [`search --p 1000003 --bound 13 --min-size 500`, seeds 1–20], [Method 1], [curves tried: mean 17.4, min 1, max 68], [`data/method1_p1000003_seeds1-20.json`],
)

#figure(image("figures/cm_orders.svg", width: 92%),
  caption: [Method 2 at $p = 1000003$: the 22 class-number-1 CM curves and
  the 13-smooth part of each order (recorded). Three reach $S >= 300$.
  `cm_aux` returned the $D = -28$ curve (order 998272). The $D = -7$ curve
  of the same order and the $D = -8$ curve with the largest smooth part were
  passed over. `cm_aux` takes the first fit, not the largest, and why it
  skipped $D = -7$ is not established here.])

#figure(image("figures/method1_trials.svg", width: 70%),
  caption: [Method 1 at $p = 1000003$, $B = 13$, $S >= 500$: curves tried
  per seed, $n = 20$ (recorded).])

== Checks

`cargo test --release --lib pkm_smooth_tower`: 6 passed. The tests cover:
- Vélu x-maps agreeing with the point map, and codomain orders equal to domain orders.
- Mixed Kummer and mixed isogeny towers.
- Real $m = 2$ decompositions.
- CM orders matching the generic point count, with the expected number of twists per $D$, at 4 primes up to 998244353.
- A CM curve feeding a verified tower.

== Scope and next steps

Single-word arithmetic ($p < 2^62$); order counting is practical to about 48
bits. No ecbench plug-in, so nothing is priced against the index-calculus
baseline or rho. Next: CM with class number $> 1$ from
`isogeny_algos::cm::hilbert_class_poly` roots mod $p$; an `ic_framework`
factor-base plug-in so the mixed tower can be charged in ecbench's unit.
