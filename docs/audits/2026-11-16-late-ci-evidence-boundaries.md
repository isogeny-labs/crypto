# Late CI evidence boundaries - 2026-11-16

PR4381 merged before its artifacts and validation jobs completed. Runs43790947165 and43790947034 then failed. The artifact workflow implementation from9023f2b5e7 was removed inb513d2ca4e while its tests remained; restore those functions and keep the current generated-step stripping and explicit declarations.

The two benchmark rows listed below fail current admission. Their exact row strings and digests are retained in benchmarks/ARCHIVE.json, outside the admitted table. All frozen reports, manifests, raw data, plots and PDFs remain unchanged. The archive is an admission correction, not new validation.

- Aimed-gather: the November7 registry binding pins an older registry. The original target's complete record compares equal and unique in the current registry, but that component identity check does not replace the full211-input custody validation. A future prospective binding must retain and verify those inputs.
- Packed filter:65 headline tolerances exceed the current1e-6 cap. Its report lives in the artifact store, outside the46 committed report files in the owner-authorized pre-cap snapshot. Do not grow that snapshot or rewrite its precision fields.

The pre-cap inventory previously depended on whether ignored store data was hydrated: it listed tracked manifests but opened any report found in the working directory. Restrict the inventory to tracked report files, as the snapshot contract states. This does not change check_validator_report or exempt store data. The new negative control hydrates a wide untracked report, checks that the snapshot remains unchanged, and confirms admission still rejects it.

Local artifact retrieval returned403; no remote credential or policy was changed. Existing manifests and CI failure logs establish the stated pin and tolerance failures. Re-admission and full store replay remain separate future work. Sources checked: docs/VALIDATION_GATE.md, docs/SABOTAGE_RESISTANCE.md, the original checker and binding history, source:3eecba0a3d223987915a11deff22ad4e (KB revision9141e386254340a9bdba3036f2bb706a is older than current changes). [pcc:unverified]

Artifact hydration also retained manifests for files now authoritative in Git. The old pull implementation overwrites a tracked successor in the new end-to-end control. Pull now filters those paths by the index before any remote access; this preserves staged successors, unstaged edits and omitted sparse files. Workflow discovery applies the same ownership distinction. The historical objects and manifest entries remain intact. Generated-only workflow changes are enumerated and checked separately from the manual PCC-base change in the replay receipt. [pcc:unverified]
