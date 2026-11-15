# Step 0: the widened k-sum index is a no-op for k <= 5 (measured, 2026-11-15, `[pcc:unverified]`)

Command (`replay.sh`, which wraps the documented `ecbench verify --dir D --replay-all --exit-code --out D.json`), run once with
the binary built before the change and once after, on the same four committed sessions (records fetched from the artifact store by
sha256 into a scratch copy; the checkout is not hydrated):

```
step0/replay.sh <ecbench_before> before <scratch>      # binary sha256 7c1e270fb1363d0cc113760dce222c0a66886f2555a5a39d40a33e04efadbfd5
step0/replay.sh <ecbench_after>  after  <scratch>      # binary sha256 93bb9e7f7a1d6cb671ba711d47e6575105101d0004b12b0e09aa84a239132b72
```

Both are `cargo build --release --bin ecbench` (rustc/cargo 1.97.0) of the working tree; "before" is HEAD 2cbc70dd50 with no
change to `src`, "after" is HEAD plus the k-sum diff (the other agent's uncommitted edit is an `examples/` file, not in either binary).

```
before ic_tableb_ksum_20261115-P20-r1 rc=0 audit OK: 1536 records, 1536 verified, 1152 replays reproduced, 0 problems
before ic_tableb_ksum_20261115-P24-r1 rc=0 audit OK: 1536 records, 1536 verified, 1152 replays reproduced, 0 problems
before prime_ksum_k5_20261028-P32 rc=0 audit OK: 240 records, 240 verified, 160 replays reproduced, 0 problems
before prime_ksum_k5_20261028-P36 rc=0 audit OK: 240 records, 240 verified, 160 replays reproduced, 0 problems
after  ic_tableb_ksum_20261115-P20-r1 rc=0 audit OK: 1536 records, 1536 verified, 1152 replays reproduced, 0 problems
after  ic_tableb_ksum_20261115-P24-r1 rc=0 audit OK: 1536 records, 1536 verified, 1152 replays reproduced, 0 problems
after  prime_ksum_k5_20261028-P32 rc=0 audit OK: 240 records, 240 verified, 160 replays reproduced, 0 problems
after  prime_ksum_k5_20261028-P36 rc=0 audit OK: 240 records, 240 verified, 160 replays reproduced, 0 problems
```

Every replay detail is `identical` (1152 of 1152 in each ic_tableb session), the replayed records include the ksum4 and ksum5 arms
(144 + 144 per ic_tableb session, 32 + 32 in the k5 P32 session), and `diff-audits.sh` finds the before and after audit JSONs
identical except for `audited_unix_ms` and `auditor_binary_sha256`:

```
ic_tableb_ksum_20261115-P20-r1.json: 1536 records, 1536 verified, 1152/1152 replays reproduced, problems=0; diff lines (excluding time and auditor hash) = 0
ic_tableb_ksum_20261115-P24-r1.json: 1536 records, 1536 verified, 1152/1152 replays reproduced, problems=0; diff lines (excluding time and auditor hash) = 0
prime_ksum_k5_20261028-P32.json: 240 records, 240 verified, 160/160 replays reproduced, problems=0; diff lines (excluding time and auditor hash) = 0
prime_ksum_k5_20261028-P36.json: 240 records, 240 verified, 160/160 replays reproduced, problems=0; diff lines (excluding time and auditor hash) = 0
```

Why this is expected: for k <= 5 `ksum_index_bits(k)` is 12, so the packed entry, insert order and keep-first rule are
unchanged; the only other differences are error messages and a table-size guard (2^31 entries) that no committed session reaches.
Raw audits: `before/`, `after/`.
