# Independent blind algebra review: square kernel

## Scope and sources

I used only the definitions and targets supplied in the review assignment. I did not inspect repository source, Lean proofs, proof maps, reports, receipts, Git history, a parent plan, or a sibling review. This is a mathematical review, not a Lean build or an assessment of any implementation.

Assume `a` is a nonnegative integer, so `N = 2^(a+1)`, `c = 2^a`, and `0 <= k,m < N` are integer indices. The denominator `P(c)` is nonzero. Write `v_2` for the 2-adic valuation on nonzero rational numbers.

## Derivation

For every valid index `k`, splitting the factors below and above `k` gives

`P(k) = (-1)^(N-1-k) k! (N-1-k)!`.

As `N-1` consists of `a+1` binary ones, the `a+1` low binary digits of `N-1-k` complement those of `k`. Legendre's formula `v_2(n!) = n-s_2(n)` therefore gives

`v_2(P(k)) = (N-1) - (s_2(k)+s_2(N-1-k)) = N-a-2`.

The same holds at `k=c`; hence **`v_2(rho(k))=0`** for every valid `k`. Equivalently, `binom(N-1,k)` is odd for every such `k`, and `rho(k)=(-1)^(c-k) binom(N-1,c)/binom(N-1,k)`.

At `t=m-c`, the factors in `B_k(t)` are `m-j`. If `m != k`, the factor for `j=m` is zero; if `m=k`, the numerator is `P(k)`. Thus

`B_k(m-c) = rho(k)` when `m=k`, and `0` otherwise.

Since `rho(k)` is nonzero, division in `S_k` is defined. In the first case, `S_k(m-c)=rho(k)^2/rho(k)=rho(k)`; in the second, it is zero. Therefore **`S_k(m-c)=B_k(m-c)`**. Taking `m=c` gives **`B_k(0)=1` if `k=c`, and `0` otherwise**, because `rho(c)=1`.

For any rational `t` with `B_k(t) != 0`, the valuation law for nonzero rationals gives

`v_2(S_k(t))=v_2(B_k(t)^2)-v_2(rho(k))=2v_2(B_k(t))`.

The nonzero condition is essential if `v_2` is defined only on nonzero rationals. A convention `v_2(0)=+infinity` would extend the identity in the extended valuation, but should not be silently mixed with a rational-valued valuation theorem. All four targets follow under the stated index and nonzero premises.

## Small cases and nearby false statements

| `a` | `N,c` | `rho(k)` for `k=0,...,N-1` | Checks |
| --- | --- | --- | --- |
| 0 | 2,1 | `[-1,1]` | `B_0(t)=t`, `B_1(t)=t+1`; at `t=-1,0` they have the required node values. Center `B_1(0)=1`, noncenter `B_0(0)=0`. |
| 1 | 4,2 | `[3,-1,1,-3]` | Each value is a 2-adic unit; the node formula gives `rho(k)` at its own node and zero at the other three. Center `rho(2)=1`. |
| 2 | 8,4 | `[35,-5,5/3,-1,1,5/3,-5,35]` | Every entry is a 2-adic unit, including the nonintegral `5/3`; the node and center arguments apply unchanged. |

The tempting replacement of row `N-1` by row `N` in the parity argument is false: `binom(2,1)=2`, while the relevant `a=0` row is `N-1=1`, with both entries odd. This is an actual counterexample to that nearby binomial-row claim, not to any of the four targets.

For `a=1, N=4, c=2`, the excluded out-of-range `k=4` gives `P(4)=24`, `P(c)=-2`, and `rho(4)=-12`, so `v_2(rho(4))=2`. This refutes a version of the unit claim without `k<N`. At `t=2`, `B_4(2)=-12` and `S_4(2)=-12`, so `v_2(S_4(2))=2` whereas `2v_2(B_4(2))=4`; the valuation identity also needs the valid-index premise. The original targets explicitly restrict `k<N`, so these are missing-premise examples for possible generalized formulations, not counterexamples to the stated claims.

## Conclusion

The four mathematical targets are valid for nonnegative integer `a`, valid indices, and nonzero `B_k(t)` where the valuation expression requires it. The algebra gives a compact proof route. I have not checked any Lean statement, implementation, generated graph, experiment, or cloud result.
