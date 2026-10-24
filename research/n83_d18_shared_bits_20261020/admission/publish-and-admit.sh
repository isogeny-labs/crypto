#!/bin/bash
# Publish only the actual host-card metadata, then admit the unchanged frozen card.
# Run detached. Credentials are arguments, never copied into the evidence directory.
set -euo pipefail
test "$#" = 6
repo=$(realpath "$1")
ops=$(realpath "$2")
key=$3 destination=$4 expected_head=$5 auditor=$(realpath "$6")
remote=/home/ec2-user/n83-shared-bits-20261024
status="$ops/publication-status.txt"
[[ "$expected_head" =~ ^[0-9a-f]{40}$ ]]
finish() {
    local code=$?
    if [ "$code" -ne 0 ]; then printf 'FAILED exit=%s; no further admission attempted\n' "$code" > "$status"; fi
}
trap finish EXIT
ssh_args=(-i "$key" -o ProxyCommand=none -o StrictHostKeyChecking=yes -o BatchMode=yes -o ConnectTimeout=10)
printf '%s\n' WAITING_FOR_PREPARED_HOST_CARD > "$status"
ssh "${ssh_args[@]}" "$destination" \
    'test -x /home/ec2-user/n83-shared-bits-20261024/wait-status || timeout 14400 cat /home/ec2-user/n83-shared-bits-20261024/notification-ready.fifo > /dev/null; exec /home/ec2-user/n83-shared-bits-20261024/wait-status /home/ec2-user/n83-shared-bits-20261024/continuation-status-v4.txt 14400' \
    > "$ops/readiness.log" 2>&1
test "$(cat "$ops/readiness.log")" = AWAITING_COMMITTED_HOST_CARD
printf '%s\n' INDEPENDENTLY_CHECKING_ACTUAL_CARD > "$status"
test ! -e "$ops/mirror-card"
scp "${ssh_args[@]}" -r "$destination:$remote/card-v1" "$ops/mirror-card" > "$ops/card-copy.log" 2>&1
"$auditor" freeze "$ops/mirror-card" "$repo" "$ops/local-frozen-card.json" > "$ops/local-freeze.log" 2>&1
jq -S 'del(.remote_root)' "$ops/mirror-card/host-card.json" > "$ops/remote-normalized.json"
jq -S 'del(.remote_root)' "$ops/local-frozen-card.json" > "$ops/local-normalized.json"
cmp "$ops/remote-normalized.json" "$ops/local-normalized.json"
jq -e '.schema=="n83.d18-shared-host-card/v1"
    and .source_commit=="4e1d311d87571aa3309ed4bfac51d06539ef2297"
    and .remote_root=="/home/ec2-user/n83-shared-bits-20261024/card-v1"
    and .controls==22 and .control_replays==22 and .natural_queries==18
    and .candidate_id==null and .selected_best_total_runtime==null' \
    "$ops/mirror-card/host-card.json" > "$ops/card-contract.log"
card_hash=$(shasum -a 256 "$ops/mirror-card/host-card.json" | cut -d ' ' -f 1)
[[ "$card_hash" =~ ^[0-9a-f]{64}$ ]]
cd "$repo"
test "$(git branch --show-current)" = codex/n83-d18-shared-bits-20261024
test "$(git rev-parse HEAD)" = "$expected_head"
test -z "$(git status --porcelain)"
# The full native validation at 5e2dc2583 remains applicable only to unchanged code.
git diff --exit-code 5e2dc2583 HEAD -- src examples Cargo.toml Cargo.lock > "$ops/native-source-check.log"
study=research/n83_d18_shared_bits_20261020/cards/20261024-v1
test ! -e "$study"
mkdir -p "$study"
cp "$ops/mirror-card/host-card.json" "$study/host-card.json"
printf '%s  host-card.json\n' "$card_hash" > "$study/host-card.sha256"
cat > "$study/ADMISSION.md" <<'MD'
# Frozen actual-host card, 2026-10-24

This preexecution record binds the full D18 shared-bit comparison to its
actual Linux source, executables, inputs, construction container and passed
native/proof prerequisites. It does not report executed controls or queries.

The worker source is `4e1d311d87571aa3309ed4bfac51d06539ef2297`. The updated
local auditor from `5e2dc2583` independently re-froze a byte-for-byte copy of
the prepared directory, including the Linux executables. Its JSON equals the
remote frozen card after removing only `remote_root`, whose difference records
the two hosts' physical directories. The local auditor additionally checks
the raw construction resource/termination policy and native/proof exits.

The source, inputs and limits are those of [the frozen protocol](../../PROTOCOL.md):
22 constructed controls, 22 fresh independent replays and 18 natural queries.
The Linux prerequisites and construction receipts are preserved in the
prepared card and will accompany the collected run archive. Full local
validation is preserved in [validation v4](../../reports/run-20261024T0640Z/RUN_REPORT.md).

The admission driver publishes this host card, verifies that the remote PR
branch names its commit, and only then sends the commit and card digest to
the detached runner. Failed validation, a changed checkout, failed publication,
a mismatching digest or expired admission prevents launch. The shared benchmark
lock is released during publication and reacquired before execution. Completed
or failed runs are collected automatically; independent complete-card audit,
result reporting and PR merge remain separate gates.

The source baseline, fixed corpus and source point set remain unchanged.
Timing and candidate claims remain null. This metadata adds no timing result;
canonical runtime graphs and benchmark rows are unchanged.
MD
printf '\nHost-card SHA-256: %s.\n' "$card_hash" >> "$study/ADMISSION.md"
git diff --check
git add "$study/host-card.json" "$study/host-card.sha256" "$study/ADMISSION.md"
test "$(git diff --cached --name-only | wc -l | tr -d ' ')" = 3
git commit -m 'chore(n83): freeze the actual Linux shared-bit host card' > "$ops/card-commit.log" 2>&1
commit=$(git rev-parse HEAD)
git push origin HEAD:codex/n83-d18-shared-bits-20261024 > "$ops/card-push.log" 2>&1
remote_head=$(git ls-remote origin refs/heads/codex/n83-d18-shared-bits-20261024 | cut -f 1)
test "$remote_head" = "$commit"
printf '%s %s\n' "$commit" "$card_hash" > "$ops/published-card.txt"
printf '%s\n' PUBLISHED_WAITING_FOR_RUNNER_ADMISSION > "$status"
printf '%s %s\n' "$commit" "$card_hash" | ssh "${ssh_args[@]}" "$destination" \
    'test "$(cat /home/ec2-user/n83-shared-bits-20261024/continuation-status-v4.txt)" = AWAITING_COMMITTED_HOST_CARD && timeout 10 sh -c "cat > /home/ec2-user/n83-shared-bits-20261024/commit-admission-v4.fifo"' \
    > "$ops/admission.log" 2>&1
printf '%s\n' HOST_CARD_PUBLISHED_AND_ADMITTED > "$status"
