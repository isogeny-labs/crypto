# Concurrent PR integration

PR2036 advanced during blob upload to05ec85b5287ebf137f1b1e16c8e91705d48cf77f.
It merges main and adds planner/curve-trait code, canonical graph updates and the
validation gate. Root preserves that exact ancestry and all unrelated changes.
Only merge conflict: benchmarks/README.md; final content equals05ec85b5, removing
our newly proposed unadmitted row while retaining every remote row/disposition.
No native SAT/guard/retained-query source changes came from the concurrent merge.

Run full release library/exporter/model-checker/native-guard checks on combined
code. Preserve their actual logs/counts separately from frozen7e8ae96c validation.
No new SAT worker/query, no conflict-cap extension or extra timed allowance.
Retained worker/guard ELFs and all experimental/S3 records remain immutable.
Integrated source history and actual validation receipts receive a separate
freshly downloaded/repacked S3 supplement; original archives are never replaced.
See report CLARIFICATION.md and forthcoming INTEGRATED-VALIDATION.json.

Combined source d5aac8e9f4ef313dfea310a712fdeca8b96b3d8a passes4481release
library tests/0fail/119ignored,19exporter,2independent model-checker and38guard
unit+7card tests. Actual logs in integrated-validation/ and structured
INTEGRATED-VALIDATION.json; original7e8ae96c validation remains unchanged.
No added admitted benchmark row, fake scalar/rho certificate or new SAT worker.

S3 supplement evidence/n83/allocation-profile-20261021/integration-verification.tar.xz:
608144bytes,21payloads plus manifest, SHA256
99aeb0c37f3d585368be87a66787102ccc154aa1b2e818e573ebe8600bb65c53.
FreshGET/allpayload/sourcebundle/validationlog comparison/deterministic repack
PASS. SOURCE-INTEGRATED.bundle retains combined code with remote05ec85b5
prerequisite; original raw03754a1f and sidecarfdf07b85 stay immutable. Actual
external receipts in integration-proof/. No merge readiness admission.
