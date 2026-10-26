#!/usr/bin/env bash
# Operational regression test; no research target or network upload executes.
set -euo pipefail
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
test_root=$(mktemp -d "${TMPDIR:-/tmp}/nist-checkpoint-test.XXXXXX")
git init -q --initial-branch=main "$test_root/repo"
git -C "$test_root/repo" config user.name checkpoint-test
git -C "$test_root/repo" config user.email checkpoint-test@example.invalid
git -C "$test_root/repo" remote add origin "$test_root/origin.git"
printf 'fixture\n' > "$test_root/repo/README.md"
git -C "$test_root/repo" add README.md
git -C "$test_root/repo" commit -qm fixture
cd "$test_root/repo"
source <(sed '/^case "${1:-}" in/,$d' "$repo/scripts/checkpoint.sh")
printf a > untracked.txt
first=$(state_hash)
second=$(state_hash)
[ "$first" = "$second" ]
printf b >> untracked.txt
[ "$first" != "$(state_hash)" ]
transport() { printf 'test: unavailable transport\n' >&2; return 1; }
set +e
(cmd_push) > "$test_root/no-transport.log" 2>&1
status=$?
set -e
[ "$status" -ne 0 ]
grep -q 'cannot upload, so nothing was checkpointed' "$test_root/no-transport.log"
if grep -Eq 'integer expression expected|unbound variable' "$test_root/no-transport.log"; then
    cat "$test_root/no-transport.log" >&2
    exit 1
fi
git() {
    if [ "${1:-}" = bundle ]; then return 23; fi
    command git "$@"
}
set +e
(cmd_push) > "$test_root/bundle-failure.log" 2>&1
status=$?
set -e
[ "$status" -ne 0 ]
grep -q 'failed to bundle unpushed commits; nothing was checkpointed' "$test_root/bundle-failure.log"
if grep -q 'test: unavailable transport' "$test_root/bundle-failure.log"; then
    echo 'bundle failure reached upload selection' >&2
    exit 1
fi
printf 'PASS: checkpoint state is stable and detects untracked-file changes.\n'
printf 'PASS: Linux file sizes permit packaging; missing transport still rejects upload.\n'
printf 'PASS: failed Git bundle stops before upload selection.\n'
printf 'Fixture logs: %s\n' "$test_root"
