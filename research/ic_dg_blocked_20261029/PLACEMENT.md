# XP-20261029-dg-blocked: leaderboard placement (evidence review record)

Decision (main session, acting for the Coordinator in this lane; 2026-10-29): the validated 28-bit rows (`icv1-fp28-t9805-a382b8a8`, `icv1-fp28-tm3463-6d2f5699`) go to a new section `dg_blocked_component`, "Component speed-up: a cache-blocked packed pair table in wall time", outside tables A–C, built by `scripts/build_ic_leaderboard.py`.

Why: PROTOCOL §10 places the row in `pair_table_component` or, if that section holds set-up figures only, in a new oracle-named component section. `pair_table_component` holds set-up figures only (`research/ic_oracle_setup_blocked_20261020`), and this result is whole-solve wall time per charged GAE, so it gets its own section. The validator (VAL-20261029-33a286) agrees with this branch.

24 and 32 bits are undecidable and get no row. No row of tables A–C and no baseline changes: charged work is identical record by record.
