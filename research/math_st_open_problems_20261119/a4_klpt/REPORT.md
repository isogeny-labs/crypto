---
title: "A4 (math-st P3.3/Q099): basic KLPT against the exact least ℓ-power norm"
date: 2026-11-23
---

Protocol: `../PROTOCOLS.md`, section A4 (frozen; written before any KLPT code).
Code: native Rust in this directory (`src/`, run by `run.sh`). Every number below
comes from `results/runs.csv` and `results/summary.json`, both written by the Rust
binary. `plot.py` only draws recorded values. Input: math-st `f631bcc`,
`measure_power_targets_p2203-245000047x18_t6_s33032028_20260713_raw.csv`
(sha256 `058485bd…830f`, in `results/input.sha256`).

![KLPT output norms (filled, 5 runs per ideal) and the exact least $\ell^e$ norm (open), against $p$. Dashed: $p$, $p^3$, $p^{3.5}$. Drawn by `plot.py` from `results/runs.csv`.](klpt_vs_optimum.svg)

## Verdict

**H4 is not met as stated: the medians sit just above its range.** In every
band and for both $\ell$, the median of $\log_p\ell^e$ is 3.73 to 4.01. H4
predicted 3.0 to 3.7. **No lead:** no median is below 2.5, and the
KLPT/optimum ratio grows like $p^{2.62}$ ($\ell=3$) and $p^{2.66}$ ($\ell=2$).
Both are above the lead threshold $p^2$ and close to H4's $p^{\approx2.5}$. Basic
KLPT is as far from the counting optimum on these ideals as its analysis says,
or a little further. The overshoot shrinks with $p$ (28-bit band, $\ell=2$:
3.725) and comes from logarithmic factors and from rounding to a power of
$\ell$ (see "Where the exponent comes from"). That fits the $p^{7/2}$
asymptotic of the original algorithm.

## Controls

| Control | Result |
|---|---|
| 1. Our ideal $I=\mathcal O_0\alpha+\mathcal O_0\ell'$ is a left ideal of norm $\ell'$ (HNF index $\ell'^2$, left closure) | 108/108 rows |
| 1. Exact $\min\mathrm{nrd}(x)/N(I)$ (exact LLL, then certified Fincke–Pohst enumeration in exact rationals) equals upstream `unconstrained_norm` | **108/108 rows** (`results/control1.csv`) |
| KLPT runs (105 usable rows × $\ell\in\{2,3\}$ × 5 seeds) | 1050 |
| KLPT gave up | 0 |
| $\beta\notin I$ (coordinates in the HNF basis of $I$ not integral) | 0 failures |
| $\mathrm{nrd}(\beta)/N(I)\ne\ell^e$ exactly | 0 failures |
| $I\bar\beta/N(I)$ not a left $\mathcal O_0$-ideal of index $\ell^{2e}$ | 0 failures |
| Output ideal contained in $\ell\mathcal O_0$ (non-primitive) | 0 |

The rows left out are the three with an empty optimum: file lines 67
(`power2_norm`), 96 (`power3_norm`) and 101 (`power2_norm`). The CSV was parsed
as RFC 4180. Two rows hold quoted fields that contain commas, and the parser
asserts that every row has all 33 header fields.

## Results per bit band

Pooled over all runs (5 per ideal). $\log_p$ ratio $=\log_p(\ell^e/\text{optimum})$.

| bits | $\ell$ | runs | median $\log_p\ell^e$ | IQR | median $\log_p$ ratio | IQR | median $\log_p N$ | median $\log_p\ell^{e_0}$ | median $\log_p\ell^{e_1}$ |
|---|---|---|---|---|---|---|---|---|---|
| 12 | 2 | 180 | 3.962 | 3.793–4.050 | 3.236 | 2.976–3.428 | 0.558 | 0.759 | 3.204 |
| 12 | 3 | 180 | 4.013 | 3.875–4.139 | 3.277 | 3.074–3.568 | 0.561 | 0.809 | 3.235 |
| 20 | 2 | 175 | 3.774 | 3.656–3.875 | 2.958 | 2.828–3.060 | 0.529 | 0.705 | 3.089 |
| 20 | 3 | 175 | 3.828 | 3.698–3.908 | 3.014 | 2.876–3.117 | 0.526 | 0.727 | 3.111 |
| 28 | 2 | 170 | 3.725 | 3.652–3.773 | 2.861 | 2.799–2.966 | 0.518 | 0.655 | 3.064 |
| 28 | 3 | 170 | 3.770 | 3.674–3.832 | 2.901 | 2.828–2.971 | 0.520 | 0.683 | 3.099 |

Least-squares fit over all runs (525 per $\ell$) against $\log_2 p$:

| $\ell$ | slope of $\log_2(\ell^e/\text{opt})$ | slope of $\log_2\ell^e$ |
|---|---|---|
| 2 | 2.660 | 3.611 |
| 3 | 2.621 | 3.614 |

## Where the exponent comes from

$e=e_0+e_1$. Translating back through step (a) adds nothing to $e$: with
$J=I\bar\delta/N(I)$ and $\beta\in J$, the element $x=\beta\delta/N$ lies in $I$
and has $\mathrm{nrd}(x)/N(I)=\ell^{e_0+e_1}$.

- **Step (a).** $N\approx p^{0.52\text{–}0.56}$. That is the shortest prime
  normalized norm among small combinations of the reduced basis, a little above
  $\sqrt p$.
- **$e_0$.** Rule: the least $e_0$ with $N\ell^{e_0}\ge p\ln p$, so that about
  $\pi\ln p$ pairs $(c,d)$ are available. This costs $\ell^{e_0}\approx p\ln p/N$,
  i.e. $\log_p\approx0.66$–$0.81$. Of that, the $\ln p$ safety factor
  contributes $\log_p\ln p\approx0.26$ (12 bits), $0.19$ (20 bits) and
  $0.15$ (28 bits).
- **$e_1$.** Rule: the least $e_1$ with $\ell^{e_1}\ge pN^4/2$. In the original
  variant $(c,d)$ is any solution of the linear congruence reduced mod $N$, so
  $|c'|,|d'|\le N^2/2$ and every candidate gives $R\ge0$. This costs
  $\log_p\approx1+4\log_pN\approx3.06$–$3.24$. A lattice (CVP) choice of $(c,d)$,
  as in later variants, would give $\ell^{e_1}\approx pN^3$ instead. That
  variant was not run.
- Rounding up to a power of $\ell$ adds up to $\log_p\ell$ per exponent. A
  residuosity parity bump adds one more $\ell$; it happened in 128 runs for
  $\ell=2$ and 125 runs for $\ell=3$.

So the median sits at about $3.5+3\cdot(\log_pN-\tfrac12)+\log_p\ln p+$ rounding.
Without the $\ln p$ factor in the $e_0$ rule, the medians would be about 0.15 to
0.26 lower: 28-bit and 20-bit bands inside $[3.0,3.7]$, 12-bit band at or just above its top.
This is an estimate from the decomposition; it was not run. The rule was fixed
before the run and is part of the measured algorithm, so the verdict above
stands. The sensitivity is reported, not used.

## Retries (all runs succeeded; counts over 525 runs per $\ell$)

| | $\ell=2$ | $\ell=3$ |
|---|---|---|
| runs with any retry | 361 | 375 |
| runs whose $e$ exceeds the rule's $e_0+e_1$ (max excess) | 132 (2) | 146 (3) |
| $\gamma$ pairs tried | 5958 | 6165 |
| $\gamma\in\ell\mathcal O_0$ rejected | 287 | 152 |
| $\ell=2$: $\gamma(1+i)\in2\mathcal O_0$, or parity impossible | 335 | – |
| $[C:D]$ with $C^2+D^2\equiv0\pmod N$ | 4 | 1 |
| residuosity failed and $\ell$ is a square mod $N$ (new $\gamma$) | 282 | 232 |
| $e_1$ parity bumps ($\ell$ not a square mod $N$) | 128 | 125 |
| $e_0$ bumps / $e_1$ bumps | 4 / 2 | 33 / 1 |
| $\beta\in\ell\mathcal O_0$ rejected in step (d) | 0 | 261 |
| step (a) needed box $[-2,2]^4$ | 55 | 55 |

No step-(c) system was inconsistent: every $\gamma$ gave a projective solution $[C:D]$.

## The $\ell=2$ handling

$\mathbb Z\langle i,j\rangle$ has lattice index 4 in $\mathcal O_0$, and modulo
$2\mathcal O_0$ it reduces to $\mathbb F_2[i]$, because $j\equiv1$ and $k\equiv i$.
So every $\mu\in\mathbb Z\langle i,j\rangle$ of even norm is $\equiv0$ or $\equiv1+i$
mod $2\mathcal O_0$. Since $(1+i)^2=2i$, a product $\gamma\mu$ of two such
elements always lies in $2\mathcal O_0$. The output ideal would then contain the
factor $2\mathcal O_0$: a backtracking path, with the norm inflated by 4 for each
such factor. The plain $\mathbb Z\langle i,j\rangle$ algorithm therefore cannot
give a primitive output for $\ell=2$. We handled it explicitly:

1. $\gamma$ is taken in all of $\mathcal O_0$:
   $\gamma=(X+Yi+uj+vk)/2$ with $X\equiv u$, $Y\equiv v\pmod 2$ and
   $X^2+Y^2=4N2^{e_0}-p(u^2+v^2)$. A $\gamma$ is rejected when
   $\gamma\in2\mathcal O_0$ or $\gamma(1+i)\in2\mathcal O_0$.
2. $\mu=N(a+bi)+c'j+d'k$ stays in $\mathbb Z\langle i,j\rangle$. It is primitive
   exactly when $c'+d'$ is odd, with $(a,b)$ ordered so that $a\not\equiv c'$
   (mod 2). Then $\mu\equiv1+i$ and $\beta\equiv\gamma(1+i)\not\equiv0$ mod
   $2\mathcal O_0$. Candidates with $c'+d'$ even are skipped (3125 skips).
3. Residuosity: $\lambda^2\equiv2^{e_1}/(p(C^2+D^2))\pmod N$. If the right-hand
   side is a non-square and 2 is a non-square mod $N$ ($N\equiv\pm3\bmod8$), then
   $e_1$ goes up by 1. Otherwise a new $\gamma$ is drawn.

For $\ell=3$ the analogous trap is that 3 is inert in $\mathbb Z[i]$: when the
sampled $(c,d)\equiv(0,0)\pmod3$, Cornacchia forces $a\equiv b\equiv0$, so
$\gamma\in3\mathbb Z\langle i,j\rangle$. Such $\gamma$ are rejected at once.
Every accepted output was checked primitive: its ideal is not contained in
$\ell\mathcal O_0$.

## Deviations from the protocol and implementation choices

1. **$\ell=2$: $\gamma\in\mathcal O_0$, not $\mathbb Z\langle i,j\rangle$** (step (b)
   of the protocol), for the index reason above. $\ell=3$ uses
   $\gamma\in\mathbb Z\langle i,j\rangle$ as written.
2. **Primitive outputs only.** $\gamma\in\ell\mathcal O_0$ and
   $\beta\in\ell\mathcal O_0$ are rejected and retried. The protocol does not
   require this. Rejection never changes the exponent rule. Upstream's optimum
   does not require primitivity.
3. **The exponent rules are ours:** $N\ell^{e_0}\ge p\ln p$ and
   $\ell^{e_1}\ge pN^4/2$, plus the bumps above. The protocol asked for "the
   standard rule" without fixing constants. Sensitivity is in the previous
   section.
4. **Step (a)** draws random-order combinations from $\{-1,0,1\}^4$ of the
   LLL-reduced basis, widening to $\{-2,\dots,2\}^4$ when none has a prime norm
   (55 runs per $\ell$). It accepts the first odd prime $N\ne\ell,p$.
5. **Sums of two squares.** Cornacchia is applied to $R=s\cdot q$, where $s$ is
   smooth over primes up to 1000 and $q$ is 1 or a prime $\equiv1\bmod4$. The
   textbook form requires $R$ prime. This affects only how many candidates
   succeed, not $e$.
6. **$\mu_0$** is parametrized as $Cj+Dk=(C+Di)j$ instead of $j(C+Di)$; this is
   the same set ($j(C-Di)$).
7. **Original-variant strong approximation:** $(c,d)$ is reduced mod $N$
   (centered $c',d'$ mod $N^2$), not chosen by CVP.
8. **Seeds and runs:** 5 runs per ideal and $\ell$; ChaCha20 seeded from 20261123,
   the file line, $\ell$ and the run index. Summary statistics pool the runs.
   The ratio-growth test is an OLS slope over all runs; the protocol did not name
   a method.
9. **Control 1 covers all 108 rows**, not only the 105 usable ones.
10. **Primality:** Miller–Rabin with the first 12 primes as bases (plus 8 more).
    This is deterministic below $3.3\cdot10^{24}$. The largest number tested is
    bounded by $\ell^{e_1}/N^2<2^{62}$ over all runs.
11. **Bug found and fixed before the final run.** The first run lifted $\lambda$
    with $C^2+D^2$ reduced mod $N$ in the numerator $(\ell^{e_1}-p\lambda^2(C^2+D^2))/N$.
    The internal exactness assertion caught it ("remainder not divisible by
    $N^2$", 206/210 runs), so no output was produced. A second run had no
    $\gamma$-primitivity filter. It was slow for $\ell=3$ (up to 11 s per run, 57k
    rejected $\beta$), and its outputs, which all passed controls, are superseded.
    The numbers here are from the final run only (`run.sh`).

## Runtime

Rust, release build, rayon. Control 1 on 108 ideals: 0.25 s. All 1050 KLPT runs
with controls: 0.54 s wall in total; the slowest single run took 0.010 s.

## Files

`Cargo.toml`, `src/{arith,quat,klpt,main}.rs`, `run.sh`, `results/runs.csv`
(one row per run, including the output element's order coordinates),
`results/control1.csv`, `results/summary.json`, `plot.py`, `klpt_vs_optimum.svg`,
`render.sh`, `REPORT.pdf`.
