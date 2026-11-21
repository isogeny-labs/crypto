# Validation progress (validator role, restart 3, started 2026-11-13)

Resume checklist only. Script outputs live in out/<target>/; the per-target
narrative is delivered in the validator's handback (the harness blocks
subagent REPORT_*.md writes).

Frozen artifact set: origin/main = c5678a98aef6bbee34df3f277e618dd0a00f8ab1.
SCR=/tmp/claude-0/-home-user/b3fd4014-06f7-5e0c-8338-36d52644c351/scratchpad/validator
Export: $SCR/exp_c5678a9/tree
  python3 -I research/validation_20261112/scripts/export_closure.py /home/user/crypto c5678a98ae... > $SCR/exp_c5678a9/filelist.txt
  git archive c5678a98ae... $(cat filelist.txt) research/chained_prime_transfer_20261111 research/cover_pushforward_20261112 \
    research/small_de_endo_fb_20261112 research/instrument_fixes_20261112 | tar -x -C $SCR/exp_c5678a9/tree
  git archive c5678a98ae... benchmarks/records/<4 records> benchmarks/20261111-chained-prime-transfer docs/VALIDATION_GATE.md scripts/validation_gate.py | tar -x -C same
  cp scripts/val_{tower_solves,cp_dump,sd_dump,sd_rho}.rs $SCR/exp_c5678a9/tree/src/bin/
Build: $SCR/build_c5678a9.sh (release, CARGO_BUILD_JOBS=2, CARGO_TARGET_DIR=$SCR/target); log $SCR/exp_c5678a9/build.log
  then: cargo build --release --offline --bin val_tower_solves --bin val_cp_dump --bin val_sd_dump --bin val_sd_rho
S3: small_de.json could not be fetched (proxy credentials rejected); regenerate by replay, compare sha256 8bcdde7c...

## Status

| # | target | status | outputs |
|---|---|---|---|
| 1 | v3 results_v3.json + certs_v3.jsonl | DONE: replay byte-identical; certs 16/16 indep.; IC re-executed in Python; NOT admitted (plain rho not strongest; rho natives not emitted) | out/t1/ |
| 2 | tower_large | DONE: replay 251/503 identical; 1680 solver logs certified p<=16001; tables 234/234; NOT admitted (plain rho) | out/t2/ |
| 3 | cover_pushforward | DONE: replay primary+15 seeds 0 non-timing diffs; 882 logs certified; svg not reproducible (2 rows); NOT admitted (plain rho) | out/t3/ |
| 4 | small_de_endo_fb | DONE: ADMITTED with caveats; VALIDATION.json written at research/small_de_endo_fb_20261112/VALIDATION.json; validation_gate.py evidence -> valid | out/t4/ |

## Cleanup
CARGO_TARGET_DIR $SCR/target deleted after all runs (binary sha256 in out/binaries.sha256). All four targets done; only the handback remains.

## Remaining commands (historical) (cwd = $SCR/exp_c5678a9/tree unless noted)
T1: done
T2: done
T3: done
T4: done
