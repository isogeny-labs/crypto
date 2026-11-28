# Required owned-search routing and export contract

This packet changes capability disposition only. It must not bypass central verification or claim that owned dataset replay/export is integrated.

## Detection and dispatch

Implement a distinct owned-search detector using the frozen v5 request, owned report adapter/schema, or reserved owned snapshot role/path. Detection must survive stripped or changed outcome labels. A shared `sat-original-gate-map` alone is insufficient to distinguish SAT initial, SAT search, affine and owned search.

In `verify_cell_with_replay_session`, route owned evidence to a dedicated bounded replay caller before the SAT-initial fallback. Without that caller/policy, fail with an owned-search-specific admission error. Do not just suppress the SAT branch; suppress it only after valid dedicated dispatch. Preserve raw custody and failed/partial receipts on rejection. Reject mixed-mode evidence under a frozen exclusive v5 configuration.

## Identity join

Bind configuration, original public input, factor-base bytes, launch binding and original map using existing independent source checks. Bind the worker report pattern, snapshot index, chunk count, final capture head and source hashes to the verified ordered capture. Replay must consume immutable hash-bound bytes, with finite cumulative work, bytes, records and deadline supplied by the caller; no per-snapshot budget reset. A complete encoding is not completed search.

Reuse the existing bounded capture/semantic reader, equation-lineage and polynomial-identity replay results rather than rewriting their logic or trusting unbound producer flags. Their current proof scope is relative captured normalized equations. Until map/descent identity is actually checked, do not advertise independently verified original-source solving. For a complete owned report, validate the exact fixed fields and false/null claim gates. For censored prefixes, retain an explicitly incomplete record rather than treating absence as a negative answer or a successful full replay.

## Export

After admitted owned replay, export a separate restricted owned-search record before generic target construction. Include immutable original/capture/snapshot provenance and only the replay-supported classifications. Keep producer classifications distinct from independently verified labels. Keep first/last fall degree and degree of regularity null unless separately established. Do not admit all-pattern exhaustion, ECDLP recovery, S3 custody, or a full solved instance from one pattern. Do not expose witness/model/search-outcome data as public input features. Unproved or partial prefixes must retain uncertainty and failed-attempt provenance.

Unsupported requests must preserve exact request and unsupported reason, without pretending execution occurred. No new capability disposition, including this packet's requested status, admits execution or training export by itself.

## Acceptance controls

One real owned cell must pass worker -> sealed manifest -> dedicated replay -> restricted export under a fixed budget. Negative controls must cover removed report labels with reserved artifacts retained, mixed mode, changed capture head/chunk order, report/request pattern mismatch, stale manifest/source hashes, insufficient cumulative replay allowance, missing replay policy, partial capture, and forged true completion/degree labels. Existing SAT-initial/search and affine replay routes must retain their prior decisions. No full build or scientific trial is authorized by this document.
