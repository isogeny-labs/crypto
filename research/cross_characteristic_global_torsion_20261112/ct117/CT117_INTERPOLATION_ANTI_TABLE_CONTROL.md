# CT117 — A scalar-blind polynomial can conceal a complete projective-DLP table

**2026-11-14. Scientific status: [pcc:unverified].** Fixed 11a1 source 59 / binary 2 negative control. Neither an algebraic cross-characteristic map nor a DLP speedup is claimed.

## Prior work and sources

Read root [#3877](https://github.com/isogeny-labs/crypto/pull/3877), [CT102 irreducible degree-60 x-field](../ct102/CT102_GLOBAL_ELEVEN_POINT_FIELD.md), [CT115 Frobenius gate](../ct115/CT115_FROBENIUS_KUMMER_DESCENT.md), and [CT116 orbit index equivalence](../ct116/CT116_ORBIT_INDEX_EQUIVALENCE.md). Inspected concurrent cryptanalysis Richelot [#1511](https://github.com/isogeny-labs/cryptanalysis/pull/1511) / norm [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) and crypto-autoresearcher Prym leads without editing their branches. Read root AGENTS.md and 2026-11-14 log. External background: rational Lagrange interpolation and Victor Shoup, [Lower Bounds for Discrete Logarithms and Related Problems](https://doi.org/10.1007/3-540-69053-0_18) (EUROCRYPT 1997). Shoup's bound concerns generic group algorithms, not special coordinate encodings with nonuniform precomputed advice.

**Sources checked (KB): KB unavailable**; research-KB MCP is not loaded and the checkout-local wrapper is inaccessible in this connector session. These primary repository and published sources are fallback, not a novelty search certificate. Conductor, full cargo suites, scientific PCC signing and paid EKS were not executed.

## General exact interpolation theorem

Given h distinct rational abscissae r_i and *any prescribed* target integers b_i, the unique rational polynomial of degree less than h agreeing at every r_i is

\[
F(X)=\sum_{i=1}^h b_i\prod_{j\ne i}\frac{X-r_j}{r_i-r_j}.
\]

Thus an online computation consisting solely of public-coordinate polynomial evaluation can still encode an offline table of h projective discrete-logarithm answers. This says nothing about an efficiently *uniformly computable* family for growing subgroup order; special structure can compress interpolation data.

## Exact fixed toy certificate

The source is E: y²+y=x³−x²−10x−20 over F_59, P=(19,22) of order 11. The binary target Kummer orbit, as CT115–116 established, is [2,65536,4697,17,4956], with representatives [1,7,5,2,3] modulo 11 and Frobenius^4(T)=[7]T. Bitmasks are polynomial-basis encodings of elements of F_(2^20), **not integer reductions modulo two**.

| m | x([m]P) | Binary x bitmask | Frob^4 orbit index |
|---|---:|---:|---:|
| 1 | 19 | 2 | 0 |
| 2 | 20 | 17 | 3 |
| 3 | 43 | 4956 | 4 |
| 4 | 46 | 65536 | 1 |
| 5 | 22 | 4697 | 2 |

Exact interpolation yields

\[
\boxed{F(X)=
\frac{1822729272584-280573814794X+15582488805X^2
-368168372X^3+3131989X^4}{1162512}.}
\]

The common denominator satisfies v₂=4. Python constructs rational Lagrange products. Independent JavaScript uses BigInt Newton divided differences and Newton-to-monomial basis conversion. Four normal/optimized runs matched byte-for-byte, SHA-256 `46b50bc0c22d432987572a848dbe056f8095120b571eb07c5b93e3e82b54e21b` (local development runs; independent signed PCC not established). [pcc:unverified]

The offline construction explicitly enumerates the five projective source scalar classes (four source additions), associates them with the Frobenius orbit (16 binary field squarings), then compiles that source DLP table into five rational coefficients. The **online evaluator** accepts only Q coordinates, checks curve and [11]Q=O, performs exact rational Horner polynomial evaluation, matches the result to the target orbit and checks the source sign. It contains no online source-x→scalar dictionary. Nevertheless, the coefficients embody all five projective log classes. All ten nonzero public toy points recover the source log; four invalid controls reject and a one-coefficient mutation is detected. [pcc:unverified]

The source-only BSGS control, rebuilt for every public input, counts [4,4,4,5,5,5,5,6,6,6] source additions. No calibrated matched-unit whole-pipeline timing, source rho/IC comparison, growing-size scaling, actual target DLP algorithm, or complete number-field cold cost has been performed. [pcc:unverified]

## Two independent mathematical incompatibilities

**(1) The compiled polynomial is not a global torsion-coordinate lift.** Let α=x(T) in CT102's fixed characteristic-zero field, with minimal polynomial of degree 60. A canonical global lift of the chosen generator has x-coordinate α, whereas F(X)−X is a *nonzero degree-four rational polynomial*. Since no degree-four nonzero polynomial can vanish at α, F(α) ≠ α. This is a rigorous degree certificate against *this particular interpolant*, not a theorem excluding arbitrary algebraic or computational transfers. [pcc:unverified]

**(2) Bitmask reinterpretation is not binary specialization.** On x(P)=19 the rational polynomial returns the integer 2, whose genuine reduction modulo 2 is zero, while bitmask 2 denotes the **nonzero** polynomial-basis field element t. Moreover v₂(den(F))=4 prevents naive reduction of this coefficient representation. The online program intentionally changes encoding semantics, rather than specializing one global algebraic element through both fixed residue places. [pcc:unverified]

## Reproduction

```bash
D=research/cross_characteristic_global_torsion_20261112/ct117
python3 -B "$D/ct117_interpolation_control.py" > /tmp/ct117-py.json
python3 -O -B "$D/ct117_interpolation_control.py" > /tmp/ct117-pyo.json
node "$D/ct117_interpolation_independent.js" > /tmp/ct117-js.json
node --no-opt "$D/ct117_interpolation_independent.js" > /tmp/ct117-jso.json
cmp /tmp/ct117-py.json /tmp/ct117-pyo.json
cmp /tmp/ct117-py.json /tmp/ct117-js.json
cmp /tmp/ct117-py.json /tmp/ct117-jso.json
printf '%s  %s\n' 46b50bc0c22d432987572a848dbe056f8095120b571eb07c5b93e3e82b54e21b /tmp/ct117-py.json | sha256sum -c
```

A scoped exact-head GitHub Actions workflow runs all four tests. Passing scoped CI checks this finite-set construction only; the independent scientific PCC verdict, full project tests, default-branch merge gates and hardware-budget approvals remain distinct.

**Next gate:** construct an arbitrary-public-Q algorithm yielding A_Q in the *fixed* degree-60 global x-field with (i) an exact field-equality certificate A_Q=x([m]T), (ii) genuine source-59 and binary-2 residue compatibility at the already pinned places, (iii) no table-encoded advice or scalar-index precomputation hidden in constants, and (iv) accounted cold construction/recognition/target solving costs for growing subgroup orders. This remains open. [pcc:unverified]
