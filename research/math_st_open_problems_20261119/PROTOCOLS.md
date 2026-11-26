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

## Amendment 2026-11-19 (after the A2 run): readings applied to H2

Recorded after the run, not before; the measurements do not depend on them.

1. "Recovers" was undefined. $\alpha_{\max}$ is the largest grid $\alpha$ with
   at least 10 of 20 planted roots recovered. The 20/20 and 1/20 readings
   (in `a2_semaev_small_roots/results/summary.json`) move it by at most one
   grid step and never past 0.14.
2. The grid had no upper end. Each row stopped after three consecutive 0/20
   cells; every cell below that was run.
3. "No upward trend": $\alpha_{\max}$ for $S_3$ rises from dimension 9 to 49
   (81 at 64 bits) and is flat at 0.14 up to 121. The kill is read as "no
   improvement at the top of the frozen range". A reader who counts the early
   rise as a trend would call the trend clause unmet; the lead condition is
   unmet either way.
4. $S_4$ has a single box lattice within dimension about 150 (dimension 125;
   the next is 729), so its trend clause is untestable.

## A4. P3.3/Q099: basic KLPT against the exact least ℓ-power norm (2026-11-23)

Written before any KLPT code exists.

**Question.** On upstream's 108 matched near-$p$ ideals (math-st `f631bcc`,
`measure_power_targets_p2203-245000047x18_t6_s33032028_20260713_raw.csv`), how
far above the exact least $2^e$ and $3^e$ equivalent norms ($\approx p/12$,
see `P3.3_counting_check.md`) does a basic KLPT land?

**Setting (upstream's conventions).** $B=(-1,-p)$ with $p\equiv3\pmod 4$,
$i^2=-1$, $j^2=-p$, $k=ij$. The maximal order is
$\mathcal O_0=\mathbb Z\langle1,i,(1+j)/2,(i+k)/2\rangle$. The input is
$I=\mathcal O_0\alpha+\mathcal O_0\ell'$, with $\alpha$ given by the CSV column
`alpha_order_coordinates` and the prime $\ell'$ by `input_ideal_norm`.

**Hypothesis H4 (prediction, not a kill).** Basic KLPT (Kohel–Lauter–Petit–
Tignol 2014) returns an equivalent ideal of norm $\ell^{e}$ with
$\log_p \ell^{e}\in[3.0,3.7]$ (median per bit band). The exact optimum has
$\log_p\approx1$. So the KLPT/optimum ratio grows like $p^{\approx2.5}$.

**Surprise (lead).** A median $\log_p\ell^e<2.5$ in any band, or a ratio
growing more slowly than $p^{2}$. Either would mean basic KLPT is much closer
to the counting optimum on these instances than its analysis says.

**Method.** Native Rust, num-bigint.
1. Rebuild each $I$. Control: our exact shortest-vector norm $\min
   \mathrm{nrd}(x)/N(I)$ must equal upstream's `unconstrained_norm` on all 105
   usable rows.
2. KLPT: (a) an equivalent ideal $J$ of prime norm $N$ from a short element of
   $I$; (b) $\gamma\in\mathbb Z\langle i,j\rangle$ with
   $\mathrm{nrd}(\gamma)=N\ell^{e_0}$, via Cornacchia; (c) $\mu=j(C+Di)$ with
   $\gamma\mu\in J$ modulo $N$; (d) strong approximation lifting $\mu$ to
   norm $\ell^{e_1}$; (e) translate back to $I$. Record $e=e_0+e_1$ (plus the
   step (a) translation) and every retry.
3. $\ell\in\{2,3\}$, each instance, fixed seeds.

**Correctness controls (every output).** $\beta\in I$ exactly, by coordinates
in an HNF basis of $I$. $\mathrm{nrd}(\beta)/N(I)=\ell^e$ exactly. The equivalent
ideal $I\bar\beta/N(I)$ is a left $\mathcal O_0$-ideal of norm $\ell^e$. An
output that fails any check counts as a failure, never as a result.

**Outputs.** Per instance: $p$, $\ell'$, $\ell$, $e_{\text{KLPT}}$,
$\log_p\ell^{e}$, the exact optimum from upstream, the ratio, retries and time.
Summary per bit band with median and IQR. Report, graph and PDF.

**Stop.** All 105 instances × 2 values of $\ell$ done, or a correctness
control fails (fix, rerun, record).

## A5. KLPT with CVP strong approximation, paired with A4 (2026-11-24)

Written before any A5 code exists. A4's code and results stay frozen.

**Question.** A4's step (d) reduces $(c',d')$ modulo $N$, so $p(c'^2+d'^2)$
can reach about $pN^4$. Its measured $e_1$ part was $\log_p\ell^{e_1}\approx
3.06$–$3.24$ (A4 summary). The solutions $(c',d')$ form a translate of a
rank-2 lattice of determinant $N^3$, $\lambda(C,D)+N\cdot\{(x,y):Cx+Dy\equiv0
\bmod N\}$. Taking its points closest to the origin (CVP on a Gauss-reduced
basis, then enumerating in increasing norm) needs only
$\ell^{e_1}\gtrsim pN^3$. How much of the gap to the counting optimum does
that recover on the same instances?

**Hypothesis H5 (paired prediction).** Same 105 rows, ℓ∈{2,3}, same seeds as
A4. Per bit band, the median of $\log_p\ell^{e}$ drops relative to A4 by
between 0.40 and 0.65, the size of $\log_p N$ (A4: 0.52–0.56). So the 28-bit
medians land in $[3.05,3.35]$, and the KLPT/optimum ratio grows like
$p^{2.0\text{–}2.3}$.

**Falsifiers.**
- A drop below 0.30 in any band means the CVP step is not effective: a bug or
  an analysis gap. Investigate before reporting.
- A drop above 0.80 is unexplained by the $N^4\to N^3$ change. Investigate
  that too.

**Method.** Copy A4 into `a5_klpt_cvp/`. Change only step (d) and the
$e_1$ rule: least $e_1$ with $\ell^{e_1}\ge pN^3$, with the same bump
policy. Candidates are lattice points in increasing norm with
$p(c'^2+d'^2)\le\ell^{e_1}$. Steps (a)–(c), (e), the controls and the seeds
are unchanged.

**Controls.** All of A4's exact output controls, on every run. Control 1 is
rerun as well. A paired table per instance: same $N$ and same $\gamma$ as A4
where the random stream allows. Any difference in N is reported.

**Stop.** All 1050 runs done, or a control fails (fix, rerun, record).

## A6. KLPT CVP: headroom in the e₁ rule (2026-11-25)

Written before any A6 code exists. A5's code and results stay frozen.

**Question.** In A5 the rule $\ell^{e_1}\ge pN^3$ leaves a search ball of
about $\pi$ lattice points. After the sum-of-two-squares and parity filters,
$e_1$ often bumps (340 and 237 bumps over 525 runs per ℓ). Does a rule with
headroom, $\ell^{e_1}\ge\kappa pN^3$, give a smaller final norm?

**Arms.** $\kappa\in\{1,2,4,8,16\}$. The $\kappa=1$ arm reproduces A5
exactly and is the control: its runs must equal A5's in every column except
wall time. Same 105 rows, ℓ∈{2,3}, same seeds. Only the rule changes.

**Hypothesis H6.** Raising $\kappa$ by a factor $f$ costs at most
$\lceil\log_\ell f\rceil$ in $e_1$ and saves bumps. Prediction: the best
$\kappa$ improves the 28-bit median $\log_p\ell^e$ over A5 by between 0.00 and
0.05. No $\kappa$ improves it by more than 0.10. Large $\kappa$ ($\ge8$) is
worse than A5.

**Falsifiers.**
- An improvement above 0.10 means bumps cost more than the ball-size model
  says. Investigate.
- If the $\kappa=1$ control differs from A5, stop: something changed besides
  the rule.

**Outputs.** Paired medians per (band, ℓ, κ), bump counts, report, graph and
PDF.

## A7. P1.6/Q018: xedni p=17 Experiment A under explicit samplers (2026-11-25)

Written before any A7 code exists.

**Question.** Jacobson–Koblitz–Silverman–Stein–Teske (2000) report 317
dependent cases in 100,000 runs of Experiment A at $p=17$. Upstream A002 could
not reproduce this because the sampler is unspecified and no certified
dependence test was available. Is the published rate robust to the choice of
sampler, or specific to it?

**Setting.** Taken from upstream `refs/jacobson-et-al2000.md` and A002: the
curve $y^2=x^3+2x+2$ over $\mathbb F_{17}$, of order 19, generator $(3,1)$,
with $P_1\ne\pm P_2$. The point count and lattice steps are as in the paper's
§5.4, as read by the implementer, who records the reading.

**Samplers (each stated fully in code and report).**
- S1: projective lifts uniform in a fixed box; nearest coefficient-lattice
  vector.
- S2: shortest projective lifts; smallest-discriminant curve in a fixed
  neighbourhood (upstream A002's choice).
- S3: projective lifts uniform in the box; coefficient vector uniform in the
  same neighbourhood.

**Dependence test.**
- *Dependent:* an exact rational relation with coefficients bounded by $B$ is
  found and verified in $E(\mathbb Q)$ with exact arithmetic.
- *Independent:* the height-pairing regulator is certified above its error
  bound.
- *Undecided:* everything else. The undecided rate is always reported, never
  dropped.

**Hypothesis H7 (kill, from the triage).** The dependent rates of S1–S3 spread
by more than 3×, with non-overlapping 95% Wilson intervals. Then the published
317/100,000 is specific to its sampler, and Q018 closes as unanswerable as
posed.

**Alternative.** All rates are within 3× of each other. Then compare them with
317/100,000.

**Runs.** 100,000 per sampler if affordable; otherwise at least 20,000 with
Wilson intervals. Fixed seeds. Native Rust.
