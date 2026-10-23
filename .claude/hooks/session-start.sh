#!/bin/bash
# SessionStart hook for Claude Code on the web. It installs Conductor and cairn
# and brings up a Conductor control plane, so the Conductor integration #1422
# committed (.mcp.json, the pre-edit hooks, the CLAUDE.md block) and the cairn
# MCP server in .mcp.json work in every cloud session. docs/CONDUCTOR.md
# explains each step.
#
# It then runs scripts/setup-workstation.sh for the rest of the toolchain
# (system libraries, PARI/GP, the codex and opencode CLIs, the cryptanalysis
# checkout, per-repo virtualenvs) and starts SageMath's install detached.
#
# It starts one cairn node (scripts/cairn-node.sh: `cairn run --no-mcp`, which
# syncs the log with the seed nodes and verifies claims) and no autonomous agent:
# no `conductor worker`, no `cairn agent run`.
#
# Idempotent: binaries are installed only when missing, so a container restored
# from the cached image goes straight to starting the control plane.
set -euo pipefail

case "${WORKSPACE_OFFLINE:-0}" in
  0|'') ;;
  1)
    offline_repo="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}"
    bash "$offline_repo/scripts/private-workspace.sh" check
    echo "Workspace tools: offline checks passed; start an approved private node with scripts/private-workspace.sh run. No tools were downloaded or services started."
    exit 0
    ;;
  *) echo "WORKSPACE_OFFLINE must be 0 or 1" >&2; exit 2 ;;
esac

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

state="$HOME/.local/state/workspace-tools"
mkdir -p "$state" "$HOME/.cairn"
chmod 700 "$state"
# The hook's stdout becomes session context, so keep it for one summary line and
# send everything else to a private log: `conductor up` prints a login token.
exec 3>&1
exec >>"$state/session-start.log" 2>&1
chmod 600 "$state/session-start.log"
echo "== $(date -u +%FT%TZ) session start"

project=crypto
repo="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}"

# Sparse checkout by default (docs/sparse-checkout.md): the harness profile
# leaves the 18 largest research/ archives off disk. CRYPTO_SPARSE_PROFILE picks
# minimal instead, or full to keep the whole checkout. Applied once, on a full
# checkout only, so a resumed session keeps the paths it added. Never fatal.
sparse_profile="${CRYPTO_SPARSE_PROFILE:-harness}"
if [ "$(git -C "$repo" config --bool core.sparseCheckout 2>/dev/null)" = true ]; then
  sparse_summary="Sparse checkout already applied ($(cd "$repo" && scripts/sparse-checkout.sh status 2>/dev/null | head -n 1))"
elif [ "$sparse_profile" = full ]; then
  sparse_summary="Full checkout (CRYPTO_SPARSE_PROFILE=full)"
elif (cd "$repo" && scripts/sparse-checkout.sh apply --profile "$sparse_profile"); then
  sparse_summary="Sparse profile $sparse_profile applied: some research/ directories are off disk but still in git; scripts/sparse-checkout.sh add --path P/ materializes one"
else
  sparse_summary="Sparse profile $sparse_profile not applied; checkout stays full"
fi

# Conductor, through the Go module proxy. The release installer cannot download
# here (the session's GitHub proxy answers 403), and @latest resolves to v0.1.0,
# whose go.mod declares the wrong module path.
if ! command -v conductor >/dev/null || ! command -v conductor-mcp >/dev/null; then
  GOBIN=/usr/local/bin GOFLAGS=-modcacherw \
    go install "github.com/aburan28/conductor/cmd/...@${CONDUCTOR_VERSION:-main}"
fi

# cairn, built from its public repository for the same reason, with the `ui`
# feature that `cairn run` needs, and a node started that joins the published
# seed nodes and stays up for the session. scripts/cairn-node.sh does both, is
# shared with the sibling repositories, and never fails the hook; `cairn mcp`
# (.mcp.json) runs from the same binary.
"$repo/scripts/cairn-node.sh" ensure || true

# A shared control plane when the environment names one; otherwise one in this
# container on the image's Postgres 16, which coordinates only this container.
if [ -n "${CONDUCTOR_ENDPOINT:-}" ] && [ -n "${CONDUCTOR_TOKEN:-}" ]; then
  plane="the shared control plane at $CONDUCTOR_ENDPOINT"
else
  pgbin=/usr/lib/postgresql/16/bin
  pgdata=/var/lib/conductor-pg
  port=55432
  if [ ! -s "$pgdata/PG_VERSION" ]; then
    install -d -o postgres -g postgres -m 700 "$pgdata"
    runuser -u postgres -- "$pgbin/initdb" -D "$pgdata" -U conductor --auth=trust -E UTF8
  fi
  if ! "$pgbin/pg_isready" -q -h 127.0.0.1 -p "$port"; then
    # A container restored from the cached image keeps the postmaster.pid of a
    # server that no longer runs, and Postgres refuses to start while that pid
    # names any live process. Remove it unless the pid is a live postgres.
    if [ -f "$pgdata/postmaster.pid" ]; then
      pid="$(head -n 1 "$pgdata/postmaster.pid")"
      if [ "$(cat "/proc/$pid/comm" 2>/dev/null)" != postgres ] \
        || grep -q '^State:[[:space:]]*Z' "/proc/$pid/status" 2>/dev/null; then
        rm -f "$pgdata/postmaster.pid"
      fi
    fi
    runuser -u postgres -- "$pgbin/pg_ctl" -D "$pgdata" -l "$pgdata/server.log" -w \
      -o "-p $port -k /tmp -c listen_addresses=127.0.0.1" start
  fi
  if ! "$pgbin/psql" -h 127.0.0.1 -p "$port" -U conductor -d postgres -tAc \
      "select 1 from pg_database where datname = 'conductor'" | grep -q 1; then
    "$pgbin/createdb" -h 127.0.0.1 -p "$port" -U conductor conductor
  fi
  (cd "$repo" && conductor up --project "$project" \
    --dsn "postgres://conductor@127.0.0.1:$port/conductor?sslmode=disable")
  plane="a control plane in this container (http://localhost:8080; it coordinates nothing outside it)"
fi

# The rest of the research toolchain: scripts/setup-workstation.sh, minus the
# parts this hook already did. It skips what is installed, so a container
# restored from the cache only re-checks. Workspace is the directory holding
# this checkout, where attached sibling repositories sit; of the missing ones
# only cryptanalysis (~1.5 GB) is cloned, since the others are attached when a
# session needs them (CRYPTO_SETUP_CLONE overrides). SageMath (~8 GiB, several
# minutes) installs detached, so the session does not wait on it and the hook
# stays inside its timeout; it skips itself below 10 GiB free.
# CRYPTO_WORKSTATION_SETUP=0 turns all of this off. Never fatal.
setup_summary="workstation setup off"
if [ "${CRYPTO_WORKSTATION_SETUP:-1}" != 0 ]; then
  setup="$repo/scripts/setup-workstation.sh"
  workspace="$(dirname "$repo")"
  if CLONE_REPOS="${CRYPTO_SETUP_CLONE-cryptanalysis}" "$setup" --workspace "$workspace" \
      --skip conductor,cairn,control-plane,sage; then
    setup_summary="workstation setup ran"
  else
    setup_summary="workstation setup failed"
  fi
  if [ "${CRYPTO_SETUP_SAGE:-1}" != 0 ] && ! command -v sage >/dev/null \
      && [ ! -x /opt/conda-sage/bin/sage ]; then
    setsid nohup "$setup" --only sage >"$state/sage-install.log" 2>&1 3>&- </dev/null &
    setup_summary="$setup_summary; SageMath install started in the background ($state/sage-install.log)"
  fi
  [ -x /opt/conda-sage/bin/sage ] && [ -n "${CLAUDE_ENV_FILE:-}" ] \
    && echo 'export PATH="/opt/conda-sage/bin:$PATH"' >>"$CLAUDE_ENV_FILE"
fi

conductor_version="$(conductor version 2>/dev/null | head -n 1)"
cairn_version="$(cairn --version 2>/dev/null | head -n 1)"
# `status` exits 1 while the node is down; under pipefail that ended the hook
# here, before the summary line.
cairn_node="$("$repo/scripts/cairn-node.sh" status 2>&1 | head -n 1 || true)"
echo "ready: $conductor_version; $cairn_version; $plane; $cairn_node; $setup_summary"
echo "Workspace tools: $conductor_version and $cairn_version are installed; Conductor is using $plane. $cairn_node. $setup_summary. $sparse_summary. Log: $state/session-start.log" >&3
