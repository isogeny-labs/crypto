# Cubic trace multipliers for the binary S3 selector

At field degree 23, cubic Frobenius-equivariant trace multipliers span the 13-dimensional left kernel of the normalized two-coordinate equation at every nonzero sum, for both tested Frobenius subspaces. At field degrees 47, 71, and 79 the entire cubic multiplier space has pointwise rank 4, while the left kernels have dimensions 25, 37, and 41. The unrestricted result follows from spectral block ranks and a span certificate in both subspaces at each size. [pcc:unverified]

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

## Unrestricted cubic multipliers

The coefficient Frobenius \(\sigma\) on \(K\) has a normal basis and therefore has each \(n\)-th root of unity as an eigenvalue once after extending scalars to \(E=\mathbb F_{2^k}\). Let \(\zeta\in E\) be a root of \(g\). Write \(Q=\{2^i\bmod n:0\le i<k\}\), let \(\phi\) act on the coordinate functions, and set

\[
Y_j=\sum_{r=0}^{n-1}\zeta^{-jr}\phi^r(Z_0),\qquad j\in Q.
\]

These satisfy \(\phi(Y_j)=\zeta^jY_j\), \(Z_1=\sum_{j\in Q}\zeta^jY_j\), and \(Y_j^2=\zeta^{2j}Y_{2j}\). In the coefficient block \(\sigma=\zeta^\ell\), the squared multiplier test applies \(g(\zeta^\ell\phi)(1+\zeta^\ell\phi)\) to \(\alpha+\zeta^\ell Z_1\phi(\alpha)\). The first operator is diagonal on products of distinct \(Y_j\). A monomial's coefficient must vanish when its weight plus \(\ell\) lies outside \(Q\cup\{0\}\), where the weight is the sum of its indices modulo \(n\). For each degree-four monomial, all contributing terms add one previously absent index, and hence have the same nonzero coefficient. The verifier first solves those rows over \(\mathbb F_2\), then handles the lower-degree rows over \(E\), including repeated-index reductions. [pcc:unverified]

The blocks for \(\ell=0,1,-1\) give the three Frobenius orbit representatives. If their kernel dimensions are \(d_0,d_+,d_-\), the unrestricted binary dimension is \(d_0+k(d_++d_-)\). The two choices of \(g\) gave the same dimensions at each field degree. [pcc:unverified]

| Field degree | Block dimensions | Unrestricted dimension | Scalar-multiple dimension |
| ---: | ---: | ---: | ---: |
| 7 | 6, 6, 5 | 39 | — |
| 23 | 35, 33, 33 | 761 | 312 |
| 47 | 50, 50, 50 | 2,350 | 2,350 |
| 71 | 109, 111, 110 | 7,844 | 7,844 |
| 79 | 134, 137, 135 | 10,742 | 10,742 |

For \(k>3\), multiplying the four displayed \(\alpha_r\) by arbitrary Boolean scalars of degree at most \(3-r\) gives an admissible subspace of dimension \(\sum_{j=0}^3(4-j)\binom{k}{j}\). Indeed, the four \(\alpha_r(s)\) are independent at every nonzero sum by the argument above, which also holds at 71 and 79 bits, where the orders of 2 are 35 and 39. Any polynomial dependency would force each scalar to vanish off zero. The indicator of zero has degree \(k\), so the allowed scalar degrees exclude such a dependency. Equality of dimensions at 47, 71, and 79 bits proves this subspace is the entire cubic family. In particular, unrestricted coefficients still give only four pointwise directions. [pcc:unverified]

For each of the six subspaces at these sizes, the verifier also finds a rational target on \(y^2+xy=x^3+x^2+1\) and a nonzero sum that passes all four checks. Adding the actual normalized S3 right-hand side raises the image rank from \(k-1\) to \(k\). These false positives rule out sufficiency of the cubic projection even on the S3 target family, in the tested geometries. Higher-degree multipliers, additional target-dependent equations, and other formulations remain open. Rank coverage, the solving degree of a projected presentation, and the index-calculus exponent are separate quantities. [pcc:unverified]

## Reproduction

Run from the repository root with Python and SymPy installed.

```sh
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/profile.py 7 23 47
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/profile.py 23 --degree 2
c++ -std=c++20 -O3 research/frobenius_s3_cubic_projection_20261123/extension_rank.cpp -o /tmp/s3-extension-rank
SYMPY_GROUND_TYPES=python python3 research/frobenius_s3_cubic_projection_20261123/unrestricted.py 7 23 47 71 79 --ranker /tmp/s3-extension-rank --verify
```

The records are in [cubic-ranks.jsonl](cubic-ranks.jsonl), [quadratic-ranks.jsonl](quadratic-ranks.jsonl), and [unrestricted-ranks.jsonl](unrestricted-ranks.jsonl). The unrestricted verifier checks direct binary-coordinate ranks at 7 and 23 bits, and compares logarithm-table arithmetic against polynomial arithmetic for the extension blocks through 47 bits. The last-fall conventions and references used for the broader experiments are recorded in the [LFD research note](../lfd_lower_bound_20261111/NOTE.md), including the relationship between last fall degree, reduced basis degree, and solving degree under a degree-compatible term order. [Caminata and Gorla](https://arxiv.org/abs/2112.05579)
