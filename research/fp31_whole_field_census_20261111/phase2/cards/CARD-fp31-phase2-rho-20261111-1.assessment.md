# Assessment: CARD-fp31-phase2-rho-20261111-1

card_id: CARD-fp31-phase2-rho-20261111-1
card_sha256: addb1a6b8b38809424f4b2717e897d9c5d10877ddb5287a9bfdf6bb51e44ae74
relay_verbatim: true (manual comparison; see reasons 1 and 8)
outcome: executed_as_specified
stopped_at_step: null
recommendation: proceed_to_analysis
interpretation: not_assessed

## Reasons (each cites the file or output it rests on)

1. Receipt cards/CARD-...-1.receipt.json (sha256 306064ca...69c81, matches the executor): status completed; commands_total 2; run-phase2 exit 0, 1 attempt, 1000.487 s, not timed out; analyze exit 0, 1 attempt, 0.169 s. The argv recorded in the receipt equal the card's argv, in order (run-phase2 then analyze). Every value the executor reported (1000.487 s, stdout sha256 93c40830...819e, analysis.json sha256 fb5e7e9c...f0741) equals the receipt and the digests I recomputed with sha256sum.
2. Bound files, recomputed now: PROTOCOL.md 3d81d9fe..., manifest.json 92635536..., run_phase2.sh c4257660..., target/release/isoclass-rho 4f089730..., fp31-class-census faeaa6c2...; all five equal the card's binds.
3. run-phase2 stdout (57 lines): 57 lines begin "ok ", 0 lines do not, no FAILED; the ok ids are exactly c000..c056 in manifest order (manifest.json has 57 entries, ids c000..c056). Stderr digest e3b0c442... (empty) for both commands.
4. Each of runs/c000..c056 contains done, walk/walk.json, class/trials.csv, class/results.json, gen/trials.csv: 0 missing files. runs/ holds exactly 57 entries, no *.partial directory.
5. Trial counts: in class/trials.csv every (arm, class) group has exactly 200 rows (TC=200), 0 violations over 58 distinct arm names and 680 (class, arm) groups; in gen/trials.csv every group has exactly 40 rows (TG=40), 0 violations over 463 distinct arm names and 3631 (class, arm) groups.
6. analysis.json: "missing": [] and 57 entries in "classes"; alpha_bonferroni field present (value not interpreted here).
7. Write scope: receipt dirty_after lists only phase2/analysis.json. git status --porcelain (HEAD 27aa11ab22) lists untracked entries only under research/fp31_whole_field_census_20261111/phase2/: analysis.json, figures/, runs_phase2.tar.gz. Nothing outside phase2/ is modified. figures/ and runs_phase2.tar.gz are not produced by either card command (not in the receipt's dirty_after); they appeared after the run and are inside write_scope, attributed to the main session, not the card.
8. Relay check: `tools/task_card.py check-relay` could not run, because `lint` now fails on the card with "commands[1].argv: names repository file .../phase2/analysis.json that binds does not pin". That file did not exist when the card was compiled (it is the analyze command's output and must_not_exist at that time) and exists now only because the card ran. This is a post-run artifact of the lint rule, not an executor deviation and not a change to the card (card sha256 unchanged). In its place I compared the executor's four reported lines against the receipt and recomputed hashes by hand: all match, none rounded, reordered or missing. This replaces the tool check; the main session may re-run check-relay against a checkout without analysis.json if it wants the tool's verdict.

## Noted, not a deviation

- Card `commit` is 53adef0e...; the repository HEAD is now 27aa11ab22 (later commits are run-log snapshots and "stop tracking live logs"). The receipt records commit 53adef0e..., the commit at run time. dirty_before listed the benchmark record and the card yaml (both now committed, not in git status).
- Isolation L0 local route is the recorded deviation in PROTOCOL.md section 7 per the card notes; not assessed here.
- No interpretation of any result is made or implied.
