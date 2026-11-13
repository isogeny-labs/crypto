# CT60 — All elliptic binary-base traces and the residue-degree gate

**Status:** [pcc:unverified]. This is a scoped exact-arithmetic test of established Frobenius theory, **not** a scalable cross-characteristic transfer or an ECDLP complexity bound.

## Theorem

Let \(\ell\) be an odd prime and \(E/\mathbb F_2\) an elliptic curve. Hasse gives trace \(t\in\{-2,-1,0,1,2\}\). Each trace is realized by an explicit smooth curve below. Frobenius \(F\) on \(E[\ell]\) has characteristic polynomial \(X^2-tX+2\pmod\ell\). A nonzero \(\ell\)-torsion point is defined over \(\mathbb F_{2^f}\) if and only if \(\det(I-F^f)=0\pmod\ell\), equivalently \(\ell\mid\#E(\mathbb F_{2^f})\).

If the polynomial splits with distinct eigenvalues \(\alpha,\beta\in\mathbb F_\ell\), the minimum possible extension degree for **some** nonzero point is \(\min(\operatorname{ord}(\alpha),\operatorname{ord}(\beta))\). If irreducible, the conjugate eigenvalues in \(\mathbb F_{\ell^2}\) have equal multiplicative orders, giving the minimum. If it has a repeated root (only \(\ell=7,t=\pm1\) here), the repeated eigenvalue still admits a nonzero eigenvector, so its order gives the single-point minimum; no full-torsion assertion follows.

For any number field \(L\), elliptic curve \(\mathcal E/L\) with good reduction at \(w\mid2\), reduced elliptic curve descending to \(\mathbb F_2\), and nonzero \(L\)-rational point of odd prime order \(\ell\), injectivity of specialization implies \([L:\mathbb Q]\ge f_w\ge f_{\min}\), where \(f_{\min}\) is the minimum across all five possible traces.

**Divisibility warning:** Unlike CT59's trace-zero case, the minimum \(f_{\min}\) need not divide every admissible \(f\). At \(\ell=37,t=-1\), the eigenvalues have orders 12 and 18; both extension degrees work, and 12 does not divide 18. The only universal conclusion here is the lower-bound inequality.

## Exact sweep across all elliptic curves defined over F2

All values below are the least extension degree containing **some** nonzero point. The five columns use \(t=-2,-1,0,1,2\).

| Odd prime \(\ell\) | t=-2 | t=-1 | t=0 | t=1 | t=2 | Best over all t |
|---:|---:|---:|---:|---:|---:|---:|
| 3 | 8 | 8 | 1 | 8 | 8 | 1 |
| 5 | 1 | 24 | 8 | 24 | 2 | 1 |
| 7 | 24 | 6 | 12 | 3 | 24 | 3 |
| 11 | 40 | 5 | 5 | 5 | 40 | 5 |
| 17 | 16 | 144 | 16 | 144 | 16 | 16 |
| 19 | 72 | 360 | 9 | 360 | 72 | 9 |
| 23 | 88 | 11 | 44 | 22 | 88 | 11 |
| 37 | 18 | 12 | 72 | 9 | 9 | 9 |
| 43 | 56 | 21 | 7 | 21 | 56 | 7 |
| 163 | 648 | 27 | 81 | 54 | 648 | 27 |
| 257 | 32 | 1376 | 32 | 1376 | 32 | 32 |
| 521 | 65 | 135720 | 520 | 135720 | 130 | 65 |
| 65537 | 64 | 2097216 | 64 | 2097216 | 64 | 64 |
| 2147483647 | 248 | 357913941 | 124 | 715827882 | 248 | 124 |
| 4294967291 | 17179869160 | 18446744030759878680 | 2147483645 | 18446744030759878680 | 17179869160 | **2147483645** |

Thus for the particular 32-bit prime \(\ell=4294967291\), even varying over **every** elliptic curve defined over \(\mathbb F_2\) cannot improve the binary residue-degree floor of **2,147,483,645**. In contrast, varying the curve at \(\ell=163\) improves the prior fixed-trace-zero figure from 81 to 27. Neither claim extends automatically to target curves defined only over \(\mathbb F_{2^k}\), \(k>1\), or to genus \(g>1\).

## Concrete smooth models for all five traces

Use \(y^2+a_1xy+a_3y=x^3+a_2x^2+a_4x+a_6\). Entries are coefficient tuples \((a_1,a_2,a_3,a_4,a_6)\).

| Trace | Coefficients | Point count over F2 |
|---:|---|---:|
| -2 | (0,0,1,1,0) | 5 |
| -1 | (1,0,0,0,1) | 4 |
| 0 | (0,0,1,0,0) | 3 |
| 1 | (1,0,1,0,1) | 2 |
| 2 | (0,0,1,1,1) | 1 |

Both implementations enumerate binary Weierstrass coefficients, explicitly test nonsingularity and count all rational points. For an odd source prime \(p\), the CRT applied separately to generalized Weierstrass coefficients allows a globally integral curve to match *any* of these binary witnesses at 2 and a prescribed nonsingular short Weierstrass model at \(p\). This concerns existence of compatible **curve models**, not compatible global lifts of arbitrary encoded points.

## Independent checks and reproducibility

From the repository root, run the Python and JavaScript scripts in this directory:

- Python: ct60_binary_trace.py with python3 -B and python3 -O -B
- JavaScript: ct60_binary_trace_independent.js with node and node --no-opt

The Python implementation computes quadratic roots via Tonelli–Shanks, exact multiplicative orders in \(\mathbb F_\ell\) or \(\mathbb F_{\ell^2}\), prime trial-division checks for the input sizes, factor-order minimality and direct point-count recurrences at small degrees. Independently authored JavaScript uses BigInt **Frobenius companion matrices**, their finite orders and a sorted exhaustive divisor search for fixed vectors, **without extracting eigenvalue roots**. It also checks the 37/12/18 falsifier.

All four local modes produced byte-identical 19-line outputs: SHA256 102df7d2d5f7c1f14427edc711cfac77b5b4ac327125f2defcf770de6cbf368e. This is local verification, **not** independent PCC certification.

## Sources, research gate and limitations

Internal prior work: CT55–59 in this directory, second-prime-transfer problem on main, root PR #3877 and PR #4015. Cross-repo GitHub search covered crypto, cryptanalysis and crypto-autoresearcher; shared KB MCP and checkout-based wrapper were unavailable here. No novelty assertion is made.

Prior mathematical theory: [Sage elliptic curves over finite fields](https://sporadic.stanford.edu/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_finite_field.html) and [Stanford Frobenius recurrence](https://crypto.stanford.edu/pbc/notes/elliptic/count.html). The results are finite consequences of the classical Hasse bound, torsion specialization and Frobenius polynomial.

Next discriminate binary curves defined over \(\mathbb F_{2^k}\) and genus-two Jacobians, while retaining the critical uniform, compatible algebraic lift for arbitrary encoded \(P,Q=[x]P\). No point-transport implementation or cold complete-cost comparison is claimed. No paid compute.

[pcc:unverified]
