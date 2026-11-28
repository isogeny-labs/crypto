# H116 kernel x-orbit experiment

Frozen protocol `H116-KERNEL-X-ORBIT-001` passed all eight cells.  The
independent Python implementation reproduced every exact correctness field,
all locked predictions, both sign-sensitive controls, and the retained v1/v2
artifact hashes. [pcc:unverified]

## Receipts

```text
7ebfb9e0780c3dc82db95e6391d219160043f5c434a6effcffbe0a844d52ca07  KERNEL_X_ORBIT_PROTOCOL.json
c4ca6bb3bc3fa9b717862c1e73190c9525d65fc4682a0051e15d05b54187189d  h116_kernel_x_orbit.rs
d7cd388324280b53c5daf0a20101f9a8ea99fdbd2712e9088ebcab7fd8a5008f  h116_kernel_x_orbit_verify.py
44032eb7a8943b9f380ba5ad605ff4668d71f9122562fab2fec60b69134d0645  KERNEL_X_ORBIT_RESULT.tsv
a28ccd070e5404ecf42040e20328958a642b7fb8b70a18713a757911e45cbe29  KERNEL_X_ORBIT_VERIFY.json
```

The retained split-pole v1 result remains
`d7ff21da47cd866f21fdb21422d0dbf48cd661e65e5963c32d6a2cc114faead9`;
the v2 parent remains
`d1c26c6f0a29a933fb82b64fafbc5c20531fe59122c924613de4bc46f00cb9bc`.

## Result

For cyclic `ker(V-1)` of order `N`, Frobenius acts by
`lambda=t-1`.  A denominator root `x(P)` is rational over `F_(p^d)` when
`lambda^d P=P` **or** `lambda^d P=-P`, because `x(P)=x(-P)`.  The frozen
prediction used

```text
g+ = gcd(N, lambda^d-1)
g- = gcd(N, lambda^d+1)
g0 = gcd(g+, g-)
roots = (g+ + g- - g0 - 1 + epsilon) / 2,
```

where `epsilon=1` for even `N` in this cyclic panel and zero otherwise.

| Curve | d | `lambda^d` | `g+` | `g-` | point-visible roots | all denominator roots | non-point-visible | simple/double |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| p5-a2-b1 | 2 | 4 mod 7 | 1 | 1 | 0 | 0 | 0 | 0 / 0 |
| p5-a2-b1 | 3 | 6 mod 7 | 1 | 7 | 0 | 3 | 3 | 0 / 3 |
| p5-a2-b1 | 4 | 2 mod 7 | 1 | 1 | 0 | 0 | 0 | 0 / 0 |
| p7-a1-b1 | 2 | 4 mod 5 | 1 | 5 | 0 | 2 | 2 | 0 / 2 |
| p7-a1-b1 | 3 | 3 mod 5 | 1 | 1 | 0 | 0 | 0 | 0 / 0 |
| p7-a1-b1 | 4 | 1 mod 5 | 5 | 1 | 2 | 2 | 0 | 0 / 2 |
| p11-a1-b1 | 2 | 9 mod 14 | 2 | 2 | 1 | 1 | 0 | 1 / 0 |
| p11-a1-b1 | 3 | 1 mod 14 | 14 | 2 | 7 | 7 | 0 | 1 / 6 |

Every enumerated root had multiplicity one or two exactly as classified by the
formal derivative.  The weighted multiplicities were `0,6,0,4,0,4,1,13`,
matching the frozen predictions.  No numerator vanished at a denominator root,
including the five roots whose corresponding points were not rational over the
sampled extension.

## Controls and interpretation

The plus-only model undercounted exactly the two preregistered cells:
`p=5,d=3` and `p=7,d=2`.  It matched the other six equality controls.  Adding
one to the denominator constant destroyed every original root, and the
independent replay found zero producer mismatches and zero locked-prediction
mismatches.

This repairs the interpretation exposed by the retained v1 failure: pointwise
sampling sees `lambda^d P=P`, while a rational x-denominator sees the quotient
action modulo `P~-P` and therefore also sees `lambda^d P=-P`.  It is finite
evidence for three toy cyclic kernels.  It does not prove cyclicity, identify a
scheme isogeny, prove the dual-isogeny or degree theorems, solve ECDLP, or imply
an asymptotic improvement. [pcc:unverified]
