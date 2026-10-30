# Final native comparison

Run 20261019-014050-boolean-native-batch-fin-43e52b.

Five interleaved native samples per method and fixture. Outputs are independently checked. Complete-root enumeration and first-witness decomposition have separate boundaries. Read the native JSON for all A/A, admission and paired-bootstrap decisions.

Curves: n17: icv1-f2m17-tm101-00378d4e, 16 Boolean variables, 205 base points; n23: icv1-f2m23-tm5197-1f85e9e1, 22 Boolean variables, 2071 base points. Full identities are derived by the native constructor in fixture-identities.json.

CPU: Intel(R) Xeon(R) Platinum 8488C; compiler: rustc 1.99.1 (58cf8f9c3 2026-10-13); reserved CPUs: 3,7. Full host/memory/ISA and source/binary manifests remain in the run receipts.

Cold costs include fresh Semaev table/oracle preparation and query. Common curve/base construction is outside every solver boundary; these are not whole IC/rho pipeline times. Missing or capped runs are retained and do not decide that roots are absent.

Final same-host run uses a dedicated c7i.2xlarge Spot VM, one Rayon worker and a complete reserved physical core. Host neighbours and frequency remain subject to A/A. Earlier hosts and attempted runs are separate evidence.

The legacy random=true fixture is the unplanted curve generator, not a random group sample. TARGET_LABEL_CORRECTION.md corrects the earlier wording; raw flags and inputs are preserved. These particular targets do not establish an average-case ranking.

Native source: final-summary.json; frozen protocol: FINAL_PROTOCOL.md. Baseline: 1340a6176. New table/setup switches remain opt-in. External Magma/msolve/FGb/PolyBoRi, Arm64 and GPU comparisons are not supplied by this run.
