I recommend **a separate restricted root-certificate reference file**, with no change to `solution_certificates`. Root completeness, PDP witnesses, and scalar recovery would remain separate evidence classes.

I verified the live `main` SHA through GitHub as `78f84234e18facfc46be52741d7fa6acadc92b88` and inspected its exporter alongside prime source `dd0f10b06253a38d503f3010df679c83356fc29d` and binary source `dd1092a0c0cdd07f032cee6f4cf692e753035a33`. Main currently includes only `independently-verified-pdp` and `restricted-independent-scalar-label` in `solution_certificates`; its restricted index already preserves all root artifacts through `source_artifacts`. The separate dataset worktree also contains newer prefix-export work. This proposal does not replace or extend that graph/prefix projection.

1. **Add one independently versioned restricted file.**

   Use `restricted/root-certificates.jsonl`, artifact role `restricted-root-certificate-references`, with rows bearing schema `ml-dataset.restricted-root-certificate-reference/v1`.

   Emit one row for each sealed cell containing a successfully replayed, recognized native root-certificate bundle. Select the exact roles `prime-root-certificates/v1` and `binary-root-certificates/v1`; reject malformed, ambiguous, or family-inconsistent artifacts through central replay.

   Use the existing bounded external sorter. A deterministic record identity can bind the schema domain, existing `target_id`, and certificate artifact SHA-256. Do not duplicate root lists, native events, per-call reports, or selected states into the new file.

2. **Make the reference contract explicit and small.**

   Each row should contain:

   | Field | Meaning |
   |---|---|
   | `record_id`, `target_id` | Deterministic record identity and join to the existing restricted target |
   | `feature_id` | Existing feature identity, nullable; absence must not discard root evidence |
   | `source_run_sha256`, `source_manifest_sha256` | Exact source occurrence, matching the restricted index |
   | `certificate` | The original `Artifact` descriptor: role, relative path, SHA-256, bytes, encoding |
   | `certificate_schema` | Exact recognized prime or binary bundle schema |
   | `claim_scope` | `distinct_field_roots_per_completed_recorded_polynomial` |
   | `recorded_calls` | Counts of finite-complete, all-field-complete, incomplete, and total supplied calls |
   | `adapter_censored`, `adapter_root_queries_completed` | Separately named adapter termination fields, copied only after replay |
   | `execution_trace_complete_verified`, `total_operation_count_verified` | Both required to remain `false` |
   | `verification` | Export-local source-cell replay passed, with the verification method explicitly identified |

   The certificate path resolves relative to the source cell named by the matching restricted index row, **not relative to the export directory**. Consumers obtain public/config/factor-base and auxiliary descriptors from that same index occurrence. This avoids duplicating its artifact graph.

   Prime v1 lacks binary v1’s full public/base/config digest binding. The reference binds exact source-file identities through the manifest; it must not imply that prime v1 independently authenticates arbitrary public annotations.

3. **Preserve per-call completeness and censoring semantics.**

   The counts must be derived from replayed report termination, rather than interpreting the bundle’s existence or `distinct_root_certificates_verified=true` as universal completion.

   - A completed finite report certifies the distinct roots of that recorded polynomial.
   - A completed zero-polynomial report certifies the explicit **all-field-elements** case. Its empty root vector must not become a zero-root label.
   - An incomplete report establishes no root absence.
   - A censored bundle may retain valid completed reports preceding its stop.
   - A valid bundle containing zero calls remains referenceable, with all call counts zero.
   - Finding a PDP witness may stop the adapter early. That does not imply that every planned root query was completed.

   Do not introduce a bundle-wide `complete_roots=true`, an UNSAT label, or a PDP/scalar success derived from root completeness. Existing PDP and scalar certificates continue through `solution_certificates` and their existing verification.

4. **Require full source replay before publishing a reference.**

   Keep `verify_cell(dir, true)` as the prerequisite, followed by the existing export run/plan/manifest consistency checks. That gate includes root arithmetic, public-domain binding, outcome consistency, event/byte admission, and auxiliary-file integrity.

   A known root artifact that fails replay aborts export; it is not silently omitted or downgraded. Unsealed attempts retain their existing exclusion records and receive no verified root reference. Legacy summaries without native certificates remain readable but receive no native-root reference.

   The exported verification field describes **the exporter’s replay**. It must not claim that a separately implemented consumer has audited the bundle. An independent consumer should resolve and hash the source artifacts, check reference/index identity, and run its existing independent root checker before reporting its own acceptance.

5. **Keep every new field outside model inputs.**

   Roots, root counts, call counts, completion/censoring status, artifact identifiers, paths, work counters, and verification results belong entirely in the restricted file. Neither static features nor causal prefix features should read them.

   Preserve feature projection, feature hashes, split assignment, and T804’s graph representation. Do not add this file to `model_input_files`. Root evidence should still be exported when `feature_id=null`, including a verified root cell without the witness artifact currently required for feature projection.

   This increment adds references, not a new root-learning task. Any later public root-state feature projection needs its own frozen leakage and causality contract.

6. **Use additive compatibility without changing existing row meanings.**

   Keep the existing feature, target, and index row schemas unchanged. In particular, retain the existing contents of `solution_certificates`.

   Add the new restricted artifact descriptor, its row count, and an explicit supported-reference-schema declaration to the export manifest. The sidecar has its own `/v1` schema; existing source certificates are neither rewritten nor migrated.

   Missing sidecars in historical exports mean “extension absent,” not “no roots” or “verified empty evidence.” New consumers require a bijection between recognized root bundles in sealed source-index rows and emitted references, catching dropped or duplicated references. Older strict consumers may require an explicit compatibility update for the new manifest artifact role; do not claim compatibility without checking them. They must ignore this unsupported restricted extension deliberately or fail closed.

7. **Validate the semantics, identity, and leakage boundaries.**

   Meaningful tests should cover:

   - Prime, binary, and Koblitz roots; both twists; odd/even binary fields; complete, censored, and zero-call bundles.
   - Explicit all-field completion versus completed finite empty-root sets.
   - Root references present while `solution_certificates` remains empty when no PDP/scalar certificate exists.
   - Exact source occurrence, artifact digest/length/role/encoding, and restricted-index joins; rehashed reference substitutions must fail independent verification.
   - Rehashed malformed root certificates, auxiliary omissions, false byte-cap labels, and outcome overclaims causing export failure.
   - Legacy summaries and unsealed attempts producing no verified root reference.
   - Reference retention with `feature_id=null`.
   - Identical existing feature/target/index bytes before and after adding the extension, plus unchanged graph/prefix output and model-input allowlists.
   - Deterministic sorting, retry identity, restricted permissions, and source artifacts remaining unchanged.

Implementation can stay within a small exporter helper plus sorter/install/manifest hooks and exporter tests, coordinated with the current exporter owner. No producer arithmetic, native root schema, public feature schema, graph projection, IAM, or scientific execution change is needed.

**This was design and source inspection only. No files were edited, and no builds, tests, or jobs were launched.**

<oai-mem-citation>
<citation_entries>
MEMORY.md:134-134|note=[used retained donor and exporter ownership guidance while checking current source]
</citation_entries>
<rollout_ids>
01a18017-002e-76d3-a61c-692f8a3ae802
</rollout_ids>
</oai-mem-citation>
