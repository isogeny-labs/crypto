# Strict replay transport correction

The original unchanged script prints `ALL_PROGRESSION_REPLAY_OK 1841 1841`
and exits zero. Its first receipt is retained under pari-replay-v1. The strict
native admission rejects an additional `elladd` stack-growth warning, beyond
the already allowed maximum-stack notice. This is an unadmitted verification
attempt, not an accepted replay or a failed scientific worker.

Keep the original script bytes, witnesses, counts and stderr allowlist. Rerun
with a 64,000,000-byte initial PARI stack, within the existing four-GiB ceiling,
to avoid dynamic stack-growth diagnostics. The separate native transport records
the actual argv, script/binary/output hashes and exit. Admission uses the same
frozen native verifier. No instruction measurement or mathematical result changes.
