# Root precision proof map and source fidelity

The original PROOF-MAP.md is preserved as the pre-edit gate. This additive
outcome binds the reported result to shared commit
`1c57c092460bee5093072602d3bb058d13bbbcf1`, Lean4.35.0, unchanged Std core.
All claims remain PCC unverified until signed main standing. [pcc:unverified]

| Report statement | Declaration | Exact hypotheses | Observed check |
| --- | --- | --- | --- |
| Evaluation loss | `root_evaluation_envelope` | u:Nat,0<u | Compile, query, gate |
| Minimum active divisor | `root_four_scale` | u:Nat,0<u | Compile, query, gate |
| Source array supplies margin | `root_array_margin` | M,u:Nat;2^(2u)*64^(4u)<=M | Compile, query, gate |
| Finite root35 | `root_precision_mul_le` | p,M,u,i:Nat;p<=M;32u<=M;i<u;a=2u in conclusion | Compile, query, gate |
| Array-guard root35 | `root_precision_of_array` | p<=M;i<u;array guard | Compile, query, gate |
| Source-normalized application | `NativeRootPrecision.normalized_root` | E>0;i<u;array guard withM=2E | Compile, query, gate,fresh replay,PCC binding |
| Exact546E+2 family | `normalized_two_identity` | E:Nat;fixedu2,a4,i1 | Compile, query, gate,fresh replay |
| Universal34 fails | `constant34_family` | E>=32;explicitremainingmargin-domainpremises | Compile, query, gate,fresh replay,PCC binding |
| Positive scalar witness | `scalar_positive` | Fixedp33,M64,u2,i1 | Compile, query, gate,fresh replay |
| Premise deletion | `missing_initial_precision`, `missing_margin`, `missing_active_valuation` | Fixedtuples,remainingpremisesexplicit | Compile, query, gate,fresh replay |

Shared namespace is `CryptoLeanCore.TaperedPrecision`; study namespace is
`NativeRootPrecision`. All new theorems transitively use only propext/Quot.sound,
except constant34_family also uses Classical.choice; the three numerical deletion
theorems use no axioms. Internal induction helpers are checked transitively.
No custom arithmetic/group/curve structure or replacement evaluator is introduced.
The source expression is opened only with existing childPrecision_formula.
[pcc:unverified]

| Bridge | What is established | Remaining condition |
| --- | --- | --- |
| Original graph normalization | Source read:M0=2E,p0=E+1 | Actual graph/call extraction not formalized here |
| Root shape | Source read:a=2u,length2^(2u),array64^(4u) | Successful checked arithmetic/source value binding |
| Array margin | Universal Nat theorem | Source guard must hold for the actual values |
| Full Setup feasibility | One exact unbounded scalar certificate | No allocated graph or production usize admission |
| Active source node | Fixedt2,valuation1,length16 checked | General source-path/domain extraction |
| Later-depth histories | Existing conditional34/history81 unchanged | Actual path/size/precision premises not supplied here |
| Runtime and memory | No runtime measurement | Intermediate rational width,allocation and complete counted bit cost |

The constant34 counterexample applies to a weaker finite root assertion, not
the prior theorem with its additional premises or the paper's asymptotic result.
The root35 theorem is not a whole-history theorem. The huge certificate exceeds
current usize and does not meet the paper's u>=lambda^(4/5) regime.
[pcc:unverified]
