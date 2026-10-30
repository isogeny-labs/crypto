# N11 compressed differential-pair sidecar protocol

Study ID `KUMMER-N11-COMPRESSED-SIDECAR-20261025`. Inputs are the 21 odd-prime
groups and 16 nonzero jumps per group in the N7 frozen input. This is a
proof-and-derivation packet. No target cohort is run.

Let `G` generate a cyclic group of odd prime order `r`, let `T` be an arbitrary
signed public target, and let `K(X)` be the exact quotient by `X ~ -X`. The
state is `(K(D), K(D+G), a, n)` with `D=aG+2^n T`. The initial state is
`(K(T),K(T+G),0,0)`. The branch function reads only the projective Kummer
coordinates. For branch `c in {0,1}`, set `a <- 2a+c mod r` and `n <- n+1`.
The differential recurrences are:

| c | next first class | next second class |
| --- | --- | --- |
| 0 | `K(2D)` from `xDBL(D)` | `K(2D+G)` from `xADD(D,D+G;G)` |
| 1 | `K(2D+G)` from `xADD(D,D+G;G)` | `K(2D+2G)` from `xDBL(D+G)` |

The existing source computes all three candidates before choosing a branch,
so its regular path uses three xADD calls. This packet prices that observed
path at 72 field multiplications and 108 additions, plus scalar-label
arithmetic. The quotient operations must be checked and must fail closed on
an undefined, all-zero, or mismatched result.

At a relation endpoint, take the two possible signed lifts of `K(D)` and
select the unique lift `X` satisfying `K(X+G)=K(D+G)`. Then the signed relation
is `X=aG+2^n T`; compute `2^n mod r` from the step count. A final Jacobian
equality check is required. The mathematical theorem gives zero pair-sign
ambiguities for nonzero `G` of odd prime order. It says nothing about the
frequency of exceptions in the concrete theta xADD, projection, or lift
routines. That frequency has no established all-state bound here. Thus the
cohort gate remains closed.

With an externally available iteration number, one 64-bit `a` replaces the
two 64-bit labels in the current pair sidecar: 72 rather than 80 bytes. If
the iteration number must be stored, both take 80 bytes. This is a storage
comparison, not a measured speedup. N7's fixed-offset state is a different
transition model and is included only as a scope-labeled reference.
