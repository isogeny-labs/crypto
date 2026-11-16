# F5 cyclic-11 genus-five factor screen

The frozen screen of all 90 exponent rows
`C_(a,b): y^11=x^a(x-1)^b` over F5, with `1<=a,b<=10` and
`a+b != 0 mod 11`, found exactly two fifth-extension traces:

| Trace t in `L(T)=1-t*T^5+3125*T^10` | Parameter rows |
|---:|---:|
| -75 | 60 |
| 90 | 30 |

Both have `5|t` and neither is zero. Waterhouse's elliptic-trace
classification allows only `t=0` among supersingular traces over
F_(5^5); ordinary traces are prime to 5. Thus neither factor is the
Weil restriction of an elliptic curve over F_(5^5). The conclusion is
for this three-branch rational Kummer stratum, not all genus-five covers.

Every row passed an independent direct eleventh-power recount over
F3125 (`VERIFIED.json`), distinct from the logarithm-residue histogram. [pcc:unverified]
The one-pass screen is saved in `SCREEN.json`. The genus-five and L-polynomial derivation uses
tame Riemann--Hurwitz, `gcd(11,5^r-1)=1` for `r=1..4`, the functional
equation, and `t=(3126-N5)/5`. `F5Arithmetic.lean` checks the finite
order-five and trace-recurrence arithmetic with no axioms; it does not
formalize the geometry or point-count programs. The related general
p-rank bound is in `../ghs_cyclic11_p_rank_bound_20261116/THEOREM.md`.

The cyclic-cover construction and Jacobi-sum technique are established
prior art; priority of this exact finite table remains unresolved.
This screen supplies neither an elliptic source nor a GHS attack cost.

Primary sources:
- Ozman--Pries--Weir, *The boundary of the p-rank 0 stratum of the moduli space of cyclic covers of the projective line* (2022): https://doi.org/10.1017/nmj.2022.12
- Agathocleous--Joux--Taufer, *Elliptic curves over Hasse pairs* (2026), Waterhouse trace cases: https://pure.mpg.de/pubman/item/item_3596867_6/component/file_3655796/Agathocleous-Joux-Taufer_Elliptic%2Bcurves%2Bover%2BHasse%2Bpairs_2026.pdf

The [trace chart](traces.svg) displays the two source-bound counts;
[RESULT.pdf](RESULT.pdf) contains the visual report. No canonical
vulnerability graph is changed because the screen yielded no eligible
elliptic source in this restricted family.
