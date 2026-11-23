# Item 3: the cost law the tree encoding implies, and where it would cross rho

2026-11-04 · theory note built on the measurements of rounds 1–3. **Class:
derived bound, not a measurement.** No result here is an algorithm cost; the
model is the standard dense Macaulay-matrix heuristic, and its inputs are
extrapolations from small cells, several of which are lower bounds. The
numbers are computed with the `awk` one-liners in §4 (no Python).

## 1. Which encoding can scale at all

| encoding | input degree | D measured (t = 4–5) | m ≥ 5 |
|---|---|---|---|
| F (S_{m+1} in e₁..e_m) | 2^{m−1} (2, 4, 8, 16, 32 at m = 2..6) | 5–6 (m=3), 8 (m=4) | S₆ and S₇ were never built in the lab, and D ≥ 16 at m = 5 |
| C (caterpillar) | ≤ 4 | ≥ 7 at m = 4 | bounded input, no symmetry quotient |
| T (cherry tree) | ≤ 4 | 4, 6, 7, ≥ 9, ≥ 10 at m = 2..6 | bounded input; the only measured encoding that runs at m = 5, 6 |
| G (glued: resultants of T) | 16 at m = 4 | ≥ 9 at m = 4 | dominated (round 3) |

At m ≥ 5, F's input degree alone (16, 32) exceeds T's measured D (≥ 9, ≥ 10)
already. So for m ≥ 5 the tree is the only encoding left, unless a weighted
grading rehabilitates the symmetric ones (item 2, `WEIGHTED.md`).

## 2. Model

Factor base V = g·μ_{2^t}, with about 2^{t−1} usable points; arity m; p ≈ 2^{mt}
(the balanced base, one decomposition per target on average). Per relation,
with log₂ throughout:

- PDP solve ≈ ω · log₂ C(N + D, D), with N ≈ m(t + 1) unknowns
  (the measured N is 4t + 2 at m = 4, 5t + 3 at m = 5);
- targets per relation ≈ m!, with the symmetry quotient charged as in the lab's
  convention;
- relations ≈ |F| ≈ 2^{t−1};
- linear algebra ≈ 2(t − 1), from a sparse |F|² solve.

The degree law has two parts, and both are extrapolated:

- **Arity:** D = 4 + 1.5(m − 2). This is the round-3 lower-bound slope; the
  measured points are 4, 6, 7 at m = 2, 3, 4.
- **Depth:** + β(t − 4). Here β = 0 is the optimistic case. β ≈ 0.5 matches
  T's staircase at m = 3 (6, 6, ≥ 7 at t = 4, 5, 6).

The total is the larger of (relations × PDP) and linear algebra, minimized over
m. "Ratio" below is log₂ cost / log₂ √p, so a ratio below 1 beats Pollard rho.

## 3. Result

| ω | β | 2^256 | 2^1024 | 2^4096 | 2^16384 |
|--:|--:|--:|--:|--:|--:|
| 2.0 | 0 | 1.22 | 0.66 | 0.35 | 0.18 |
| 2.0 | 0.5 | 2.33 | 1.42 | 0.84 | 0.48 |
| 2.8 | 0 | 1.49 | 0.78 | 0.41 | 0.22 |
| 2.8 | 0.5 | 3.12 | 1.91 | 1.12 | 0.65 |

At ω = 2.8, β = 0 the optimum runs m = 4 at 256 bits (cost 2^190.8 against rho's
2^128), m = 6 at 1,024 bits and m = 10 at 4,096 bits; full table in §4.

- **Asymptotically**, under these laws, the ratio falls without bound, so the
  model is subexponential, as THEORY.md §3 predicted for D = O(m). This holds
  only if the laws extrapolate. They rest on lower bounds at m = 5, 6 and on
  t ≤ 6.
- **At cryptographic size it does not come close.** At 2^256, even the most
  optimistic row (ω = 2, β = 0) costs 2^{156} against rho's 2^{128}. With the
  measured depth slope it is 2^{298}–2^{399}. The crossover with rho lies roughly between
  2^{500} and 2^{6000}, depending on the row (interpolated between the table columns).
- **What would change this:** a D law with a slope well below 1 per summand
  and β = 0. Or a PDP solver whose cost is not governed by the dense Macaulay
  size C(N+D, D), for example one that exploits the tree's sparsity or
  treewidth. The lab's SAT/treewidth leads found no exponent movement
  (`research/ic_prime_eth_leads_20261031/SYNTHESIS.md`).

**Consequence for the other items.** Confirming P2 on a big host (item 1) can
only make the asymptotic claim stronger; it cannot close the gap at 256 bits.
A swap-symmetry block split (item 4) is worth at most 2^{ω−1}, under
2 bits, against a gap of at least 28 bits even in the best row.

## 4. Commands

The best-m search with ω = 2.8 and β = 0:

```sh
awk 'function lf(n,  s,i){s=0; for(i=2;i<=n;i++)s+=log(i); return s/log(2)} function lb(n,k){return lf(n)-lf(k)-lf(n-k)}
BEGIN{ for (bits=256; bits<=65536; bits*=2) { best=1e12;
  for (m=2;m<=400;m++){ t=int(bits/m+0.999); if(t<2)continue; D=int(4+1.5*(m-2)+0.5); N=m*(t+1); c=(t-1)+lf(m)+2.8*lb(N+D,D); la=2*(t-1); c2=(la>c)?la:c;
    if(c2<best){best=c2;bm=m;bt=t;bD=D}}
  printf "2^%-6d best m=%-3d t=%-5d D=%-4d log2 cost=%8.1f  log2 rho=%8.1f  ratio=%.3f\n", bits,bm,bt,bD,best,bits/2, best/(bits/2) } }'
```

Output (recorded):

```
2^256    best m=4   t=64    D=7    log2 cost=   190.8  log2 rho=   128.0  ratio=1.491
2^512    best m=4   t=128   D=7    log2 cost=   274.0  log2 rho=   256.0  ratio=1.070
2^1024   best m=6   t=171   D=10   log2 cost=   399.0  log2 rho=   512.0  ratio=0.779
2^2048   best m=8   t=256   D=13   log2 cost=   580.0  log2 rho=  1024.0  ratio=0.566
2^4096   best m=10  t=410   D=16   log2 cost=   844.8  log2 rho=  2048.0  ratio=0.413
2^8192   best m=14  t=586   D=22   log2 cost=  1226.7  log2 rho=  4096.0  ratio=0.299
2^16384  best m=20  t=820   D=31   log2 cost=  1780.2  log2 rho=  8192.0  ratio=0.217
2^32768  best m=26  t=1261  D=40   log2 cost=  2583.1  log2 rho= 16384.0  ratio=0.158
2^65536  best m=38  t=1725  D=58   log2 cost=  3742.3  log2 rho= 32768.0  ratio=0.114
```

The table in §3 uses the same model with `W` (ω) and `beta` as variables, and
`t ≥ 4`.
