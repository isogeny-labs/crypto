# Factor-base corrective source ready for attempt03

Attempt01 stopped at source preflight: three correctly hashed restored files
lacked their Git executable bit. Cargo did not start. Those modes were restored;
failed logs/receipts were retained and freshly downloaded from S3.

Attempt02 at e1ffaf5b81b6 compiled and ran the dataset suite: 283 passed, 8 failed,
1 ignored. Both source preflight and readback passed. Seven native tests stopped
at a Frobenius-power JSON representation mismatch; one strict-policy test exposed
unknown-field acceptance on the legacy enum unit. These are observed source
failures, not negative mathematical results. Failure evidence is immutable.

Corrective source reads the existing integer Frobenius power and adds a strict
input-only wire enum for policy parsing without changing public serialization.
Targeted regression tests are added, not yet executed. The plan fix passed exact
independent source review. Native behavior beyond the formerly failing parser
remains unverified until attempt03 completes.

Inventory03 explicitly binds Git modes plus bytes for2731 files/253915749bytes.
The installed Python auditor and actual Bash mode-loop checks passed10 tests.
Full v2 CLI/export integration remains pending; scratch worker-binding changes
are not applied. Current CLI continues to admit v1 plans only. Full original
corpus goal remains active, and verified cloud dataset cells remain0.
