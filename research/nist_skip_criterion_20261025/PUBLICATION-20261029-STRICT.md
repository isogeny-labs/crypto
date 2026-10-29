# Strict-count result: publication state

At 15:31 UTC on 29 October 2026, the empirical result and PDFs are complete
locally. Result commit `713202779` and normal current-main merge `ce2cd12c8`
are not yet pushed. Existing regular [PR 2666](https://github.com/isogeny-labs/crypto/pull/2666)
has the new title and scientific description; its last confirmed remote head
is `1238726d9`. This note records delivery status separately from the frozen
[result report](strict_elkies_20261029/REPORT.md).

The canonical export passes with zero findings. All 55 preparation, 79 earlier
execution, 51 strict-result and 77 parent bindings pass. The prior 75 parent
lines are unchanged. The committed-state progress exporter emits 236 valid
JSONL rows in under 60 seconds and retains the verified 6,342 / 100,548 / 210
NIST metrics. The full-PR 10 MiB size check passes. No whole-repository pass
or hosted current-head check is claimed.

The artifact check identifies one plot that must use storage:
`strict_elkies_20261029/strict-counts.png`, 85,071 bytes, SHA-256
`b6593808eca74f4efc663f8c2caada2627c13559512ef88e69409417404d4e75`.
Its upload command has not executed: automatic approval review failed on three attempts
because its selected review model is at capacity. This is a service failure,
not a determination that the action is unsafe. The standalone object's remote
presence is unknown; no successful upload or readback is fabricated.

The next action is the same reviewed `tools/artifacts/artifacts.py push`
for that plot, using the existing authorized client. Preserve the frozen
study `.gitignore`, put an exact plot ignore entry in root `.gitignore`,
and bind the new artifact pointer and successful full-GET receipt additively.
Then rerun artifact admission, commit the packaging correction, push this
branch normally, refresh PR publication details, inspect hosted gate/size/
artifacts and review, and upload the final checkpoint. No policy, account,
credential, storage destination or publication route needs to change.

Preparation, complete native evidence and document archives already have
verified full remote downloads and all-member checks. The four-page main PDF
and two supporting one-page PDFs are inspected. Scientific execution is
finished; no paid job or DLP is running. Historical use, vulnerability and
coverage beyond the declared finite families remain unresolved.
