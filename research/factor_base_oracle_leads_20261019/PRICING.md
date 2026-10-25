# Component counters and the unresolved total cost

All arms use the same counter definitions and every attempted asymmetric
branch is retained. Counts come from the native run and independent audit,
not an instruction calibration or an isolated clock comparison.

| Counter | What it covers | Remaining gap |
|:--|:--|:--|
| `field_reduction_multiplications` | Existing F4 F2 row-reduction counter across the substitution tree | Not all solver work; no group-addition conversion |
| `encoding_term_products` | Symbolic field-coefficient term-pair products in exact S3/membership assembly | Assembly, lookups and allocation not completely counted |
| `extraction_pair_checks` | Actual sign/point pairs checked after root decoding | Root-search, sorting and equality costs not fully counted |
| `lift_map_evaluations` | Actual quotient-map evaluations during preimage lookup | Field/map/scan cost is uncalibrated; correction and setup also need pricing |
| `rank_gain` | Training rank on distinct nonzero signed cofactor projections | Matrix arithmetic and storage costs not completely counted |
| Resource-stopped calls | Events and failed-call metadata retained | Their last partial counters are incomplete, never zero work |

F4's `solve` includes substitution and finite-field root enumeration.
A low productive degree or low reduction counter can move work into that
enumeration. It does not establish cheap whole successful decomposition.
The systems, roots and returned point witnesses let that question be
investigated without treating a cheap refutation as a success.

## Symmetric native-operation band

Every row, including both references and the A/A repeat, has the same
missing measurements: symbolic/preprocessing visits, word/Boolean
operations outside the existing reduction counter, root-search work,
hashing, allocations, equality/lookup costs, target and inventory setup,
mapping, verification and rank work. Some emitted counters above give
partial coverage, not complete counts for those categories.

| Weight in group-addition equivalents per unpriced native operation | Complete total for any arm | Cross-arm total-cost ranking |
|:--|:--|:--|
| 0 | Unknown: counted units also lack a complete matched calibration | Unknown |
| 0.1 | Unknown: required native counts missing | Unknown |
| 1 | Unknown: required native counts missing | Unknown |

No arm is ranked as a total-cost winner at any point of the band. A raw
component-count comparison on equally completed positive cases has that
component's scope only. Different numbers of completed cases or point
successes cannot be silently given the same denominator. Original
point-truth enumeration is separately retained audit work, never supplied
as the solver answer.

No full-DLP S, matched-rho ratio, complete cold cost, accepted runtime gain,
or fastest-tool claim is emitted. Magma F4/F5, msolve/FGb and applicable
Singular engines remain the named missing external comparisons (not found
on PATH at setup). No licensing/startup or different-host timing is used
as a substitute. The exact system JSON is retained for future export and
matched-answer comparisons.
