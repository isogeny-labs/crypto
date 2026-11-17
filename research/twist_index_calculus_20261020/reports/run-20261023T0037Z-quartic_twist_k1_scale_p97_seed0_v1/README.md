# run-20261023T0037Z-quartic_twist_k1_scale_p97_seed0_v1

[Raw record](../../quartic_twist_k1_scale_p97_seed0_v1.json) SHA-256 `5cfcc2e4af6850d06c65cc56ad47a90712635f31e1cf356f580a3d73b456b2ad`; [independent audit](../../quartic_twist_k1_scale_p97_seed0_audit_v1.json) SHA-256 `b2043ca81cf2b4ce524a900f28efb3eea0cae13a036aa15a34c00afc06383b59`.

Measurement source: commit `a815598a57`; [producer](../../../../examples/quartic_twist_k1_scale.rs) SHA-256 `ed5a6f4daa9242ea94c3813daa1e2a066b01c43d5370e0db8aadc162e778ff5e`; [auditor](../../../../examples/quartic_twist_k1_scale_audit.rs) SHA-256 `4ba2a5734f35a52c575b1763973a6d0ee3d398ae17edc7b907c339a9586bd405`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. One public synthetic target `[271]G`.

Command: `target/release/examples/quartic_twist_k1_scale research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 0` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Native status `pass`; independent audit `pass`; 310 of 558 frozen residuals; anchored rank 6/6; recovered 271.
- Independent exact k=1 signed sets: 16 reference, 16 polynomial, 0 missing; 0 algebraic system failures.
- Counted IC: 13,839,508,536 Fp multiplications and 53,111 group operations. Same-process twist24 rho: 802,322 Fp multiplications and 5,463 group operations.
- All 72 physical factor logs were independently computed with a 9410-point BSGS baby table; the separate exact oracle built 2520 signed L0 pair entries.
- Public generator anchor log: 1; the first signed physical factor may be the generator or its negation.

![Recorded rank and polynomial field-work trace](graph.svg)

The graph plots native recorded values only. L0 wall times in the raw record are not isolated, and hash/allocator CPU and calibrated GAE are absent. This is not a priced IC/rho or external-solver speed comparison.

[PDF](report.pdf).
