# Exact n=9 norm-fiber Riemann--Roch gate

## Outcome

The registered one-step degree-four router is algebraically functional at
n=9: it constructs certified effective representatives, produces full-rank
projected systems at B=64 in both image-compatible arms, and recovers all 16
blind logs in each arm. [pcc:unverified]

The image-compatible relation yield is nevertheless indistinguishable at this
scale from the ordinary-ambient control for cost-model purposes.  At B=64 the
transported, kernel-coset and ambient arms produced respectively 198, 171 and
187 relations from 65,024 candidates.  The registered random-label prediction
is 0.002687 useful relations/candidate; the observed probabilities are
0.003045, 0.002630 and 0.002876. [pcc:unverified]

The frozen finite-crossover condition therefore fails.  The model balances
relation collection `r^4/B^3` against sparse linear algebra `B^2` at
`B=r^(4/5)`, giving `r^(8/5)`.  For the actual n=83 prime order with exactly
81 exponent bits, this is 129.6 leading exponent bits versus 40.5 for
`r^(1/2)`, a constant-free gap of 89.1 bits.  No finite crossover is predicted,
so the registered protocol stops after n=9 and does not run n=13,19,23 or n=83.
[pcc:unverified]

## Exact instance and reconstruction

- Field: `GF(2^9)` with modulus `z^9+z+1`.
- Target: `y^2+xy=x^3+1`, exhaustively counted as 508 points.
- Prime subgroup: order 127, cofactor four, 127 distinct generator multiples.
- Kernel fixture: from `D=[(1,0)]-[infinity]`, the instrument constructed
  `K=[3]D-phi^*phi_*(D)`, checked `K != 0`, `phi_*(K)=O`, and retained
  `phi_*(D) != O` as the false-kernel control. [pcc:unverified]

For every class `X=(u0,v0)` and every constant `c`, the instrument formed

```text
V = h+v0+c*u0,
U = (f+h*V+V^2)/u0,
```

then required exact division, monic degree four and native reduction of
`(U,V)` back to `X`.  The exhaustive candidate partitions were:

| arm | candidates | certified degree 4 | split over GF(2^9) | rejected degree drop |
|---|---:|---:|---:|---:|
| transported `K=0` | 65,024 | 64,512 | 2,844 | 512 |
| fixed nonzero `K` coset | 65,024 | 65,024 | 2,781 | 0 |
| ordinary ambient control | 65,024 | 65,024 | 2,650 | 0 |
| exceptional controls | 2,048 | 512 | 54 | 1,536 |

The 512 transported rejections are exactly the identity class across all
constants.  Nonsplit candidates were counted and excluded from the B=1
relation base; none was misreported as smooth.  Every admitted split support
had total multiplicity four, all source points passed the curve equation, and
both its direct pushforward and cofactor-projected RHS agreed. [pcc:unverified]

## Rank and collision measurements

Rows are shown as `collected/rank` over `F_127`.

| B | transported `K=0` | nonzero `K` coset | ordinary ambient |
|---:|---:|---:|---:|
| 8 | 0/0 | 0/0 | 0/0 |
| 16 | 2/2 | 0/0 | 0/0 |
| 32 | 20/12 | 4/4 | 6/6 |
| 64 | 198/64 | 171/64 | 187/64 |
| 96 | 954/96 | 909/96 | 950/96 |
| 126 | 2,844/126 | 2,781/126 | 2,650/126 |

At the first full-rank bound B=64, rank/row was 0.323, 0.374 and 0.342,
respectively.  The projected collision rates were 9.60%, 3.51% and 2.81%; the
transported arm has more collisions, but not a corresponding collection or
rank advantage.  The CSR estimates were 5,068, 4,636 and 5,066 bytes.
[pcc:unverified]

The full-rank systems solved the logs of all selected projected labels without
consulting the enumerated log table; the later comparison matched every entry.
The separately invoked validator recomputed every row, rank, solution,
collision and byte count from the raw relation records.  It was written in
this session and is not an independent PCC verifier. [pcc:unverified]

## Individual logarithms

Using the first full-rank system, B=64:

| arm | verified recoveries | candidates total | candidates/target | worst target |
|---|---:|---:|---:|---:|
| transported `K=0` | 16/16 | 7,922 | 495.1 | 3,193 |
| fixed nonzero `K` coset | 16/16 | 4,356 | 272.3 | 1,143 |

The relation routine received only each point `Q` and the public mask `s`; the
planted scalar was used to construct `Q` and replay the recovered answer.
[pcc:unverified]

## Complete cost interpretation

With degree-four split probability about `1/24` and a projected factor-base
fraction `B/r`, useful probability is

```text
p(B) about (1/24)*(B/r)^4.
```

The exact B=64 measurements track this prediction in all three main arms.
Collecting enough rows costs `Theta(r^4/B^3)` candidate tests after including
the required `B` relations; sparse linear algebra is at least `Theta(B^2)`
field operations.  The individual-log search costs `Theta(r^4/B^4)` and is
lower order at the balanced B.  Factor-base construction and setup are also
lower order.  Thus:

```text
B = r^(4/5)
precomputation = r^(8/5)
individual log = r^(4/5)
```

The measured image arms show no factor-base closure effect that changes those
exponents.  Projecting an ambient relation makes its RHS valid in the image,
but does not improve its degree-four smoothness probability or useful-rank
rate. [pcc:unverified]

## Evidence and scope

- Raw certificate: `runs/exact-n9-20261118T0708Z/certificate.json`, SHA-256
  `06d35ba1234579889966faa34ae8e3abfbaae823dce74799cdbd91ccb52471ce`.
- Validator: `runs/exact-n9-20261118T0708Z/validator.json`, SHA-256
  `4e5b0c3a7f194f344b6a051febe14ba6fc12cfd117a84358457fb8fdf9061bf0`.
- The validator reports 11 passing replay groups. [pcc:unverified]

This is exhaustive only for the registered constant one-step family over all
127 subgroup inputs in `K=0` and one fixed nonzero kernel coset.  It is not an
exhaustion of the dimension-two kernel, a general Riemann--Roch basis, or an
n=83 timing result.
