# Run card: PROTOCOL.md (quadratised chain refutation degree)

Prerequisites: `scripts/checkpoint.sh init` succeeds (needs `CHECKPOINT_UPLOAD_TOKEN`); route per
CLAUDE.md (isolab EKS). Nothing below runs until the checkpoint push succeeds.

1. Build and test (no experiment):
   - `cd research/representation_leads_20261109/derive && cargo test --release && cargo build --release`
   - `cargo test --release --example replead_closure` (repository root)
2. Regenerate and pin the systems:
   - `target/release/replead-derive export > ../results/systems.jsonl`
   - `sha256sum ../results/systems.jsonl` must print
     `251302b560bfb05502e5e455997ce6f207a119faa0c959a7480da197ab94b86c`; otherwise stop.
3. For each cell C in n11-m3-l3-a0, n11-m3-l3-a1, n11-m3-l4-a0, n11-m3-l4-a1, n13-m3-l3-a0,
   n13-m3-l3-a1, n11-m4-l3-a0, n11-m4-l3-a1, and each draw K in 0..20, run the four arms in the
   rotated order of PROTOCOL.md "Variable ledger":
   - dense: `cargo run --release --example replead_closure -- research/representation_leads_20261109/results/systems.jsonl --cell C --draw K --arm A --from 2 --to 6 --max-cols 70000 >> runs/C.jsonl`
   - for every degree the dense line reports `"measured":false`, up to 6:
     `... --cell C --draw K --arm A --sparse D --max-entries 1500000000 >> runs/C.jsonl`
4. Stop the cell and mark it void on any line with `"instrument_fault":true`.
5. Commit `runs/`, the receipt, and the masked analysis; the decision is computed per PROTOCOL.md.
