# fb-cost-model native rerun on isolab-eks (specs only)

Origin: ml-cryptanalysis `experiments/fb-cost-model-curves-eks/PROTOCOL_NATIVE.md` (sha256 de80a73c...c27e, byte copy here as PROTOCOL.md).
Two ecbench specs, K1 (a2 = 1 Koblitz) at n = 17 and 19, l = 3: gated ridge pick vs gated hand-rule pick as explicit
`binary-subspace:...,basis=` factor bases, `descent-algebraic:m=3` with `f4-f2`, strong rho reference and A/A control, L2, 5 rounds, warmup 1.
No code change was needed: `BinarySubspaceBase` already takes an explicit basis. Nothing here has been run; no ecbench binary
was available when these were written. After merge to main, dispatch `isolab-experiment.yml` with `source_revision` = the merge commit
and `spec_path` = `research/fb_cost_model_native_eks_20261029/specs/k1_n17_l3.json` (pilot first with targets_per_curve 1).
