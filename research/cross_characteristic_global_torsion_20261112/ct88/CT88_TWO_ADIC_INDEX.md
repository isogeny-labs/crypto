# CT88 — Exact 2-adic order-index obstruction in the degree-eight subgroup field

**Scientific status: [pcc:unverified].** A fixed-instance algebraic number theory certificate for the CT86–87 11a1[7] tower. This proves a concrete defect in the power-basis order at the binary target, **not** ramification of the field, an ECDLP lower bound, or a fast transfer.

## Result: the polynomial generator is not 2-maximal

Recall CT86's globally defined subgroup coefficient
`s=x(T)+x([2]T)+x([3]T)` and `F=Q(s)`, with irreducible monic polynomial

```text
g(S)=S^8-8 S^7-840 S^6-29862 S^5-39690 S^4
     -1696464 S^3-6423347 S^2+3828478 S-2102915.
```

An **exact integer discriminant** computation, independently by rational-polynomial Euclidean resultants and a BigInt fraction-free Sylvester determinant, gives

```text
disc(g) = -126276666637416719355500270478117660693463886295963968396774334577180672
v_2(disc(g)) = 16.
g(S) mod 2 = S^8+S^2+1 = (S^4+S+1)^2.
```

At 2 the fixed elliptic curve `y²+y=x³−x²−10x−20` has exactly five F2-points, so its Frobenius trace is `t₂=−2`, and the characteristic polynomial on 7-torsion is `X²+2X+2` modulo 7. Its companion action on the **eight projective lines of E[7]** has exactly **two disjoint cycles of length four**, as exhaustively computed in both programs. By good reduction and 7 prime to 2, the finite étale 7-torsion representation is unramified at 2. Hence every subfield of the full 7-division field, including the CT86 degree-eight line field F, is unramified at 2; the two primes of F above 2 have residual degrees **4 and 4**, without ramification.

For the order `Z[s] ⊆ O_F`, the index–discriminant identity is

```text
disc(g) = [O_F : Z[s]]^2 disc(F).
```

Since `v₂(disc(F))=0`, it follows **exactly**

```text
v₂([O_F : Z[s]]) = 8.
```

Thus the 2-primary part of the order index is **2^8=256**. In particular, reducing the monic defining polynomial `g` modulo 2 and applying the squarefree Dedekind factorization rule would be invalid. The doubled irreducible quartic here reflects **nonmaximality of the order**, not field ramification; the true residue splitting is 4+4.

The explicit element

```text
t=(s^4+s+1)/2
```

is an algebraic integer **outside** Z[s]. Proof: the polynomial `H(S)=S^4+S+1` annihilates the residue of s at every place over 2, because `g mod 2=H²`. Unramifiedness makes 2 a local uniformizer, so `H(s)/2` is integral at all places above 2; at other primes the denominator 2 is a unit. Since `deg H<deg g` and the power-basis coefficients contain 1/2, `t∉Z[s]`. This gives a concrete generator-improvement direction but is **not yet a certified 2-maximal integral basis**.

## Consequence for CT87 relative-cubic specialization

CT87 expresses its globally integral symmetric cubic coefficients using rational polynomials `c₂(s)/q₂` and `c₃(s)/q₃`, with

```text
q₂ = 23624165350186,   v₂(q₂)=1;
q₃ = 10252887761980724,v₂(q₃)=2.
```

Consequently these **specific rational presentations do not permit coefficientwise evaluation at the characteristic-two target**: the denominators become zero modulo 2, even though the actual subgroup-kernel symmetric coefficients have integral binary values. CT86 computed them as `(s,e₂,e₃)=(1545,1906,1261)` in the fixed F_(2^12) basis. Their reduction needs prime-ideal-aware cancellation / a 2-integral relative model, *not* naive rational-polynomial substitution.

This is a **local basis/denominator obstruction to the current implementation**, not a mathematical obstruction to cross-characteristic point transport. In particular no false implication “repeated g mod2 ⇒ 2 ramifies in F” is made.

## Independent executable validation

- `ct88_index.py`: rational Euclidean polynomial-division/resultant recursion with Python Fractions; independent 8-line projective Frob₂ traversal.
- `ct88_index_independent.js`: exact BigInt 15×15 Sylvester/Bareiss determinant; separately written 8-line projective Frob₂ traversal.
- Both certify the full signed discriminant, its valuation, the squared mod-2 quartic, Frobenius cycle decomposition and the two denominator valuations.
- Python, Python -O, Node and Node --no-opt passed locally with byte-identical JSON, SHA256 `4953ec9689c3ce372eda1d53cc53eceb3340f7dc013270bcd3d0cf909956f096`. This is local producer-side evidence; independent PCC signing and exact-head CI require separate review.

From the repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct88
python3 -B "$D/ct88_index.py" > /tmp/ct88-py.json
python3 -O -B "$D/ct88_index.py" > /tmp/ct88-opt.json
node "$D/ct88_index_independent.js" > /tmp/ct88-js.json
node --no-opt "$D/ct88_index_independent.js" > /tmp/ct88-jsopt.json
cmp /tmp/ct88-py.json /tmp/ct88-opt.json
cmp /tmp/ct88-py.json /tmp/ct88-js.json
cmp /tmp/ct88-py.json /tmp/ct88-jsopt.json
printf '%s  %s\n' 4953ec9689c3ce372eda1d53cc53eceb3340f7dc013270bcd3d0cf909956f096 /tmp/ct88-py.json | sha256sum -c
```

## Scope, prior work and next acceptance gate

Prior internal records: CT80 7-division polynomial; CT82 fixed-place torsion reduction; CT85 global endomorphism rigidity; CT86 cyclic subgroup quotient and its mod-2 square warning; CT87 explicit 8×3×2 tower. Both cryptanalysis #1523 and root crypto #3877 retain separate genus-two/three/native chain ownership.

Primary external references checked: [PARI general number fields, nfroots and nfcertify](https://pari.math.u-bordeaux.fr/dochtml/html-stable/General_number_fields.html), [Sage finite residue fields and nonmaximal-order example](https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/residue_field.html), and the classical good-reduction prime-to-p finite étale torsion theorem (Néron–Ogg–Shafarevich criterion). The shared research-KB service and checked-out wrapper are unavailable in this connector runtime; **no complete KB survey or novelty claim**.

**Next:** construct a certified 2-maximal basis for F and an integral relative cubic in F[Z] valid at the actual chosen binary prime, factor it by an input-independent algorithm, select one oriented global lift for public Q at the **fixed** source place, and evaluate at the fixed target place. Charge field setup, integral-basis construction, root selection, residue transport and target solving against matched cold rho/IC on growing orders. No paid CPU/GPU jobs, protected merge, or speedup evidence are claimed here.
