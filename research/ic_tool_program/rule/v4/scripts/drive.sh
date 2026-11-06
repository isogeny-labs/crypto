#!/bin/bash
# Thin orchestration of rule v4 (PROTOCOL.md): every timed command under the
# heavy-compute lock in chunks of at most 25 minutes; each stage resumes what
# its output tree holds.  Usage: drive.sh <stage>... where a stage is one of
# train select confirm final reference run analyse replay.  <scratch>/bin holds the binaries.
set -u
SP=${SP:?scratch dir holding bin/ and heavy.lock}
ROOT=$(cd "$(dirname "$0")/.." && pwd)
V3=$ROOT/../v3
LOCK=${LOCK:-$SP/heavy.lock}
BIN=$SP/bin
IC=$BIN/ic-lever
RV=$BIN/ic_rule_v4
ISO=$BIN/isolated_bench
LOG=$ROOT/drive.log
chunks() {
  while true; do
    flock "$LOCK" "$@"
    rc=$?
    echo "$(date -u +%FT%TZ) exit $rc: ${*:1:6}" >> "$LOG"
    [ $rc -eq 3 ] && continue
    return $rc
  done
}
for stage in "$@"; do
  case $stage in
    train)
      chunks "$RV" train --ic "$IC" --root "$ROOT" --v3 "$V3" --grid "$ROOT/training/grid.json" --max-minutes 25 || exit 1 ;;
    select)
      "$RV" select --root "$ROOT" --v3 "$V3" --grid "$ROOT/training/grid.json" || exit 1 ;;
    confirm)
      chunks "$RV" train --confirm --ic "$IC" --root "$ROOT" --v3 "$V3" --grid "$ROOT/training/grid.json" --max-minutes 25 || exit 1 ;;
    final)
      "$RV" select --final --root "$ROOT" --v3 "$V3" --grid "$ROOT/training/grid.json" || exit 1
      (cd "$ROOT" && { echo; echo "# recipe selected $(date -u +%FT%TZ) (before the first evaluation row)"; sha256sum recipe.json training/stage1.json training/summary.json; } >> FROZEN.txt) ;;
    reference)
      chunks "$RV" reference --fixture "$BIN/koblitz_rho_fixture" --isolate "$ISO" --root "$ROOT" --max-minutes 25 || exit 1 ;;
    run)
      chunks "$RV" run --ic "$IC" --isolate "$ISO" --root "$ROOT" --v3 "$V3" --recipe "$ROOT/recipe.json" \
        --targets 32 --aa 8 --lev 8 --max-other-cpu 3.2 --max-minutes 25 || exit 1 ;;
    analyse) "$RV" analyse --root "$ROOT" --v3 "$V3" || exit 1 ;;
    replay) "$RV" replay --root "$ROOT" --v3 "$V3" --rerun 2 --ic "$IC" --recipe "$ROOT/recipe.json" || exit 1 ;;
    *) echo "unknown stage $stage"; exit 2 ;;
  esac
done
