# CT20: F101 degree-seven genus-two gluing inventory

Date: 2026-11-11. Local independently written development oracle; **[pcc:unverified]**. Existing paired elliptic degree-seven maps come from crypto PR #3919 and cryptanalysis PR #1483. The objective here is an explicit genus-two **model**, not yet an executable Jacobian map.

## Prior work / custody

Read crypto PR #3877, the current session log, QUEUE.md, CHECKPOINT.md, CT19_PLAN.md, CT20_PLAN.md, and the unchanged algorithms in isogeny_algos/src/genus2.rs, isogeny_algos/src/kernel/kohel.rs, isogeny_algos/src/fpr.rs. Primary mathematics is Howe-Leprévost-Poonen (2000), Section 3.2, https://arxiv.org/abs/math/9809210 (author listing https://ewhowe.com/papers/paper11.html). These are standard gluing formulas; no novelty claim. KB MCP and the encrypted crypto-autoresearcher atlas remain inaccessible here, so the prior-art search is incomplete.

Source E1=(1,46), order109, kernels A=[9,76,20,1] -> (5,39), B=[72,100,35,1] -> (26,58). Source E2=(44,21), order113, kernels A=[77,86,71,1] -> (13,42), B=[59,46,8,1]. Root a of a³+6a²+13a+56 in F101³ has Frobenius conjugates a_i; paired elliptic 2-torsion abscissae are a_i+2 and 56/a_i+38.

At a nonkernel 2-torsion abscissa x, Kohel's map simplifies to
`phi_x(x)=7*x+2*c2-(6*x²+2*a)*(3*x²+2*c2*x+c1)/h(x)`
for h(x)=x³+c2*x²+c1*x+c0. The separate Python oracle evaluates these images in F101³, checks the target cubics/Frobenius cycles, and repeats the published genus2::glue formula without importing Rust arithmetic.

## Fixed local output, unsigned

All four raw sextics are smooth F101 models. The coefficients below are ordered from constant to leading:

| E1/E2 | raw sextic [c0,...,c6] | c6 square? | direct monic even chart |
| --- | --- | --- | --- |
| A+A | [32,0,26,0,49,0,41] | no | unsupported |
| A+B | [38,0,48,0,9,0,46] | no | unsupported |
| B+A | [87,0,15,0,13,0,21] | yes | [33,0,44,0,92,0,1] |
| B+B | [19,0,86,0,34,0,79] | yes | [68,0,42,0,26,0,1] |

For **every** row the oracle enumerates all x in F101 and F101² and checks #C(F101)=120, #C(F101²)=10436, L(T)=1+18T+279T²+1818T³+10201T⁴ and #Jac(F101)=12317. Each row has squarefree sextic. The two nonsquare-leading rows must **not** be silently normalized to monic: dividing by 41/46 would twist, not give a base-field isomorphism. Their raw models are valid; other rational charts have not been excluded.

**First supported chart B+A:** C1: Y²=t⁶+92t⁴+44t²+33. The direct quotient has long cubic (33,44,92,1), short (17,10), order113, and matches E2-A (13,42) by u=7 (both coefficient power equations). The reciprocal quotient has long (79,6,44,1), short (34,9), order109, and matches E1-B (26,58) by u=7. The two quotient components are **swapped** relative to the input pair.

**Second supported chart B+B:** C1: Y²=t⁶+26t⁴+42t²+68. Direct short (86,67), order113, matches E2-B (69,50), u=21. Reciprocal short (69,5), order109, matches E1-B (26,58), u=21. Opposite signs -u also give the same x-isomorphism with reversed y.

Original failure retained as a discriminating control: individual quadratic factors of the HLP sextic live in F101³ and generally do **not** descend, although the sextic product does. The reference implementation was corrected to project the product only; otherwise the first row falsely reports failure. All tests and figures are local producer assertions, not independent signed PCC verification.

## Additional native regression\n\nThe branch also includes `isogeny_algos/tests/ct20_f101_gluing.rs`. It replays all four images using the existing `FpR::from_modulus`, unchanged `Kohel::kohel` and HLP `genus2::glue`, then checks prime-field coefficient descent, L-polynomial, and the supported first/reciprocal quotient identifications. It does not implement the Jacobian map or provide a signed independent PCC verdict. Its new native CI test must pass before treating the Rust fixture as compiled/tested; the separate standard-library oracle already passed its published GitHub regression. [pcc:unverified]\n\n## Reproduction and independent controls

```sh
python3 -S research/chained_jacobian_transfers_20261110/ct20_gluing_oracle.py
python3 -m py_compile research/chained_jacobian_transfers_20261110/ct20_gluing_oracle.py
```

Run without Python -O: assert statements are the acceptance gates. The script needs only the standard library. It uses an independently implemented cubic quotient field and separate quadratic-field square census plus Euclidean squarefree check, frozen expected coefficients and checked quotient isomorphism equations. The locally tested development version passed; the GitHub rewrite of that oracle must also receive exact-head CI/replay before any acceptance. No native Rust tests or signed PCC verdict were executed in this run.

## Mathematical status / next action

The transported Galois-equivariant 2-torsion graph gives an abstract separable degree-(7,7) map between principally polarized quotients by standard HLP/Frey-Kani theory. The displayed sextics are explicit target **curve models**, not yet an executable divisor correspondence. The next step is to construct N0 and P1 in balanced Mumford coordinates and prove P1*alpha*N0 kills all geometric J0[2] before factoring through [2]. Then check both dual [7] identities on all 12,317 rational divisor classes, all 109 original source scalars, and 64 product 4-torsion lifts to select a transported Richelot kernel. Preserve direct elliptic degree-seven maps as performance controls. No DLP cost advantage, cross-prime map or paid compute is claimed. **[pcc:unverified]**
