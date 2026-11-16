# Runtime-only assembly recovery: independent scoped review

No blocking discrepancy found in recover.py and the deliberately unadmitted binding template. The exact producer source is 81173011355c5fe39267b9d6d644334272060716277ccd6085f28dc116a6bc1d. This review is prospective: no Docker/build command was executed.

## Custody and adoption

All 12 producer manifest members, seven recovery input hashes and 19 original context entries were recomputed. Original base-client terminal exit 0 and client_unknown=false are retained. The privileged IID readback receipt digest agrees with the exact retained owner-tag image 347667d6ecc9294ab7bfcc83411fb47f7548c8d1ca5f7d315cca23fc4b115476; current IID metadata is regular, root-owned 0600, single-link and 71 bytes. No privileged IID read was repeated by this reviewer.

The recovery requires exact original command argv, owner, sole base attempt, source/buildinfo/artifact bindings, recipes and full reconstructed context inventory. It then rechecks same daemon, pinned parent, both adopted base ID and original tag, ownership/phase/revision labels, platform, user, command, working directory, OnBuild and retained layers. Image presence alone is not treated as original build completion. Runtime tag presence or ambiguous absence stops; it is not replaced.

## Narrow effects and limits

The only build loop contains runtime, not base. It binds BASE_IMAGE to the adopted ID and uses original owner/runtime tag. Runtime post-readback requires terminal client 0, scoped IID, matching tag/ID/labels/platform and a layer list extending the retained base. No pull, container, removal, login, push or scientific action is present.

IID reads share the supervised command runner but allow only sudo -n /usr/bin/cat and one of two exact paths. Path components are checked for symlinks; file type, root owner, 0600 mode, link count and size are checked, and device/inode/size/mtime must remain stable after reading. No chmod/chown or arbitrary privileged command is added. This is the existing trusted-host model, not an adversarial same-user filesystem isolation proof.

Deferred signals and prelaunch uncertain ownership remain. Clients are terminated/reaped before further commands; ambiguous launch/build state stops mutation and retains exact-tag reconciliation. Total 600 seconds, 60-second readback reserve, 240-second single runtime attempt and 15-second ordinary calls remain. Filesystem operations remain cooperatively bounded; client termination does not establish daemon cancellation. Existing owned images remain for reconciliation; no cleanup mutation occurs.

Inspection uses anonymous temporary raw storage. Both Config.Env and ContainerConfig.Env are name/hash-redacted before saving. JSON parse/errors do not persist an inspection stdout file. The original packet remains unchanged, including its earlier ContainerConfig field and IID-read failure.

## Binding and review boundary

The template preserves all original binding fields except source/review references and admitted=false. Execution's original-field equality requires admitted=true, intentionally preventing this unadmitted template from proceeding. Root must fill prospective_review_sha256 with this review's hash, make the explicit admission decision and preserve every other field. A concrete filled-binding check precedes execution.

Prior involvement: reviewer authored original assembly supervisor and route packet, but not this recovery patch. The review focuses on the additive recovery path and its concrete custody; prior independent source and final-provenance reviews remain separate. The actual local image rechecks are future supervisor actions. Runtime qualification and scientific admission are not supplied here.
