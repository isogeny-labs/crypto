# Independent validation (2026-11-12; `validator-report.json`): pass, gate open, with errata

An independent validator re-checked this directory at `55d2f329d2` [pcc:unverified]:
- the frozen artifact set and hashes;
- the freeze order: protocol committed at 03:11:02Z, sessions from 03:11:47Z;
- the binary: a rebuild differs only in symbol names and build-id, and replays p 34 1,260/1,260 identical;
- `verify --replay-all`: 3 × 1,260 identical;
- [d]G = Q recomputed in its own Rust code for 4,410/4,410 records, with the negative controls rejected, and all 15 curves checked;
- all 63 saved GAE comparisons and 105 band rows reproduced within 1e-15;
- the slope, J1-J4, and the charging symmetry of aut=3 and rho-cubic in the band (E1 fixed);
- rho-cubic as the strongest matched rho (A = 6, 1.02-1.07 times its floor).

Verdict **pass**, disposition validated, scoped to the GAE comparison. `scripts/validation_gate.py evidence` reports `valid`. `VALIDATION.json` covers the GAE comparison only.

Errata:
- **N1 (my error; affects the bit-op line above).** The bitop/v1 rule for `aut_canonicalisations_uncharged` was placed in the generic rule table, which `ic.*` records never reach. These records therefore still leave the canonicalisations unpriced in bit-ops, and the "Bit-ops" figures above are too favourable to the aut=3 arms.
  - Pricing each canonicalisation at 2 field multiplications + 2 word operations (the validator's recomputation): sib8-p512-aut3 / tw2cj-lazy is **0.589 / 0.538 / 0.540** (not 0.368 / 0.334 / 0.334), and sib8-p512-aut3 / rho-cubic is **0.995 / 0.881 / 0.917** (not 0.621 / 0.547 / 0.568) at p 34 / 37 / 40 [pcc:unverified].
  - The statement in this stage's frozen PROTOCOL.md that bitop/v1 prices them was false when the data was taken. The rule now sits in `ic_rule` (commit after this note); re-pricing one p 40 record with `ecbench work` moves it from 70,424 to 113,654 bit-ops/√r.
  - Every IC bit-op price stays a lower bound: other walk counters are still unpriced in bitop/v1.
- **N2.** `compare --unit bitop/v1` does not flag records whose bit-op price is incomplete, and drops failed runs instead of censoring them (0 failures here).
- **N3.** The raw `records.jsonl` were committed for the artifact-store migration, which the bot has since done. The bench record still says `uploaded = false`.
- **N4.** `compare.txt` contains broken-pipe messages from `| head` in `compare.sh`. Cosmetic: all comparison files were saved.
- **N5.** The measuring binary was built before `4e2ea6c1be` was committed. Its code and data are identical to a fresh build of that commit, but no build commit is recorded.
- **Also noted.** At L0 the IC arm takes 5.2-5.3 times rho-cubic's end-to-end wall time. The walk carries 99.75-99.96 percent of its GAE.
