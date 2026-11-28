# The pairs-mode build-index map kept on the device (round 7; protocol, frozen before the run)

## Question
Round 6's stage timings (`research/ic_gpu_rootfind_setup_20261115`, daemon `timing pairs` lines, A100,
|F_x| = 4096) put 45–60 ms of the 88–98 ms session creation in the last stage: copying the 16.8M-entry
probe-index → build-index map (67 MB) from the device into a freshly allocated, pageable host buffer.
The key kernel is 2 ms, dedupe 7 ms, compact 5 ms, probe insert 3 ms. Now the map stays on the device
(`RfCudaSession64::d_pairb`); after each batch `k_pairb_translate` rewrites every reported hit's index
to the first pair's build index before the (small) relation list is copied back. The host never holds
the map in CUDA mode (the emulation path keeps its host map; the Rust pairs tests pass on it).

## Measurements
`rootfind_serve --bench-pairs` (as round 6, abort on any key mismatch); `specs/smoke.json`;
`specs/rf3devmap.json` (round 6's arms and curves, new seeds): `rootfind-subtract:m=3` and the GPU arm
(`probe_build=pairs`, daemon, cuda), both `linalg=incremental-gauss-echelon`, |F_x| 2048 and 4096; GPU
A/A at 4096; rho and A/A. Daemon with `RF_SERVE_TIMING=1`.

## Predictions
1. Identical trials, relations, logarithm, S and every phase's charge across the IC arms (a wrong
   translation would change the confirmed pairs and the charges).
2. Session creation at |F_x| = 4096 (daemon `timing init ... create`, lookup=4): median at most 0.045 s
   (round 6: 0.088–0.098 s).
3. The GPU arm's median oracle set-up at 4096 at most 0.07 s (cross-session, indicative).

## Reading
L0 on a shared pod; walls descriptive; the ecbench sha256 is recorded at dispatch. S3: same waiver.
