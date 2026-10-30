# Clean source package 01

Classification: SOFTWARE source custody and build validation. Frozen before
package creation on 2026-10-30. This step produces no research samples.

Create a new standalone Git repository containing exactly the regular files in
the already tested proposal snapshot 09. Do not modify that snapshot or another
task's checkout. The input manifest SHA-256 is
`f8ac4b4d42e18d971f89ace64afe78ecb9bd8f910b2de658d8ca54a4f867038f`.
Its base is `655b690ef56d6869023a41326a127bf319cf4342`; its overlay SHA-256 is
`8ce140a8527b61fdb50be78eadd44e172afaa3348bb6b8c9e909d5bc946afc5f`.

Verify the input manifest, file hashes, full file list, and modes before and after
copying. Copy only the NUL-delimited manifest paths; reject absolute, parent,
Git metadata, newline, and tab paths. Reject links and nonregular files. Verify
the copy before Git initialization, then stage only those explicit paths.
The resulting root commit represents a standalone validation source closure,
not a commit delivered to the upstream repository. Preserve this distinction
in the package receipt and subsequent build receipts.

Acceptance requires exact copied hashes and modes, exact tracked file list,
clean Git status, a verified Git bundle containing the new commit, and unchanged
snapshot inputs after copying. Preserve the input manifests beside the bundle
and record their hashes. Never set build dirty=false for an uncommitted overlay.
Subsequent builds may embed this package's actual commit and dirty=false only
after checking that package's clean source and inventory before and after.

Archive the package receipt, input manifests, and Git bundle to the existing
artifact store; verify a fresh download. Do not create cloud workers, expand a
cloud spending grant, or claim the old test receipts came from a new binary.
The 479 dataset tests and 31 controller tests establish source-level prior
validation; newly built binaries require their own applicable validation gates.

Stop on any mismatch, existing destination, copy failure, or source mutation.
Retain partial preparation and its log; do not silently overwrite or retry it.
Local preparation cap: 1800 seconds plus 30 seconds termination grace. One
preparation process; no Cargo invocation in this preparation step.
