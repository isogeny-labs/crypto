# Honest partial resultant exports

A native per-cell journal now distinguishes attempted cells from cells that never started. Failure exports preserve canonical records only for attempted cells, list unstarted cells separately and remain explicitly invalid for comparison. No native comparison panel or cloud job has been run.

## Journal boundary

The successor main.rs is generated from the immutable prior panel runner. It writes and syncs a start event immediately before invoking each backend, outside meter::measure. After the checked result row is flushed and synced, it writes and syncs a completion event. Each event binds ordinal, phase, protocol and input hashes. A capture successor includes the journal hash in its external receipt. generate.py records every edit and verifies exact reverse reconstruction of both predecessor files.

A durable start event defines the beginning of an attempt, including possible interruption before arithmetic begins. Absence of a completion event does not prove that arithmetic failed or never finished. A fully written row without its completion event remains unconfirmed; its counters are not promoted. The global attempt still retains the raw bytes. Writes and syncs add outside-interval wall work, so this runner is for counted comparisons; do not use it to infer uninstrumented timing.

## Partial export semantics

failure.py checks the failed external receipt, raw/journal hashes, event ordering and bindings, and every fully parsed row's identity, value and metric shape. It accepts only a sequential prefix with at most one pending attempt. A last unterminated JSON fragment is retained and counted as tail bytes; malformed complete records or impossible row/journal relationships are rejected. Source files and corrupt output remain evidence even when no export can be formed.

The canonical export contains one run per journal start: confirmed cells are succeeded; a cell without completion is interrupted with no counted measurements. **All rows are excluded from comparison** under the failed-panel rule. Never-started cells have no invented run or attempt; failure-flow.json lists their ordinals. An empty launch failure exports zero runs and all240 unstarted cells. A nonzero exit after all cells complete still yields valid_comparison=false.

The export passes canonical JSON schema, internal consistency and file checks. Its full-protocol completeness findings are saved unchanged in failure-flow.json. A shortfall is expected to fail the canonical complete-panel check; this is not waived or called a passing comparison. The previous schema's attempts>=1 remains intact. This resolves the representation problem without changing canonical authority or fabricating unexecuted attempts.

success.py additionally requires all240 completion markers, no truncated tails and a matching journal receipt before delegating to the previous complete-panel analyzer/export adapter. Its canonical export includes the journal artifact. The v2 draft protocol binds this source and adapter set and passes canonical validation, but source commit and production binary remain null pending publication/build binding. No frozen version or authentic admission was created.

## Software evidence

validation-01 audits exact source reversibility, compiles the native successor, runs inherited p13 arithmetic/control tests plus native journal creation/sync/exclusive-file checks, validates the draft, and runs four synthetic Python methods. Synthetic checks cover all481 prefixes of a480-event journal, full rows before completion markers, invalid sequences/pins/types/values, truncated tails, partial/empty canonical exports, all-completed nonzero exits and rejection of an incomplete success journal. Synthetic data never execute native candidate algorithms on the48-case panel. [pcc:unverified]

Replay with the recorded Python venv: `python validate.py NEW_DIRECTORY`. Build90s/native30s/draft30s/synthetic90s, one sequential invocation; source/compiler/binary/log hashes retained. No native or synthetic failure. Two inherited atomic deprecation warnings are retained. The observed EKS inventory was unchanged: seven failed, four succeeded, one suspended/pending; no job mutation. The overview is not authoritative remaining-funds or quota admission.

Sources: previous native panel and canonical capture/export adapters; pinned canonical schema/tool/normative document under feature-native-resultant-protocol/authority, commit41bfaed260093c69da299cbad936205530909b6f. These define incomplete-export findings, attempts and exclusions. No schema bypass. KB review remains unavailable in this session; prior source paths were read directly. No novelty or independent-review claim.

Next: publish and bind the actual journal-runner source revision, obtain admitted production build/preflight and actual funds/capacity receipts, then freeze/publish before panel data. Original4563-case cohort and proposed one-attempt120s/1CPU/1024MiB/USD1 envelope unchanged. No funds reserved.

Evidence: PCC-20261107-1be94f, unsigned producer attestations. Initial report rendering failed on a missing SVG group close; corrected and inspected.
