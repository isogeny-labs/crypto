# XP-20261024-rho-canonicalisation-control

**Draft, unqueued.** Card and frozen-draft protocol for rerunning the N83 K1103/K1107 panel's sixteen rho arms under a second canonicalisation, to measure how much the rho basis choice moves an IC/rho ratio at n=83.

Origin: REVIEW.md §5.2 M5 and §3 item 6 of `/Volumes/SSD990-2/ic-regime-review-20261024/`. The audit that found the premise partly moot is `research/rho_canonicalisation_audit_20261024/README.md`.

## What changed from the original plan

The M5 text said "port normal-basis canonicalisation from cryptanalysis". That is already in crypto: the N83 panel's rho (`examples/n83_rho_optimization_eval.rs`, `StrongRhoG::canonicalize` in `src/cryptanalysis/koblitz_strong_rho.rs`) is normal-basis least rotation. So the card's arm N needs no port. The card's arm P is the polynomial-basis orbit minimum (`src/cryptanalysis/koblitz_index_calculus.rs` `canonicalize_signed_rho`), which is the older reference that cryptanalysis's R1 measured. Writing arm P is the only code work.

## Before promotion to ready

1. Write arm P with a receipt that names its canonical basis.
2. Stage the sixteen point files from `research/n83_cold_base_confirm_20261021/targets.jsonl`. The directory is on `origin/main` (commit `753b622349` is an ancestor); the same files also sit on ci branches.
3. Record SHA-256 of the point files, the source commit and the executed binary.
4. Write the verify script the card names (`research/rho_canonicalisation_audit_20261024/verify_rho_receipts.py`).
5. Pick a cairn space and an environment recipe. Operation counts are valid at L0; any wall figure needs L2 on a quiet Linux host.

Files: `card.json` (schema `backlog.card/v1`, validated against `backlog/schema/card.v1.json`), `PROTOCOL.md` (draft, blob `9660763a895bc6c178913f35965a7efe2a78148b`).
