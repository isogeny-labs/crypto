#!/usr/bin/env bash
# Setup script for cloud agent sessions: Claude Code on the web and Codex cloud.
# Paste one line into the environment's setup-script field:
#
#   Claude Code (starts one directory above the repositories):
#     bash crypto/scripts/cloud-setup.sh
#   Codex (starts inside /workspace/<repo>):
#     bash scripts/cloud-setup.sh
#
#   bash crypto/scripts/cloud-setup.sh --dry-run   # print, change nothing
#
# docs/cloud-setup.md explains each step and where to set each credential.
#
# What it does, in order, each step warning instead of failing:
#   1. disk     apply the harness sparse profile to the crypto and
#               crypto-autoresearcher checkouts: their research and run
#               archives stay in git but off disk (about 11 GB), so SageMath fits
#   2. tools    scripts/setup-workstation.sh: apt toolchain, Rust, Go, Node,
#      + repos  uv, Conductor, cairn, codex/opencode/gh, aws, kubectl, the lean
#               SageMath build (sagelib + maxima) when the disk allows, and the
#               missing sibling repositories as partial clones (blobs over
#               CLONE_BLOB_LIMIT stay on the server) with their virtualenvs
#   3. creds    a git credential helper that reads GitHub credentials from the
#               session's environment when git needs them (gh, aws and the
#               agent CLIs read their variables themselves); print which
#               credential variables setup can see (names only, never values)
#   4. cleanup  drop package caches so the session keeps its disk
#
# Credentials are never written by this script, with one opt-in exception for
# Codex (PERSIST_SECRETS below). Set them in the environment's settings:
#   Claude Code: environment variables or Network secrets. The setup script
#     cannot read them; the session can, so everything here reads them lazily.
#   Codex: environment variables last the whole task; secrets exist only while
#     this script runs.
#
# Credential variables it recognizes (all optional):
#   GitHub   GH_TOKEN | GH_SERVICE_TOKEN | GH_APP_ID + GH_APP_INSTALLATION_ID +
#            GH_APP_PRIVATE_KEY  (scripts/setup-workstation.sh, `github`)
#   AWS      AWS_ACCESS_KEY_ID + AWS_SECRET_ACCESS_KEY [+ AWS_SESSION_TOKEN],
#            AWS_REGION; CHECKPOINT_BUCKET for scripts/checkpoint.sh
#   Iso Lab  ISOLAB_URL | KUBECONFIG + ISOLAB_KUBE_CONTEXT
#   Conductor CONDUCTOR_ENDPOINT + CONDUCTOR_TOKEN
#   Agents   ANTHROPIC_API_KEY, OPENAI_API_KEY
#   Compute  RUNPOD_API_KEY, MODAL_TOKEN_ID + MODAL_TOKEN_SECRET, HF_TOKEN
#
# Overrides:
#   CLOUD_RUNTIME     claude | codex | local      (detected)
#   WORKSPACE         directory holding the repositories (parent of this checkout)
#   GITHUB_OWNER      owner of the repositories                 (isogeny-labs)
#   CLONE_REPOS       siblings to clone when missing
#                     (crypto cryptanalysis crypto-autoresearcher cryptanalysis-rl)
#   CLONE_BLOB_LIMIT  partial-clone blob limit                  (1m)
#   CRYPTO_SPARSE_PROFILE     crypto: harness | minimal | full  (harness)
#   CRYPTO_AR_SPARSE_PROFILE  crypto-autoresearcher: harness | full  (harness)
#   SETUP_SKIP        extra setup-workstation components to skip, comma-separated
#   WITH_SAGE=0       skip SageMath;  SAGE_MIN_FREE_GB (4) gates the lean build
#   WITH_LEAN=1       also install elan for cryptanalysis's Lean proofs
#   SETUP_CLEANUP=0   keep the package caches
#   PERSIST_SECRETS=1 Codex only: copy the recognized credential variables
#                     present during setup into ~/.config/cloud-setup/env
#                     (mode 600), sourced by login shells. Off by default:
#                     Codex removes secrets before the agent runs on purpose.
set -uo pipefail   # no -e: a failed step warns and the next one runs

DRY_RUN=0
case "${1:-}" in
  --dry-run) DRY_RUN=1 ;;
  -h|--help) sed -n '2,/^set -uo/p' "$0" | sed '$d; s/^# \{0,1\}//'; exit 0 ;;
  '') ;;
  *) echo "unknown option: $1 (try --help)" >&2; exit 2 ;;
esac

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(dirname "$here")"
export WORKSPACE="${WORKSPACE:-$(dirname "$repo")}"
export GITHUB_OWNER="${GITHUB_OWNER:-isogeny-labs}"
export CLONE_REPOS="${CLONE_REPOS-crypto cryptanalysis crypto-autoresearcher cryptanalysis-rl}"

if [ -z "${CLOUD_RUNTIME:-}" ]; then
  if [ "${CLAUDE_CODE_REMOTE:-}" = true ]; then CLOUD_RUNTIME=claude
  elif [ -d /workspace ] && [ "${WORKSPACE#/workspace}" != "$WORKSPACE" ]; then CLOUD_RUNTIME=codex
  else CLOUD_RUNTIME=local
  fi
fi

WARNINGS=()
log()  { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m !!\033[0m %s\n' "$*" >&2; WARNINGS+=("$*"); }
have() { command -v "$1" >/dev/null 2>&1; }
free_mib() { df -Pm "$1" 2>/dev/null | awk 'NR == 2 {print $4}'; }

log "cloud setup: runtime $CLOUD_RUNTIME, workspace $WORKSPACE, $(free_mib "$WORKSPACE") MiB free"

# ------------------------------------------------------------------ 1. disk ---
# Each repository's own sparse profile, applied once on a full checkout. The
# excluded paths stay in git (skip-worktree) and come back with the tool's
# `add`. Together the two harness profiles free about 11 GB, which is what
# leaves room for SageMath next to four attached checkouts.
sparse() {
  local name="$1" profile="$2" tool="$3" dir="$WORKSPACE/$1" before
  shift 3
  [ "$profile" = full ] && return 0
  [ -f "$dir/$tool" ] || return 0
  if [ "$(git -C "$dir" config --bool core.sparseCheckout 2>/dev/null)" = true ]; then
    log "$name: sparse checkout already applied"; return 0
  fi
  if [ "$DRY_RUN" = 1 ]; then log "$name: would apply the $profile sparse profile"; return 0; fi
  log "$name: sparse checkout ($profile profile)"
  before="$(free_mib "$WORKSPACE")"
  if (cd "$dir" && "$@" >/dev/null); then
    log "$name: freed $(( $(free_mib "$WORKSPACE") - before )) MiB ($tool add --path P/ brings a path back)"
  else
    warn "$name: sparse profile $profile not applied; the checkout stays full"
  fi
}
crypto_profile="${CRYPTO_SPARSE_PROFILE:-harness}"
ar_profile="${CRYPTO_AR_SPARSE_PROFILE:-harness}"
sparse crypto "$crypto_profile" scripts/sparse-checkout.sh \
  bash scripts/sparse-checkout.sh apply --profile "$crypto_profile"
# crypto-autoresearcher's ledger sweeps (validate_ledger, goal_portfolio_health,
# portfolio_kpis) refuse a sparse checkout; coordination sessions set
# CRYPTO_AR_SPARSE_PROFILE=full on the setup line, or run
# `python3 tools/sparse_checkout.py disable` there.
sparse crypto-autoresearcher "$ar_profile" tools/sparse_checkout.py \
  python3 tools/sparse_checkout.py apply "$ar_profile"

# ------------------------------------------------------- 2. tools + repos ---
# The Conductor control plane and the cairn node are processes; the
# SessionStart hook starts them in each session, so they are skipped here.
skip="control-plane${SETUP_SKIP:+,$SETUP_SKIP}"
[ "${WITH_SAGE:-1}" = 0 ] && skip="$skip,sage"
ws_args=(--skip "$skip")
[ "$DRY_RUN" = 1 ] && ws_args+=(--dry-run)
SAGE_LEAN=1 GITHUB_LAZY=1 SAGE_MIN_FREE_GB="${SAGE_MIN_FREE_GB:-4}" \
  bash "$here/setup-workstation.sh" "${ws_args[@]}" ||
  warn "setup-workstation.sh reported a failure; its warnings are above"

if [ "$DRY_RUN" = 1 ]; then
  [ "${WITH_LEAN:-0}" = 1 ] && log "would install elan"
elif [ "${WITH_LEAN:-0}" = 1 ] && ! have elan; then
  log "Lean (elan; the toolchain in lean-toolchain downloads on first use)"
  curl -fsSL https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh |
    sh -s -- -y --default-toolchain none || warn "elan not installed"
fi

# ----------------------------------------------------------------- 3. creds ---
# The credential variables this script knows about. git, gh, aws and the
# agent CLIs read them from the session's environment when they run.
cred_vars=(GH_TOKEN GITHUB_TOKEN GH_SERVICE_TOKEN GH_APP_ID GH_APP_INSTALLATION_ID GH_APP_PRIVATE_KEY
  AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY AWS_SESSION_TOKEN AWS_REGION AWS_DEFAULT_REGION
  CHECKPOINT_BUCKET ISOLAB_URL KUBECONFIG ISOLAB_KUBE_CONTEXT CONDUCTOR_ENDPOINT CONDUCTOR_TOKEN
  ANTHROPIC_API_KEY OPENAI_API_KEY RUNPOD_API_KEY MODAL_TOKEN_ID MODAL_TOKEN_SECRET HF_TOKEN)

# Login shells and the agents' shells source this file. It holds PATH and,
# only with PERSIST_SECRETS=1 on Codex, the credential variables.
envfile="$HOME/.config/cloud-setup/env"
if [ "$DRY_RUN" = 1 ]; then
  log "would write $envfile (PATH) and source it from ~/.bashrc and ~/.profile"
else
  mkdir -p "$(dirname "$envfile")" && chmod 700 "$(dirname "$envfile")"
  (
    umask 077
    {
      echo '# written by scripts/cloud-setup.sh'
      echo 'export PATH="$HOME/.local/bin:$HOME/.cargo/bin:/usr/local/go/bin:$HOME/go/bin:$HOME/.elan/bin:/opt/conda-sage/bin:$PATH"'
    } >"$envfile"
    if [ "${PERSIST_SECRETS:-0}" = 1 ] && [ "$CLOUD_RUNTIME" = codex ]; then
      n=0
      for v in "${cred_vars[@]}"; do
        [ -n "${!v:-}" ] || continue
        printf 'export %s=%q\n' "$v" "${!v}" >>"$envfile"; n=$((n + 1))
      done
      log "persisted $n credential variable(s) to $envfile (mode 600)"
    fi
  )
  [ "${PERSIST_SECRETS:-0}" = 1 ] && [ "$CLOUD_RUNTIME" != codex ] &&
    warn "PERSIST_SECRETS=1 applies to Codex only; nothing persisted on $CLOUD_RUNTIME"
  line="[ -f \"$envfile\" ] && . \"$envfile\""
  for rc in "$HOME/.bashrc" "$HOME/.profile"; do
    touch "$rc"; grep -qxF "$line" "$rc" || printf '%s\n' "$line" >>"$rc"
  done
fi

# Names only. On Claude Code the setup script cannot see the environment's
# variables, so an empty list here says nothing about the session.
present=()
for v in "${cred_vars[@]}"; do [ -n "${!v:-}" ] && present+=("$v"); done
log "credential variables visible to setup: ${present[*]:-none}"
[ "$CLOUD_RUNTIME" = claude ] &&
  log "Claude Code passes environment variables to the session, not to this script; git, gh and aws read them when used"

# --------------------------------------------------------------- 4. cleanup ---
[ "$DRY_RUN" = 1 ] && { log "would clean the apt, uv, go, npm, micromamba and cargo caches"; exit 0; }
[ "${SETUP_CLEANUP:-1}" = 0 ] && exit 0
log "cleanup"
before="$(free_mib "$WORKSPACE")"
have apt-get && { apt-get clean 2>/dev/null || true; }
have uv && uv cache clean >/dev/null 2>&1
have go && go clean -cache >/dev/null 2>&1
have npm && npm cache clean --force >/dev/null 2>&1
have micromamba && micromamba clean -a -y >/dev/null 2>&1
rm -rf "$HOME/.cargo/registry/cache" "$HOME/.cargo/git/checkouts" 2>/dev/null
log "cleanup freed $(( $(free_mib "$WORKSPACE") - before )) MiB; $(free_mib "$WORKSPACE") MiB free"

if [ "${#WARNINGS[@]}" -gt 0 ]; then
  echo; log "cloud-setup warnings"; printf '   - %s\n' "${WARNINGS[@]}"
fi
exit 0
