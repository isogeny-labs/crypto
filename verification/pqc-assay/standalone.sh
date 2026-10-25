#!/usr/bin/env bash
# Run the complete, unmodified upstream suite from its pinned Git submodule.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
# shellcheck source=toolchain.env
source "$here/toolchain.env"
suite="$here/upstream"
relative=verification/pqc-assay/upstream

usage() {
    echo 'Usage: standalone.sh [check | setup | make TARGET... [NAME=VALUE...]]'
}
command=${1:-check}
if [[ $# -gt 0 ]]; then shift; fi
case "$command" in
    -h|--help) usage; exit 0 ;;
    check|setup) [[ $# -eq 0 ]] || { usage >&2; exit 2; } ;;
    make) [[ $# -gt 0 ]] || { usage >&2; exit 2; } ;;
    *) usage >&2; exit 2 ;;
esac

# Do not recurse: upstream's paper/ submodule is private and is not a proof input.
if [[ ! -e "$suite/.git" ]]; then
    git -C "$root" submodule update --init --checkout -- "$relative"
fi
test "$(git -C "$root" ls-files --stage -- "$relative" | awk '{print $1, $2}')" = "160000 $ASSAY_UPSTREAM_REV"
test "$(git -C "$suite" rev-parse HEAD)" = "$ASSAY_UPSTREAM_REV"
git -C "$suite" diff --exit-code HEAD

mkdir -p "$here/build"
run() {
    printf 'upstream=%s\ncommand=%s\n' "$ASSAY_UPSTREAM_REV" "$command"
    printf 'argument=%s\n' "$@"
    cd "$suite"
    case "$command" in
        check)
            make target-identity claim-lint
            python3 proof/hqc/gen_hqc.py --check
            ;;
        setup)
            ./scripts/setup.sh
            if [[ -z "${SKIP_ISABELLE:-}" ]]; then
                ./scripts/setup_isabelle_cryptol.sh
            fi
            ;;
        make) make "$@" ;;
    esac
}
run "$@" 2>&1 | tee "$here/build/standalone-$command.txt"
