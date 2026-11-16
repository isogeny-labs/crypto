# sourced helper: log a command, its exit code and output hashes
LOG=raw/COMMANDS.log
logrun() {  # logrun OUTFILE cmd...
  local out="$1"; shift
  local t0=$(date -u +%FT%TZ)
  "$@"; local rc=$?
  local h=""
  [ -f "$out" ] && h=$(sha256sum "$out" | cut -d' ' -f1)
  echo "$t0 rc=$rc cmd: $* | out=$out sha256=$h" >> $LOG
  return $rc
}
