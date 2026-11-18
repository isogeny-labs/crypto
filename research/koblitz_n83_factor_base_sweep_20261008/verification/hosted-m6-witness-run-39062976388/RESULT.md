# Hosted N83 six-summand witness ladder: bounded access control

The [hosted run 39062976388](https://github.com/aburan28/crypto/actions/runs/39062976388) completed at source commit `3f43c2d16230b0b55722c01e7a6993b4c4b47b50`. Its 78-file raw artifact passed the independent [audit](audit.py), recorded in [audit.json](audit.json). The audit pins source bytes, image, worker, three regenerated S3 factor-base archives, frozen fixture, release-test logs, every raw stage receipt, Docker isolation, cleanup and original-field five-node S3 equations. Rebuilding the fixture from the archived full points also passed original-group and subgroup checks. The artifact tree SHA-256 is `8b52bc32721fc8fb77d0c5924a2fe40940d37287483043ab2ab36b019313d675`.

| Pool | 10 x blocks fixed | 6 summand x blocks fixed | Target only |
| --- | --- | --- | --- |
| Gray residual, 97 signed orbits | SAT at 2,049/5,000 conflicts; 106 s | `UNKNOWN_conflict_cap` at 10,000; 286 s | `UNKNOWN_conflict_cap` at 20,000; 999 s |
| Hash residual, 600 signed orbits | SAT at 3,022/5,000 conflicts; 273 s | `UNKNOWN_conflict_cap` at 10,000; 314 s | `UNKNOWN_conflict_cap` at 20,000; 975 s |

The ten-block stage pins the six known summand x coordinates and four intermediate-sum x coordinates. The fixed x tuple has an independently replayed lift to six full subgroup points summing to its public planted target. Both arms therefore demonstrate that the six-summand encoding can reach a known valid tuple under the full factor-base domains. Removing the four intermediate pins or all ten pins reached the declared conflict caps; those outcomes are censored, not UNSAT verdicts. The two arms use **different planted targets**, so their conflicts and walls do not rank the Gray and hash bases.

The observed supervisor wall was 2,953 seconds on the hosted runner. The charged experiment envelope is the frozen **10,800 seconds**, including setup, tests, archive regeneration, worker build, all six stages, and upload. The hosted release library gate passed 4,084 tests with 117 ignored, and the touched example gate passed. The artifact contains the 54-base regenerated pilot manifest and the three exact archives used here. Those archives match the previously verified objects under `s3://crypto-autoresearcher/factor-bases/icv1/etc/` by SHA-256, BLAKE3 and compressed length; this run's broader archive remains local to GitHub Actions and the independent audit scratch.

This known-answer control does not estimate natural-target relation yield, double-large-prime closure, useful rank, linear algebra, individual-log extraction, paired rho, or fully charged single-target index-calculus runtime. `selected_best_total_runtime` remains null. The independent audit can be rerun with:

```sh
uv run --with blake3==1.0.8 python \
  research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-m6-witness-run-39062976388/audit.py \
  --artifact DOWNLOADED_ARTIFACT_DIRECTORY
```
