#!/usr/bin/env bash
# One-shot workstation setup for the aburan28 research stack:
#
#   * toolchains   - C/C++ (gcc, clang, cmake), Rust (rustup), Go >= 1.25,
#                    Node 22, Python 3 + uv, PostgreSQL 16, PARI/GP, FLINT, GMP
#   * coordination - conductor / conductor-mcp / conductord, cairn
#   * SageMath     - 10.9 from conda-forge via micromamba, plus the
#                    `sage -python` shim that frozen experiments call
#   * agent CLIs   - claude (Claude Code), codex, opencode, gemini (optional)
#   * repositories - crypto, cryptanalysis, crypto-autoresearcher, cloned into
#                    $WORKSPACE and set up (Python venvs, kb, C library build)
#
# Ubuntu/Debian (apt) and macOS (Homebrew). Idempotent: anything already
# present is left alone, so re-running is a cheap way to repair a machine.
# Nothing here starts an autonomous agent (`conductor worker`, `cairn agent run`).
#
#   scripts/setup-workstation.sh                 # everything
#   scripts/setup-workstation.sh --no-sage       # skip the ~8 GiB SageMath env
#   scripts/setup-workstation.sh --only agents   # one component
#   scripts/setup-workstation.sh --list          # component names
#   scripts/setup-workstation.sh --dry-run       # print, change nothing
#
# Environment overrides:
#   WORKSPACE           where repositories are cloned          (~/src)
#   GITHUB_OWNER        owner of the repositories              (aburan28)
#   CONDUCTOR_VERSION   go module version of conductor         (main)
#   CAIRN_REV           cairn git revision                     (default branch)
#   SAGE_VERSION        SageMath version                       (10.9)
#   SAGE_PREFIX         micromamba root for the Sage env       (/opt/conda-sage, or ~/.local/conda-sage without root)
#   GO_VERSION          Go installed when go is missing/old    (1.26.9)
#   GO_INSTALL_DIR      where that Go goes                     (/usr/local/go)
#   NODE_MAJOR          Node.js major version                  (22)
#   WITH_GEMINI=1       also install @google/gemini-cli
#   WITH_LATEX=1        also install a TeX Live (papers/, iacr-eprint skill)
#   WITH_RUST_BUILD=1   also `cargo build --release` the crypto workspace (slow)
#   CLONE_REPOS         space-separated repos to clone when missing (all three);
#                       repos already in $WORKSPACE are set up either way
#
# GitHub service-account credentials (the `github` step). Never commit them:
# set them as secrets in the environment (cloud: environment settings, Network
# secrets or environment variables; elsewhere: your shell or a secrets manager).
#   GH_APP_ID, GH_APP_INSTALLATION_ID   a GitHub App and its installation, with
#   GH_APP_PRIVATE_KEY                  the App's PEM private key (or
#   GH_APP_PRIVATE_KEY_FILE             a path to it); tokens are minted per use
#   GH_SERVICE_TOKEN                    otherwise: a machine-user token (PAT)
#   GH_SERVICE_HOST                     host the credential is for (github.com)
#
# .claude/hooks/session-start.sh runs this in cloud sessions (docs/CONDUCTOR.md).
set -uo pipefail   # no -e: one failed component warns and the rest still run

case "${WORKSPACE_OFFLINE:-0}" in
  0|'') ;;
  1) exec bash "$(cd "$(dirname "$0")/.." && pwd)/scripts/private-workspace.sh" check ;;
  *) echo "WORKSPACE_OFFLINE must be 0 or 1" >&2; exit 2 ;;
esac

ALL_COMPONENTS=(system rust go node python conductor cairn sage agents github repos control-plane)

WORKSPACE="${WORKSPACE:-$HOME/src}"
GITHUB_OWNER="${GITHUB_OWNER:-aburan28}"
CONDUCTOR_VERSION="${CONDUCTOR_VERSION:-main}"
SAGE_VERSION="${SAGE_VERSION:-10.9}"
GO_VERSION="${GO_VERSION:-1.26.9}"
GO_MIN="1.25"
NODE_MAJOR="${NODE_MAJOR:-22}"
REPOS=(crypto cryptanalysis crypto-autoresearcher)
CLONE_REPOS="${CLONE_REPOS-${REPOS[*]}}"

DRY_RUN=0
declare -A SKIP=()
ONLY=()

usage() { sed -n '2,35p' "$0" | sed 's/^# \{0,1\}//'; }

while [ $# -gt 0 ]; do
  case "$1" in
    --dry-run) DRY_RUN=1 ;;
    --only) shift; IFS=, read -r -a ONLY <<<"${1:?--only needs a component list}" ;;
    --skip) shift; IFS=, read -r -a s <<<"${1:?--skip needs a component list}"; for c in "${s[@]}"; do SKIP[$c]=1; done ;;
    --no-sage) SKIP[sage]=1 ;;
    --no-agents) SKIP[agents]=1 ;;
    --no-repos) SKIP[repos]=1 ;;
    --no-control-plane) SKIP[control-plane]=1 ;;
    --workspace) shift; WORKSPACE="${1:?}" ;;
    --list) printf '%s\n' "${ALL_COMPONENTS[@]}"; exit 0 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown option: $1 (try --help)" >&2; exit 2 ;;
  esac
  shift
done

# ---------------------------------------------------------------- helpers ---
log()  { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
ok()   { printf '\033[1;32m ok\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m !!\033[0m %s\n' "$*" >&2; WARNINGS+=("$*"); }
WARNINGS=()
have() { command -v "$1" >/dev/null 2>&1; }
run()  { if [ "$DRY_RUN" = 1 ]; then printf '   + %s\n' "$*"; else "$@"; fi; }
want() {
  local c="$1"
  [ -n "${SKIP[$c]:-}" ] && return 1
  [ "${#ONLY[@]}" -eq 0 ] && return 0
  local o; for o in "${ONLY[@]}"; do [ "$o" = "$c" ] && return 0; done
  return 1
}
# version_ge A B: true when A >= B (dotted numeric).
version_ge() { [ "$(printf '%s\n%s\n' "$2" "$1" | sort -V | head -n1)" = "$2" ]; }

OS="$(uname -s)"
ARCH="$(uname -m)"
case "$ARCH" in x86_64|amd64) GOARCH=amd64; MMARCH=64 ;; aarch64|arm64) GOARCH=arm64; MMARCH=aarch64 ;; *) GOARCH="$ARCH"; MMARCH="$ARCH" ;; esac

if [ "$(id -u)" -eq 0 ]; then SUDO=(); elif have sudo; then SUDO=(sudo); else SUDO=(); fi
as_root() { if [ "$(id -u)" -eq 0 ] || [ "${#SUDO[@]}" -gt 0 ]; then run "${SUDO[@]}" "$@"; else warn "needs root, skipped: $*"; return 1; fi; }

PKG=""
if [ "$OS" = Darwin ]; then PKG=brew; elif have apt-get; then PKG=apt; elif have dnf; then PKG=dnf; fi

LOCAL_BIN="$HOME/.local/bin"
mkdir -p "$LOCAL_BIN"
export PATH="$LOCAL_BIN:$HOME/.cargo/bin:/usr/local/go/bin:$HOME/go/bin:$PATH"

# Every PATH entry this script relies on, written once to the user's shell rc,
# and to $CLAUDE_ENV_FILE when a Claude Code SessionStart hook runs this.
persist_path() {
  local line="$1" rc
  if [ -n "${CLAUDE_ENV_FILE:-}" ]; then
    grep -qxF -- "$line" "$CLAUDE_ENV_FILE" 2>/dev/null || run sh -c 'printf "%s\n" "$1" >> "$2"' sh "$line" "$CLAUDE_ENV_FILE"
  fi
  for rc in "$HOME/.bashrc" "$HOME/.zshrc"; do
    [ -f "$rc" ] || [ "$rc" = "$HOME/.bashrc" ] || continue
    grep -qxF -- "$line" "$rc" 2>/dev/null || run sh -c 'printf "%s\n" "$1" >> "$2"' sh "$line" "$rc"
  done
}

# ----------------------------------------------------------------- system ---
install_system() {
  log "system packages ($PKG)"
  case "$PKG" in
    apt)
      local pkgs=(build-essential clang lld cmake ninja-build pkg-config git curl wget ca-certificates
        unzip xz-utils bzip2 jq ripgrep shellcheck cppcheck valgrind clang-tidy
        python3 python3-dev python3-pip python3-venv
        libgmp-dev libmpfr-dev libmpc-dev libflint-dev libssl-dev libffi-dev
        pari-gp libpari-dev postgresql postgresql-contrib libpq-dev
        poppler-utils ghostscript graphviz)
      [ "${WITH_LATEX:-0}" = 1 ] && pkgs+=(texlive-latex-extra texlive-science texlive-fonts-recommended latexmk)
      local missing=() p
      for p in "${pkgs[@]}"; do
        dpkg-query -W -f='${Status}' "$p" 2>/dev/null | grep -q 'install ok installed' || missing+=("$p")
      done
      if [ "${#missing[@]}" -eq 0 ]; then ok "all ${#pkgs[@]} apt packages present"; return; fi
      pkgs=("${missing[@]}")
      as_root env DEBIAN_FRONTEND=noninteractive apt-get update -qq || true
      as_root env DEBIAN_FRONTEND=noninteractive apt-get install -y -qq --no-install-recommends "${pkgs[@]}" \
        || warn "some apt packages failed; re-run or install them by hand"
      ;;
    dnf)
      as_root dnf install -y gcc gcc-c++ clang cmake ninja-build git curl jq ripgrep ShellCheck cppcheck valgrind \
        python3 python3-devel gmp-devel mpfr-devel libmpc-devel flint-devel openssl-devel pari-gp \
        postgresql-server postgresql-contrib libpq-devel poppler-utils graphviz \
        || warn "some dnf packages failed"
      ;;
    brew)
      have brew || { warn "Homebrew not found: install it from https://brew.sh and re-run"; return; }
      run brew install cmake ninja pkg-config git jq ripgrep shellcheck cppcheck llvm gmp mpfr libmpc flint \
        pari postgresql@16 poppler graphviz python@3.12 || warn "some brew packages failed"
      [ "${WITH_LATEX:-0}" = 1 ] && run brew install --cask mactex-no-gui
      ;;
    *) warn "no supported package manager; install a C toolchain, cmake, GMP/MPFR/FLINT, PARI/GP and PostgreSQL 16 yourself" ;;
  esac
}

# ------------------------------------------------------------------- rust ---
install_rust() {
  log "Rust"
  if ! have rustup; then
    run sh -c 'curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile default'
    export PATH="$HOME/.cargo/bin:$PATH"
  fi
  # Install a toolchain only when there is none: re-running must not move an
  # existing one to a newer stable under a session's builds.
  if ! rustup show active-toolchain >/dev/null 2>&1; then
    run rustup toolchain install stable --profile default
    run rustup default stable
  fi
  run rustup component add clippy rustfmt
  have cargo-deny || run cargo install --locked cargo-deny || warn "cargo-deny not installed (cryptanalysis 'make suite-lint')"
  persist_path 'export PATH="$HOME/.cargo/bin:$PATH"'
  have cargo && ok "$(cargo --version)"
}

# --------------------------------------------------------------------- go ---
install_go() {
  log "Go (>= $GO_MIN)"
  local cur=""
  have go && cur="$(go env GOVERSION 2>/dev/null | sed 's/^go//')"
  if [ -n "$cur" ] && version_ge "$cur" "$GO_MIN"; then ok "go $cur"; return; fi
  if [ "$PKG" = brew ]; then run brew install go; return; fi
  local os=linux; [ "$OS" = Darwin ] && os=darwin
  local tgz="go${GO_VERSION}.${os}-${GOARCH}.tar.gz" dest="${GO_INSTALL_DIR:-/usr/local/go}"
  if [ "$DRY_RUN" = 1 ]; then
    log "would verify and stage $tgz before replacing $dest"
    return
  fi
  local download expected
  download="$(mktemp -d "${TMPDIR:-/tmp}/go-download.XXXXXXXX")" || return 1
  # The official release index supplies the checksum for this exact archive.
  # A failed download or validation must leave the installed toolchain intact.
  if ! curl -fsSL --retry 2 "https://go.dev/dl/?mode=json&include=all" -o "$download/releases.json" ||
     ! curl -fsSL --retry 2 "https://go.dev/dl/$tgz" -o "$download/$tgz"; then
    rm -rf "$download"
    warn "Go download failed; existing installation preserved"
    return 1
  fi
  expected="$(jq -er --arg name "$tgz" \
    '[.[].files[] | select(.filename == $name) | .sha256] | if length == 1 then .[0] else error("archive not found uniquely") end' \
    "$download/releases.json")"
  if [[ ! "$expected" =~ ^[0-9a-f]{64}$ ]]; then
    rm -rf "$download"
    warn "Go release checksum unavailable; existing installation preserved"
    return 1
  fi
  # All renames occur on the destination filesystem. Keep the old installation
  # until checksum, extraction and the staged binary's version all pass. The
  # lock also serializes setup hooks from sibling repositories.
  if ! as_root bash -c '
    set -euo pipefail
    archive=$1 expected=$2 version=$3 dest=$4
    root=$(dirname "$dest")
    name=$(basename "$dest")
    case "$name" in .|..|/) echo "invalid Go installation directory" >&2; exit 1 ;; esac
    mkdir -p "$root"
    exec 9>"$root/.$name-install.lock"
    flock -x 9
    stage=$(mktemp -d "$root/.go-install.XXXXXXXX")
    cleanup() {
      status=$?
      trap - EXIT
      if [ "$status" -ne 0 ] && { [ -e "$stage/previous" ] || [ -L "$stage/previous" ]; }; then
        if [ ! -e "$dest" ] && [ ! -L "$dest" ]; then
          if ! mv -T "$stage/previous" "$dest"; then
            echo "Go rollback failed; original installation retained at $stage/previous" >&2
            exit 1
          fi
        else
          echo "Go destination changed; original installation retained at $stage/previous" >&2
          exit 1
        fi
      fi
      rm -rf "$stage"
      exit "$status"
    }
    trap cleanup EXIT
    trap "exit 1" HUP INT TERM
    cp "$archive" "$stage/archive.tar.gz"
    printf "%s  %s\n" "$expected" "$stage/archive.tar.gz" | sha256sum -c -
    tar -C "$stage" -xzf "$stage/archive.tar.gz"
    [ "$(env GOROOT="$stage/go" "$stage/go/bin/go" env GOVERSION)" = "go$version" ]
    if [ -e "$dest" ] || [ -L "$dest" ]; then
      mv -T "$dest" "$stage/previous"
    fi
    mv -T "$stage/go" "$dest"
  ' workstation-go "$download/$tgz" "$expected" "$GO_VERSION" "$dest"; then
    rm -rf "$download"
    warn "Go installation failed; prior installation preserved or rollback location reported above"
    return 1
  fi
  rm -rf "$download"
  local escaped_bin
  printf -v escaped_bin '%q' "$dest/bin"
  persist_path "export PATH=$escaped_bin:\"\$HOME/go/bin:\$PATH\""
  export PATH="$dest/bin:$PATH"
  have go && ok "$(go version)"
}

# ------------------------------------------------------------------- node ---
install_node() {
  log "Node.js $NODE_MAJOR"
  if have node && [ "$(node -p 'process.versions.node.split(".")[0]')" -ge "$NODE_MAJOR" ]; then ok "node $(node --version)"; return; fi
  case "$PKG" in
    brew) run brew install "node@$NODE_MAJOR" ;;
    apt) run sh -c "curl -fsSL https://deb.nodesource.com/setup_${NODE_MAJOR}.x | ${SUDO[*]:-} bash -" \
           && as_root apt-get install -y -qq nodejs ;;
    *) warn "install Node.js >= $NODE_MAJOR yourself (https://nodejs.org)" ;;
  esac
}

# Global npm installs into ~/.local so they never need sudo.
npm_global() {
  have npm || { warn "npm missing, cannot install $*"; return 1; }
  if [ ! -w "$(npm prefix -g 2>/dev/null)/lib" ] 2>/dev/null; then
    run npm config set prefix "$HOME/.local"
  fi
  run npm install -g "$@"
}

# ----------------------------------------------------------------- python ---
install_python() {
  log "Python tooling (uv + research libraries)"
  have uv || run sh -c 'curl -LsSf https://astral.sh/uv/install.sh | sh'
  export PATH="$HOME/.local/bin:$PATH"
  persist_path 'export PATH="$HOME/.local/bin:$PATH"'
  # User-level libraries the plotting/report scripts in crypto/ import
  # (matplotlib, reportlab, numpy, ...). Repo-specific deps go in each repo's venv.
  run uv tool install --quiet ruff || true
  run uv tool install --quiet pre-commit || true
  have uv && ok "$(uv --version)"
}

# -------------------------------------------------------------- conductor ---
install_conductor() {
  log "Conductor ($CONDUCTOR_VERSION)"
  if have conductor && have conductor-mcp && have conductord; then ok "$(conductor version 2>/dev/null | head -n1)"; return; fi
  have go || { warn "go missing; run with --only go first"; return 1; }
  # @latest resolves to v0.1.0 whose go.mod has the wrong module path: use @main.
  run env GOBIN="$LOCAL_BIN" GOFLAGS=-modcacherw \
    go install "github.com/aburan28/conductor/cmd/...@${CONDUCTOR_VERSION}" \
    || warn "conductor install failed (try: curl -fsSL https://raw.githubusercontent.com/aburan28/conductor/main/scripts/install-release.sh | bash -s -- aburan28/conductor ~/.local/bin)"
}

# ------------------------------------------------------------------ cairn ---
install_cairn() {
  log "cairn"
  if have cairn; then ok "$(cairn --version 2>/dev/null | head -n1)"; return; fi
  have cargo || { warn "cargo missing; run with --only rust first"; return 1; }
  run cargo install --locked --root "$HOME/.local" \
    --git "https://github.com/$GITHUB_OWNER/cairn" ${CAIRN_REV:+--rev "$CAIRN_REV"} cairn \
    || warn "cairn install failed"
  mkdir -p "$HOME/.cairn"
}

# ------------------------------------------------------------------- sage ---
install_sage() {
  log "SageMath $SAGE_VERSION (conda-forge, ~8 GiB)"
  if [ -z "${SAGE_PREFIX:-}" ]; then
    if [ -w /opt ] || [ "${#SUDO[@]}" -gt 0 ]; then SAGE_PREFIX=/opt/conda-sage; else SAGE_PREFIX="$HOME/.local/conda-sage"; fi
  fi
  local env="$SAGE_PREFIX/envs/sage"
  if [ -x "$env/bin/python3" ] && "$env/bin/python3" -c \
      "from sage.env import SAGE_VERSION as v; import sys; sys.exit(v != '$SAGE_VERSION')" 2>/dev/null; then
    ok "sage $SAGE_VERSION at $env"
  else
    local parent; parent="$(dirname "$SAGE_PREFIX")"
    local free_kb; free_kb="$(df -Pk "$parent" 2>/dev/null | awk 'NR==2{print $4}')"
    if [ -n "$free_kb" ] && [ "$free_kb" -lt 10485760 ]; then
      warn "<10 GiB free under $parent, skipping SageMath (set SAGE_PREFIX elsewhere)"; return
    fi
    if [ ! -d "$SAGE_PREFIX" ]; then
      if [ -w "$parent" ]; then run mkdir -p "$SAGE_PREFIX"; else as_root install -d -o "$(id -u)" -g "$(id -g)" "$SAGE_PREFIX"; fi
    fi
    local mm="$LOCAL_BIN/micromamba"
    if ! have micromamba && [ ! -x "$mm" ]; then
      local plat=linux; [ "$OS" = Darwin ] && plat=osx
      run sh -c "curl -fsSL https://micro.mamba.pm/api/micromamba/${plat}-${MMARCH}/latest | tar xj -C '$HOME/.local' bin/micromamba"
    fi
    have micromamba && mm="$(command -v micromamba)"
    run "$mm" create -y -r "$SAGE_PREFIX" -p "$env" -c conda-forge "sage=$SAGE_VERSION" \
      || { warn "SageMath install failed"; return; }
  fi
  # `sage -python script.py` is the upstream CLI convention frozen trial plans
  # use; conda-forge's `sage` has no -python, so wrap it.
  run mkdir -p "$SAGE_PREFIX/bin"
  if [ "$DRY_RUN" = 0 ]; then
    cat >"$SAGE_PREFIX/bin/sage" <<EOF
#!/bin/sh
if [ "\$1" = "-python" ]; then shift; exec "$env/bin/python3" "\$@"; fi
exec "$env/bin/sage" "\$@"
EOF
    chmod +x "$SAGE_PREFIX/bin/sage"
  fi
  persist_path "export PATH=\"$SAGE_PREFIX/bin:\$PATH\""
  export PATH="$SAGE_PREFIX/bin:$PATH"
}

# ----------------------------------------------------------------- agents ---
install_agents() {
  log "agent CLIs"
  if have claude; then ok "claude $(claude --version 2>/dev/null | head -n1)"
  else run sh -c 'curl -fsSL https://claude.ai/install.sh | bash' || npm_global @anthropic-ai/claude-code || warn "claude not installed"; fi
  if have codex; then ok "codex $(codex --version 2>/dev/null | head -n1)"
  else npm_global @openai/codex || warn "codex not installed"; fi
  if have opencode; then ok "opencode $(opencode --version 2>/dev/null | head -n1)"
  else npm_global opencode-ai || run sh -c 'curl -fsSL https://opencode.ai/install | bash' || warn "opencode not installed"; fi
  if [ "${WITH_GEMINI:-0}" = 1 ]; then have gemini || npm_global @google/gemini-cli || warn "gemini not installed"; fi
  have gh || case "$PKG" in brew) run brew install gh ;; apt) as_root apt-get install -y -qq gh || warn "gh not installed" ;; esac
}

# ----------------------------------------------------------------- github ---
# A GitHub service account for git and gh, from secrets in the environment and
# never from this file. With a GitHub App, git asks ~/.local/bin/gh-app-token
# for a credential each time a github.com request needs one, and the helper
# mints a one-hour installation token (cached until five minutes before it
# expires). With a token, the helper hands that token out. The helper is scoped
# to https://$host, and git only consults it when the server asks for
# credentials, so public and proxy-served repositories are unaffected.
write_gh_app_token_helper() {
  local helper="$LOCAL_BIN/gh-app-token"
  cat >"$helper" <<'HELPER'
#!/usr/bin/env bash
# Generated by scripts/setup-workstation.sh. Prints a GitHub token for the
# service account named by GH_APP_ID/GH_APP_INSTALLATION_ID/GH_APP_PRIVATE_KEY
# (or _FILE), or by GH_SERVICE_TOKEN. Usage:
#   gh-app-token            print a token
#   gh-app-token git get    git credential-helper protocol
#   gh-app-token gh-login   (re)log gh in with a fresh token
set -euo pipefail
set +x
api="${GH_SERVICE_API:-https://api.github.com}"
cache="${XDG_CACHE_HOME:-$HOME/.cache}/gh-app-token"
umask 077

b64url() { openssl base64 -A | tr '+/' '-_' | tr -d '='; }

mint() {
  if [ -n "${GH_APP_ID:-}" ] && [ -n "${GH_APP_INSTALLATION_ID:-}" ]; then
    # One cache per identity: a token minted for one API host, App or
    # installation must never be handed out for another.
    local id dir_id
    id=$(printf '%s\n%s\n%s' "$api" "$GH_APP_ID" "$GH_APP_INSTALLATION_ID" \
      | { sha256sum 2>/dev/null || shasum -a 256; } | cut -c1-32)
    dir_id="$cache/$id"
    mkdir -p "$dir_id"
    local tok="$dir_id/token" exp="$dir_id/expires" now
    now=$(date +%s)
    if [ -s "$tok" ] && [ -s "$exp" ] && [ "$(cat "$exp")" -gt $((now + 300)) ]; then
      cat "$tok"; return
    fi
    local dir key jwt head body sig resp
    dir=$(mktemp -d); trap 'rm -rf "$dir"' RETURN
    key="$dir/key.pem"
    if [ -n "${GH_APP_PRIVATE_KEY_FILE:-}" ]; then cp "$GH_APP_PRIVATE_KEY_FILE" "$key"
    elif [ -n "${GH_APP_PRIVATE_KEY:-}" ]; then
      case "$GH_APP_PRIVATE_KEY" in
        *$'\n'*) printf '%s\n' "$GH_APP_PRIVATE_KEY" >"$key" ;;
        *) printf '%b\n' "$GH_APP_PRIVATE_KEY" >"$key" ;;   # "\n"-escaped one-line form
      esac
    else echo "gh-app-token: GH_APP_PRIVATE_KEY or GH_APP_PRIVATE_KEY_FILE is not set" >&2; return 1
    fi
    head=$(printf '{"alg":"RS256","typ":"JWT"}' | b64url)
    body=$(printf '{"iat":%d,"exp":%d,"iss":"%s"}' $((now - 60)) $((now + 540)) "$GH_APP_ID" | b64url)
    sig=$(printf '%s.%s' "$head" "$body" | openssl dgst -sha256 -sign "$key" -binary | b64url)
    jwt="$head.$body.$sig"
    # The JWT goes in a header file, not argv, so it never shows in `ps`.
    printf 'Authorization: Bearer %s\n' "$jwt" >"$dir/auth"
    resp=$(curl -fsS -X POST -H @"$dir/auth" -H 'Accept: application/vnd.github+json' \
      "$api/app/installations/$GH_APP_INSTALLATION_ID/access_tokens") \
      || { echo "gh-app-token: GitHub refused the App credentials" >&2; return 1; }
    jq -er .token <<<"$resp" >"$tok.new"
    date -d "$(jq -er .expires_at <<<"$resp")" +%s >"$exp" 2>/dev/null || echo $((now + 3000)) >"$exp"
    mv "$tok.new" "$tok"
    cat "$tok"
  elif [ -n "${GH_SERVICE_TOKEN:-}" ]; then
    printf '%s\n' "$GH_SERVICE_TOKEN"
  else
    echo "gh-app-token: no GitHub service credential (GH_APP_* or GH_SERVICE_TOKEN) in the environment" >&2
    return 1
  fi
}

case "${1:-}" in
  git)
    [ "${2:-}" = get ] || exit 0   # store/erase: nothing to persist
    t=$(mint) || exit 0            # no credential: let git fall through
    printf 'username=x-access-token\npassword=%s\n' "$t" ;;
  gh-login)
    mint | env -u GH_TOKEN -u GITHUB_TOKEN gh auth login --hostname "${GH_SERVICE_HOST:-github.com}" --with-token ;;
  *) mint ;;
esac
HELPER
  chmod 755 "$helper"
}

install_github_auth() {
  log "GitHub service-account credentials"
  local host="${GH_SERVICE_HOST:-github.com}" kind
  if [ -n "${GH_APP_ID:-}" ] && [ -n "${GH_APP_INSTALLATION_ID:-}" ] \
      && { [ -n "${GH_APP_PRIVATE_KEY:-}" ] || [ -n "${GH_APP_PRIVATE_KEY_FILE:-}" ]; }; then
    kind="GitHub App $GH_APP_ID (installation $GH_APP_INSTALLATION_ID)"
  elif [ -n "${GH_SERVICE_TOKEN:-}" ]; then
    kind="service token"
  else
    warn "no GitHub service credential set (GH_APP_ID + GH_APP_INSTALLATION_ID + GH_APP_PRIVATE_KEY[_FILE], or GH_SERVICE_TOKEN); skipped"
    return 0
  fi
  if [ "$DRY_RUN" = 1 ]; then
    printf '   + write %s/gh-app-token, git credential helper for https://%s, gh auth login (%s)\n' "$LOCAL_BIN" "$host" "$kind"
    return
  fi
  have git || { warn "git missing; cannot configure GitHub credentials"; return 1; }
  write_gh_app_token_helper
  # Mint once now, so a bad key or ID fails here and not on the first push.
  if ! "$LOCAL_BIN/gh-app-token" >/dev/null; then
    warn "GitHub credential check failed for $kind; git and gh left unconfigured"
    return 1
  fi
  local section="credential.https://$host.helper"
  git config --global --unset-all "$section" 2>/dev/null || true
  git config --global --add "$section" ""   # drop helpers inherited for this host
  git config --global --add "$section" "!$LOCAL_BIN/gh-app-token git"
  if have gh; then
    GH_SERVICE_HOST="$host" "$LOCAL_BIN/gh-app-token" gh-login \
      || warn "gh login with $kind failed"
    # An installation token lasts an hour: `gh-app-token gh-login` refreshes it.
    # In a shell where GH_TOKEN is set, gh uses that variable over this login.
    [ -n "${GH_TOKEN:-}${GITHUB_TOKEN:-}" ] && warn "GH_TOKEN/GITHUB_TOKEN is set in this environment and takes precedence for gh; git still uses $kind"
  fi
  ok "git (https://$host) and gh use $kind"
}

# ------------------------------------------------------------------ repos ---
clone_repo() {
  local name="$1" dir="$WORKSPACE/$1"
  if [ -d "$dir/.git" ]; then ok "$name present at $dir"; return; fi
  case " $CLONE_REPOS " in *" $name "*) ;; *) return 0 ;; esac
  local free_kb; free_kb="$(df -Pk "$WORKSPACE" 2>/dev/null | awk 'NR==2{print $4}')"
  if [ -n "$free_kb" ] && [ "$free_kb" -lt 5242880 ]; then
    warn "<5 GiB free under $WORKSPACE, not cloning $name"; return 1
  fi
  run git clone "https://github.com/$GITHUB_OWNER/$name.git" "$dir" || { warn "clone of $name failed"; return 1; }
}

setup_crypto() {
  local dir="$WORKSPACE/crypto"; [ -d "$dir" ] || return 0
  log "crypto: Python venv for plotting/report scripts"
  run uv venv --quiet --allow-existing "$dir/.venv"
  run uv pip install --quiet --python "$dir/.venv/bin/python" \
    numpy scipy sympy matplotlib pandas reportlab pyyaml networkx cypari2 python-flint pytest \
    || warn "crypto venv: some packages failed"
  [ "${WITH_RUST_BUILD:-0}" = 1 ] && { log "crypto: cargo build --release"; (cd "$dir" && run cargo build --release) || warn "crypto build failed"; }
  return 0
}

setup_cryptanalysis() {
  local dir="$WORKSPACE/cryptanalysis"; [ -d "$dir" ] || return 0
  log "cryptanalysis: C library (cmake) + Python bindings"
  (cd "$dir" && run make lib) || warn "cryptanalysis: make lib failed"
  run uv venv --quiet --allow-existing "$dir/.venv"
  run uv pip install --quiet --python "$dir/.venv/bin/python" ruff pytest numpy || true
  [ -f "$dir/bindings/python/pyproject.toml" ] && \
    { run uv pip install --quiet --python "$dir/.venv/bin/python" -e "$dir/bindings/python" || warn "cryptanalysis python bindings failed"; }
  [ -f "$dir/suite/python/indexcalc/requirements-sat.txt" ] && \
    { run uv pip install --quiet --python "$dir/.venv/bin/python" -r "$dir/suite/python/indexcalc/requirements-sat.txt" || true; }
  return 0
}

setup_autoresearcher() {
  local dir="$WORKSPACE/crypto-autoresearcher"; [ -d "$dir" ] || return 0
  log "crypto-autoresearcher: editable install + kb"
  run uv venv --quiet --allow-existing "$dir/.venv"
  run uv pip install --quiet --python "$dir/.venv/bin/python" \
    -r "$dir/requirements-dev.txt" -r "$dir/requirements-agent.txt" \
    -e "${dir}[dev,fast,gf2,index-calculus,agent,research-loop,campaign-mcp,rho-dp]" \
    || warn "crypto-autoresearcher: install failed"
  [ -d "$dir/kb" ] && { (cd "$dir/kb" && run uv sync --quiet --extra index --extra mcp --extra dev) || warn "kb: uv sync failed"; }
  [ -d "$dir/.git" ] && { (cd "$dir" && run make hooks) || true; }
  return 0
}

install_repos() {
  log "repositories in $WORKSPACE"
  run mkdir -p "$WORKSPACE"
  local r; for r in "${REPOS[@]}"; do clone_repo "$r" || true; done
  setup_crypto; setup_cryptanalysis; setup_autoresearcher
}

# ---------------------------------------------------------- control plane ---
start_control_plane() {
  log "Conductor control plane"
  have conductor || { warn "conductor missing, no control plane"; return; }
  if [ -n "${CONDUCTOR_ENDPOINT:-}" ] && [ -n "${CONDUCTOR_TOKEN:-}" ]; then
    ok "using shared control plane at $CONDUCTOR_ENDPOINT"; return
  fi
  local dir="$WORKSPACE/crypto"; [ -d "$dir" ] || dir="$PWD"
  # `conductor up` uses a reachable Postgres, else Docker, and prints a login token.
  (cd "$dir" && run conductor up --project crypto) || warn "conductor up failed; see docs/CONDUCTOR.md"
  (cd "$dir" && run conductor doctor) || true
}

# ------------------------------------------------------------------- main ---
log "workstation setup: $OS/$ARCH, package manager: ${PKG:-none}, workspace: $WORKSPACE"
want system        && install_system || true
want rust          && install_rust || true
want go            && install_go || true
want node          && install_node || true
want python        && install_python || true
want conductor     && install_conductor || true
want cairn         && install_cairn || true
want sage          && install_sage || true
want agents        && install_agents || true
want github        && install_github_auth || true
want repos         && install_repos || true
want control-plane && start_control_plane || true

echo
log "summary"
for c in gcc clang cmake cargo go node uv psql gp conductor conductor-mcp cairn sage claude codex opencode gh; do
  if have "$c"; then printf '   %-14s %s\n' "$c" "$(command -v "$c")"; else printf '   %-14s \033[1;33mmissing\033[0m\n' "$c"; fi
done
if [ "${#WARNINGS[@]}" -gt 0 ]; then
  echo; log "warnings"; printf '   - %s\n' "${WARNINGS[@]}"
fi
echo
echo "Open a new shell (or 'source ~/.bashrc') to pick up PATH changes."
echo "API keys are not handled here: set ANTHROPIC_API_KEY / OPENAI_API_KEY or run 'claude', 'codex login', 'opencode auth login'."
