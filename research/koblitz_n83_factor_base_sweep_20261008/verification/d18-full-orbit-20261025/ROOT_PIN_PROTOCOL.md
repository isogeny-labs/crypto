# Full-D18 selector-and-phase pinned planted control

The shared-root `m=4` planted control at source
`1f04629635101e04ac662bef28ce589b92bc929b` built its complete
2,807,752-clause model but reached a 300-second cap without a solver answer.
This follow-up changes only the constructed control: for each of its four
frozen representative points, it additionally pins the exact one-hot orbit
selector and phase-zero bits. It retains the same pinned summand and
intermediate x coordinates, target, full base, signed-Frobenius row and
original-field group/rank/tamper checks. The natural-query worker does not
receive these pins.

Before execution, commit and test the opt-in source. Reuse the exact
freshly fetched and hash-checked 32,643-column S3 archive and public corpus
from `SHARED_ROOT_PROTOCOL.md`. Run only
`primary-full-d18-m4-l0-shared-root-pinned BASE_DIR FIXTURE_DIR NEW_OUTPUT_JSON`
with the same 300-second internal whole-process cap, independent 310-second
outer cap and at most 100,000 SAT conflicts. Retain source/binary hashes,
input GET receipts, UTC interval, stdout/stderr, raw timing/resource output,
phase, shape, cap or result, and a SHA-256 inventory. Audit selected
representative indices and phase-zero labels before accepting a result.

A `PASS_CONSTRUCTED_CONTROL` must include a SAT model, original-field S3
check, exact group lift, signed-orbit row, changed-target rejection and
rank increment one. The additional pins make this a circuit and verifier
correctness control, not a free-selector search sample. A cap, failed check,
missing receipt or process failure is censored/invalid. Do not infer natural
yield or an IC runtime winner; keep `selected_best_total_runtime=null`.
