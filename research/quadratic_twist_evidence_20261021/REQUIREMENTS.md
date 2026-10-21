# Requirement-to-evidence map

| Requested/required item | Actual status | Evidence or gap |
| --- | --- | --- |
| PR destination `aburan28/crypto` | this import targets that repository | replacement PR links recorded in the PR description |
| Visible proof that experiments ran | historical transcript, source hashes and raw results retained | `historical/run.log`, `historical/run-metadata.json`, archive |
| Preserve all previous experiments | historical records/source preserved without edits | original 12-file checksum manifest inside archive |
| Native execution in this repository | native certificate audit and controls implemented | `audit.cpp`, `run-audit.sh`, `test-audit.sh`, fresh receipts |
| Replay the full historical census natively | not implemented | 942-model census, S3/S4 enumeration, trace-zero/subfield enumeration and relation collector need a native port |
| Verify algebraic certificates independently | native verification complete for the three saved IC cases | matrix ranks, field logarithms, Miller pairing and public-scalar checks |
| General written proofs | preserved as historical report | archived `report.html` and `report.py`; no stronger formal status asserted |
| Checked Lean formalization / identity-certificate registration | not completed in this import | no Lean toolchain; historical symbolic checks are not Lean proofs or registered IDC1 certificates |
| Repository Conductor check | unavailable (exit 127) | CLI absent; `.conductor/WORKFLOW.md` absent at base revision |
| Repository `cargo test --release --lib` | unavailable (exit 127) | no Cargo/Rust toolchain in this environment |
| Native compiler checks and semantic controls | passed | native audit/control transcripts |
| Source-linked report, graph, PDF | supplied | `reports/` with recorded-data rendering source |
| General GHS implementation / asymptotic advantage | not claimed | original study explicitly leaves both unresolved |

This import is not a native full-suite rerun or a new performance experiment.
The native audit supplies independently recomputed certificate facts only in its
declared fixture set. The original broad hardness question remains open.
