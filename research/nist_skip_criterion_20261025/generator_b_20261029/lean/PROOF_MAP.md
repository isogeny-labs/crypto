# Conditional equation proofs

All declarations are in `CurveEquation.lean`, namespace `GeneratorCoefficient`.
`OnCurve p b x y` is the stated integer-representative congruence for a=−3;
`SeedEquation p c b` is the explicit assumption c*b²≡−27. The proofs are valid
for any integer p, so no unproved primality or group assumption is hidden.

| Declaration | Exact additional assumptions | Conclusion modulo p |
| --- | --- | --- |
| `at_zero` | `OnCurve p b 0 y` | y²=b |
| `at_one` | `OnCurve p b 1 y` | y²=b−2 |
| `transfer_seed_square` | z≡b and `SeedEquation p c b` | c*z²=−27 |
| `seed_at_zero` | `OnCurve p b 0 y`, `SeedEquation p c b` | c*y⁴=−27 |
| `seed_at_one` | `OnCurve p b 1 y`, `SeedEquation p c b` | c*(y²+2)²=−27 |

Lean 4.34.1 (commit5045d0056413266e57c625dcd7c365b10e377c52), core `Std` only.
The actual module build and transitive audit pass; exact commands, tool hashes
and outputs are retained beside this map. `propext` and `Quot.sound` are the
standard foundations used. No sorry, custom claim axiom, native_decide or
kernel bypass is used. A generated `OnCurve.eq_1` is also audited, hence the
auditor’s six declarations for five explicit research theorems.

Shared-library reference is pinned to aburan28/crypto-lean revision
148f5dc389b341401e06d76abe7783c4ccf42f2e. That project advertises Lean 4.35.0;
it is **not imported** here. This core-only check uses the unchanged local
repository auditor under the explicitly pinned4.34.1 compiler, with its exact
hash recorded. It is not a claim that the shared4.35.0 project or the whole
repository proof gate ran. No gate source was changed. `lean4checker` is
unavailable; no fresh checker receipt is claimed.

These are conditional congruences. They do not certify the actual seed/point
inputs, a selector, an attack, or the full P-224 isomorphism. The identity/v1
certificate schema evaluates unconditional polynomial equalities over the
fixed field 2^61−1 and cannot encode these hypotheses or arbitrary p. No false
unconditional certificate such as y²=b in free variables is manufactured;
the faithful evidence here is the checked conditional theorem and its map.
