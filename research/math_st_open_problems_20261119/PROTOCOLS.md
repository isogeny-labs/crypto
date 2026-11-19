# Preregistered protocols (written before any run)

Committed before the code of each attack is run. Changes after a run are
appended as dated amendments, never edited in place.

## A1. P2.3: maximum trinomial root counts R(n, d)

**Question.** In the generic group model, an algorithm given
$(g, g^x, g^{x^d})$ in a group of prime order $n$ learns only collisions among
elements $g^{a+bx+cx^d}$. Each collision makes $x$ a root of a nonzero
trinomial $a+bX+cX^d$. With $q$ group operations, the success probability is
at most about $q^2 R(n,d)/n$, where
$R(n,d)=\max_{(a,b,c)\ne 0}\#\{x\in\mathbb F_n : a+bx+cx^d=0\}$.
So the generic lower bound is $\Omega(\sqrt{n/R(n,d)})$. Cheon's algorithm
reaches $\sqrt{n/\gcd(d,n-1)}$ when $d\mid n-1$.

**Known lower envelope** (elementary):
$R(n,d)\ge L(n,d)=\max\{\gcd(d,n-1),\ 1+\gcd(d-1,n-1)\}$, from $X^d-1$ and
$X(X^{d-1}-1)$.

**Hypothesis H1 (kill).** $R(n,d)\le L(n,d)+O(\log n)$ for every $d$. Then the
generic bound is pinned at $\sqrt{n/L}$ up to logarithmic factors, and no
further generic speed-up exists for this input shape.

**Alternative (lead).** There exist $d$ with $R(n,d)\ge 4L(n,d)$ and
$R(n,d)\ge 4\log_2 n$. Such $d$ form a candidate family for a new attack.

**Method.** Exact, exhaustive. For $c=1$ and the scaling $x\mapsto ux$,
$x^d+bx+a$ maps to $x^d+bu^{1-d}x+au^{-d}$. Representatives of $b$ are therefore
$0$ and one $b$ per coset of $(\mathbb F_n^*)^{d-1}$, i.e. $\gcd(d-1,n-1)$ of
them. For each $(d,b)$, $R$ is the largest multiplicity of a value of
$x\mapsto x^d+bx$ on $\mathbb F_n$. The $c=0$ trinomials have at most one root.
Native Rust, using a discrete-log table for $x^d$.

**Frozen inputs.** Every prime $n$ with $2^{7}\le n<2^{12}$, all
$d\in[2,n-2]$; and the primes $n\in\{65521, 65519, 65497\}$ (just under
$2^{16}$), all $d$.

**Controls.**
1. A brute-force count over every $(a,b)$ for primes $n<200$ must agree with
   the reduced method.
2. $R(n,d)=R(n,d^{-1}\bmod (n-1))$ whenever $\gcd(d,n-1)=1$.
3. $R(n,d)\ge L(n,d)$ always.

**Outputs.** Per $(n,d)$: $R$, $L$, the attaining $b$, and $\gcd(d,n+1)$.
Summary: the distribution of $R-L$ and of $R/L$, and every $(n,d)$ meeting the
lead condition.

**Stop.** All frozen inputs done, or a control fails (then fix and rerun, and
record the failure).

## A2. P1.2/Q004: small roots of Semaev polynomials modulo p

**Question.** For $E/\mathbb F_p$ and a target $x_3=x(R)$, find $x_1,x_2$
with $|x_i|\le p^{\alpha}$ and $S_3(x_1,x_2,x_3)\equiv 0\pmod p$. The same
with $S_4$ and three unknowns. Counting thresholds: a random target has about
one such decomposition at $\alpha=1/2$ ($S_3$) and $\alpha=1/3$ ($S_4$).

**Hypothesis H2 (kill).** Coppersmith/Howgrave-Graham lattices recover planted
small roots only for $\alpha<0.6\times$ the threshold ($<0.30$ for $S_3$,
$<0.20$ for $S_4$), with no upward trend as the lattice dimension grows within
the frozen range.

**Lead.** $\alpha_{\max}\ge 0.6\times$ the threshold at some frozen size.

**Scope caveat, recorded before the run.** Even complete success would not beat
Pollard rho for index calculus. A factor base of size $p^{1/m}$ needs linear
algebra of about $p^{2/m}$, which beats $p^{1/2}$ only for $m\ge5$, i.e.
$S_6$ with 5 unknowns and degree 16 in each, far outside lattice reach. A2
settles Q004's coordinate-aware decoder question; it is not an IC speed-up
attempt.

**Method.** Planted instances: random curve, random small $x_1,x_2$ (and
$x_3$ for $S_4$) with points on $E$, target $x(P_1\pm P_2)$. Standard shift
polynomials $x^iy^jp^{k}f^{\ell}$, LLL, then resultants or root-finding on the
short vectors. Native Rust.

**Frozen inputs.** $p$ of 32, 48 and 64 bits; 20 planted instances per
$(p,\alpha)$ cell; $\alpha$ on a grid of step $0.02$; lattice parameters up to
dimension about 150.

**Stop.** The grid is done, or two consecutive dimension increases give no
improvement in $\alpha_{\max}$.

## A3. P5.2/Q025: orbit collisions as a residue character of the logarithm

Pen-and-paper, with Lean where feasible. Preregistered claims:

- **(A)** Let $\omega\in\operatorname{End}(E)$ have degree $d\ge2$ and not be
  an automorphism, and act on $\langle P\rangle$ (prime order $r$) by $\lambda$
  of multiplicative order $m$. Then $r\mid\deg(\omega^m-1)\le(d^{m/2}+1)^2$,
  so $m\ge\log r/\log d-O(1)$.
- **(B)** Reduction: for $H=\langle\lambda\rangle$ of index $k$ in
  $\mathbb F_r^*$, deciding whether $P,Q$ lie in one $H$-orbit is computing
  whether $\chi_k(\log_G P/\log_G Q)=1$ for the order-$k$ character $\chi_k$.
- **(C)** In the generic group model with an oracle for $\omega$ (which acts
  linearly on labels), predicting $\chi_k(\log P)$ with advantage $\varepsilon$
  needs $\Omega(\sqrt{\varepsilon r})$ operations.

**Kill (closes Q025 in the generic model).** (A)–(C) proved.
**Lead.** A gap in (C): an oracle-model or curve-specific route to
$\chi_k(\log P)$ faster than $r^{1/6}$.
