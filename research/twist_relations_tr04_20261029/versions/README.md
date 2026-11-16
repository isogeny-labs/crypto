# Protocol filing history

No primary TR-04 comparison data was produced under any earlier filing.

- `protocol-v1.json` is the exact original valid frozen file from22:18:02UTC,
  canonical SHA256 `bbfdd9109f37523198ced6ff25f7d50d28b373ee36249e7c8b9a01a0171aadea`.
  Its informational execution-state extension retained stale draft wording.
- `protocol-v2-invalid-attempt.json.txt` retains the exact failed metadata
  filing attempt. The native scope/hash helper accepted it, but the full
  canonical JSON Schema rejected `amendments[0].filed`: a timestamp was supplied
  where YYYY-MM-DD is required. It was never a valid active protocol or used
  for primary data. The text suffix distinguishes failed input evidence from
  active JSON configuration; its bytes have not been rewritten.
- The active `../protocol.json` version3 corrects those metadata fields before
  data. Its numerical inputs, algorithms, budgets, cost accounting and decision
  rules match version1. A pinned complete schema and full preflight validation
  now accompany the deliberately narrower native hash/scope checker.

Archived files retain their original relative analysis-source paths, resolved
against the parent study directory. Native `hash` and full JSON Schema checks
can inspect their exact bytes without rewriting them. Validate active metadata
with the full schema **before** freezing, then validate schema and native hash
again after freezing; neither check substitutes for the other.
