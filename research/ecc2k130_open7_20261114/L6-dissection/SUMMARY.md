# L6 dissection / Schroeppel-Shamir for m=4 MITM: summary

Verdict: CLOSED-BY-PRIOR (prior-art audit only; no measured runs, no PROTOCOL.md frozen). [pcc:unverified]

research-kb and the lead atlas were unavailable in this container. No novelty claim is made.

## Prior work found
- Per-target MITM oracles with the pair table built once and reused across targets:
  research/binary_mitm_oracle_l8_20261015, ..._l9_l10_20261015, ..._l11_l12_20261016.
  These are stage-only records. They state that the 2^(2l) table replaces the 2^(2l) scan, so relation collection stays Theta(2^n).
- m=4 2+2 MITM scaling with a shared table: research/ic_m4_mitm_scaling_20261031/README.md.
  The planted-solve exponent is about 0.41. The uniform-relation cost becomes rarity-dominated (local exponent 1.41 at 40 to 44 bits).
  The README names memory (B^2 table) as the wall and lists external-memory or distinguished-point tables as the next step. Schroeppel-Shamir and dissection are not implemented there.
- The algebraic arms (F4, F6, SAT, S4 hybrids) are slower than the MITM floor at m=3 and m=4: research/ic_m3_algebraic_vs_mitm_20261014/README.md.
- An orbit-compressed pair table already exists as the incumbent: one entry per <sigma,-1> orbit of pairs (compact-orbit / mitm-frobenius, src/cryptanalysis/koblitz_sixsum_extract.rs header, src/cryptanalysis/ecbench/methods.rs). This covers residual (ii), the per-orbit table storage.
- research/ic_leads_review_20261018/README.md section 2.5 cites Dinur-Keller-Marmor (ePrint 2025/783) as formally closing further pair-table and k-sum engineering at ECC2K-130. Lookup-based IC hybrids are non-adaptive.
- Wagner / four-list: research/lead_sweep_20261110/LEAD-EXO-four-list-ktree-join (null result) and research/pdp_strategy_evaluation_20261031/cards/e3.md (closure for homomorphic filters).
- "dissect" in curve_candidates_dissect_20261021 and nist_skipped_curve_dissection_20261021 means the DiSSECT curve-assessment tool. It is not Dinur-Dunkelman-Keller-Shamir dissection.

## Residual reasoning
Schroeppel-Shamir and dissection only change memory: they cut |F|^2 to about |F| at about |F|^2 time per target. They do not lower the time or the operation count per relation.
Under COMMON.md and the success criterion, a change that only reduces memory is not an improvement in the counted metric.
With balanced l = n/m, the time per target is the rho wall. The only possible win comes from reusing the table across targets, and that reuse is already the incumbent design.
Combining the per-orbit table with dissection could remove a memory feasibility wall at larger rungs. That is an engineering enabler, not a cost improvement, so no run was made.

## Open threads
- A memory-only engineering step (a Schroeppel-Shamir 2+2 join on the compact-orbit table) for rungs where the table does not fit. It should be measured only if a rung is memory-blocked.
