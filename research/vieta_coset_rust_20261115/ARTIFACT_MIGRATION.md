# Artifact migration and PR follow-up

The repository's artifact-autofix bot committed `3288492515fbaa20bd660c2ab45f213c923d59c1`
after the local study was published. It replaces selected tracked data files
with five `.artifacts.json` manifests and managed ignore entries. This is a
storage migration after measurement, not a new experiment or changed epoch.

The [producer audit](scale-20261115T2053Z/artifact-migration-audit.json) compares
every migrated path against its pre-migration Git blob: all 24 hashes and byte
lengths agree. Rust sources, frozen protocols and the two pending PCC claim files
are unchanged. These are producer observations [pcc:unverified]; the audit does
not assert a local S3 retrieval or independent signed standing.

To materialize store-backed files before replay or rendering, use the existing
artifact tool with the repository's configured read access:

```sh
python3 tools/artifacts/artifacts.py pull research/vieta_coset_rust_20261115
python3 tools/artifacts/artifacts.py pull benchmarks/20261115-vieta-coset-local-scale
```

The user-authorized Git checkpoints also remain readable at
`d124756e58082bebae16cb97eb65fdb3ed4e787a`, including the original data bytes.
For example:

```sh
git show d124756e58082bebae16cb97eb65fdb3ed4e787a:research/vieta_coset_rust_20261115/scale-20261115T2053Z/repair/three-p224-m128.jsonl > /tmp/vieta-repair-records.jsonl
```

No benchmark was rerun and no new cloud experiment was submitted in this
follow-up. Both original and repaired epochs retain their existing limitations.

## CI disposition at the first follow-up

- Migrated head `328849251`: review and artifact migration checks succeeded;
  current-head gate/size/artifacts checks have not run. It is not merge ready.
- Old head `d124756e5`: validation gate, size, replay and Rust lint succeeded.
  Artifact policy failed before the bot migrated data. The local artifact policy
  check now passes [pcc:unverified]. Old-head results do not satisfy new-head gates.
- PCC and learning failures showed newer main ledger/lesson entries absent from
  the old branch. They require merging main while preserving append-only data.
- Benchmark-records CI failed only on the unrelated open record
  `20261115T0609Z-genus-ladder-sge-scale`, which is also open on fetched main
  `a3168ac9f`. That record is outside this PR's scope.
- PR 4358 is conflicted. Copilot was asked to merge main in comment
  [6902948102](https://github.com/isogeny-labs/crypto/pull/4358#issuecomment-6902948102),
  without force-pushing, changing evidence, dropping journal blocks or choosing
  one ledger side. Shared conflicted files are left untouched while pending.

The next bounded check-in must review Copilot's response or use the documented
timeout fallback, then validate the resulting current head. No gate is waived.
