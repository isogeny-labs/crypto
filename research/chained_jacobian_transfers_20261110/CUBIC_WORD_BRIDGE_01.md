# Exact coefficient square values in UInt64

The original cubic square's UInt64 coefficient operators have exactly the same natural output values as the retained natural/modular model at q=223 and reduction constant3, for canonical input coefficients. Both unchanged original Lean4.35/4.34.1 gates pass27 audited declarations/0 trusted on identical source5743a021. Data2aa7f3 and ten exact original PCC statement bindings retain the complete proof, source and attempts. This is unsigned component evidence. [pcc:unverified]

## The expression tree being modeled

The complete original common.rs has SHA256832fd28810909194875a9438e164ec1b45dcefbaa83257b4e3fad3095ef865d6. Its coefficient type is `[u64;3]`. With the two operands equal, q223/c3, its five temporaries are `(a*a)%223`, `(a*b+b*a)%223`, `(a*c+b*b+c*a)%223`, `(b*c+c*b)%223`, `(c*c)%223`. Its output is `((t0+3*t3)%223,(t1+3*t4)%223,t2)`. The new `wordSquare223` keeps that expression tree, its left association, ordinary word operations and every remainder. [pcc:unverified]

Lean's standard UInt64 operations expose `(x+y).toNat=(x.toNat+y.toNat)%2^64`, the analogous product identity and `(x%y).toNat=x.toNat%y.toNat`. These are the primary semantic interfaces used by the proof. Canonical Fin223 inputs embed without truncation. The theorem concerns these coefficient operators; it does not include the original mutable operation-counter updates, constructor or compiler. [pcc:unverified]

## Remove each possible wrap before comparing values

For every canonical coefficient x, 0≤x≤222. The original expressions satisfy these bounds, before their indicated remainder. They are arithmetic bounds, not timings or operation-cost measurements. [pcc:unverified]

| Expression | Bound | Use |
|:--|--:|:--|
| A coefficient product | 49,284 | All nine schoolbook products are exact words. [pcc:unverified] |
| A sum of two products | 98,568 | Both additions used for the two-term temporaries are exact. [pcc:unverified] |
| A sum of three products | 147,852 | Both left-associated additions in the middle temporary are exact. [pcc:unverified] |
| Any reduced temporary | 222 | Every natural remainder is canonical. [pcc:unverified] |
| Three times a reduced temporary | 666 | Both reduction-constant products are exact words. [pcc:unverified] |
| Final temporary plus scaled remainder | 888 | Both final additions are exact words. [pcc:unverified] |

Every bound is below2^64. The proof first removes the corresponding modulo2^64 operations using explicit product, pair, triple, scale and final-sum lemmas. It then simplifies the remaining modulo223 expressions to the exact earlier natural/model definition. No enumeration of223 cubed triples occurs. [pcc:unverified]

`word_square_coordinates` identifies decoded coefficients with `CubicNormDescent.originalSquare223`. The additional `word_square_canonical` proves all three output values are below223 for any UInt64 inputs, directly from their final word remainders. Consequently `word_square_exact_values` identifies the three raw `.toNat` values with the actual canonical values of the earlier model. It proves more than equality after an extra decoding remainder. [pcc:unverified]

## Carry the retained obstruction to raw word outputs

The exact old cubic-norm source is retained byte-for-byte as the prefix of the additive standalone module. Its commutative-ring determinant/square/unit argument and canonical Fin223 nonroot calculations remain unchanged. That argument already excludes original natural/model squares(12,0,0) and(24,0,0). Coordinate correspondence transports both exclusions to the new decoded word model; equality of word triples would imply equality of their decoded values, giving the raw-word exclusions. [pcc:unverified]

The allowed positive control is checked at both levels: `wordSquare223 2 0 0=(4,0,0)` and its decoded version. Exact bindings are in evidence/cubic-word-final-raw-value-proof-03/exact-ten-new-statement-bindings.json. Only propext, Classical.choice and Quot.sound appear in the original audits. Claims remain unsigned under the original PCC standing rules. [pcc:unverified]

## Attempts and scope

The first direct omega attempt did not resolve its nested2^64/223 coordinate goals and produced erroneous downstream sorryAx declarations. Full source/stdout/stderr/exit5cb553 remain preserved and are not accepted proofs. Explicit bounded remainder elimination fixes that proof without changing the model or statement. The decoded-only intermediate module5a85de passes both original gates22/0 and remains separately whole. The final raw-output extension2aa7f3 passes both original gates27/0. [pcc:unverified]

| Requirement | Evidence or dependency |
|:--|:--|
| Original five coefficient temporaries and reductions | Exact UInt64 source model plus complete original Rust source. [pcc:unverified] |
| Canonical-input word arithmetic | Explicit bounds, exact decoded coordinates and raw natural values. [pcc:unverified] |
| Both retained constants and an allowed square | Raw/decoded12/24 exclusions and raw/decoded4 controls. [pcc:unverified] |
| Original proof tools | Two unchanged gates27 audited/0 trusted; ten new exact declaration bindings. [pcc:unverified] |
| Original whole function and executable | Counter state, constructor selection and compiler/native-binary correspondence remain separate. [pcc:unverified] |
| Actual field/Jacobian and global geometry | Not supplied by this coefficient theorem. [pcc:unverified] |

Sources checked: original Kf.mul; immutable CubicNormDescent source18384e and original dual-gate records; standard Lean Init/Data/UInt/Lemmas.lean operator interfaces. KB mechanism/theorem wrapper queries both return127/no executable, and the autoresearcher vault remains locked. Neither is evidence of novelty or absence. Next retain this proof with dedicated source-pinned CI and continue the concrete geometric/program bridges and runtime preparation. [pcc:unverified]
