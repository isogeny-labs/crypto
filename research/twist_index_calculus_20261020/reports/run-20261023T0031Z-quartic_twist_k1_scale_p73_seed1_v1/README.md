# run-20261023T0031Z-quartic_twist_k1_scale_p73_seed1_v1

[Raw record](../../quartic_twist_k1_scale_p73_seed1_v1.json) SHA-256 `6f970c1d0be37d0948ec3e2d3bac0fbc24f4ddb04801fa00b41651fb73360389`; [independent audit](../../quartic_twist_k1_scale_p73_seed1_audit_v1.json) SHA-256 `84a44f6bc0f11d7d50c0d6eff37aa28e49932bc72531923dd596269c8c0c3df7`.

Measurement source: commit `a815598a57`; [producer](../../../../examples/quartic_twist_k1_scale.rs) SHA-256 `ed5a6f4daa9242ea94c3813daa1e2a066b01c43d5370e0db8aadc162e778ff5e`; [auditor](../../../../examples/quartic_twist_k1_scale_audit.rs) SHA-256 `4ba2a5734f35a52c575b1763973a6d0ee3d398ae17edc7b907c339a9586bd405`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. One public synthetic target `[271]G`.

Command: `target/release/examples/quartic_twist_k1_scale research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 1` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Native status `partial`; independent audit `partial`; 20 of 120 frozen residuals; anchored rank 8/8; recovered 271.
- Independent exact k=1 signed sets: 11 reference, 9 polynomial, 2 missing; 1 algebraic system failures.
- Counted IC: 1,183,617,228 Fp multiplications and 10,891 group operations. Same-process twist24 rho: 366,965 Fp multiplications and 3,030 group operations.
- All 96 physical factor logs were independently computed with a 5330-point BSGS baby table; the separate exact oracle built 4512 signed L0 pair entries.
- Public generator anchor log: 28408032; the first signed physical factor may be the generator or its negation.

![Recorded rank and polynomial field-work trace](graph.svg)

The graph plots native recorded values only. L0 wall times in the raw record are not isolated, and hash/allocator CPU and calibrated GAE are absent. This is not a priced IC/rho or external-solver speed comparison.

[PDF](report.pdf).
