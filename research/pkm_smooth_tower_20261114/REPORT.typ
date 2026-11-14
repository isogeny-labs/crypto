#set page(paper: "a4", margin: 1.8cm)
#set text(size: 10pt)
#set heading(numbering: "1.")

= PKM factor bases from smooth maps: `pkm-smooth-tower`

2026-11-14 · isogeny-labs/crypto PR \#4091 · code at PR head ·
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
  on random points; and with class number $> 1$: every $|D| <= M$ with
  $h(D) <= H$, the class polynomial $H_D$ (`isogeny_algos::cm`) reduced
  mod $p$, its roots by Cantor–Zassenhaus, every twist certified as
  $p + 1 minus.plus t$. Choosing $p$ itself ("kind of trapdoor") is out of scope.
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

== Class number $> 1$ (Hilbert class polynomials)

`cm --p P --bound 13 --min-size 300 --max-disc 1000 --max-h 16`, recorded in
`data/cm_hilbert_p*_D1000_h16.json`; wall time 1.1–1.5 s each on this
container (measured once, unisolated).

#table(columns: 6, align: left,
  [$p$], [$D$ scanned / split], [roots], [rejected / incomplete], [CM curves], [tower],
  [1000003], [452 / 54], [262], [0 / 0], [528], [$D = -28$, $S = 1408 = 2^7 dot 11$ (as class number 1)],
  [1000033], [452 / 78], [388], [0 / 0], [782], [$D = -99$, $S = 891 = 3^4 dot 11$],
  [1000039], [452 / 48], [263], [0 / 0], [530], [$D = -124$, $S = 800 = 2^5 dot 5^2$],
)

At $p = 1000033$ and $1000039$ no class-number-1 curve fits $S >= 300$; the
larger scan finds one at each. Every split $D$ gave $h(D)$ roots and every
root a certified curve, which is the check that the rounded $H_D$ is right
for that use.

#figure(image("figures/cm_hilbert.svg", width: 100%),
  caption: [Smooth part of every CM curve order against $h(D)$ at three primes
  (recorded).])

== Checks

`cargo test --release --lib pkm_smooth_tower`: 8 passed. The tests cover:
- Vélu x-maps agreeing with the point map, and codomain orders equal to domain orders.
- Mixed Kummer and mixed isogeny towers.
- Real $m = 2$ decompositions.
- CM orders matching the generic point count, with the expected number of twists per $D$, at 4 primes up to 998244353.
- A CM curve feeding a verified tower.
- Root finding mod $p$ against planted roots (with an irreducible quadratic factor).
- The Hilbert scan at two primes: no failed polynomial, no incomplete split, no rejected root. Its orders for $h > 1$ match the generic count, and it reproduces every class-number-1 curve.

== Scope and next steps

Single-word arithmetic ($p < 2^62$); order counting is practical to about 48
bits. No ecbench plug-in, so nothing is priced against the index-calculus
baseline or rho. The class polynomial is computed over $CC$ in fixed point,
and has been run here only to $|D| <= 1000$, $h <= 16$. `cm_aux_from` is first-fit, not
best-fit. Next: an `ic_framework` factor-base plug-in so the mixed tower can
be charged in ecbench's unit.
