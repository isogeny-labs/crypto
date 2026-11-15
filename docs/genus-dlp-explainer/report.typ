// Why discrete logarithms are cheaper on Jacobians of genus >= 2.
// Explainer: literature summary with diagrams and equations. No measurements.
// Build: /tmp/report-venv/bin/python -c "import typst; typst.compile('report.typ', output='report.pdf')"
#set page(paper: "a4", margin: (x: 16mm, y: 16mm), numbering: "1 / 1")
#set text(font: "DejaVu Sans", size: 9.5pt)
#set par(leading: 0.62em, justify: false)
#set heading(numbering: none)
#show heading.where(level: 1): it => block(above: 1.2em, below: 0.6em, text(size: 14pt, weight: "bold", it.body))
#show heading.where(level: 2): it => block(above: 1.0em, below: 0.4em, text(size: 11pt, weight: "bold", it.body))
#show link: set text(fill: rgb("#1d3557"))

// ---------- drawing helpers (coordinates in mm, origin top-left of the figure box) ----------
#let mm(v) = v * 1mm
#let fig(w, h, body) = box(width: mm(w), height: mm(h), body)
#let at(x, y, body) = place(top + left, dx: mm(x), dy: mm(y), body)
#let seg(x1, y1, x2, y2, color: black, thick: 0.6pt, dash: none) = place(top + left,
  line(start: (mm(x1), mm(y1)), end: (mm(x2), mm(y2)), stroke: (paint: color, thickness: thick, dash: dash)))
#let arr(x1, y1, x2, y2, color: black, thick: 0.6pt, head: 2.0, dash: none) = {
  let dx = x2 - x1
  let dy = y2 - y1
  let n = calc.sqrt(dx * dx + dy * dy)
  let ux = dx / n
  let uy = dy / n
  let bx = x2 - head * ux
  let by = y2 - head * uy
  seg(x1, y1, bx, by, color: color, thick: thick, dash: dash)
  let px = -uy * head * 0.45
  let py = ux * head * 0.45
  place(top + left, polygon(fill: color, stroke: none,
    (mm(x2), mm(y2)), (mm(bx + px), mm(by + py)), (mm(bx - px), mm(by - py))))
}
#let node(x, y, w, h, body, fill: white, color: black, thick: 0.6pt) = at(x, y,
  box(width: mm(w), height: mm(h), fill: fill, stroke: (paint: color, thickness: thick), radius: 1.2mm,
    inset: 1.4mm, align(center + horizon, text(size: 8pt, body))))
#let lbl(x, y, body, size: 7.5pt, color: black, anchor: left, w: none) = at(x, y, box(width: if w == none { auto } else { mm(w) }, align(anchor, text(size: size, fill: color, body))))
#let pt(x, y, r: 0.9, color: black) = at(x - r, y - r, circle(radius: mm(r), fill: color, stroke: none))
#let ring(x, y, r: 1.1, color: black) = at(x - r, y - r, circle(radius: mm(r), fill: white, stroke: 0.6pt + color))
#let blue = rgb("#1d3557")
#let red = rgb("#9d0208")
#let green = rgb("#386641")
#let grey = rgb("#777777")
#let paleblue = rgb("#eef3f9")
#let palered = rgb("#fbeeee")
#let palegreen = rgb("#eef6ee")
#let caption(body) = block(above: 0.5em, below: 1.0em, text(size: 8pt, fill: grey, body))
#let claim(body) = box(fill: rgb("#f4f4f4"), inset: 2mm, radius: 1mm, width: 100%, body)

// =====================================================================================
= Why discrete logarithms are cheaper on Jacobians of genus 2 and above
2026-11-15 · isogeny-labs/crypto · explainer with diagrams and equations · literature summary, no new measurement · every cost statement here is an asymptotic result quoted from the cited paper and stands as `[pcc:unverified]` in this repository's sense.

#claim[
  *One-sentence answer.* A Jacobian of genus $g$ over $bb(F)_q$ has about $q^g$ elements, but every element is a sum of at most $g$ points of a curve that has only about $q$ points; that gap gives index calculus a factor base of size $q$, a smoothness test that is one polynomial factorisation, and a constant smoothness probability $1\/g!$, none of which exist on an elliptic curve over a prime field. Pollard rho sees only the group and costs $q^(g\/2)$; index calculus pays about $q^(2-2\/g)$ for hyperelliptic curves and $q$ for non-hyperelliptic genus 3, so the advantage starts at $g = 3$, while $g = 2$ is a tie in the exponent.
]

== Diagram 1: the gap that index calculus exploits
#fig(178, 62)[
  #node(2, 20, 42, 16, fill: palegreen, color: green)[curve $C \/ bb(F)_q$ \ $\#C(bb(F)_q) approx q$ points]
  #node(64, 18, 52, 20, fill: paleblue, color: blue)[$"Sym"^g (C)$: unordered $g$-tuples of points \ about $q^g \/ g!$ of them use only $bb(F)_q$-points]
  #node(134, 20, 42, 16, fill: palered, color: red)[$"Jac"(C)(bb(F)_q)$ \ $\#"Jac" approx q^g$ elements]
  #arr(44, 28, 64, 28)
  #lbl(45, 22.5, [$g$-fold sums])
  #arr(116, 28, 134, 28)
  #lbl(117, 19.5, [reduced divisors, \ one per class], w: 17)
  // bottom: factor base and smoothness
  #node(2, 44, 42, 12, fill: palegreen, color: green)[factor base $cal(F) subset C(bb(F)_q)$ \ $|cal(F)| = q^alpha$, $alpha <= 1$]
  #arr(23, 44, 23, 36, color: green)
  #lbl(48, 44, [$D$ is *smooth* $<==> D = P_1 + dots + P_g$ with every $P_i in cal(F)$ \ test: factor $u(x)$ of degree $g$ over $bb(F)_q$ \ probability for $alpha = 1$: $arrow.r 1\/g!$ (constant in $q$)], size: 8pt)
  // top: what each algorithm sees
  #lbl(134, 2, [*rho sees only the red box*: birthday bound $sqrt(q^g) = q^(g\/2)$], size: 8pt, color: red, w: 42)
  #lbl(2, 2, [*index calculus sees the green set*: $q^alpha$ unknowns, relations from smooth $D$, then sparse linear algebra in $q^(2 alpha)$], size: 8pt, color: green, w: 56)
  #lbl(64, 2, [an elliptic curve has $g = 1$: the three boxes coincide; there is no set smaller than the group to decompose into], size: 8pt, color: grey, w: 52)
]
#caption[Diagram 1. Group size versus generating set. The Jacobian is the quotient of the $g$-th symmetric power of the curve by linear equivalence, and every class of degree-$g$ divisors has a unique reduced representative, so the picture is close to a bijection on most of the group. For $g = 1$ the symmetric power is the curve itself and the gap vanishes.]

The rest of this note walks through the chain of equalities and inequalities that turn Diagram 1 into exponents: Mumford representation and the $1\/g!$ law (page 2), the relation-versus-linear-algebra balance (page 3), the exponents by genus (page 4), why non-hyperelliptic curves are worse still (page 5), the transfers that pull other groups into this regime (page 6), what it does and does not mean for a prime-field elliptic curve (page 7), and references with questions (page 8).

#pagebreak()
// =====================================================================================
= 1. Elements decompose into points: Mumford representation
Take an imaginary hyperelliptic curve of genus $g$ over $bb(F)_q$ with $q$ odd, $C : y^2 = f(x)$, $deg f = 2g + 1$, $f$ squarefree, one point $infinity$ at infinity. Every class in $"Jac"(C)(bb(F)_q)$ has a unique *reduced* representative

$ D = sum_(i=1)^k P_i - k dot infinity, quad k <= g, quad P_i = (x_i, y_i), quad P_i != iota(P_j) "for" i != j, $

where $iota (x, y) = (x, -y)$ is the hyperelliptic involution. Mumford's representation encodes $D$ as a pair of polynomials

$ D arrow.l.r (u, v), quad u(x) = product_(i=1)^k (x - x_i), quad v(x_i) = y_i, quad deg v < deg u <= g, quad u | v^2 - f, $

and Cantor's algorithm adds two such pairs in $O(g^2)$ field operations. The decomposition into points is therefore not a choice of the attacker: *it is the data structure*. The $x$-coordinates of the points are the roots of $u$.

#claim[
  *Smoothness.* Fix a factor base $cal(F) subset.eq C(bb(F)_q)$. A reduced divisor is $cal(F)$-smooth when $u$ splits into linear factors over $bb(F)_q$ and each resulting point lies in $cal(F)$. For $cal(F) = C(bb(F)_q)$ the test is: *does $u$ split completely?* One distinct-degree factorisation of a degree-$g$ polynomial answers it.
]

== Diagram 2: the three ways a degree-3 polynomial can factor
#fig(178, 40)[
  // panel 1: split
  #node(2, 2, 54, 10, fill: palegreen, color: green)[type $(1,1,1)$: $u = (x - a)(x - b)(x - c)$, $a, b, c in bb(F)_q$]
  #seg(6, 30, 52, 30, color: grey)
  #pt(14, 30, color: green) #pt(28, 30, color: green) #pt(44, 30, color: green)
  #lbl(12, 32, [$a$]) #lbl(26, 32, [$b$]) #lbl(42, 32, [$c$])
  #lbl(6, 14, [three $bb(F)_q$-rational points: *smooth*], size: 7.5pt, color: green, w: 50)
  #lbl(6, 22, [probability $arrow.r 1\/3! = 1\/6$], size: 7.5pt)
  // panel 2: (2,1)
  #node(62, 2, 54, 10, fill: palered, color: red)[type $(2,1)$: $u = (x - a)(x^2 + b x + c)$, quadratic irreducible]
  #seg(66, 30, 112, 30, color: grey)
  #pt(76, 30, color: green)
  #ring(96, 26, color: red) #ring(96, 34, color: red)
  #seg(96, 27.1, 96, 32.9, color: red, dash: "dotted")
  #lbl(74, 32, [$a$]) #lbl(98, 25, [conjugate pair in $bb(F)_(q^2)$], color: red, size: 7pt)
  #lbl(66, 14, [one rational point plus a conjugate pair over $bb(F)_(q^2)$: *not smooth*], size: 7.5pt, color: red, w: 50)
  #lbl(66, 22, [probability $arrow.r 1\/2$], size: 7.5pt)
  // panel 3: (3)
  #node(122, 2, 54, 10, fill: palered, color: red)[type $(3)$: $u$ irreducible of degree 3]
  #seg(126, 30, 172, 30, color: grey)
  #ring(149, 26.5, color: red) #ring(143, 34, color: red) #ring(155, 34, color: red)
  #lbl(126, 14, [one point over $bb(F)_(q^3)$ and its two conjugates: *not smooth*], size: 7.5pt, color: red, w: 50)
  #lbl(126, 22, [probability $arrow.r 1\/3$], size: 7.5pt)
]
#caption[Diagram 2. Genus 3: the factorisation type of $u$ is the cycle type of a random permutation in $S_3$ in the large-$q$ limit (Chebotarev for the generic cubic), so the fully split type has density $1\/g!$. Only the left panel is a sum of three rational points.]

== The $1\/g!$ law, exactly
The number of monic squarefree polynomials of degree $g$ over $bb(F)_q$ is $q^g - q^(g-1)$; the number that split into $g$ distinct linear factors is $binom(q, g)$. Their ratio tends to $1\/g!$. The reduced divisors with $deg u = g$ number about $q^g$ and the smooth ones about $(\#C(bb(F)_q))^g \/ g! approx q^g\/g!$, so the same law governs the Jacobian.

#align(center)[
  #table(
    columns: 5, align: (center, right, right, right, right), stroke: 0.4pt + grey, inset: 3pt,
    [$g$], [$q$], [split: $binom(q, g)$], [squarefree: $q^g - q^(g-1)$], [ratio vs $1\/g!$],
    [2], [101], [5 050], [10 100], [0.50000 vs 0.50000 (exact for every $q$)],
    [3], [101], [166 650], [1 020 100], [0.16337 vs 0.16667],
    [3], [10 007], [166 966 845 035], [1 002 001 330 294], [0.16663 vs 0.16667],
    [4], [101], [4 082 925], [103 030 100], [0.03963 vs 0.04167],
    [4], [10 007], [417 584 079 432 535], [10 027 027 312 252 058], [0.04165 vs 0.04167],
  )
]
#caption[Exact integer counts from the two closed forms above (computed in this session with integer arithmetic; they are combinatorial identities, not curve measurements). Compare with integer factoring, where the probability that a random $n$-bit integer is $B$-smooth decays like $u^(-u)$ with $u = log n \/ log B$: there the smoothness probability is the enemy, here it is a constant.]

#pagebreak()
// =====================================================================================
= 2. From a constant smoothness probability to an exponent
Index calculus on a group of order $N approx q^g$ with factor base $cal(F)$, $|cal(F)| = q^alpha$:

$ "relation collection:" quad &(\#"relations needed") times ("trials per relation") = q^alpha times (q^(alpha))^(-g) q^(g) dot g! = g! dot q^(alpha + g(1 - alpha)), \
  "linear algebra (sparse, weight" <= g "per row):" quad &|cal(F)|^2 = q^(2 alpha). $

The first line uses the smoothness probability $(|cal(F)|\/q)^g\/g! = q^(g(alpha - 1))\/g!$ for a random reduced divisor whose $g$ points must all fall in $cal(F)$. Balancing the two lines gives the first family of exponents:

$ alpha + g(1 - alpha) = 2 alpha quad ==> quad alpha^* = g/(g+1), quad "cost" = tilde(O)(q^(2 alpha^*)) = tilde(O)(q^(2 - 2/(g+1))). $

== Diagram 3: the balance for genus 3
#fig(178, 58)[
  // axes
  #seg(10, 50, 100, 50, color: black) #seg(10, 50, 10, 4, color: black)
  #lbl(92, 51.5, [$alpha$ (factor base $= q^alpha$)], size: 7.5pt)
  #lbl(3, 2, [exponent $e$], size: 7.5pt)
  #lbl(8.5, 51.5, [0], size: 7pt) #lbl(88, 51.5, [1], size: 7pt)
  #seg(90, 49, 90, 51, color: black)
  #lbl(4, 47.5, [0], size: 7pt) #lbl(4, 33.5, [1], size: 7pt) #lbl(4, 19.5, [2], size: 7pt) #lbl(4, 5.5, [3], size: 7pt)
  #seg(9, 36, 11, 36, color: black) #seg(9, 22, 11, 22, color: black) #seg(9, 8, 11, 8, color: black)
  // X(a)=10+80a ; Y(e)=50-14e
  // linear algebra e=2a: (0,0)->(1,2)
  #seg(10, 50, 90, 22, color: blue, thick: 1pt)
  #lbl(20, 43, [linear algebra: $e = 2 alpha$], size: 7.5pt, color: blue)
  // relations g=3: e=3-2a: (0,3)->(1,1)
  #seg(10, 8, 90, 36, color: green, thick: 1pt)
  #lbl(14, 6, [relation collection, $g = 3$: $e = g - (g - 1) alpha = 3 - 2 alpha$], size: 7.5pt, color: green)
  // intersection a=0.75 e=1.5 -> (70,29)
  #ring(70, 29, color: black)
  #lbl(40, 21.5, [$alpha^* = 3\/4$, $e = 3\/2$: ties rho at $g = 3$], size: 7.5pt)
  #seg(66, 25, 69.3, 28.2, color: black, thick: 0.4pt)
  // GTTD point a=2/3, e=4/3 -> (63.33, 31.33)
  #pt(63.3, 31.3, color: red, r: 1.1)
  #lbl(58, 39.5, [double large primes move the green line: \ $alpha = 1 - 1\/g = 2\/3$, $e = 4\/3$ (GTTD 2007)], size: 7.5pt, color: red)
  #arr(63, 39, 63.3, 32.8, color: red, thick: 0.5pt)
  // Gaudry a=1,e=2 -> (90,22)
  #pt(90, 22, color: grey, r: 1.1)
  #lbl(91, 23, [Gaudry 2000: \ $alpha = 1$, $e = 2$], size: 7.5pt, color: grey)
  // rho reference line e=1.5 -> y=29
  #seg(10, 29, 90, 29, color: red, dash: "dashed", thick: 0.5pt)
  #lbl(12, 29.5, [Pollard rho at $g = 3$: $e = g\/2 = 3\/2$], size: 7pt, color: red)
  // right side text
  #lbl(118, 2, [*Reading the picture.* Shrinking the factor base makes the matrix cheaper (blue falls) but relations rarer (green rises); the crossing is the exponent. \ \ Large-prime variations do not change the blue line. They lower the green one by letting a relation keep one or two points *outside* $cal(F)$ and cancelling them later in a graph of partial relations. Each free slot removes one factor $q^(1-alpha)$ from the trial count. \ \ With two free slots the crossing moves to $alpha = 1 - 1\/g$ and the exponent to $2 - 2\/g$.], size: 7.5pt, w: 58)
]
#caption[Diagram 3. Cost exponents as functions of the factor-base exponent $alpha$ for genus 3. The two straight lines are the two displayed formulas; the crossing is the optimum of that family. The red point is the double-large-prime optimum quoted from Gaudry, Thomé, Thériault and Diem.]

== The published ladder of exponents (hyperelliptic, $g$ fixed, $q arrow.r infinity$)
#align(center)[
  #table(
    columns: 4, align: (left, center, left, left), stroke: 0.4pt + grey, inset: 3pt,
    [method], [exponent $e$ in $tilde(O)(q^e)$], [mechanism], [source],
    [Pollard rho (reference)], [$g\/2$], [birthday paradox on $N approx q^g$], [standard],
    [Gaudry 2000, full factor base], [$2$], [$alpha = 1$, sparse linear algebra dominates], [Gaudry, Eurocrypt 2000],
    [Harley's reduced factor base], [$2 - 2\/(g+1)$], [balance above, no large primes], [in Gaudry 2000],
    [Thériault 2003, one large prime], [$2 - 4\/(2g+1)$], [one free slot per relation], [Thériault, Asiacrypt 2003],
    [Gaudry–Thomé–Thériault–Diem 2007, Nagao 2007], [$2 - 2\/g$], [two free slots, graph of partial relations], [Math. Comp. 76],
  )
]
Where it bites: $2 - 2\/g < g\/2$ exactly when $g >= 3$. At $g = 2$ both are $1$, and rho wins on constants and on memory, which is why genus-2 Jacobians are rated as hard as elliptic curves of the same group size.

#pagebreak()
// =====================================================================================
= 3. Exponents by genus
#align(center)[#image("fig_exponents.svg", width: 96%)]
#caption[Figure 1. Exponent $e$ with cost $q^(e + o(1))$ against genus for the published methods (closed forms from the table on page 3 and from Section 4; source `make_figures.py`, values in `fig_exponents.json`). Everything below the black rho line is an asymptotic win for index calculus. The non-hyperelliptic line (dark red) starts at $g = 3$ because every genus-2 curve is hyperelliptic.]

#align(center)[
  #table(
    columns: 7, align: (center, center, center, center, center, center, center), stroke: 0.4pt + grey, inset: 3pt,
    [$g$], [rho $g\/2$], [Gaudry $2$], [reduced $2 - 2\/(g+1)$], [Thériault $2 - 4\/(2g+1)$], [GTTD $2 - 2\/g$], [non-hyperelliptic $2 - 2\/(g-1)$],
    [2], [1], [2], [1.333], [1.200], [*1.000 (tie)*], [none (all genus 2 is hyperelliptic)],
    [3], [1.5], [2], [1.500], [1.429], [*1.333*], [*1.000*],
    [4], [2], [2], [1.600], [1.556], [*1.500*], [*1.333*],
    [5], [2.5], [2], [1.667], [1.636], [*1.600*], [*1.500*],
    [6], [3], [2], [1.714], [1.692], [*1.667*], [*1.600*],
  )
]
#caption[Bold entries beat the matched rho exponent. Note that the gap grows with $g$ while the index-calculus exponent stays below 2: this is the small-genus regime. Once $g$ grows with $log q$ the exponents stop being the right description and the costs become subexponential (Section 5).]

== Two cautions on reading exponents
- *Hidden constants and $log$ factors.* $tilde(O)$ hides polynomial factors in $g$ and $log q$, factorials such as $g!$, and the cost of a Jacobian group operation, which is $O(g^2)$ field operations by Cantor's algorithm versus a handful for an elliptic addition. At $g = 3$ and cryptographic $q$ the asymptotic win of $q^(4\/3)$ over $q^(3\/2)$ is real but the crossover field size is a matter of implementation, not of the exponent.
- *Memory.* Rho runs in constant memory with distinguished points. Index calculus stores $q^alpha$ factor-base elements and a sparse matrix of that dimension. For $g = 2$ the $q^1$ matrix is the whole reason rho remains the method of record.

#pagebreak()
// =====================================================================================
= 4. Non-hyperelliptic curves: geometry hands over the relations
A non-hyperelliptic curve of genus 3 is canonically a *smooth plane quartic* $C : F(X, Y, Z) = 0$, $deg F = 4$. For any line $ell$ and a fixed reference line $ell_infinity$, the rational function $ell \/ ell_infinity$ has divisor

$ "div"(ell \/ ell_infinity) = (P_1 + P_2 + P_3 + P_4) - (Q_1 + Q_2 + Q_3 + Q_4), quad {P_i} = ell inter C, {Q_i} = ell_infinity inter C, $

so in the Jacobian $P_1 + P_2 + P_3 + P_4 equiv D_infinity$ for *every* line. Choose $P_1, P_2 in cal(F)$; the line through them is determined; it meets $C$ in two further points, and with probability about $1\/2!$ both are $bb(F)_q$-rational. There is no random walk and no reduction step: a relation costs one line intersection, which is one degree-2 polynomial to factor.

== Diagram 4: a secant line gives a free relation; the hyperelliptic model does not
#grid(columns: (74mm, 104mm), gutter: 0mm,
  fig(74, 60)[
    #at(0, 0, curve(stroke: 1pt + blue,
      curve.move((mm(6), mm(22))),
      curve.cubic((mm(6), mm(4)), (mm(28), mm(4)), (mm(33), mm(19))),
      curve.cubic((mm(38), mm(4)), (mm(62), mm(4)), (mm(62), mm(22))),
      curve.cubic((mm(62), mm(40)), (mm(38), mm(40)), (mm(33), mm(25))),
      curve.cubic((mm(28), mm(40)), (mm(6), mm(40)), (mm(6), mm(22))),
      curve.close()))
    #seg(1, 15, 69, 15, color: red, thick: 0.8pt)
    #pt(7.3, 15, color: red) #pt(31.0, 15, color: red) #pt(35.0, 15, color: red) #pt(60.5, 15, color: red)
    #lbl(4, 9, [$P_1$], color: red) #lbl(27, 9, [$P_2$], color: red) #lbl(35.5, 16.5, [$P_3$], color: red) #lbl(58, 9, [$P_4$], color: red)
    #lbl(64, 12, [$ell$], color: red)
    #lbl(2, 43, [plane quartic $C$ (schematic real picture), $deg = 4$: $P_1 + P_2 + P_3 + P_4 equiv D_infinity$ for every line $ell$], size: 7.5pt, color: blue, w: 68)
  ],
  fig(104, 60)[
    #seg(2, 22, 100, 22, color: grey)
    #at(6, 15, ellipse(width: mm(12), height: mm(14), stroke: 1pt + blue))
    #at(23, 13, ellipse(width: mm(14), height: mm(18), stroke: 1pt + blue))
    #at(42, 15, ellipse(width: mm(12), height: mm(14), stroke: 1pt + blue))
    #at(0, 0, curve(stroke: 1pt + blue, curve.move((mm(62), mm(22))), curve.cubic((mm(66), mm(12)), (mm(80), mm(6)), (mm(98), mm(3)))))
    #at(0, 0, curve(stroke: 1pt + blue, curve.move((mm(62), mm(22))), curve.cubic((mm(66), mm(32)), (mm(80), mm(38)), (mm(98), mm(41)))))
    // vertical line through the middle oval
    #seg(30, 8, 30, 36, color: green, thick: 0.8pt)
    #pt(30, 13, color: green) #pt(30, 31, color: green)
    #lbl(31, 10, [$P$], color: green) #lbl(31, 31.5, [$iota(P)$], color: green)
    #lbl(27, 37, [$x = a$: $P + iota(P) equiv 0$, trivial], size: 7pt, color: green)
    // slanted line: from (2,30) to (88,14)
    #seg(2, 30, 95, 12.7, color: red, thick: 0.8pt)
    #pt(9.1, 28.7, color: red) #pt(14.9, 27.6, color: red) #pt(23.4, 26.0, color: red) #pt(36.6, 23.6, color: red)
    #pt(42, 22.6, color: red) #pt(54, 20.3, color: red) #pt(68, 17.7, color: red)
    #lbl(2, 2, [slanted line $y = m x + c$: up to $2g + 1 = 7$ points], size: 7pt, color: red)
    #lbl(2, 43, [hyperelliptic $y^2 = f(x)$, $deg f = 7$ ($g = 3$, schematic real picture): the only cheap lines give a trivial relation or one with $2g + 1$ points, all of which must land in $cal(F)$: probability about $1\/(2g+1)!$], size: 7.5pt, color: blue, w: 100)
  ]
)
#caption[Diagram 4. Left: a secant line through two factor-base points on a plane quartic meets it in two more points, giving a four-term relation at constant cost. Right: on the hyperelliptic model the vertical lines give the trivial relation $P + iota(P) = 0$ and the non-vertical lines pass through $2g + 1$ points. Intersection positions are illustrative.]

== The exponents
Diem's analysis with two large primes among the $d - 2$ free intersection points gives, for a curve with a plane model of degree $d >= 4$ (Diem, ANTS 2006):

$ tilde(O)(q^(2 - 2\/(d - 2))) quad ==> quad d = 4 "(plane quartic, non-hyperelliptic" g = 3"):" tilde(O)(q), quad "versus rho" tilde(O)(q^(3\/2)). $

Every non-hyperelliptic curve of genus $g >= 3$ has a plane model of degree $g + 1$ (project the canonical curve of degree $2g - 2$ in $bb(P)^(g-1)$ from $g - 3$ of its own points), so the same paper gives $tilde(O)(q^(2 - 2\/(g-1)))$ for non-hyperelliptic genus $g$, slightly below the hyperelliptic $2 - 2\/g$; Diem and Kochinke (2013) make the linear systems explicit. The invariant behind the difference is *gonality*: a hyperelliptic curve carries a degree-2 map to $bb(P)^1$ whose fibres are $\{P, iota(P)\}$ and yield nothing, while the low-degree plane model of a non-hyperelliptic curve has pencils of lines whose fibres are small *and* non-trivial.

#pagebreak()
// =====================================================================================
= 5. Transfers: how other groups fall into this regime
#fig(178, 88)[
  #node(2, 4, 40, 14, fill: paleblue, color: blue)[$E(bb(F)_(q^n))$, $n > 1$ \ elliptic curve over an extension field]
  #node(64, 2, 50, 18, fill: palegreen, color: green)[$"Jac"(H)(bb(F)_q)$, $H$ hyperelliptic, \ genus $g$ about $2^(n-1)$ in the good cases \ (Weil restriction contains a curve)]
  #node(126, 2, 50, 18, fill: palered, color: red)[hyperelliptic index calculus: \ $q^(2 - 2\/g)$ for small $g$, \ $L_(q^g)(1\/2)$ then $L(1\/3)$ for large $g$]
  #arr(42, 11, 64, 11) #lbl(43, 5, [Weil descent \ (GHS 2002)], size: 7pt)
  #arr(114, 11, 126, 11)
  #node(64, 26, 50, 19, fill: palegreen, color: green)[decomposition attack: factor base \ ${P in E(bb(F)_(q^n)) : x(P) in bb(F)_q}$, $|cal(F)| approx q$ \ a point is a sum of $n$ factor-base points]
  #arr(42, 11, 64, 33) #lbl(43, 26, [Semaev polynomials, Gaudry 2009, Diem 2011], size: 6.5pt, w: 19)
  #node(126, 26, 50, 19, fill: palered, color: red)[$tilde(O)(q^(2 - 2\/n))$ for fixed $n$ \ (Gröbner step exponential in $n$); \ same shape as the Jacobian case]
  #arr(114, 35.5, 126, 35.5)
  #node(2, 52, 40, 14, fill: paleblue, color: blue)[$"Jac"(H_3)(bb(F)_q)$ \ hyperelliptic genus 3]
  #node(64, 52, 50, 14, fill: palegreen, color: green)[$"Jac"(C_4)(bb(F)_q)$ \ non-hyperelliptic genus 3 (plane quartic)]
  #node(126, 52, 50, 14, fill: palered, color: red)[Diem 2006: $tilde(O)(q)$ \ versus rho $tilde(O)(q^(3\/2))$]
  #arr(42, 59, 64, 59) #lbl(43, 49, [Smith 2008: explicit isogeny, about 18.6% of curves], size: 6.5pt, w: 20)
  #arr(114, 59, 126, 59)
  #node(2, 73, 40, 10, fill: paleblue, color: blue)[$E(bb(F)_p)$, $p$ prime]
  #node(64, 70, 112, 17, fill: rgb("#f4f4f4"), color: grey)[no proper subfield: no Weil descent and no decomposition into a smaller point set. \ Only the trivial transfers remain (Section 6): gluings into genus 2 over $bb(F)_p$, costing $tilde(O)(p) >= sqrt(p)$ of rho.]
  #arr(42, 78, 64, 78)
]
#caption[Diagram 5. Three published routes into the higher-genus regime, and the prime-field case that has none of them. Every arrow is a group homomorphism that is efficiently computable in the forward direction, which is all a transfer needs.]

== Large genus: the function-field regime
When $g$ grows with $log q$ the Jacobian behaves like the class group of a function field, and the degree-$g$ divisors are "smooth" over degree-1 places with Dickman-type probability in the ratio $g \/ (text("smoothness degree"))$. Adleman, DeMarrais and Huang (1994) first made this subexponential; Enge and Gaudry (2002) give the bound

$ L_(q^g)(1\/2, sqrt(2)) = exp((sqrt(2) + o(1)) sqrt(g log q dot log(g log q))) quad "for" g \/ log q arrow.r infinity, $

and Enge, Gaudry and Thomé (2011) and Diem (2011) reach $L(1\/3)$ for $C_(a,b)$ curves in suitable parameter ranges, the exact analogue of the number field and function field sieves. This is the regime Weil descent aims at: it is why an elliptic curve over $bb(F)_(2^n)$ with composite $n$ is treated as suspect.

#pagebreak()
// =====================================================================================
= 6. What it means for an elliptic curve over a prime field
The question behind this note, for this programme, is whether moving a prime-field ECDLP into a Jacobian can buy an exponent. The accounting is set by one inequality.

#claim[
  *Field-size floor.* Let $E \/ bb(F)_p$ with a subgroup of prime order $r approx p$, and let $phi : E arrow.r "Jac"(C)$ be any efficiently computable homomorphism with $C$ of genus $g$ over $bb(F)_q$, injective on the subgroup. Then $bb(F)_p subset.eq bb(F)_q$, so $q >= p$, and
  $ "cost of index calculus on" "Jac"(C)(bb(F)_q) >= q^(2 - 2\/g) >= p^(2 - 2\/g) >= p^1 > p^(1\/2) approx "rho on" E. $
  For non-hyperelliptic $g = 3$ the bound is $q >= p$ and cost $tilde(O)(q) >= tilde(O)(p)$, again above rho.
]

#fig(178, 38)[
  #node(2, 4, 44, 12, fill: paleblue, color: blue)[$E(bb(F)_p)$, order $approx p$ \ rho: $p^(1\/2)$]
  #node(66, 4, 46, 12, fill: palegreen, color: green)[$"Jac"(C)(bb(F)_q)$, $q >= p$ \ order $approx q^g >= p^g$]
  #node(130, 4, 46, 12, fill: palered, color: red)[index calculus $q^(2 - 2\/g) >= p^(2 - 2\/g)$ \ $g = 2$: $p$. $g = 3$: $p^(4\/3)$. plane quartic: $p$]
  #arr(46, 10, 66, 10) #lbl(46, 3, [$phi$, needs $bb(F)_p subset.eq bb(F)_q$], size: 7pt, w: 20)
  #arr(112, 10, 130, 10)
  #lbl(2, 20, [The win on page 4 came from a group of size $q^g$ built from $q$ points. Here the group of interest already has size $p$, so a target Jacobian of the same field size is $g$ times too big, and its index calculus pays for the whole $q^g$. The transfer pays off only when the base field can *shrink*: $E(bb(F)_(p^n))$ descends to $bb(F)_p$ with $q = p$ and group size $p^n$, which is exactly Section 5.], size: 8pt, w: 174)
]
#caption[Diagram 6. The ordering of costs for a transfer out of a prime-field curve. The inequality $q >= p$ is the whole content; it is what Weil descent escapes by starting from an extension field.]

== Consequences, stated as reasoning rather than as measurement
- *Genus-2 gluings over $bb(F)_p$* ($E times E'$, or an isogenous abelian surface, as a Jacobian): group size $p^2$, hyperelliptic index calculus at $tilde(O)(p)$, rho on the original at $p^(1\/2)$. No exponent is gained; the sibling notes `docs/GHS_SCREEN.md` and `docs/GHS_TRANSPORT.md` record the descent side of this.
- *Genus-3 targets over $bb(F)_p$*: $p^(4\/3)$ hyperelliptic, $p$ for a plane quartic. Still above $p^(1\/2)$.
- *What would change the picture*: a decomposition of $E(bb(F)_p)$ itself into a set of size $p^(1\/n)$ with constant decomposition probability and a cheap test. Summation polynomials give the decomposition but the test is a polynomial system whose cost over a prime field is not known to be subexponential; that open problem, not the genus, is the obstacle.
- *Where higher genus does threaten elliptic curves*: over $bb(F)_(q^n)$, $n > 1$, by Weil descent (GHS) when the descent genus is small enough, and by the decomposition attack at $tilde(O)(q^(2 - 2\/n))$ for small fixed $n$. Binary curves over composite-degree fields and curves over $bb(F)_(p^2)$, $bb(F)_(p^3)$ are the classical examples.

== Secondary effects that do not move an exponent
- *Automorphisms.* Rho gains an ideal factor $sqrt(|"Aut"|)$ by walking on orbits (Duursma, Gaudry, Morain 1999). The hyperelliptic involution gives $sqrt(2)$, the same as negation on an elliptic curve; curves with extra automorphisms give more, in any genus. Canonicalisation and fruitless-cycle costs are still to be priced in any concrete instance.
- *Group-operation cost.* Cantor's algorithm costs $O(g^2)$ field multiplications; explicit formulae for $g = 2, 3$ are a small constant times an elliptic addition over a field of the same size. This slows rho and index calculus alike and only shifts crossovers.

#pagebreak()
// =====================================================================================
= 7. Questions with answers
- *Does genus 2 give a faster solver than an elliptic curve of the same group size?* Not in the exponent: $2 - 2\/g = 1 = g\/2$ at $g = 2$. Rho wins on constants and memory. The advantage starts at $g = 3$.
- *Why does $1\/g!$ appear?* The factorisation type of a random degree-$g$ polynomial follows the cycle-type distribution of $S_g$ as $q arrow.r infinity$; the identity type, "all roots in $bb(F)_q$", has density $1\/g!$. The table on page 2 shows the exact finite-$q$ counts.
- *Why can non-hyperelliptic genus 3 be solved in $tilde(O)(q)$ while hyperelliptic genus 3 needs $q^(4\/3)$?* Lines on a plane quartic meet it in 4 points; the hyperelliptic model has no pencil of functions with small non-trivial fibres (Diagram 4). The gap is gonality 3 versus 2.
- *Is a hyperelliptic genus-3 curve then safe?* Not in general: Smith's isogeny carries about 18.6% of them to plane-quartic Jacobians, where Diem's algorithm applies.
- *Does any of this speed up ECDLP over $bb(F)_p$?* No. Any computable transfer keeps the base field, and the index calculus on the target then costs at least $p$ against rho's $p^(1\/2)$ (Section 6). The published threats to elliptic curves from higher genus all start from an extension field.
- *Is the exponent the whole story at $g = 3$?* No. $g!$, the $O(g^2)$ group law, the large-prime graph and memory all sit inside $tilde(O)$; the crossover field size is an implementation question that this note does not measure.

== Notation
$q$: size of the base field, $p$ a prime. $g$: genus. $"Jac"(C)$: Jacobian, order about $q^g$ (Weil: between $(sqrt(q) - 1)^(2g)$ and $(sqrt(q) + 1)^(2g)$). $(u, v)$: Mumford representation. $iota$: hyperelliptic involution. $cal(F)$: factor base, $|cal(F)| = q^alpha$. $tilde(O)$: up to polynomial factors in $log q$ and $g$. $L_N (a, c) = exp((c + o(1)) (log N)^a (log log N)^(1-a))$.

== Sources
#set text(size: 8.3pt)
Links are given as recorded in this session's notes and were not re-fetched here; DOIs and ePrint numbers are stable identifiers to check against.
+ D. Cantor, *Computing in the Jacobian of a hyperelliptic curve*, Math. Comp. 48 (1987).
+ L. Adleman, J. DeMarrais, M.-D. Huang, *A subexponential algorithm for discrete logarithms over the rational subgroup of the Jacobians of large genus hyperelliptic curves*, ANTS I (1994).
+ P. Gaudry, *An algorithm for solving the discrete log problem on hyperelliptic curves*, Eurocrypt 2000. #link("https://doi.org/10.1007/3-540-45539-6_2")[doi:10.1007/3-540-45539-6_2]
+ N. Thériault, *Index calculus attack for hyperelliptic curves of small genus*, Asiacrypt 2003. #link("https://doi.org/10.1007/978-3-540-40061-5_5")[doi:10.1007/978-3-540-40061-5_5]
+ P. Gaudry, E. Thomé, N. Thériault, C. Diem, *A double large prime variation for small genus hyperelliptic index calculus*, Math. Comp. 76 (2007). #link("https://doi.org/10.1090/S0025-5718-06-01900-4")[doi:10.1090/S0025-5718-06-01900-4]
+ K. Nagao, *Index calculus attack for Jacobian of hyperelliptic curves of small genus using two large primes*, Japan J. Indust. Appl. Math. 24 (2007).
+ C. Diem, *An index calculus algorithm for plane curves of small degree*, ANTS VII (2006). #link("https://doi.org/10.1007/11792086_38")[doi:10.1007/11792086_38]
+ C. Diem, S. Kochinke, *Computing discrete logarithms with special linear systems*, preprint (2013).
+ B. Smith, *Isogenies and the discrete logarithm problem in Jacobians of genus 3 hyperelliptic curves*, Eurocrypt 2008; J. Cryptology 22 (2009). #link("https://eprint.iacr.org/2007/428")[ePrint 2007/428]
+ A. Enge, P. Gaudry, *A general framework for subexponential discrete logarithm algorithms*, Acta Arith. 102 (2002).
+ A. Enge, P. Gaudry, E. Thomé, *An $L(1\/3)$ discrete logarithm algorithm for low degree curves*, J. Cryptology 24 (2011). #link("https://doi.org/10.1007/s00145-010-9057-y")[doi:10.1007/s00145-010-9057-y]
+ C. Diem, *On the discrete logarithm problem in class groups of curves*, Math. Comp. 80 (2011).
+ P. Gaudry, F. Hess, N. Smart, *Constructive and destructive facets of Weil descent on elliptic curves*, J. Cryptology 15 (2002). #link("https://doi.org/10.1007/s00145-001-0011-x")[doi:10.1007/s00145-001-0011-x]
+ I. Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves*, ePrint 2004/031. #link("https://eprint.iacr.org/2004/031")[ePrint 2004/031]
+ P. Gaudry, *Index calculus for abelian varieties of small dimension and the elliptic curve discrete logarithm problem*, J. Symbolic Comput. 44 (2009). #link("https://doi.org/10.1016/j.jsc.2008.08.005")[doi:10.1016/j.jsc.2008.08.005]
+ C. Diem, *On the discrete logarithm problem in elliptic curves*, Compositio Math. 147 (2011). #link("https://doi.org/10.1112/S0010437X10005075")[doi:10.1112/S0010437X10005075]
+ I. Duursma, P. Gaudry, F. Morain, *Speeding up the discrete log computation on curves with automorphisms*, Asiacrypt 1999. #link("https://doi.org/10.1007/978-3-540-48000-6_10")[doi:10.1007/978-3-540-48000-6_10]
