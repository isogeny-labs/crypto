# P0: recover the newest genus_ladder instrument

Binary: `genus_ladder` built from commit `fffa1b1243` (sparse worktree), sha256
`128cd69bb85005c621018c1080046e97ca6be7cd204d2b7102e5bf5a79463364`.

Command:

    genus_ladder run --primes 2053 --genera 4 --curves 1 --seeds 1 --arms c --cc 2.0 \
      --qc 3.0 --max-h 2 --max-trials 50000000 --seed-search 600 --threads 1 \
      --out p0_g4_p2053.jsonl

Gate (count identity): the cell curve_seed 40, seed 1 gives trials 31009,
cantor_mul 12844280, cantor_inv 227572, rel_mul 6914274, la_mul 4112038,
certified true. That matches field for field the recorded row in
`fffa1b1243:research/genus4_positive_control_20261116/validation/out/replay_s1_2053_4099_prodbin.jsonl`.
**Gate passed.** Standing: `[pcc:unverified]` (same-container reproduction).
