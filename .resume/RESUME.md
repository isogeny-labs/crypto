# Resume: crypto @ claude/nice-babbage-bn0fn0

## Goal

Owner goal (2026-11-15): lower the counted cost (GAE, S = GAE/sqrt(r)) of the index-calculus incumbent on at least one
row of tables A-C by a verified, measured margin. Outputs byte-identical, or changed only under a new method id. Every
claimed gain needs a second-host replay, an independent validator report, a matched rho reference with the section 5a band,
`validation_gate.py`, and a PR that updates the IC leaderboard, records benchrec metrics and reports the gap to rho. Also
fix the #3991 `ic-rf-pairs` wall-time regression. Stop when one row improves beyond its A/A noise or three preregistered
leads are falsified, with the evidence recorded either way.

## Lead / hypothesis

L1 (registered): the k = 5 k-sum table (`ksum:m=5`) costs less counted GAE than the Table B pair-table recipe on Table B's own
prime curves (fp20, fp24 primary; fp22 secondary). Prediction R_0 in [0.55, 0.95] at 20 bits, [0.30, 0.60] at 24 bits.
Falsified if the pooled 95% upper bound is >= 1 at any weight on both primary rows.

## Spec

`research/ic_tableb_ksum_20261115/PROTOCOL.md`, `specs/P{20,22,24}-r{1,2,3}.json` (9 sessions), hashes in
`REGISTRATION.sha256` (registered 2026-11-15T03:46:33Z, commit on this branch). Pilot (4 targets, different seeds) disclosed, unused.

## State

- L1 registered and pushed; no evidence run yet. Binary to build: `origin/main` 7b3c057e1c via
  `research/wallfix_runpod_20261114/{mksrc.sh,build.sh}` on a RunPod pod (isolab-eks blocked: account budget 1605 > 1300),
  replay on this container with the producing binary.
- #3991 diagnosis done (Callgrind, audit-ir seq 41/51/61): +0.90-0.95% Ir in `rootfind::row_points` and
  `RootfindPairsOracle::decompose_relations`, from the runtime `batched: bool` parameter. Fix not yet written or built.
- Lead atlas unavailable (VLT1, no vault key); research-kb unavailable (no catalog). Do not look for the key.
- /home/user/isogeny-labs (47 MB sparse clone) remains: a safety check blocked its removal; delete by hand.

## Next steps

1. Create RunPod pod `bdr-tbk` (kit: research/runpod_kit_20261112/rp.sh, `$RUNPOD_API_KEY`), upload source tarball of 7b3c057e1c,
   build, run the 9 specs, `verify --replay-all`, fetch sessions and binary, delete the pod.
2. Replay all sessions here, `ecbench compare --weight w` pooled and per replicate, `ecbench band --ranks`, evaluate the
   registered tests, independent validator agent, VALIDATION.json, leaderboard section, benchrec, benchmarks entry, PR.
3. Separate small PR: make `batched` a compile-time choice in `rootfind::row_points`; re-run audit-ir, expect delta < 0.05%.

## Runs in flight

None.

## Stop and success conditions

Success: one primary row passes the registered L1 test beyond replicate noise and lands with all gates. Falsified: both
primary rows fail; then register L2 and L3 in their own protocols before data. Stop at three falsified leads.
