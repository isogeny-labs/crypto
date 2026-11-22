#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show link: underline

= Residue and character leads on $Q = k G$ (R1–R4)

Study `research/residue_leads_20261120` · PR #4850 · protocol `RESIDUE-LEADS-20261120` v1 frozen at commit `7d8c0633` (before data) · harness commits `94f6a75c` (v1 run) and `78657d08` (final source, full replay) · 2026-11-20 · Claude.

*Scope.* These are exact computations over toy curves: prime fields $p approx 2^(16..20)$ and binary fields $"GF"(2^m)$ with $m = 11..19$. Every metric is an exact count, or a statistic computed from exact counts. No clocks are used. There is no speed claim and no leaderboard row. Statements about ECC2K-130 are labelled *heuristic extrapolation*.

== Outcome

#table(columns: (auto, 1fr, auto), inset: 4pt, stroke: 0.4pt,
  [*lead*], [*result*], [*class*],
  [R1 homomorphic residues], [Constant on prime-order $⟨G⟩$ on every curve: 0 violations. Positive controls are non-constant on $E$: the trace condition matches $2E$ exactly, and the 2-descent image has size 4.], [theorem + exact],
  [R2 non-homomorphic bias], [0 of 729 tests correlate with $k$, either through any group character or through $k mod m$ for $m in {2,3,4,5,7,8,16}$. Algebraic characters reach $max_a |S| = (2.000 plus.minus 0.004) sqrt(q)$, so the Weil bound is *attained, never beaten*. That makes them "more uniform than random".], [comparative (v2 null)],
  [R3 Semaev pre-filter], [No useful pre-filter among 90 tests. The largest lift on a prime field is 1.042 with $p = 2 times 10^(-3)$. The binary WHT over all $2^m$ linear functionals of $x(R)$ peaks at $|z| = 5.13$ ($p = 1.2 times 10^(-3)$, threshold $1.1 times 10^(-4)$).], [comparative],
  [R4 rho classes], [The bound $s >= 2 log(sqrt(n) - 1) \/ log d$ holds on 20 of 20 $D = -7$ curves. Koblitz $tau$ is *extremal* ($s = m$). No new cheap class exists beyond automorphisms and the subfield Frobenius.], [theorem + exact],
)

== R1 — homomorphic residues carry no information about $k$

*Theorem (R1-thm).* If $phi : ⟨G⟩ -> A$ is a homomorphism, $|⟨G⟩| = n$ is prime and $gcd(n, |A|) = 1$, then $phi(G)^n = 1$ and $phi(G)^(|A|) = 1$, so $phi(G) = 1$. Every quadratic or higher residue symbol, trace or halving test, Kummer or descent map, and Tate class with $ell != n$ is therefore constant on $⟨G⟩$.

*Exact checks* (`results/r1.json`):

- *Binary curves.* 9 curves: Koblitz curves and random-$b$ curves, $m = 11, 13, 17, 19$, with cofactors 2 and 4. $"Tr"(x(P)) = "Tr"(a)$ holds for all $P in ⟨G⟩$, with 0 violations. On all of $E$, $P in 2E <==> "Tr"(x(P)) = "Tr"(a)$ holds with 0 mismatches, and exactly half of the affine points lie in $2E$. That half is the positive control. This is the theorem behind the encyclopedia's entry C5, where a trace-zero pre-filter recorded 0 rejections in 29,624 checks (PR \#1862).
- *Prime curves with full rational 2-torsion.* $p = 65539, 262147, 1048583$. The 2-descent map $delta(P) = (chi(x - e_1), chi(x - e_2))$ is constant on $⟨G⟩$ (0 exceptions). It has image size 4 on $E$, and 0 of 10,000 random pairs fail the homomorphism property.

*Side observation in R2.* For $m = 17$ the trace mask is $1$. There, $x_0 = "Tr"(x)$ is constant on $⟨G⟩$, so $M = 0.006 sqrt(q)$ in Fig. 1.

== R2 — non-homomorphic residue predicates

For each predicate $f$ and each instance, $S_f(a) = sum_(k=0)^(n-1) f(k G) e^(2 pi i a k \/ n)$ is computed exactly over all $k$ with a Bluestein DFT. The self-test against a naive DFT agrees to $5 times 10^(-14)$. The statistic is $M_f = max_(a != 0) |S_f(a)|$, together with $chi^2$ tables of $f$ against $k mod m$.

#figure(image("report/fig1_r2_spectrum.svg", width: 100%), caption: [R2 (`results/r2_v2.json`). Blue: the algebraic characters sit on $2 sqrt(q)$ (18 rows, range 1.9958–2.0035; the excess is at most 3 terms, from the boundary points mapped to $+1$). Red: prime-field bits lie at the random-function scale. Green: in characteristic 2, every coordinate bit of $x$ is a trace functional $"Tr"(c x)$ and is Weil-bounded. Black crosses: planted controls, all detected ($p <= 1.2 times 10^(-86)$).])

*Null-model defect found and corrected (Amendment 3, post-data, labelled).* Because $x(-P) = x(P)$, the $x$-predicates are even functions of $k$. The $y$-predicates are odd: $chi(-y) = -chi(y)$ for $p equiv 3 med (4)$, and $"lsb"(p - y) = 1 - "lsb"(y)$. Of the 81 test predicates, 74 are even or odd. Their DFT is real (or imaginary), so the v1 complex-Gaussian tail was too light. v1 showed a systematic excess for bit predicates ($M\/sqrt(n ln n) = 1.14$–$1.33$) and flagged two binary rows as "correlated". Under v2's real-Gaussian tail:

- every A/A arm passes, including 64 random even and 64 random odd functions per instance (KS $<= 0.234$, limit 0.25);
- the prime bit predicates have $p_"upper" in [0.31, 1.00]$, which is random-function behaviour;
- both v1 "correlated" rows disappear.

Both versions are kept (`r2.json`, `r2_v2.json`).

*Residues of $k$.* The smallest $chi^2$ $p$-value over 81 predicates × 7 moduli is $1.6 times 10^(-4)$, against a threshold of $1.37 times 10^(-5)$. None is significant. The planted $[k equiv 0 med (3)]$ control is detected at $p <= 4.9 times 10^(-19)$ on every instance.

*Heuristic extrapolation to ECC2K-130* ($q = 2^131$, $n approx 2^129$). Any predicate studied here has a correlation with any group character of at most about $c sqrt(q) \/ n$, with $c approx 2$–$3.5$ as measured. That gives a bias of about $2^(-62)$. Detecting it takes about $2^(124)$ samples, against about $2^(60.9)$ rho iterations.

== R3 — residues as pre-filters for Semaev ($m = 2$) decomposition

For every $R in ⟨G⟩$, exactly: does $R = plus.minus P_i plus.minus P_j$ hold with $P_i, P_j$ in the factor base? The factor base is $x < B$ on prime fields (density 0.25) and $x in "span"{1, dots, t^(l-1)}$, $l = floor(m\/2)$, on binary fields (Amendment 1, pre-data).

#figure(image("report/fig2_r3_prefilter.svg", width: 100%), caption: [R3 (`results/r3.json`). Left: lift for each predicate; crosses are the planted control $1[R in D]$ flipped with probability 0.4. Right: maximum $|z|$ of the Walsh–Hadamard transform over all $2^m$ linear functionals of $x(R)$ (green), against a planted codimension-2 subspace set (black, $|z| = 13$–$210$).])

- *Prime fields, 36 tests.* The largest lift is 1.042, from bit8(x) on P16 ($z = 3.09$). For even predicates, $R$ and $-R$ are counted twice, so the honest $z$ is the recorded $z$ divided by $sqrt(2)$, here 2.19. That correction can only lower significance. Odd predicates are exactly balanced (lift 1.000) because $D = -D$.
- *Binary fields.* The per-predicate lifts of up to 1.24 come from $m = 11, 13$, where $n <= 4091$ and $p >= 0.11$. At $m = 11$ even the planted control reaches only $z = 3.2$ ($p = 1.3 times 10^(-3)$), so those rows *decide nothing*: the experiment is underpowered there. At $m = 17, 19$ the control gives $z = 28$–$73$ and the predicates give lifts $<= 1.05$.
- *WHT (the new question beyond entry C5).* No linear functional of $x(R)$ other than the trace, which is constant on $⟨G⟩$, predicts decomposability. The maximum is $|z| = 4.16$–$5.13$ over $2^(m-1) - 1$ effective functionals, all $p >= 1.2 times 10^(-3)$.
- *Protocol error, disclosed.* The protocol's expected positive-control lift of "> 3" was an arithmetic mistake. At density 0.25 and flip probability 0.4 the lift is $(0.6 dot 0.25\/0.45)\/(0.4 dot 0.25\/0.55) approx 1.83$, which is what was measured (1.79–1.87 on prime fields). Detection itself is unambiguous ($z = 45$–$179$).

== R4 — larger rho classes from endomorphisms

*Theorem (R4-thm).* Let $alpha in "End"(E)$ have norm $d >= 2$, and let its eigenvalue $lambda$ on $⟨G⟩$ have order $s$. Then $⟨G⟩ subset ker(alpha^s - 1)$ and $alpha^s != 1$. Hence $n <= deg(alpha^s - 1) = N(alpha^s - 1) <= (d^(s\/2) + 1)^2$, that is, $s >= 2 log(sqrt(n) - 1)\/log d$.

A class walk must canonicalise $s$ images, each at cost $c$ group operations. The net rho factor is then $sqrt(s)\/(1 + s c)$ (R4-cost, a heuristic model).

#figure(image("report/fig3_r4_classes.svg", width: 100%), caption: [R4 (`results/r4.json`). Left: exact orders against the bound. Generic norm-2 CM eigenvalues ($D = -7$, 20 curves) have $s = 58$–$16318$, far above the bound. Koblitz $tau$ meets it ($s = m$ against $s_min = 10$–$18$). Right: the cost model. Only near-free maps ($c lt.tilde 0.1$) gain, which leaves automorphisms (orange) and Frobenius in a normal basis (green).])

- The $j = 0$ and $j = 1728$ controls verify the eigenvalue directly ($zeta(G) = lambda G$ on 20 of 20 curves each), with $s = 3$ and $s = 4$.
- The $D = -7$ curves always have even order, because $ker alpha$ is the rational 2-torsion point $(-7, 0)$ (Amendment 2).
- *Consequence for ECC2K-130 (heuristic).* Any norm-2 class needs $s >= 129$. The only map with $c approx 0$ is $tau$, already used in the $plus.minus tau^i$ classes. A small $s$ with a cheap $alpha$ forces $n | N(alpha^s - 1)$, which is subfield structure. So rho classes add nothing new here.

== Conclusion and next steps

A residue tells you nothing about $k$:

- *Homomorphic residues* are trivial on a prime-order $⟨G⟩$. This is a theorem, checked exactly above.
- *Non-homomorphic residues* are bounded by Weil. The bound is attained, and the resulting bias is about $2^(-62)$ for ECC2K-130 (heuristic).
- *As Semaev pre-filters*, neither residue predicates nor any linear functional of $x(R)$ moves decomposability at the sizes measured.
- *For rho classes*, the subfield Frobenius is already extremal.

Open threads:

- Lean statements of R1-thm and R4-thm (not done).
- *Quadratic* functionals of $x(R)$ and $m = 3$ decompositions for R3; the linear WHT cannot see them.
- An explanation of why $chi(y)$ attains only $2 sqrt(q)$ when its sheaf has rank 4.
- R3 at $m >= 23$, to tighten the minimal detectable lift.

*Reproduction.* `cd native && cargo build --release && ./target/release/residue_leads {r1|r4|r3|r2v2} <dir>`. The binary sha256 is `460b817d…`. The replay from `78657d08` matches all five result files byte for byte (r1, r4, r3, r2_v2, aa_r2_v2). The host was a local 4-vCPU container: isolab-eks was unavailable (no AWS identity) and the S3 checkpoint could not upload (no token). Exact counts do not depend on the host. The knowledge base was unavailable (`SOURCES.md`).
