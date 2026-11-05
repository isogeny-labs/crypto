# PR #2850: raw-data restoration and archival indexes

The validation gate's committed-tree test could not read the four session records.jsonl files after their artifact-store migration. Added the exact study sessions prefix to the existing credentialed artifact pull step. No validation, verifier, freeze, workflow trigger or permission was removed or relaxed. Actionlint accepts the changed workflow. [pcc:unverified]

Recovered all four original raw files from archival Git snapshot `ecfaa38c10908eaadbb105f9c255e0010b6fb46b`, immediately before migration `44bbef796955b64e18a5dd834b20df0a4a46b1aa`. Every byte count and SHA-256 matches the frozen session manifest. Local `validation_gate.py evidence` now reports valid for this entry. This is a static path/hash/schema check, not an independent scientific replay or signed verdict. [pcc:unverified]

Added four currently dated retrospective bench.record/v1 indexes, one per retained session. Their commit field explicitly pins the archival input snapshot; commit_role and provenance record that the executed binary Git commit and dirty state were not captured. The original binary SHA-256 and historical host/session metadata are retained. Commands are documentary launcher transcriptions, not newly executed runs. No current machine identity, historical exit status or executed source commit is invented. [pcc:unverified]

The indexes quote only each session's reported record count as a neutral count metric. Original component-cost values, ratios, caveats and native/replay receipts stay unchanged in their frozen files; no new fully priced S headline or scientific standing is introduced. Derived S3 addresses are not a claim of an upload or remote HEAD by this session. [pcc:unverified]

The local benchmark gate and current-main entry check pass across all 41 records. Existing 37 closed records are unchanged. Progress files come from the unchanged official generator. Fresh current-head CI must still check remote custody, all applicable checks and blocking review state. [pcc:unverified]

All three regenerated progress PDF pages were visually inspected. The archived record counts appear only in the neutral count summary, and the existing dense S-series legends retain their previous crowding. The shared renderer is unchanged. [pcc:unverified]
