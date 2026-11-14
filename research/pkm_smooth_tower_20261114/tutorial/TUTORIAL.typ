#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10.5pt)
#set heading(numbering: "1.")
#set par(justify: true)
#show raw.where(block: true): set block(fill: luma(245), inset: 7pt, radius: 3pt, width: 100%)

#let note(body) = block(fill: rgb("#eef3fb"), inset: 8pt, radius: 3pt, width: 100%, body)
#let check(body) = block(fill: rgb("#f3f8ec"), inset: 8pt, radius: 3pt, width: 100%, [*Check by hand.* ] + body)

#align(center)[
  #text(size: 17pt, weight: "bold")[Index calculus on prime-field elliptic curves \ with smooth maps: a worked tutorial]
  #v(2pt)
  Petit–Kosters–Messeng (PKC 2016), slides 12–17, and the `pkm-smooth-tower` tool \
  isogeny-labs/crypto · `research/pkm_smooth_tower_20261114/tutorial/` · 2026-11-14
]

#note[
  *How to read this.* Each idea is followed by a tiny example over $FF_31$ or
  $FF_101$ that you can check with pencil and paper. Every example number was
  produced by `pkm-smooth-tower` and is saved in `tutorial/data/`. The
  hand checks show where it comes from. This is a teaching document. It
  makes no claim about how fast the attack is [pcc:unverified], and §9 says
  what is and is not known.
]

= The problem: index calculus needs a factor base

Let $E: y^2 = x^3 + a x + b$ over $FF_p$, $P$ a point of prime order $n$, and
$Q = [k]P$. The *ECDLP* asks for $k$. Generic algorithms (Pollard rho) take
about $sqrt(n)$ group operations.

*Index calculus* tries to do better in three steps:

+ Pick a *factor base* $cal(F) subset E(FF_p)$ of "small" points.
+ Collect *relations*. For random $a, b$, try to write
  $R = [a]P + [b]Q = plus.minus F_1 plus.minus dots.c plus.minus F_m$ with every $F_i in cal(F)$.
  This is called *decomposing* $R$.
+ With about $|cal(F)|$ relations, linear algebra modulo $n$ eliminates the
  $F_i$ and gives $k$.

Over finite fields $FF_q^*$, "small" means "a product of small primes".
Points on an elliptic curve have no such notion. Semaev's idea (2004) was to
make "small" mean *the $x$-coordinate lies in a special set $V$*:
$ cal(F) = {(x, y) in E(FF_p) : x in V}. $
Over an extension field $FF_(q^n)$ one takes $V = FF_q$, the subfield, and
Weil descent turns a decomposition into a polynomial system over $FF_q$. A
prime field $FF_p$ has no subfield, so it is not clear what $V$ should be.

*PKM's answer:* let $V$ be the set of roots in $FF_p$ of a *composition of
low-degree maps*:
$ V = {x in FF_p : L_(n')(dots.c L_2(L_1(x)) dots.c) = c}, quad |V| approx product_j deg L_j. $

= Summation polynomials

Semaev's summation polynomials turn "these points add up to $O$" into an
equation in their $x$-coordinates alone. For three points,
$ S_3(x_1, x_2, x_3) = (x_1 - x_2)^2 x_3^2 - 2 ((x_1 + x_2)(x_1 x_2 + a) + 2b) x_3 + (x_1 x_2 - a)^2 - 4b(x_1 + x_2). $
*Fact:* $S_3(x_1, x_2, x_3) = 0$ exactly when there are signs with
$plus.minus P_1 plus.minus P_2 plus.minus P_3 = O$ for some points $P_i$
(over the algebraic closure) with $x(P_i) = x_i$. Read it as "$x_3$ is the
$x$-coordinate of $P_1 + P_2$ or of $P_1 - P_2$". For fixed $x_1, x_2$ it is a
quadratic in $x_3$, and its two roots are those two abscissae.

Larger ones are built by resultants:
$S_(m+1)(x_1, dots, x_m, X) = "Res"_U (S_m (x_1, dots, x_(m-1), U), S_3(x_m, U, X))$,
of degree $2^(m-1)$ in each variable. They grow fast. The tool never forms
them. It uses the equivalent *chain* with $m - 2$ helper unknowns $u_k$
(the abscissae of the partial sums):
$ S_3(x_1, x_2, u_1) = 0, quad S_3(u_1, x_3, u_2) = 0, quad dots, quad S_3(u_(m-2), x_m, X) = 0. $

*Decomposing* $R$ then means: find $x_1, dots, x_m in V$ with
$S_(m+1)(x_1, dots, x_m, x(R)) = 0$, then lift each $x_i$ to a point and
fix the signs.

*How big should $V$ be?* Each $x in V$ lifts to a curve point about half the
time, and then to two points $plus.minus$. So $|cal(F)| approx |V|$. There are
about $|cal(F)|^m \/ m!$ sums of $m$ factor-base points, and the group has
about $p$ elements. For a random $R$ to decompose with reasonable
probability, take $|V| approx p^(1\/m)$. That is the "$approx p^(1\/m)$" on slides 12 and 14.

= Why a composition of maps?

You could take $V$ = the roots of one random polynomial of degree
$S approx p^(1\/m)$. But then the equation "$x in V$" has degree $S$, which is
enormous. PKM instead factor the membership test into $n'$ small steps:
$ x_(i,2) = L_1(x_(i,1)), quad x_(i,3) = L_2(x_(i,2)), quad dots, quad L_(n')(x_(i,n')) = c. $
Each equation has degree only $q_j = deg L_j$, at the price of $n'$ new
unknowns per point. Their *product* of degrees is the size of $V$:
$|V| = product q_j$. For this to work, every step must be *exactly*
$q_j$-to-1 on the values that matter, with all preimages in $FF_p$. That is the
point of the two constructions below.

#figure(image("figures/preimage_tree.svg", width: 100%),
  caption: [$V$ is the bottom row of a tree of preimages. Each level is one
  map; a map of degree $q$ has $q$ preimages per node, so $|V| = 2 dot 3 = 6$.
  Left: the Kummer tower of §4. Right: the isogeny tower of §5 (recorded data;
  its middle level is not labelled).])

= Construction 1: the Kummer tower ("$p - 1$ smooth", slide 14)

If $S = q_1 q_2 dots.c q_(n')$ divides $p - 1$, then $FF_p^*$ (cyclic of
order $p - 1$) contains the group $mu_S$ of $S$-th roots of unity. Take
$ L_j (x) = x^(q_j), quad c = g^S, quad V = g dot mu_S = {x : x^S = g^S}. $
Each step $x arrow.r.bar x^q$ is exactly $q$-to-1 on the relevant coset,
because $mu_q subset mu_S$.

*Example* (`data/kummer_p31.json`). $p = 31$, $p - 1 = 30$, degrees
$(2, 3)$, $S = 6$, $g = 1$. The tool finds $zeta = 6$ and
$V = {1, 6, 5, 30, 25, 26} = mu_6$.

#check[
  $6^2 = 36 equiv 5$ and $6^3 = 6 dot 5 = 30 equiv -1$, so $6$ has order
  exactly 6. The first map squares: $1^2 = 30^2 = 1$, $6^2 = 25^2 = 5$,
  $5^2 = 26^2 = 25$. That gives the middle row ${1, 5, 25}$ of the tree, each
  value hit twice. The second map cubes: $5^3 = 125 = 4 dot 31 + 1 equiv 1$ and
  $25^3 = 5^6 equiv 1$. Everything lands on $c = 1$.
]

*Limitation.* This needs a large smooth factor of $p - 1$. For a given
prime that factor might not exist, which leads to the next construction.

= Construction 2: the isogeny tower (slide 15)

Replace the group $FF_p^*$ (order $p - 1$) by the group of an *auxiliary
curve* $E' \/ FF_p$ (order $p + 1 - t$ for some $|t| <= 2 sqrt(p)$). There are many
such curves, so many group orders to choose from.

== The idea

Suppose $E'(FF_p)$ has a cyclic subgroup $G = ⟨ Q ⟩$ of order $S$.
There is an isogeny $phi: E' arrow.r E''$ with kernel exactly $G$.
Pick a point $R'$ with $2R' ∉ G$ and set
$ V = x(R' + G) = {x(R' + [k]Q) : k = 0, dots, S - 1}, quad c = x(phi(R')). $

*Why this is the zero set of $x(phi(dot)) = c$.* For $g in G$,
$phi(R' + g) = phi(R') + phi(g) = phi(R')$, so every point of $R' + G$ maps to
abscissa $c$. Conversely, if $x(phi(P)) = c$ then $phi(P) = plus.minus phi(R')$, so
$P in R' + G$ or $P in -R' + G$. Both give the same abscissae, since
$x(-P) = x(P)$. *Why $|V| = S$.* Two points of $R' + G$ share an abscissa only
if one is minus the other: $R' + g = -(R' + g')$, i.e. $2R' in G$, which we excluded.

== Splitting $phi$ into small steps

Because $G$ is cyclic of order $S = q_1 dots.c q_(n')$, $phi$ factors into
isogenies of prime degree:
$phi = phi_(n') compose dots.c compose phi_1$. Step $j$ has kernel generated by
$[S_j \/ q_j] Q_j$, where $Q_1 = Q$, $Q_(j+1) = phi_j (Q_j)$ and
$S_j = q_j dots.c q_(n')$. Each $L_j$ is the $x$-part of $phi_j$, a rational
function $N_j \/ D_j$ of degree $q_j$, given by *Vélu's formula*:
$ L(x) = x + sum_(T) (v_T / (x - x_T) + u_T / (x - x_T)^2), $
summing over one point $T$ from each pair $plus.minus T$ of the nonzero kernel points, with
$g_T = 3 x_T^2 + a$; $v_T = 2 g_T$ and $u_T = 4 y_T^2$, except for a
2-torsion point ($y_T = 0$), where $v_T = g_T$ and $u_T = 0$. The image curve is
$y^2 = x^3 + (a - 5v)x + (b - 7w)$ with $v = sum v_T$ and $w = sum (u_T + x_T v_T)$.

== Worked example at $p = 101$ (`data/isogeny_p101_x50.json`)

Method 1 (§6) found, after 5 random curves,
$ E': y^2 = x^3 + 66x + 89, quad \#E'(FF_101) = 90 = 2 dot 3^2 dot 5, $
with $Q = (76, 40)$ of order $S = 6$ and degrees $(2, 3)$. The tool reports
$ L_1 (x) = (x^2 + 63x + 55)/(x + 63), quad L_2 (x) = (x^3 + 79x^2 + 25x + 24)/(x^2 + 79x + 20), $
$R' = (60, 77)$, $V = {60, 82, 19, 91, 14, 67}$ and $c = 96$. The middle curve is
$E'_2 : y^2 = x^3 + 94x + 3$ and the last is $E'_3 : y^2 = x^3 + 69x + 75$.

#check[
  *Step 1* (degree 2). The denominator $x + 63$ says the kernel point has
  $x_T = -63 equiv 38$. Then $g_T = 3 dot 38^2 + 66 = 3 dot 30 + 66 = 156 equiv 55$,
  so $L_1 = x + 55\/(x - 38) = (x^2 + 63x + 55)\/(x + 63)$. The codomain has
  $a = 66 - 5 dot 55 = -209 equiv 94$, and $w = 38 dot 55 equiv 70$ gives
  $b = 89 - 7 dot 70 = -401 equiv 3$. ✓

  *Step 2* (degree 3). The denominator is $(x - 11)^2 = x^2 - 22x + 121 equiv x^2 + 79x + 20$,
  so $x_T = 11$ on $E'_2$. Here $y_T^2 = 11^3 + 94 dot 11 + 3 equiv 45$, so
  $v_T = 2(3 dot 121 + 94) equiv 5$ and $u_T = 4 dot 45 equiv 79$. Then
  $x (x - 11)^2 + 5(x - 11) + 79 = x^3 + 79x^2 + 25x + 24$. ✓ The
  codomain has $a = 94 - 25 = 69$ and $b = 3 - 7(79 + 11 dot 5) equiv 75$. ✓
]

Note how $V$ pairs up under $L_1$: $R' + [k]Q$ and $R' + [k + 3]Q$ differ by
the order-2 kernel point $[3]Q$. So $V_0, V_3$ share an image, and so do
$V_1, V_4$ and $V_2, V_5$ (figure above, right).

= Finding the auxiliary curve (slide 16)

The isogeny tower needs $E'$ with a large *smooth* cyclic subgroup.

*Method 1: random curves.* Draw random $(a', b')$, count points, factor the
order, and keep the curve if its $B$-smooth part reaches the target size.
The tool records how many curves it tried. At $p = 1000003$, $B = 13$,
$S >= 500$, twenty seeds needed 17.4 curves on average (min 1, max 68;
`../data/method1_p1000003_seeds1-20.json`).

*Method 2: complex multiplication (CM).* Some curves have their order known
in advance. Write
$ 4p = t^2 + |D| v^2 $
with $D < 0$ a discriminant. Cornacchia's algorithm finds $(t, v)$, or proves
there is none, in about $log p$ steps. If there is a solution, the curves with
CM by the order of discriminant $D$ have trace $plus.minus t$, so order
$p + 1 minus.plus t$. For $D = -4$ there are also the traces $plus.minus 2v$, and for
$D = -3$ the traces $plus.minus (t plus.minus 3v)\/2$ (extra twists). Their
$j$-invariants are the roots mod $p$ of the *Hilbert class polynomial* $H_D$,
which has degree $h(D)$, the class number. For the 13 discriminants with
$h(D) = 1$, $H_D = X - j$ with $j$ a known integer: $j(-4) = 1728$,
$j(-16) = 287496$, $j(-19) = -884736$, $j(-43) = -884736000$, …

*Example* (`data/cm_p101.json`). At $p = 101$ the tool lists 10 CM curves:

#table(columns: 6, align: left,
  [$D$], [$4 dot 101 = t^2 + |D|v^2$], [traces], [orders], [smooth parts], [one curve],
  [$-4$], [$20^2 + 4 dot 1^2$], [$plus.minus 20, plus.minus 2$], [82, 122, 100, 104], [$2; 2; 2^2 5^2; 2^3$], [$y^2 = x^3 + 36x$ (order 100)],
  [$-16$], [$2^2 + 16 dot 5^2$], [$plus.minus 2$], [100, 104], [$2^2 5^2; 2^3$], [$y^2 = x^3 + 3x + 98$],
  [$-19$], [$10^2 + 19 dot 4^2$], [$plus.minus 10$], [92, 112], [$2^2; 2^4 7$], [$y^2 = x^3 + 67x + 29$],
  [$-43$], [$19^2 + 43 dot 1^2$], [$plus.minus 19$], [83, 121], [none (83 prime, $121 = 11^2$)], [—],
)

#check[
  Verify each second column: $400 + 4 = 404$, $4 + 400$, $100 + 304$, $361 + 43$. ✓
  Why no $D = -3, -7, -8, -11$? $p$ must be a square mod $|D|$ (roughly), and
  $101 equiv 2 thin (mod 3)$, $101 equiv 3 thin (mod 7)$ (squares mod 7 are $1, 2, 4$),
  $101 equiv 5 thin (mod 8)$ and $101 equiv 2 thin (mod 11)$ all fail. Also
  $1728 equiv 11 thin (mod 101)$: that is the $j$ the tool prints for $D = -4$.
]

For one fixed $p$, class number 1 gives only a handful of orders. Allowing
$h(D) > 1$ (`--max-disc`) gives many more: at $p = 1000033$, a scan of
$|D| <= 1000$, $h <= 16$ found 782 CM curves and a usable $D = -99$ curve,
while class number 1 alone found none (`../REPORT.pdf`, §1.3).

#note[
  Slide 16 also says Method 2 is "more efficient when you can choose $p$
  yourself (kind of trapdoor)": pick $p$ so that a CM order is smooth. The
  tool deliberately only takes $p$ as given and does not do this.
]

= Putting it together: the relation system (slide 17)

For $m$ points and $n'$ steps, the unknowns are $x_(i,j)$ ($i <= m$, $j <= n'$),
and the equations are
$ cases(
  S_(m+1)(x_(1,1), dots, x_(m,1), X) = 0,
  x_(i,j+1) D_j (x_(i,j)) = N_j (x_(i,j)) & quad 1 <= i <= m","  1 <= j < n',
  N_(n') (x_(i,n')) = c D_(n')(x_(i,n')) & quad 1 <= i <= m.
) $
(The slide writes the last line as $0 = L_(n')(x_(i,n'))$, with $c$ folded
into the last map.)

*Example* (`data/system_p101_m2_x50.ms`, msolve format: variables, then
$p$, then equations). The factor base lives on a *different* curve,
$E: y^2 = x^3 + 2x + 3$ over $FF_101$. The auxiliary $E'$ only supplies $V$.
The target has $X = x(R) = 50$:
```
x1_1,x1_2,x2_1,x2_2
101
(x1_1-x2_1)^2*50^2-2*((x1_1+x2_1)*(x1_1*x2_1+2)+6)*50+(x1_1*x2_1-2)^2-12*(x1_1+x2_1),
x1_2*(1*x1_1+63)-(1*x1_1^2+63*x1_1+55),
(1*x1_2^3+79*x1_2^2+25*x1_2+24)-96*(1*x1_2^2+79*x1_2+20),
x2_2*(1*x2_1+63)-(1*x2_1^2+63*x2_1+55),
(1*x2_2^3+79*x2_2^2+25*x2_2+24)-96*(1*x2_2^2+79*x2_2+20)
```
Line 1 is $S_3(x_(1,1), x_(2,1), 50)$. Lines 2–3 say "$x_(1,1) in V$" in two steps,
and lines 4–5 say the same for $x_(2,1)$. Four unknowns, five equations, degrees
at most 3 in each tower variable, instead of one equation of degree 6.

For $m = 2$ the tool also solves this directly. For each $x_1 in V$ it solves
the quadratic $S_3(x_1, dot, 50) = 0$, keeps roots in $V$, lifts both to
points and *checks by adding them*. It finds one relation:
$ (60, 36) + (67, 83) = R, quad x(R) = 50. $

#check[
  Both points are on $E$: $36^2 = 1296 equiv 84$ and $60^3 + 2 dot 60 + 3 equiv 62 + 123 equiv 84$;
  $83^2 equiv 21$ and $67^3 + 2 dot 67 + 3 equiv 86 + 137 equiv 21$. The chord slope is
  $lambda = (83 - 36)\/(67 - 60) = 47 dot 7^(-1) = 47 dot 29 equiv 50$ (since $7 dot 29 = 203 equiv 1$),
  so $x_3 = lambda^2 - 60 - 67 = 2500 - 127 equiv 76 - 127 equiv 50$. ✓
  Both abscissae, 60 and 67, are in $V$. Of the six elements of $V$, only these
  two lift to $E$, so this factor base has 4 points.
]

= Using the tool

```
# Kummer form (needs S | p - 1)
pkm-smooth-tower kummer  --p 31 --degrees 2,3 --g 1
# Method 1, then the isogeny tower; optional system for m points
pkm-smooth-tower isogeny --p 101 --bound 5 --min-size 6 --seed 1 \
    --curve 2,3 --target-x 50 --m 2 --system sys.ms
# Method 2: class number 1, or Hilbert polynomials with --max-disc
pkm-smooth-tower cm --p 101 --bound 7
pkm-smooth-tower cm --p 1000033 --bound 13 --min-size 300 --max-disc 1000
```
Every tower is checked before it is printed: $|V| = S$ distinct values, and
$L(v) = c$ on all of $V$. For $p < 2^22$ it also checks that $L(x) = c$ has no other root in $FF_p$.
Code: `src/cryptanalysis/pkm_smooth_tower.rs` (library, 8 unit tests) and
`src/bin/pkm_smooth_tower.rs`.

= What is known, and what is not

*Building is easy.* Building $V$, the maps and the system is fast and fully
checkable, as in every example here.

*What this repository has measured.* These are results from earlier studies,
quoted from their reports. They have no pcc claim id yet [pcc:unverified].

- *End to end against rho, degree-2 towers.* Index calculus on PKM's Kummer
  tower ran in the `ecbench` harness, with every group operation charged. On
  five curves of 20–28 bits it cost *29–141 times Pollard rho* (with the
  negation map), and the ratio grows with the curve size
  (`research/pkm_tower_ecbench_20261013/`). The isogeny tower, with its
  auxiliary-curve search charged, cost 1.00–1.33 times a plain factor base of
  the same size and stayed *18–136 times above rho*
  (`research/pkm_isogeny_tower_ecbench_20261015/`).
- *Why the tower did not help there.* Those runs decompose with tables and
  root finding over $V$ ($m = 2$), and for that $V$ is just a set. The tower
  base cost the same as a plain base of equal size, within 1 %. In that
  study's words: "PKM's structure helps an algebraic solver, not a table."
- *The algebraic (Gröbner/F4) side.* Here the tower's shape does matter.
  On degree-2 Kummer towers, the F4 solving degree climbs slowly with the
  tower depth (3, 4, 4, 4, 4, 5, 5, 5, 5, 6 at one 31-bit prime;
  `research/pkm_matched_prime_20261106/`). Regrouping the levels into
  degree-4 or degree-8 steps cut F4's work by a *constant* factor in three of
  four cells (×0.17 to ×0.34). In the fourth (×0.54) the study's preset rule
  found no material effect. In no cell did it change how the work grows. A random system with the
  same shape gained the same, so the gain comes from the system's shape, not
  from the curve (`research/pkm_radix_grading_20261104/`).
- *At real sizes.* For P-192 ($m = 4$), the isogeny route's auxiliary-curve
  search alone is estimated, not measured, at about $2^74$ group operations,
  against rho's $2^95.8$ (`research/pkm_isogeny_tower_ecbench_20261015/`).
  The deciding cost, solving the systems at $m >= 3$, has not been measured
  at any size near that.

*Not yet tried.* Every Gröbner measurement above used maps of degree 2 (or
groupings of them). Towers with *odd* degrees $q_j = 3, 5, 7, dots$, or mixed
ones, which is what this tool adds, have not been through F4. Neither has
any comparison between towers of the same $|V|$ but different degree mixes.
That is the cheapest open experiment this tutorial points to.

*Bottom line.* At the sizes measured, this attack is slower than Pollard
rho, by 18–141 times. Whether some tower shape changes how the cost *grows*
is open. The tutorial makes no speed claim.

= Exercises

+ At $p = 31$, list $V$ for degrees $(3, 2)$ (cube first, then square). Is it
  the same set as in §4? Is the middle row of the tree the same?
+ Why must $2R' ∉ G$? Find what goes wrong at $p = 101$ if you took $R' = Q$.
+ At $p = 101$, $D = -4$ gives four orders. Which ones could support a tower
  with $S = 25$? With $S = 8$?
+ Using $S_3$, show that if $x_1 = x_2$ then the roots in $x_3$ are
  $x(2P_1)$ and $x(O)$ ("at infinity"), and explain what that means for the quadratic.
+ For $m = 3$, write the summation part of the system with one helper $u_1$.
  How many unknowns does the full system have when $n' = 2$?

*Answers.* (1) The same set: $V = mu_6$ either way, since $x^6 = 1$ is the
composite condition. The middle row becomes the cubes ${1, 30}$ instead of the
squares ${1, 5, 25}$. (2) If $2R' in G$, two points of $R' + G$ are negatives
of each other, so they share an abscissa and $|V| < S$. With $R' = Q in G$,
$V = x(G)$ contains $x(O)$, which is not a field element, and the
pairs $plus.minus [k]Q$ collapse. (3) $S = 25$ needs the order 100 curve
($2^2 5^2$); $S = 8$ needs order 104 ($2^3 dot 13$), and the
subgroup must be cyclic. (4) $S_3(x_1, x_1, x_3)$ loses its $x_3^2$ term,
so it has degree 1 in $x_3$: one finite root, which is $x(2P_1)$. The other root
went to infinity because $P_1 - P_1 = O$. (5) $S_3(x_(1,1), x_(2,1), u_1)$ and
$S_3(u_1, x_(3,1), X)$. There are $3 dot 2 + 1 = 7$ unknowns.

#v(6pt)
#text(size: 9pt)[*References.* I. Semaev, "Summation polynomials and the discrete
logarithm problem on elliptic curves", IACR ePrint 2004/031. C. Petit, M. Kosters,
A. Messeng, "Algebraic approaches for the elliptic curve discrete logarithm problem
over prime fields", PKC 2016. J. Vélu, "Isogénies entre courbes elliptiques",
C. R. Acad. Sci. Paris 273 (1971). G. Cornacchia (1908) on $x^2 + d y^2 = m$.]
