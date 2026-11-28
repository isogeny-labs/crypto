#!/usr/bin/env bash
# Transport-only wrapper control. The fake binaries never call the census code.
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
study=$(cd "$here/.." && pwd)
fixture=$(mktemp -d "${TMPDIR:-/tmp}/norm-branch-runner-test.XXXXXX")
trap 'rm -rf -- "$fixture"' EXIT

cat > "$fixture/fake-produce" <<'FAKE_PRODUCE'
#!/usr/bin/env bash
set -euo pipefail
[[ $# -eq 3 && ! -e "$1" ]]
printf x >> "${FAKE_PRODUCER_COUNT:?}"
cells=$(jq -n '[range(0;40) | {}]')
jq -n --arg source "$2" --arg protocol "$3" --argjson cells "$cells" \
  '{schema:"norm.branch-census/v1",source_revision:$source,
    protocol_sha256:$protocol,cells:$cells}' > "$1"
printf 'cells=40 verdict=INCONCLUSIVE reason=transport-fixture\n'
FAKE_PRODUCE

cat > "$fixture/fake-check" <<'FAKE_CHECK'
#!/usr/bin/env bash
set -euo pipefail
[[ $# -eq 2 && -f "$1" && ! -e "$2" ]]
source_revision=$(jq -er .source_revision "$1")
protocol_sha256=$(jq -er .protocol_sha256 "$1")
jq -n --arg source "$source_revision" --arg protocol "$protocol_sha256" \
  '{schema:"norm.branch-census-check/v1",source_revision:$source,
    protocol_sha256:$protocol,pass:true,errors:[]}' > "$2"
printf 'checks=0 verdict=INCONCLUSIVE reason=transport-fixture\n'
FAKE_CHECK
chmod 755 "$fixture/fake-produce" "$fixture/fake-check"

source_revision=0000000000000000000000000000000000000001
parent_sha=$(sha256sum "$study/protocol.json" | cut -d' ' -f1)
runner_sha=$(sha256sum "$here/protocol.json" | cut -d' ' -f1)
producer_sha=$(sha256sum "$fixture/fake-produce" | cut -d' ' -f1)
checker_sha=$(sha256sum "$fixture/fake-check" | cut -d' ' -f1)
jq -n \
  --arg source "$source_revision" \
  --arg parent "$parent_sha" \
  --arg runner "$runner_sha" \
  --arg producer "$producer_sha" \
  --arg checker "$checker_sha" \
  '{schema:"norm.branch-census-build/v1",source_revision:$source,
    parent_protocol_sha256:$parent,runner_protocol_sha256:$runner,
    producer_sha256:$producer,checker_sha256:$checker}' \
  > "$fixture/build-info.json"

export FAKE_PRODUCER_COUNT="$fixture/producer-count"
"$here/run.sh" \
  "$fixture/output" \
  "$fixture/fake-produce" \
  "$fixture/fake-check" \
  "$source_revision" \
  "$study/protocol.json" \
  "$study/PROTOCOL.sha256" \
  "$here/protocol.json" \
  "$here/PROTOCOL.sha256" \
  "$fixture/build-info.json"

(cd "$fixture/output" && sha256sum -c SHA256SUMS)
jq -e '.status=="verified_complete" and .verdict=="INCONCLUSIVE"' \
  "$fixture/output/execution.json" >/dev/null
jq -e '.exit_status==0' "$fixture/output/job-exit.json" >/dev/null
[[ "$(wc -c < "$FAKE_PRODUCER_COUNT")" -eq 1 ]]

if "$here/run.sh" \
  "$fixture/output" \
  "$fixture/fake-produce" \
  "$fixture/fake-check" \
  "$source_revision" \
  "$study/protocol.json" \
  "$study/PROTOCOL.sha256" \
  "$here/protocol.json" \
  "$here/PROTOCOL.sha256" \
  "$fixture/build-info.json" >/dev/null 2>&1; then
  echo 'runner unexpectedly accepted an existing output directory' >&2
  exit 1
fi
[[ "$(wc -c < "$FAKE_PRODUCER_COUNT")" -eq 1 ]]
