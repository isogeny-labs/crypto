# run-20261023T0030Z-quartic_twist_k1_scale_p37_seed0_v1

[Raw record](../../quartic_twist_k1_scale_p37_seed0_v1.json) SHA-256 `b3c3287b2f5cd3b3cfe41d9dfb4eca4a7d397be05ea9aae5505c65f0eadc304e`; [independent audit](../../quartic_twist_k1_scale_p37_seed0_audit_v1.json) SHA-256 `ae9fc983d4720ffbcef96ab375e3d3e53ffd38a8b57f7d03c69b345bbc032c78`.

Measurement source: commit `a815598a57`; [producer](../../../../examples/quartic_twist_k1_scale.rs) SHA-256 `ed5a6f4daa9242ea94c3813daa1e2a066b01c43d5370e0db8aadc162e778ff5e`; [auditor](../../../../examples/quartic_twist_k1_scale_audit.rs) SHA-256 `4ba2a5734f35a52c575b1763973a6d0ee3d398ae17edc7b907c339a9586bd405`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. One public synthetic target `[271]G`.

Command: `target/release/examples/quartic_twist_k1_scale research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 37 0` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Native status `pass`; independent audit `pass`; 8 of 61 frozen residuals; anchored rank 4/4; recovered 271.
- Independent exact k=1 signed sets: 4 reference, 4 polynomial, 0 missing; 0 algebraic system failures.
- Counted IC: 116,050,118 Fp multiplications and 3,936 group operations. Same-process twist24 rho: 348,941 Fp multiplications and 2,745 group operations.
- All 48 physical factor logs were independently computed with a 1369-point BSGS baby table; the separate exact oracle built 2520 signed L0 pair entries.
- Public generator anchor log: 1; the first signed physical factor may be the generator or its negation.

![Recorded rank and polynomial field-work trace](graph.svg)

The graph plots native recorded values only. L0 wall times in the raw record are not isolated, and hash/allocator CPU and calibrated GAE are absent. This is not a priced IC/rho or external-solver speed comparison.

[PDF](report.pdf).
