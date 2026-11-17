# Runtime mount destinations: independent source review

Reviewer `/root/strict_storage_reviewer`; input and prospective-plan commit recorded in commit.txt (`0de65e421b...`). I did not author the proposal. Outcome: **source delta is faithful to the stated proposal; runtime mechanism remains untested**. No image build, Docker call, container operation, cloud action, qualifier execution or scientific workload was performed. This review is not execution admission and renews no budget.

## Verified immutable facts

The exact patch adds only a comment and two COPY directives, leaving ARG BASE_IMAGE, FROM, runtime COPY, USER0:0 and the empty ENTRYPOINT unchanged. The new destinations are `/fixtures/.mountpoint` and `/qualification/exec-gate`. Both source blobs are empty and Git mode100644. Their published-checkout files were independently observed to be ordinary zero-byte files with mode0644, matching the pinned bytes. Git's mode records non-executable regular-file status, not all Unix permission bits; eventual staged context permissions must be bound separately. Both COPY directives specify numeric root user/group ownership. There is no added RUN, executable payload or changed gate implementation.

Old/new recipe hashes and the unified patch match bindings.json exactly. The pinned unchanged qualifier hashes to `5dc1c3f6261bc5d8e38b7efb454058044a4658b18603553757a9c5a42419e240`. Its strict assertion remains `require(rc==0 and not diff.strip(),'unexpected writable-layer change')` at line220. There is no exception for these paths or other writable changes.

The retained receipt remains failed: passed_cases contains only Q0; failure is unexpected writable-layer change; qualification_complete is false; cleanup_complete is true. The retained diff records A /fixtures, A /qualification and A /qualification/exec-gate. Its image ID and binary digest remain historical identities. This source review does not authenticate the original execution beyond the preserved receipt, and does not reinterpret the diagnostic exit or partial pids observation as qualification success.

## Mechanism and missing-mount attacks

Docker documents that COPY creates absent destination parents and preserves source file permissions. Bind mounts obscure preexisting destination data; `--mount` normally errors if the host source is absent. These facts support the proposed destination construction, but do not establish this daemon's writable-layer diff behavior. [Dockerfile reference](https://docs.docker.com/reference/dockerfile/#copy), [Bind mounts](https://docs.docker.com/engine/storage/bind-mounts/).

The marker supplies a regular file beneath /fixtures so that the parent directory exists in image data. The gate placeholder supplies the exact expected file path beneath /qualification. Numeric ownership avoids dependence on image username lookup. Neither path is a trailing-slash destination that would place the source basename unexpectedly.

For active Q1–Q3 the unchanged qualifier explicitly mounts the fixtures directory and gate file readonly, checks mount type/source/destination and RW=false after creation, and requires exactly the expected bind count before start. Missing or wrong destination mounts therefore fail the source acceptance path before tested execution. The host gate is separately required to be an ordinary single-link0555 file matching its bound hash; fixture members, hashes and modes are checked. The empty nonexecutable0644 placeholder cannot substitute the real gate under the requested UID1000 launch. Q0 deliberately does not mount or execute the gate. If a fixture mount were absent, the image placeholder contains no request data; it is not a successful fixture fallback. These are source-path observations, not new daemon negative-control results.

Wrong ownership or metadata in the actual layer remains unmeasured. Parent-directory permissions, inherited symlinks or path conflicts, copied context permissions, and actual engine mount setup must still be inspected against the exact base/context. The recipe does not explicitly fix every possible inherited parent metadata state. The strict diff assertion will continue rejecting any nonempty reported writable-layer change; source placement does not whitelist or hide it from that assertion. Docker diff itself has its existing observational scope and is not a new whole-filesystem integrity proof.

## Base binding and copy-only boundary

The recipe still accepts BASE_IMAGE; the proposal's bindings do not yet supply an exact new image ID or qualification receipt. The historical assembly PLAN.md requires an accepted exact local base and rejecting nonempty upstream OnBuild before building, but it is not fresh execution admission. The proposed two-line delta introduces no new instruction that runs image code. An unconditional claim that a copy-only Dockerfile alone prevents all base-trigger execution would be too strong: Docker runs inherited ONBUILD instructions at downstream FROM and clears them afterward. Consequently the qualifier's post-build empty-OnBuild check cannot replace pre-build inspection of the exact base. [Docker ONBUILD reference](https://docs.docker.com/reference/dockerfile/#onbuild).

Retain the existing pre-build base checks, bind the actual base ID/config, exact context member bytes/modes, recipe and builder inputs before any admitted assembly. The historical recipe/runtime bindings were read as context and copied to this review; they are not promoted to new admitted bindings. The proposal already states that execution requires complete bindings and fresh admission, so this is an unresolved preparation obligation, not evidence that an unauthorized build occurred.

## What is still untested

1. Whether this precise image construction eliminates all three observed additions without other metadata changes on the admitted daemon/storage driver.
2. Actual final-layer destination and ancestor types, permissions and ownership for the exact base/context.
3. Fresh positive Q1 plus negative Q2/Q3 results, their pre-exec observations, empty diff and cleanup using a newly bound image.
4. Any timing/scientific cohort result.

The appropriate next step is to complete exact assembly/admission bindings while preserving the old failure, then obtain the bounded execution authorization required by the coordinator. This report grants none.

## Attestation and source inventory

Read the immutable prospective review-plan, old and proposed recipes, patch, two context files, bindings.json, status.json, REPORT.md, retained Q1 diff and retained qualification receipt at the input commit. Read exact pinned runtime-qualifier-v5/supervise.py, particularly source/fixture bindings, container create/inspection, run_case, diff assertion and post-build image inspection. Also read the separately archived historical local assembly PLAN.md and bindings.json under /var/tmp/pencil-local-image-assembly-proposal-20261031-v1, and official Docker documentation linked above. source-checks.json and input-sha256.txt preserve mechanical findings and input hashes. No sibling new reports or private outputs were read. Context is inherited, so no blind-derivation claim is made. Prior Joint C artifacts were not modified. No scientific status or execution authorization is changed.
