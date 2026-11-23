---
title: "A1 (math-st P2.3): exact trinomial root counts and the generic cost of Cheon-type inputs"
date: 2026-11-19
---

**Outcome.** The preregistered kill hypothesis H1 holds on every frozen input.
Across all 1,263,300 pairs $(n,d)$, the maximum trinomial root count is
$R(n,d)\le L(n,d)+10$. No pair meets the lead condition. In the generic group
model, therefore, inputs $(g,g^x,g^{x^d})$ cost
$\Theta(\sqrt{n/L(n,d)})$ up to the measured additive excess. That cost is
already reached by Cheon's algorithm and its base-changed form (below). This
search finds no new generic speed-up for this input shape. The statement is
measured for $n<2^{16}$; it is not proved for all $n$.

Protocol: `../PROTOCOLS.md` section A1, committed at `7db75ae` before the code
ran. Code: `src/main.rs`, a native Rust binary built with
`cargo build --release` and run on a 4-core cloud container. Upstream question:
math-st P2.3 [unverified assertion].

## What was computed

$R(n,d)=\max_{(a,b,c)\neq0}\#\{x\in\mathbb F_n: a+bx+cx^d=0\}$, exactly, for

- every prime $2^7\le n<2^{12}$ (533 primes) and every $d\in[2,n-2]$;
- $n\in\{65497,65519,65521\}$ and every $d$.

**Method.** Exhaustive in $d$, reduced in $b$. The substitution $x\mapsto ux$
maps the middle coefficient $b$ to $bu^{1-d}$. So one representative per coset
of $(\mathbb F_n^*)^{d-1}$, together with $b=0$, covers every trinomial up to
scaling, and each $(d,b)$ is a value-multiplicity count over $\mathbb F_n$.

**Envelope.** $L(n,d)=\max\{\gcd(d,n-1),\,1+\gcd(d-1,n-1)\}$, attained by
$X^d-1$ and $X^d-X$.

## Controls (all passed)

| control | result |
|---|---|
| 1. reduced method equals brute force over all $(a,b)$, every prime $n<200$ and every $d$ | 4,090 pairs, 0 mismatches |
| 2. $R(n,d)=R(n,d^{-1}\bmod n-1)$ when $\gcd(d,n-1)=1$ | checked on every frozen prime, 0 failures |
| 3. $R\ge L$ | checked on every row, 0 failures |

## Results

| bits of $n$ | primes | pairs $(n,d)$ | mean $R-L$ | mean over primes of $\max_d(R-L)$ | max $R-L$ | max $R$ when $L\le3$ | lead rows |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 8 | 23 | 4,292 | 0.97 | 3.52 | 5 | 7 | 0 |
| 9 | 43 | 16,338 | 1.29 | 4.63 | 6 | 8 | 0 |
| 10 | 75 | 57,416 | 1.41 | 5.32 | 7 | 9 | 0 |
| 11 | 137 | 208,576 | 1.69 | 6.01 | 8 | 10 | 0 |
| 12 | 255 | 780,150 | 1.90 | 6.60 | 9 | 11 | 0 |
| 16 | 3 | 196,528 | 2.34 | 9.33 | 10 | 12 | 0 |

All values are measured (`data/summary_bits.csv`, `data/summary.json`, written
by `a1_trinomial_roots analyze`).

![Left: excess $R-L$ against bit size; the dotted line is the lead threshold $4\log_2 n$ on $R$. Right: histogram of $R-L$ over all 1,263,300 pairs (log scale). Rendered from the recorded summary by `plot_a1.py`.](a1_excess.svg)

**Three regularities in the data:**

1. **The divisor cases are exact.** Of the 182,620 pairs with $L\ge16$ (the
   regime where Cheon-type algorithms gain), 182,618 have $R=L$ exactly. Large
   root counts come only from $X^d-1$ and $X^d-X$.
2. **Elsewhere the excess is extreme-value sized.** When $L\le3$, the largest
   $R$ is $\text{bits}-1$ at 8–12 bits and 12 at 16 bits. The mean over primes
   of the per-prime maximum excess grows by about 0.6 per bit, the size
   expected for the largest of about $n\cdot\gcd(d-1,n-1)$ value
   multiplicities of a map that behaves randomly.
3. **$\gcd(d,n+1)$ has no effect.** The maximum excess is 8–10 in every
   $\gcd(d,n+1)$ bucket (1, 2, 3, 4–15, 16–63, $\ge64$). With only $x$ and
   $x^d$ in the exponent there is no generic $n+1$ effect. Cheon's $n+1$
   variant needs many powers $x^i$, which these inputs do not supply.

**Recorded prediction, missed.** Before the 16-bit runs ended I predicted
max $R-L\in[12,14]$ and max $R$ (with $L\le3$) in $[14,16]$
(`PREDICTION_16BIT.md`, written at 06:50Z). The measured values are 10 and 12.
The prediction extrapolated maxima taken over 255 primes per size to 3
primes. The per-prime statistic in the table is the right one to extrapolate.

## Interpretation

**Lower bound (standard).** A generic algorithm making $q$ queries succeeds
with probability at most about $q^2R(n,d)/n$, so it needs
$q=\Omega(\sqrt{n/R})$.

**Upper bound (two algorithms).**

- Cheon (2006): $O(\sqrt{n/\gcd(d,n-1)}+\sqrt{\gcd(d,n-1)})$.
- The same algorithm with base $h=g^x$. Then $g^{x^d}=h^{x^{d-1}}$, and
  $x^{d-1}$ lies in the subgroup of $\mathbb F_n^*$ of order
  $(n-1)/\gcd(d-1,n-1)$. This gives
  $O(\sqrt{n/e}+\sqrt e)$ with $e=\gcd(d-1,n-1)$. This corollary is presumably
  known; it is recorded here, not claimed as new.

**Together.** With $R\le L+10$, the generic exponent for these inputs is
settled on the measured range, and both sides are already achieved.
**Kill criterion met; P2.3's "generalize Cheon to other $d$" has no generic
room** for the input shape $(g,g^x,g^{x^d})$.

**Open, and not claimed.** A proof that $R(n,d)\le L(n,d)+O(\log n)$ for all
primes $n$. Known trinomial root bounds (Kelley–Owen and others; check) are
far weaker in general. Such a proof would make this a theorem. It is the
natural Lean target once a toolchain is available. Inputs with more powers of
$x$ (Cheon's $n+1$ case, Satoh's $\Phi_k$ generalization; check) are outside
this experiment.

## Reproducing

Build and run commands, and SHA-256 hashes of the raw CSVs (about 27 MB, kept
out of Git), are in `data/MANIFEST.md`. Then:

```
./target/release/a1_trinomial_roots control
./target/release/a1_trinomial_roots analyze data/summary.json data/summary_bits.csv \
  data/sweep_128_4096.csv data/prime_65497.csv data/prime_65519.csv data/prime_65521.csv
```

Runtime on the 4-core container: 12 s for the small-prime sweep, and
22 s, 30 s and 118 s for the three 16-bit primes.
