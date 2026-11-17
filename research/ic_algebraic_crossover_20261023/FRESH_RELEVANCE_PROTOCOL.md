# Fresh-fixture target curve relevance

2026-10-25, frozen before output. Extend the original seven-fixture
curve-membership diagnostic to all 1,152 fresh fixtures, without selecting
on the new result or altering any branch, input, quota or acceptance gate.
This addresses the observed distinction between finite Boolean fixtures
and ordinary curve/subgroup queries.

Read all 36 frozen fresh JSONL files (nine cells, four families, 32 draws).
Use each exact field representation and E0: y^2+xy=x^3+1, consistent with
the recorded b=1 S3 chain. Solve each target's quadratic using the existing
independent linear-map oracle, verify every root by substitution, and check
existence against an independently evaluated Artin--Schreier trace criterion.
Bind each file by SHA-256; preserve every SAT and UNSAT draw.

Report liftable and nonliftable targets separately within Boolean SAT and
UNSAT classes, per file and overall. Subgroup membership, signed point
decompositions, query-law representativeness and all complete-cost fields
remain unknown. A nonliftable target is a valid algebraic fixture but not
an ordinary E0 query point. A lift is insufficient to establish subgroup
membership. Do not silently discard either class or retrofit target laws.

Native code: examples/ic_fresh_relevance.rs. Freeze source before output,
use the existing 12-GiB/900-second local envelope, retain complete native
JSON, hashes, process status and a descriptive inspected graph/PDF. This is
correctness/relevance only, not a speed result. No paid allocation.
