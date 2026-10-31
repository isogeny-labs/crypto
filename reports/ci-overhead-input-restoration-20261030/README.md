# Benchmark record restoration for PR2407

The actual failed gate122863924740/run40701982201 found twelve absent records.jsonl files after canonical artifact migration. Normally merged current main954c0bb into the original branch; native src, Cargo manifest/lock and volcano path-crate source now have an empty diff from main. The original optimization is already present there; this repair adds no new native algorithm.

The existing validation workflow now restores the exact research/ecbench_overhead_review_20261022/sessions prefix before checking its evidence. The existing generator canonicalizes the command. No gate, budget, trigger, permission or raw record is changed.

Local S3 credentials were unavailable. All twelve original records were instead recovered from the exact pre-migration Git commit a0d6488098e8031ab8366ec19e8deac4cb219553^; each byte count and SHA-256 matches both its canonical artifact manifest and the original validator report. These are original bytes, not reconstructed runs. The affected evidence check returns valid, all28artifact controls pass, workflow wiring and whitespace pass.

The complete62-control local attempt is retained:61pass and the committed-tree control fails on unrelated sparse missing evidence inputs. No local full-gate success is claimed; fresh full hosted checks remain required. The main CI integration in PR2818 separately addresses the canonical registry binding and current catalog consistency. This PR must inherit that fix and pass its own current-head checks before merge. Auto-merge is off.

All original records, reports, protocol, plots and PDF remain unchanged. Existing historical wall-time figures retain their original four-vCPU/allow-busy scope; no fresh performance measurement or qualification is asserted. Scientific graphics were checked and need no rebuild because no reported number or conclusion changed.
