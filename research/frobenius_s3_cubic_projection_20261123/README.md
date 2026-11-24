# Cubic trace multipliers for the binary S3 selector

At field degree 23, cubic Frobenius-equivariant trace multipliers span the 13-dimensional left kernel of the normalized two-coordinate equation at every nonzero sum, for both tested Frobenius subspaces. At field degree 47 the same family has pointwise rank 4, while the left kernel has dimension 25. The 47-bit result follows from a symbolic span certificate for both subspaces. [pcc:unverified]

| Field degree | Selector dimension | Cubic equivariant space | Pointwise rank | Full left-kernel dimension |
| ---: | ---: | ---: | ---: | ---: |
| 7 | 3 | 6 | 5 | 5 |
| 23 | 11 | 35 | 13 | 13 |
| 47 | 23 | 50 | 4 | 25 |

## Construction

Let \(\tau(u)=u^2\) on \(K=\mathbb F_{2^n}\). For each tested field, take \(U=\ker g(\tau)\), where \(g\) is one of the two odd-degree factors of \((z^n+1)/(z+1)\) over \(\mathbb F_2\). Write \(k=\dim U=(n-1)/2\). The verifier checks that \(U\) is trace zero and totally isotropic under the trace pairing, so \(U^\perp=U+\mathbb F_2\). The Frobenius coordinate functions are \(Z_i(s)=s^{2^i}\) for \(0\le i<k\). Moore invertibility makes them a basis for linear coordinate functions over \(K\). Products with distinct indices give a basis for the filtered polynomial functions, since squares reduce to linear forms. Frobenius equivariance restricts their coefficients to \(\mathbb F_2\).

A multiplier \(\alpha\) annihilates the unknown \(x\) in \(x^2+sx=p(s)\) exactly when \(\sqrt\alpha+s\alpha\in U^\perp\). The symbolic calculation squares this expression and applies \(g(\tau)(\tau+1)\). It uses the function-ring reductions \(Z_i^2=Z_{i+1}\) and \(Z_{k-1}^2=\sum_{i<k}g_iZ_i\). The resulting matrix has 232 columns at degree 23 and 2,048 columns at degree 47. Construction uses these polynomial coordinates without enumerating the elements of \(U\). The field-arithmetic check independently verifies the multiplier condition at every evaluated representative. [pcc:unverified]

For \(s\ne0\), the kernel of \(x\mapsto x^2+sx\) on \(U\) is \(\{0,s\}\), so the left kernel has dimension \(n-k+1\). The pointwise rank is constant on each Frobenius orbit by equivariance. At 7 and 23 bits the verifier visits every nonzero orbit. At 47 bits it samples 128 nonzero sums per subspace and checks the following symbolic certificate over the entire function space. [pcc:unverified]

## Span certificate at 47 bits

Write \(\alpha_0=1\) and, for \(1\le r\le3\),

\[
\alpha_r(s)=\sum_{j=0}^{r-1} Z_{-j}(s)\prod_{i=1}^{r-j-1}Z_i(s),
\qquad Z_i(s)=\tau^i(s).
\]

The script forms the span of each \(\alpha_r\) multiplied by the Frobenius-invariant scalar functions of degree at most \(3-r\). A Frobenius-invariant scalar takes values in \(\mathbb F_2\). For both 47-bit factors, this span has dimension 50 and equals the entire equivariant cubic space. At every sum its evaluations are therefore in the span of four values. [pcc:unverified]

For the lower bound on rank, \(T_s(a)=\sqrt a+sa\) sends \(\alpha_r(s)\) to \((\tau^r+\tau^{-r})s\) for \(r>0\) by cancellation, and sends 1 to \(1+s\). On \(U\), \(u=\tau+\tau^{-1}\) has minimal polynomial of degree 23. If a Frobenius conjugate of an eigenvalue \(\zeta\) fixes \(\zeta+\zeta^{-1}\), it sends \(\zeta\) to itself or its inverse. The powers of 2 modulo 47 have odd order 23, so inversion is excluded. The three operators are \(u,u^2,u^3+u\), which are independent on every nonzero \(s\in U\). The value \(1+s\) has trace one. This gives rank four at every nonzero sum. [pcc:unverified]

The certificate concerns Frobenius-equivariant multipliers. The unrestricted cubic space at 47 bits remains open. Rank coverage provides a criterion for the projected equations to test consistency for arbitrary right-hand sides. It leaves the solving degree of the projected equations, the cost of solving them, and the index-calculus exponent as separate questions.

## Reproduction

Run from the repository root with Python and SymPy installed.

```sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/profile.py 7 23 47
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/profile.py 23 --degree 2
```

The records are in [cubic-ranks.jsonl](cubic-ranks.jsonl) and [quadratic-ranks.jsonl](quadratic-ranks.jsonl). The last-fall conventions and references used for the broader experiments are recorded in the [LFD research note](../lfd_lower_bound_20261111/NOTE.md), including the relationship between last fall degree, reduced basis degree, and solving degree under a degree-compatible term order. [Caminata and Gorla](https://arxiv.org/abs/2112.05579)
