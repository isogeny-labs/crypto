# PROGRESS (agent T, open case O3)

- [x] read PROOF.md s2/s5, ic_control.rs, instrument.rs, negation_remeasure.rs (bin and module)
- Design: whole-J IC = ic_control walk + a random full-J generator Z; relation cur = aG'+bT'+kZ,
  extra unknown zeta = log_{G'}(hZ); row: sum eps*l_j - k*zeta - h*b*t = h*a (mod r).
  Needs pub(crate) on private helpers of ic_control.rs (visibility only).
- [x] PROTOCOL.md FROZEN (frozen.sha256) before data; smoke on p=211,419 only, out of repo
- [x] full_jacobian_ic.rs + mod.rs line + src/bin/full_jacobian_ic.rs written; ic_control.rs visibility edit (pub(crate) on pm pa ppow splits_completely roots_of mulmod addm subm combine peel wiedemann Row Poly Dsu). compile pending
- [ ] tests, clippy
- [ ] benchrec init/run/add-raw/metric (leave OPEN)
- [ ] run, RESULTS.md, json, svg, run.log, PROOF O3 note (read only: report)
- 17:30 first run killed at p=51203 stage 2 (would have started p=102407, est > 1 h; not affordable); rerun with --stage2 25601,51203 under the same open record. Run command: ./target/release/full_jacobian_ic --threads 2 --stage2 25601,51203 --budget-s 2400
- tests (cargo test --release --lib chained_transfer::full_jacobian) queued on flock behind agent S; log scratchpad/test1.log
