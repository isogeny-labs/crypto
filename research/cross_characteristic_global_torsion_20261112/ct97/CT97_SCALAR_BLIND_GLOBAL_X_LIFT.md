# CT97 — Scalar-blind recognition of every nonzero source **x-coordinate** lift

**2026-11-14; [pcc:unverified].** A bounded constructive advance for CT80's seven-torsion toy. It returns an **exact algebraic x-coordinate** at one fixed global source place and its compatible characteristic-two x-coordinate, without receiving or searching the input scalar. It does **not** return the oriented y-coordinate, a full point lift, a complete ECDLP solving reduction, or a faster-than-rho benchmark.

## Starting point and references

Reuse CT80's degree-24 field \(L=\mathbb Q(\alpha)\), CT82's fixed reduction \(\alpha\mapsto9\in\mathbb F_{37}\), CT86–87's cubic over a degree-eight subgroup field, CT89–91's fixed binary prime \(\alpha\mapsto2\in\mathbb F_{2^{12}}\), and CT95's rational-template degree obstruction. CT96 now independently certifies the polynomial's irreducibility mod 3.

Relevant prior algorithms are ordinary cubic discriminants, Galois resolvents, relative-polynomial factorization, number-field rational linear algebra, and reduction of integral algebraic elements at fixed primes. The fact that the cubic is cyclic is already established in CT87. This experiment is a new *implementation on our fixture*, not a novel cubic-factorization theorem. The research-KB MCP and repo wrapper remain unavailable in this connector context; original repo files and the Sage/PARI documentation referenced in CT87 and CT96 were inspected as fallback.

## Exact scalar-independent algebraic construction

CT87 fixes the relative cubic over \(S=\mathbb Q(s)\):

\[
h(Z)=Z^3-sZ^2+e_2 Z-e_3,\quad
s=\alpha+x([2]T)+x([3]T).
\]

The coefficients \(e_2,e_3\) are **input-independent setup**, with public CT87 rational expressions in \(s\). The setup computation originally used division-polynomial multiplication maps; this must be included in cold cost, and should not be confused with a free online oracle.

The new CT97 square-root discriminant is the *fixed element* \(\delta=c_\delta(s)/q_3\), where \(q_3=10252887761980724\) and the ascending coefficient vector is

```text
[1330115088493357156, 656532317695454585,
 17598156120098064, 12706441996691436,
 225325011610699, -309225601391,
 -443542601663, 4007355953]
```

An independent JavaScript BigInt computation, using CT87's explicit \(g(s)\), \(e_2\), and \(e_3\), verifies the **integer polynomial congruence modulo \(g\)** for

\[
\delta^2=s^2e_2^2-4e_2^3-4s^3e_3-27e_3^2+18se_2e_3.
\]

No numerical square root is assumed. In \(L\), put \(h'(\alpha)=3\alpha^2-2s\alpha+e_2\). Since the cubic is separable and \(\alpha\) is its known root, its other two roots are

\[
\boxed{\beta_\pm=
\frac{s-\alpha\ \pm\ \delta/h'(\alpha)}2.}
\]

This follows from \(h'(\alpha)=(\alpha-\beta_+)(\alpha-\beta_-)\) and the oriented cubic discriminant \(\delta=(\alpha-\beta_+)(\alpha-\beta_-)(\beta_+-\beta_-)\). The Python program verifies the **exact** Vieta equalities for all three constructed roots in \(\mathbb Q[X]/\psi_7(X)\) before looking at any public target.

For each algebraic root, a \(24\)-row rational linear nullspace solver searches **increasing total numerator+denominator degree**, from 1 to 17, for \(A(X)/B(X)\) equal to that root *exactly in \(L\)*. The program is not passed the multiplicative index \(m\), a scalar table, a degree pair per input, or a target binary label. The result is accepted only after checking \(A(\alpha)=\beta B(\alpha)\) in the quotient ring and both fixed-place denominator units.

The fixed source residue \(\alpha\mapsto9\) now identifies the unique one of these roots with the public source \(x(Q)\). That exact rational expression is evaluated at the **same fixed** binary prime \(\alpha\mapsto2\) using explicit \(\mathbb F_{2^{12}}\) arithmetic. There is no post hoc selection of a binary root from its desired logarithm.

| Input public \(x(Q)\) over \(\mathbb F_{37}\) | Reconstructed degrees (num/den) | Output binary \(x\) |
|---:|---:|---:|
| 9 | 1 / 0 | 2 |
| 27 | 4 / 3 | 17 |
| 28 | 9 / 8 | 1562 |

The degree-(9,8) result resolves CT95's short-template obstruction on the toy without an expensive \(18\)-dimensional 37-adic LLL. A negative source control (10) is rejected.

**Critical distinction:** the independent JS verifier checks the discriminant certificate and the recovered three source/binary rational expressions against exact division-polynomial identities; it does not separately implement the SymPy nullspace recognizer. Therefore this is an independently checked *output*, not a separately discovered algorithm, and is unsigned until the PCC gate. [pcc:unverified]

## What is not solved

1. For \(Q=(x,y)\), this reconstructs **only the x-coordinate**. The two global ordinates above each \(\beta\) differ by elliptic negation. The public source ordinate must select its unique compatible lift in the **degree-48** oriented point field and that same element must be specialized at the fixed binary place. Neither operation is implemented here.
2. The precomputed CT87 cubic and discriminant still depend on knowing the subgroup global model, not on a compressed representation that scales to cryptographic \(\ell\). CT96's dense \(\Theta(\ell^2)\) field-size gate remains.
3. The modest three-element root set is not evidence for better cold runtime than source BSGS, negation rho or existing index calculus. SymPy-based number-field arithmetic, setup, denominator clearing, finite-field specialization and target DLP must be timed and charged on the same inputs. No measured speedup is claimed.
4. To generalize beyond a cyclic cubic, replace this resolvent with a certified factorization over the fixed global number field and a scalar-independent root/prime-compatibility mechanism. This is not automatically possible with a small relative degree.

## Exact replay

```sh
D=research/cross_characteristic_global_torsion_20261112/ct97
python3 -m pip install sympy==1.14.0
python3 -B "$D/ct97_selective_x_root.py" >/tmp/ct97-py.json
python3 -O -B "$D/ct97_selective_x_root.py" >/tmp/ct97-opt.json
node "$D/ct97_discriminant_independent.js" >/tmp/ct97-js.json
node --no-opt "$D/ct97_discriminant_independent.js" >/tmp/ct97-jsopt.json
cmp /tmp/ct97-py.json /tmp/ct97-opt.json
cmp /tmp/ct97-py.json /tmp/ct97-js.json
cmp /tmp/ct97-py.json /tmp/ct97-jsopt.json
printf '%s  %s\n' 8b62b8a610d7be2a262dc82075b316e626dc35f668fb55517dcd4e61a10abcb5 /tmp/ct97-py.json | sha256sum -c
```

The core calculation was prototyped locally against SymPy 1.14.0 and reproduced all three x roots and rational degrees; **the exact committed-code four-way CI still needs inspection**. It is not a full repository native/Conductor audit. No EKS workload, unbounded compute, protected merge or scientific PCC signature.

## Next gate

Solve the **oriented y-coordinate lift without first recovering the input scalar**, preferably by factoring \(Y^2+Y-f(\beta)\) in the fixed global point field, selecting the source residue, and reducing the exact same element at CT80's fixed binary place with integral handling at 2. Only then is a complete public-point transport test possible. Afterward run complete cold-cost comparisons and try a second independent point-field fixture.
