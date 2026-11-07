# Complete-reference reporting guard

Source audit found that band means are computed over verified runs, while Autolab copied the rho mean without requiring all reference runs to verify. That can put a censored mean into distance reporting. The incumbent mean used for training selection has the same omission. [pcc:unverified]

The candidate accepts a cost mean only when runs>0, verified==runs and every cost is finite and nonnegative. A distance additionally needs positive finite reference costs and finite ratios. An IC improvement versus its incumbent remains independent of reference availability; optional rho_s prevents writing a censored mean while retaining that improvement. Legacy populated rho_s arrays keep their JSON encoding. No historical ledger is edited. [pcc:unverified]

Five standalone native checks pass, including partial/empty cohorts, complete-cohort ratios, invalid denominators/overflow and legacy array serialization. serde-harness.rs includes the exact candidate ArmConfig and Incumbent definitions; serde-receipt.json records the dependency hashes and build/test exits. This is not an integrated Autolab test result. [pcc:unverified]

The current native build uses the unchanged source snapshot. integration.patch and ic_autolab_candidate.rs are prepared but unapplied. After that build completes, verify the source hash in receipt.json, apply the patch to src/bin/ic_autolab.rs, run all Autolab tests and use the resulting source in final validation. [pcc:unverified]
