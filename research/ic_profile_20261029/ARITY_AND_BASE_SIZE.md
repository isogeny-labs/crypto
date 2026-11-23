# Arity does not buy what it looked like it would

Exact folded-key coverage on the 20-bit profile curve, contiguous bases of
`N` abscissae, 2-sum and 3-sum sets (`examples/fb_arity.rs`):

| N | signed pts | 2-sum keys | 2-sum cov | 3-sum keys | 3-sum cov |
|--:|--:|--:|--:|--:|--:|
| 24 | 48 | 576 | 0.102% | 9,631 | 1.712% |
| 32 | 64 | 1,023 | 0.182% | 21,969 | 3.906% |
| 45 | 90 | 2,011 | 0.358% | 56,262 | 10.003% |
| 64 | 128 | 4,053 | 0.721% | 132,473 | 23.554% |
| 90 | 180 | 7,980 | 1.419% | 232,985 | 41.425% |
| 128 | 256 | 15,922 | 2.831% | 279,729 | 49.736% |

**Retraction.** I predicted a ~35% `S` gain from switching the prime oracle to
m = 3, on the grounds that coverage grows by a factor of |F|. That was wrong:
the m = 3 loop costs one probe per base point, so the coverage gain and the
probe cost grow together:

```
probes per relation   m=2: 1 / cov2          m=3: |F| / cov3
N=45                  m=2: 280               m=3: 900
N=128                 m=2: 35                m=3: 515
```

Both scale as `r / N^2`; m = 3 only pays a larger constant. At every size in
the table m = 2 is cheaper in probes, so the m = 2 vs m = 3 run is not worth
its cloud time.

**The size law.** For m = 2 the relation phase is `~ r / N^2` probes and setup
is `~ c N` (measured c = 2,070/45 = 46 GAE per abscissa), so
`S(N) ~ (r/N^2 + cN) / sqrt(r)` and `N* ~ (4r/c)^(1/3)`. For this curve that is
`N* ~ 37`; the measured optimum is 45, and the sweep's 32/45/56/64 shape is the
flat basin around it. The exponent `r^(1/3)` is the usual index-calculus base
size, reproduced from measurement rather than assumed.

So at 20 bits the factor-base question is answered on all three axes —
selection (+0.6%), size (optimum 45), arity (m = 2 cheaper than m = 3) — and
none of them is worth a session. The remaining lever is not the base: it is the
cost per probe, or a relation mechanism that does not pay one probe per base
point (the large-prime collector, still unbuilt on the prime side).
