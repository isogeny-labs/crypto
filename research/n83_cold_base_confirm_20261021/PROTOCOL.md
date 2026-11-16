# N83 fresh cold comparison of K1103 and K1107

The finite adjacent-K screen found K1107/root11 at 116.222 seconds and K1103/root11 at 117.637 seconds on one exposed point. The subsequent sixteen-point finalist panel placed both K1107 layouts ahead of the tested K1104, K1112, and K1120 layouts in average one-target cold process time. This protocol compares K1103 and K1107 directly on sixteen new public points with the same `root11_dynamic` pipeline. It is a factor-base comparison under a fixed index layout; it cannot establish an optimum over untested constructions.

## Frozen inputs

- Curve `EC1N83Ckb1a1haacb69ee9f37`: polynomial-basis `GF(2^83)`, binary model `y² + xy = x³ + x² + 1`, subgroup order `8569786107849059`, cofactor `1128547018`, generator and point encoding from the cited candidate manifests.
- Source commit `d13e28e76d3709c19f15cf2c352d4ab912812335`; IC example SHA-256 `8a2fe55dd90dc33892c197d27260182bf783fc8212fb485d8297daf2bbba8e12`. Build the IC and rho examples from this exact checkout with `cargo --locked`, and retain source and executed-binary hashes.
- K1103: 183,098 usable points, 1,103 effective columns, prior exact candidate `IC1N83Ckb1a1fb183098PDP4rootRCguidedLAgaussTDdirectISO0hab4dd05a14ae`, expected base hash `07beb76604b0c9767dafec14aaf50f4de446d2b767f855be79f6ca715d016183`.
- K1107: 183,762 usable points, 1,107 effective columns, prior exact candidate `IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0h8c736ae9c7b4`, expected base hash `b0aea97c1c8b9b46e13df67e6af9c33ca73034d2ec3eff33a2392ceb1452b40e`.
- The two copied candidate manifests record their original source commit `551b9dfc632dce7a5ab8218b1b1dff6ae4b4d785`. The executed `d13e28e...` checkout has the same IC source SHA-256 and `Cargo.lock` SHA-256; the verifier checks both content hashes, each manifest's canonical candidate ID, and the base hash and count emitted by every process.
- Generate 16 points as `Q_i = s_i G`, `s_i = 1 + LE(SHA256(domain || u32le(i))) mod (r-1)`, for `i=0..15` and domain `n83-cold-base-k1103-k1107-sixteen-20261021-v1`. Commit public points, verifier-only scalar sidecars, manifest hashes, and an exclusion audit against previous N83 public-point fixtures before dispatch.

## Execution and admission

For each point run four fresh processes with empty base, index, and log caches. Odd points run `K1103a, K1107a, K1107b, K1103b`; even points reverse that order. Independently solve the same public point by rho before IC on even points and after IC on odd points. Use one admitted Tier 1 cloud allocation with at least four exposed CPUs, 15.5 GiB memory, and 2 GiB free disk, preserving affinity, cgroup, CPU, memory, calibration, source, input, and execution-order receipts. Cap each IC arm at 480 seconds and rho at 120 seconds. Keep all failed, timeout, OOM, and noisy rows.

The verifier must replay each constructed base, relation, rank, target answer, and same-point rho answer. It must confirm the expected base hash and exact usable-point count for both candidates on every point. Each candidate's A/B cold-process spread must be at most 2% of its A/B mean on every point. Any missing, failed, noisy, resource-rejected, or mismatched row keeps the aggregate selector null.

For each admitted point average the two cold process intervals for each candidate, then form `K1103 - K1107` in milliseconds. Report the mean, sample deviation, and two-sided 95% paired Student-t interval over all 16 points. Select K1107 within this pair only if the lower endpoint is strictly positive; select K1103 only if the upper endpoint is strictly negative. Otherwise retain `BOUNDED_UNKNOWN` and a null selection. Keep each one-target online interval, its exclusive phases, and the same-point rho online interval separate from the cold selection. A result from this pair remains bounded by the two specified factor bases and fixed index layout.
