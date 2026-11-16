# Launch note: detached launches (2026-11-02)

This file is bound by no card. It exists because P2 and P3 cards bind their
`PROTOCOL.md` hash, so the amendment sections of the protocols could not take
this note without invalidating the pinned cards. It adds no protocol value.

1. Runs are now launched detached, so they outlive the executor tool's
   30-minute limit on `run_in_background` commands:

   ```
   setsid nohup python3 tools/task_card.py run <card> > cards/<card-id>.launch.log 2>&1 < /dev/null &
   ```

   (`<card>` is the card path, `cards/<card-id>.launch.log` is under
   `research/isogeny_class_ic_yield_20261102/`.) The executor then reads the
   receipt when `tools/task_card.py run` has exited.
2. The command, argv, env, card file and binaries are unchanged: the three
   cards below are the same files (same sha256) as before attempt 1, and each
   still lints (`CARD OK`) at its pin.
3. Attempt 1 of the three cards below took no data used in analysis. Each was
   ended by the executor tool's 30-minute limit, or by the main session to
   avoid it, before the command finished. No receipt was written, no declared
   output exists, and the partial outputs are retained unread under
   `_failed/<card>-attempt1/` and are not pooled, analysed or cited.

| card | pin | worktree | ended | partial output (moved aside) |
| --- | --- | --- | --- | --- |
| CARD-isogeny-ic-yield-p1-tier1-20261102-1 | d6153ce46eed4986d63d0a3d6405de7ed1b36101 | /home/user/crypto-p1run | killed by the tool limit at 05:08 (started 04:38) | `cards/_failed/CARD-isogeny-ic-yield-p1-tier1-20261102-1-attempt1/` (`tier1.log`, `tier1_rows.jsonl.gz`) |
| CARD-isogeny-ic-yield-p2-c32A-m3-20261102-3 | b2d8929120ecba7ddb0a057681816e458767e16c | /home/user/crypto-p2run | SIGTERM by the main session at about 6 min (exit 143) | `p2_prime_analogues/runs/_failed/CARD-isogeny-ic-yield-p2-c32A-m3-20261102-3-attempt1/c32A_m3/` |
| CARD-isogeny-ic-yield-p3-a-main-k5-20261102-1 | d97f5953b24e05096208a54df6e699827d021541 | /home/user/crypto-p3run | stopped by the main session at about 5 min | `p3_ecc2k130/full/_failed/CARD-isogeny-ic-yield-p3-a-main-k5-20261102-1-attempt1/p3a_main_k5.jsonl` |

The P1 partial files are in `cards/_failed/` and not in `p1_eccp131/runs/`
because that card's `must_not_exist` is the whole `runs/` directory.
