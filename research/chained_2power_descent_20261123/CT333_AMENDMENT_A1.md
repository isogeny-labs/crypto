# CT333 Amendment A1 — nonvacuous malformed-isotropic-plane negative control

**2026-11-23 UTC; additive correction before any amended-source execution.** The original frozen CT333 source and JSON are preserved at blobs `744efe7bff8cfa8a524fe5189d8dc6a7dc21b83f` and `f627174d022de627777680c251597f196cbfd8ca` respectively. [pcc:unverified]

A coordinator source audit after publication found a defective negative control: `bad_plane_image()` raised `AssertionError` on both branches of its membership check. Consequently the original reported `mutations_rejected:3` counted one **vacuous** check; only the invalid permutation and duplicated-root matching rejections were genuine fail-closed controls. The 720-permutation arithmetic, two-representation agreement, kernel counts, and class-size checks do not depend on that control and are unchanged. Do not cite the earlier three as independently effective mutation controls.

**Narrow amendment registered now:** replace only the malformed-plane validation with a helper `require_valid_plane(plane)` that asserts `len(plane)==3`, three distinct nonzero entries and membership in the actual `PLANES` set. Feed the deliberately repeated-vector corrupted plane to the helper; it must reject. Add a negative proof-of-sensitivity test that a genuine `PLANES[0]` is accepted, and compare normal/-O output again. No workload, Frobenius orbit algorithm, geometry, experiment budget or source-owner change.

Do not overwrite frozen CT333_RESULT.json. Publish the amended source and a separately named `CT333_RESULT_A1.json`, source blob/hash, exact command and an explicit readback. Update the result report and PDF limitation; do not claim this producer check is a signed PCC or independent validation.
