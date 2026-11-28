# Actual-curve census: complete finite result, coordinates still open

PCC-20261123-0b48fa records unsigned finite data. At published source
c37b4dfb14b184186aca10b987e8926c67adcdad, all seven native phases passed:
versions, formatting, release compilation, strict Clippy and the census.
Build/pre-call/post-call guards matched. A's earlier strict-format failure
and unchanged source remain in first-A-format-negative. [pcc:unverified]

| F3 curve sector | Models | Elliptic Weil factors | Horizon | v2=12 at horizon | Ordinary without elliptic factors |
| --- | ---: | ---: | ---: | ---: | ---: |
| 3+3 | 28 | 16 | 12 | 16 | 12 |
| 5+1 | 48 | 0 | 20 | 48 | 36 |

Every curve's coefficients, F3/F9 fibre counts, Weil data and full extension
order/valuation arrays are in the whole28,832B first-B-native/census.stdout,
SHA2560700a8a4c5a6e4d96e7b17b1bb657ba7f50df46589b88d5287a50e12e1e0d9e7.
All1,296 Newton/exterior-square orders agree with companion determinants;
all76 copied N2+2 mutants are rejected and pristine inputs restored. The
field controls and known x^6+x^4+x^2+2 control pass. [pcc:unverified]

The predetermined split fixture is id0:
y^2=x^6+x^5+x^2+1, N1=6,N2=12,
P=(T^2+3T+3)(T^2-T+3), #J(F3^12)=282375475200 with v2=12.
It is an actual smooth genus-two curve, not a product substituted for a
Jacobian. The factorization does not itself construct its elliptic maps.
[pcc:unverified]

The predetermined ordinary nonsplit minimal fixture is **absent**. All12
nonsplit3+3 models have t=0,b=1 and horizon v2=16. For example id5 has
f=1+x+x^2+x^3+2x^4+x^5+x^6 and #J(F3^12)=283262386176.
This rejects that minimal-Sylow selection criterion over this exact F3
population. It does not prevent constructing J[8] on those curves: all
J[8] is still rational over F3^12, but the whole two-primary subgroup is
larger. No ordinary-nonsplit model is silently replaced or dropped.
[pcc:unverified]

Every5+1 curve has irreducible mod2 Frobenius and no elliptic Weil factor.
The written MATHEMATICAL_CONTRACT argument excludes nonzero same-field
elliptic-to-Jacobian homomorphisms there; arbitrary computational reductions
and extension-defined maps remain outside that restriction. [pcc:unverified]

Next: separately freeze genuine divisor/field arithmetic for selected id0,
construct all4,096 J[8] classes and an actual Frobenius matrix, including
the nonrational Weierstrass basepoint correction. Reuse audited native
Mumford/Cantor formulas; do not repeat the CT332 abstract lift population.
No actual mod8 coordinates, Weil pairing, Jacobian chain map, Lean theorem,
signed verdict or complete ECDLP cost advantage is supplied by this census.
[pcc:unverified]

Whole compiled ELFs/metadata remain private RAM with remote custody pending;
all small raw streams/argv/manifests are committed. The report/coordinate
successor and normal CI/merge gates remain unfinished, so PR #5007 stays draft.
[pcc:unverified]
