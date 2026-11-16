#!/usr/bin/env bash
# Claude Code hook for tools/vault (encrypted research paths). Inert until the
# repository adopts vault, i.e. until .vault/vault.json exists.
#
#   SessionStart       build vault if missing and `vault unlock` with the
#                      session's AWS identity (KMS decrypt on the realm keys).
#                      Never fatal: without a key the tree stays ciphertext.
#   PreToolUse  Bash   git commit: `vault guard --staged --verify`.
#                      git push / gh pr create: `vault guard --verify` over
#                      every commit not yet on a remote. Refuses (exit 2) on
#                      plaintext in a protected path, or when vault is missing.
set -uo pipefail

input="$(cat)"
field() { printf '%s' "$input" | jq -r "$1 // empty" 2>/dev/null; }
event="$(field .hook_event_name)"
cwd="$(field .cwd)"
root="$(git -C "${cwd:-${CLAUDE_PROJECT_DIR:-.}}" rev-parse --show-toplevel 2>/dev/null)" || exit 0
[ -f "$root/.vault/vault.json" ] || exit 0

vault_bin() {
  if command -v vault >/dev/null 2>&1; then command -v vault; return; fi
  local b="$HOME/.local/bin/vault"
  if [ ! -x "$b" ] && [ -f "$root/tools/vault/Cargo.toml" ]; then
    cargo install --quiet --locked --path "$root/tools/vault" --root "$HOME/.local" >&2 || return 1
  fi
  [ -x "$b" ] && echo "$b"
}

case "$event" in
  SessionStart)
    if v="$(vault_bin)" && (cd "$root" && "$v" unlock >/dev/null 2>&1); then
      echo "vault: research paths decrypted ($(cd "$root" && "$v" status | grep -c '^realm') realm(s)); commits are guarded."
    else
      echo "vault: NOT unlocked (no KMS access or build failed); encrypted paths show ciphertext and cannot be staged. Run: vault unlock"
    fi
    ;;
  PreToolUse)
    cmd="$(field .tool_input.command)"
    COMMIT_RE='(^|[;&|( ]|\$\()git +([^ ]+ +)*commit\b'
    PUSH_RE='(^|[;&|( ]|\$\()git +push\b|\bgh +pr +create\b'
    want=""
    printf '%s' "$cmd" | grep -qE "$COMMIT_RE" && want=commit
    printf '%s' "$cmd" | grep -qE "$PUSH_RE" && want=push
    [ -n "$want" ] || exit 0
    if ! v="$(vault_bin)"; then
      echo "vault: this repository encrypts research paths but the vault binary is unavailable; build it (cargo install --locked --path tools/vault) before committing or pushing." >&2
      exit 2
    fi
    if [ "$want" = commit ]; then
      # `git commit -a` stages at commit time; the installed pre-commit hook covers that case.
      out="$(cd "$root" && "$v" guard --staged --verify 2>&1)" || { printf '%s\n' "$out" >&2; exit 2; }
    else
      out="$(cd "$root" && "$v" guard --verify --range "HEAD --not --remotes" 2>&1)" || { printf '%s\n' "$out" >&2; echo "Push refused: protected paths would be published in plaintext." >&2; exit 2; }
    fi
    exit 0
    ;;
esac
exit 0
