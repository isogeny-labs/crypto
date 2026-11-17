#!/usr/bin/env bash
# Infrastructure contract checks: no real credentials, installs or lab jobs.
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
fixture=$(mktemp -d)
trap 'rm -rf "$fixture"' EXIT
mkdir -p "$fixture/bin" "$fixture/client with spaces"
export ISOLAB_AGENT_STATE_DIR="$fixture/state"
export ISOLAB_TEST_TRACE="$fixture/trace"
export ISOLAB_TEST_MODE=ok
unset ISOLAB_URL ISOLAB_TOKEN ISOLAB_CREDS ISOLAB_BIN ISOLAB_MCP_BIN KUBECONFIG ISOLAB_KUBE_CONTEXT
export PATH="$fixture/bin:$PATH"
launcher="$repo/scripts/isolab-agent.sh"

cat > "$fixture/client with spaces/client" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
: > "$ISOLAB_TEST_TRACE"
if [[ $# -gt 0 ]]; then printf '%s\n' "$@" > "$ISOLAB_TEST_TRACE"; fi
if [[ "$ISOLAB_TEST_MODE" == fail ]]; then
  printf 'sensitive fake URL: %s\n' "${ISOLAB_TOKEN:-}" >&2
  exit 19
fi
if [[ "${1:-}" == mcp ]]; then
  [[ "$ISOLAB_MCP_TRANSPORT" == stdio ]]
  [[ "$ISOLAB_TOKEN" == fixture-secret-never-print ]]
  printf '{"jsonrpc":"2.0","id":1,"result":{}}\n'
elif [[ $# == 0 ]]; then
  [[ "$ISOLAB_KUBE_CONTEXT" == fixture-context ]]
  printf '{"jsonrpc":"2.0","id":1,"result":{}}\n'
else
  printf '{}\n'
fi
STUB
chmod +x "$fixture/client with spaces/client"
export ISOLAB_BIN="$fixture/client with spaces/client"
export ISOLAB_MCP_BIN="$ISOLAB_BIN"
for tool in aws kubectl; do ln -s /bin/true "$fixture/bin/$tool"; done
cat > "$fixture/bin/uv" <<'STUB'
#!/usr/bin/env bash
echo 'unexpected install' >> "$ISOLAB_TEST_TRACE"
exit 91
STUB
chmod +x "$fixture/bin/uv"

expect_failure() {
  if "$@" > "$fixture/out" 2> "$fixture/err"; then
    echo 'unexpected success' >&2; exit 1
  fi
}
expect_failure bash "$launcher" check invalid
expect_failure bash "$launcher" mcp nats
grep -q ISOLAB_URL "$fixture/err"
[[ ! -e "$ISOLAB_TEST_TRACE" ]]
export ISOLAB_URL=nats://fixture.invalid:4222
export ISOLAB_TOKEN=fixture-secret-never-print
export ISOLAB_MCP_TRANSPORT=streamable-http
bash "$launcher" mcp nats > "$fixture/out"
jq -e '.jsonrpc == "2.0"' "$fixture/out" >/dev/null
[[ $(cat "$ISOLAB_TEST_TRACE") == mcp ]]
bash "$launcher" check nats > "$fixture/out"
[[ $(cat "$ISOLAB_TEST_TRACE") == workers ]]
grep -q 'no job submitted' "$fixture/out"
expect_failure env ISOLAB_CREDS="$fixture/missing" bash "$launcher" mcp nats
expect_failure env ISOLAB_BIN="$fixture/missing" bash "$launcher" mcp nats
expect_failure env ISOLAB_TEST_MODE=fail bash "$launcher" check nats
if grep -q fixture-secret-never-print "$fixture/out" "$fixture/err"; then
  echo 'probe leaked its credential' >&2; exit 1
fi
expect_failure bash "$launcher" mcp eks
export KUBECONFIG="$fixture/kubeconfig"
printf '{}\n' > "$KUBECONFIG"
expect_failure bash "$launcher" mcp eks
grep -q ISOLAB_KUBE_CONTEXT "$fixture/err"
export ISOLAB_KUBE_CONTEXT=fixture-context
bash "$launcher" mcp eks > "$fixture/out"
jq -e '.jsonrpc == "2.0"' "$fixture/out" >/dev/null
[[ ! -s "$ISOLAB_TEST_TRACE" ]]
bash "$launcher" check eks > "$fixture/out"
printf 'call\nisolab_k8s_overview\n{}\n' > "$fixture/expected"
cmp "$fixture/expected" "$ISOLAB_TEST_TRACE"
expect_failure env ISOLAB_TEST_MODE=fail bash "$launcher" check eks
if grep -q fixture-secret-never-print "$fixture/out" "$fixture/err"; then
  echo 'probe leaked its credential' >&2; exit 1
fi
rm "$ISOLAB_TEST_TRACE"
expect_failure env WORKSPACE_OFFLINE=1 bash "$launcher" setup nats
expect_failure env WORKSPACE_OFFLINE=1 bash "$launcher" setup eks
[[ ! -e "$ISOLAB_TEST_TRACE" && ! -e "$ISOLAB_AGENT_STATE_DIR" ]]
echo 'Iso Lab agent contracts passed: explicit backends, preserved environment, stdio-only startup, bounded read-only probes, redacted errors and offline setup.'
