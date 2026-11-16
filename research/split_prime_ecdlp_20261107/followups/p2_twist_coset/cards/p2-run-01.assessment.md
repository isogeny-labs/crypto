# Assessment: CARD-p2-twist-coset-20261107-1

card_id: CARD-p2-twist-coset-20261107-1
card_sha256: d75b1445c1032a0576ed7c70d4b449c49fbe97f1fd56875cf781a8b2695a88dd (matches receipt.card_sha256)
receipt_sha256: 225bc8f4080240106639d4c1866f4aaea56960c7cd3a63da9f0e84c6eb9a0914 (matches dispatch)
relay_verbatim: not_fully_verifiable (see caveat 1); every number in the executor excerpt equals the receipt and disk
outcome: executed_as_specified
stopped_at_step: null
recommendation: proceed_to_analysis
interpretation: not_assessed

## Checks (all against files on disk)
- Receipt: schema task-card-receipt.v1, status completed, commands_total 7, all exit_code 0, attempts 1, no timeouts. Order: build, sha256-p2coset, sha256-msolve, driver-m1, driver-m2, driver-m4, driver-m3 (the card order, m4 before m3). Commit 72004b435c02aae8bd7e5aff9fb31429e146cec8.
- Bound hashes recomputed and equal to the card: PROTOCOL.md, followups/README.md, p2coset/README.md, src/main.rs, Cargo.toml, Cargo.lock, p2-run-01.driver.sh, /usr/bin/msolve (c2722288...59fd4).
- p2coset binary sha256 recomputed: 47c827e30867799fff7c91f7b54e04637419ffb0b09a399aed2f969ec979c9ec (equals sha256-p2coset.stdout and the executor text). build.stderr shows "Finished release" in 7.22 s.
- Output digests recomputed with sha256sum and equal to the receipt outputs and the driver STAGE lines:
  m1.jsonl 2339b349...0484; m2.jsonl 87c6c611...8b4a; m3.jsonl f37f55ad...67f9; m4.jsonl 580fa2c5...0d27.
- Row counts by key (kind, exit) in the jsonl files: m1 48 call, m2 108 call, m4 24 call, m3 288 call + 144 msolve (432 rows). Every row has exit 0; ids unique in each file. STAGE lines: m1 calls=48, m2 calls=108, m4 calls=24, m3 calls=288 cells=144 cells_not_emitted=0 msolve_nonzero=0 msolve_timeout=0; nonzero_exit_calls=0 everywhere. All equal the executor's figures.
- All driver stderr files and sha256 stderr files are empty (receipt stderr_sha256 = empty-string digest).
- Run directory contains exactly: build/driver/sha256 stdout and stderr files, m1..m4 jsonl, and m1..m4 subdirectories (m1 96, m2 216, m3 1008, m4 48 files). Nothing is newer than the receipt inside it. The run directory did not exist before (must_not_exist), and no file within it was overwritten (write-once tool; unique ids).
- Outside the run directory, p2 tree: only cards/p2-run-01.receipt.json was added after the card (in write_scope). PROTOCOL.md hash is unchanged (equals the bind). smoke/ is empty and dated 21:33, before the run. The p2coset source tree has no target directory (CARGO_TARGET_DIR was in the scratchpad).

## Caveats (for the main session, not result interpretation)
1. Relay: I received only the executor's verbatim excerpt (saved at /tmp/claude-0/-home-user/0af71fd6-cb66-5cbf-9292-d617eb53cca4/scratchpad/p2-report.txt), not the full returned text, so a full-text verbatim check was not possible. `task_card.py check-relay` could not be used: it re-lints the card first and exits CARD FAIL (caveat 2). The excerpt's values match the receipt and the stdout files. The BENCHREC=... prefix is a harness addition outside the card argv; I could not see it in the receipt (argv entries are the card's).
2. Card lint now fails: `python3 tools/task_card.py lint <card>` prints CARD FAIL. Five lines (commands[1], [3], [4], [5], [6]) say the argv names repository file /tmp/claude-0/.../targets/p2run/release/p2coset that binds does not pin. The binary is built by the card's own first command and its hash is recorded after the build (sha256-p2coset) rather than pinned in binds. The tool ran the card and the receipt says completed, so this is a card-authoring defect, not an execution deviation; the post-build hash 47c827e3... is the recorded identity. The director/main session should decide whether to record this as a lint exception.
3. Files outside the p2 write_scope that changed during the run: benchmarks/records/20261107T2144Z-split-prime-p2-twist-coset.json (already dirty before the run per receipt dirty_before; 830 bytes, mtime 21:44; the harness benchrec record, not written by a card command), and records and run files of the sibling cards p1_rr_dickson (203 entries, run-20261107T214621Z, started after) and p3_cross_trace (18 entries) and their benchmarks/records files. These belong to other concurrently running cards, not to p2 argv. The repository is not a git checkout here, so `git status --porcelain` was replaced by the receipt's dirty_before/dirty_after lists. p2 run ran concurrently with them on the same host; the isolation status (L0, counted metrics only per the card route) is the card's own statement.
4. The seeds table in the card was not re-derived; the executor did not compute anything.

Classification: executed_as_specified (all 7 steps, every expected call count, all hashes match, nothing written in the p2 tree outside write_scope), with the card-lint and relay-text caveats above.
