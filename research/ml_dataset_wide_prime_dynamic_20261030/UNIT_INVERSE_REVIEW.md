# Bounded unit-inverse dispatch review — 2026-10-30

## What was done

Source216c0e3eb10649d5e9610c239b6e2128247a22cf adds a32-unit dispatch before nonzero inversion. The stored Montgomery identity returns itself; other inputs retain the original4096-unit exponentiation charge and algorithm. Zero remains InvalidInput before admission. Root and an independent reviewer inspected patch2d1282cef370c277daf3ebf7c67a9c19b024d7212deddb45d6f0b6bcf193517f. Three test groups cover exact/one-less admission, expiry, encoded identity versus integer1, general-path operation parity, and fixed ideal basis/pair/pivot consistency. Compilation and tests are pending.

## Decisions made

Reuse existing field arithmetic and the native design in DESIGN.md (inversion and Montgomery representation), fp_backend.rs, and the preserved native panel diagnosis. This is an identity simplification, with no new mathematical or performance claim. Nonunits now require4128 units; this behavior change is explicit. Caps, pair order, useful-work predicates and general inversion are unchanged. Prior432-case source6e94 evidence remains324passes/108failures; no new panel yet. Session log path is reserved by foreign T828, so this owned task note carries the session entry without editing that path.

## Open threads

Complete source-bound focused tests and strict lint, then only if supported prepare a fresh source-bound existing software gate. Producer and independent replay v1 patches remain unintegrated pending review corrections. Extensive scientific cloud dataset remains incomplete; no deadline or budget extension.

## Errors

Fresh migrated EKS inventory failed role assumption. No IAM changes or launches. Diem remains deadline-expired50/52 and the saved freeze gate is unmet. Current reconciliation/review checkpoint was uploaded with conditional PUT and verified by HEAD, ETag-bound GET, archive and contained SHA256 checks.

## Terminal validation update — 2026-10-30T14:49Z

Source216c0e3 compiled;542focused tests passed,0failed,7ignored. Strict release Clippy passed. Exact test executable SHA256c811a2479e17bdf95275c0dbe6a4c85439f3d7c5070e022b55de69f57f48efda. One source-bound invocation of the unchanged432-case original-chain test completed with432unique rows,324useful passes and108failures, exit101. Native stops:24complete,18unit_derived,390work_limit. The all-arity useful-work gate remains failed; no runtime activation or corpus completion follows. This is no speedup claim.

Full raw failed panel archive: s3://setec-astronomy-artifacts/ml-dataset/20261030/validation/native-432-unit-216c-failed/e64263b0447fad56a4f7c2a7de294ecadb83d2bd23a04d6b42b6c86311f448a5/panel.tar.gz

Conditional PUT, HEAD, ETag-bound fresh GET, byte equality, archive SHA256 and contained-file hashes verified2026-10-30T14:49:23Z. Validation and retained v2 patch reviews separately archived under ml-dataset/20261030/validation/unit-inverse-216c-and-v2-review/e3349bffaeb33af821eb013c2c25d382ca5fbc8d8020ab663230e004cd5ef7fa/validation.tar.gz. No repeated gate or new cloud job.
