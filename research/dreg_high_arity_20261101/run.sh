#!/usr/bin/env bash
# Frozen, append-only orchestration for protocol.json. Scientific
# classification belongs to dreg_high_arity_analyze, never to this shell.
set -euo pipefail

usage() {
  cat >&2 <<'EOF'
usage: run.sh LADDER_BIN CLOSURE_BIN ANALYZER_BIN STEP

STEP is one of:
  gen-m5-n11-l2
  gen-m5-n13-l3
  gen-m6-n11-l2
  audit-draws
  analyze
  identity-m4-n12-l5-d24
  control-m{5,6}-n{11,13}-l{2,3}-dDRAW
  unsat-m{5,6}-n{11,13}-l{2,3}-dDRAW

Only the three registered cells m5-n11-l2, m5-n13-l3, and m6-n11-l2
are accepted. A closure STEP runs exactly one closure subprocess.
EOF
  exit 2
}

[[ $# -eq 4 ]] || usage
LADDER_ARG=$1
CLOSURE_ARG=$2
ANALYZER_ARG=$3
STEP=$4

HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
PROTOCOL=$HERE/protocol.json
SOURCE_MANIFEST=$HERE/sources.sha256
BINARY_MANIFEST=$HERE/binary.sha256
DRAWS=$(realpath -m "${DREG_DRAWS_DIR:-$HERE/draws}")
OUTPUT_ROOT=$(realpath -m "${DREG_OUTPUT_ROOT:-$HERE}")
RUNS=$OUTPUT_ROOT/runs
CHECKPOINTS=$OUTPUT_ROOT/checkpoints
SEED=20261101
MAX_ATTEMPTS=4

# This is the complete environment visible to each native scientific
# executable. Inherited KIC_*, F4_F2_*, and debug/tuning variables therefore
# cannot silently change the algorithm. Absence means the frozen source's
# default; the fixed PATH and four scientific variables below are recorded in
# every attempt meta.json.
SCIENTIFIC_ENV=(
  env -i
  PATH=/usr/bin:/bin
  F4_F2_MAX_ROWS=50000000
  F4_F2_MAX_COLS=50000000
  RAYON_NUM_THREADS=4
  DREG_CKPT_SECONDS=300
)

CELLS=("5:11:2:43:44" "5:13:3:54:52" "6:11:2:56:55")

die() {
  echo "error: $*" >&2
  exit 1
}

is_hex() {
  local width=$1 value=$2
  [[ $value =~ ^[0-9a-f]{$width}$ ]]
}

cell_file() {
  local m=$1 n=$2 ell=$3
  printf '%s/m%s-n%s-l%s.jsonl' "$DRAWS" "$m" "$n" "$ell"
}

registered_cell() {
  local m=$1 n=$2 ell=$3
  [[ $m:$n:$ell == 5:11:2 || $m:$n:$ell == 5:13:3 || $m:$n:$ell == 6:11:2 ]]
}

cell_shape() {
  local m=$1 n=$2 ell=$3 row cm cn ce cv cq
  for row in "${CELLS[@]}"; do
    IFS=: read -r cm cn ce cv cq <<<"$row"
    if [[ $m:$n:$ell == "$cm:$cn:$ce" ]]; then
      printf '%s:%s\n' "$cv" "$cq"
      return 0
    fi
  done
  return 1
}

manifest_hash() {
  local label=$1
  awk -v label="$label" '
    $1 ~ /^[0-9a-f]{64}$/ && $2 == label { value=$1; count++ }
    END { if (count == 1) print value; else exit 1 }
  ' "$BINARY_MANIFEST"
}

protocol_body_hash() {
  local canonical
  canonical=$(jq -cS 'del(.freeze)' "$PROTOCOL") || return 1
  printf '%s' "$canonical" | sha256sum | awk '{print $1}'
}

verify_bindings() {
  [[ -s $PROTOCOL && -s $SOURCE_MANIFEST && -s $BINARY_MANIFEST ]] ||
    die "protocol, sources.sha256, and binary.sha256 must all exist"

  LADDER=$(realpath -e "$LADDER_ARG") || die "missing ladder binary: $LADDER_ARG"
  CLOSURE=$(realpath -e "$CLOSURE_ARG") || die "missing closure binary: $CLOSURE_ARG"
  ANALYZER=$(realpath -e "$ANALYZER_ARG") || die "missing analyzer binary: $ANALYZER_ARG"
  [[ -x $LADDER && -x $CLOSURE && -x $ANALYZER ]] ||
    die "all three frozen native binaries must be executable"

  LADDER_SHA=$(sha256sum "$LADDER" | awk '{print $1}')
  CLOSURE_SHA=$(sha256sum "$CLOSURE" | awk '{print $1}')
  ANALYZER_SHA=$(sha256sum "$ANALYZER" | awk '{print $1}')
  LADDER_SOURCE_SHA=$(sha256sum "$REPO/examples/dreg_ladder.rs" | awk '{print $1}')
  CLOSURE_SOURCE_SHA=$(sha256sum "$REPO/examples/dreg_closure.rs" | awk '{print $1}')
  ANALYZER_SOURCE_SHA=$(sha256sum "$REPO/examples/dreg_high_arity_analyze.rs" | awk '{print $1}')
  [[ $LADDER_SHA == "$(manifest_hash dreg_ladder)" ]] || die "ladder binary hash mismatch"
  [[ $CLOSURE_SHA == "$(manifest_hash dreg_closure)" ]] || die "closure binary hash mismatch"
  [[ $ANALYZER_SHA == "$(manifest_hash dreg_high_arity_analyze)" ]] ||
    die "analyzer binary hash mismatch"

  PROTOCOL_SHA=$(jq -er '.freeze.protocol_sha256' "$PROTOCOL") ||
    die "protocol has no frozen hash"
  SOURCE_COMMIT=$(jq -er '.freeze.commit' "$PROTOCOL") || die "protocol has no frozen commit"
  is_hex 64 "$PROTOCOL_SHA" || die "protocol hash is not SHA-256"
  is_hex 40 "$SOURCE_COMMIT" || die "source commit is not a full Git object id"
  [[ $(jq -er '.freeze.status' "$PROTOCOL") == frozen ]] || die "protocol is not frozen"
  [[ $(jq -er '.freeze.filed_before_affected_data' "$PROTOCOL") == true ]] ||
    die "protocol was not filed before affected data"
  [[ $PROTOCOL_SHA == "$(protocol_body_hash)" ]] || die "protocol changed after freeze"
  BUILD_CONTAINER_DIGEST=$(jq -er '.arms[0].instrument.flags.container_image_digest' "$PROTOCOL") ||
    die "protocol lacks the build container digest"
  FROZEN_RUSTC_VERSION=$(jq -er '.arms[0].instrument.flags.rustc_version' "$PROTOCOL") ||
    die "protocol lacks the compiler identity"

  jq -e --arg commit "$SOURCE_COMMIT" --arg ladder "$LADDER_SHA" \
    --arg closure "$CLOSURE_SHA" --arg analyzer "$ANALYZER_SHA" \
    --arg ladder_source "$LADDER_SOURCE_SHA" --arg closure_source "$CLOSURE_SOURCE_SHA" \
    --arg analyzer_source "$ANALYZER_SOURCE_SHA" \
    --arg container "$BUILD_CONTAINER_DIGEST" --arg rustc "$FROZEN_RUSTC_VERSION" '
      (.arms | length == 5) and
      all(.arms[];
        .instrument.commit == $commit and
        .instrument.binary_sha256 == $closure and
        .instrument.flags.ladder_binary_sha256 == $ladder and
        .instrument.flags.analyzer_binary_sha256 == $analyzer and
        .instrument.flags.ladder_source_sha256 == $ladder_source and
        .instrument.flags.closure_source_sha256 == $closure_source and
        .instrument.flags.analyzer_source_sha256 == $analyzer_source and
        .instrument.flags.container_image_digest == $container and
        .instrument.flags.rustc_version == $rustc
      ) and
      ($container | test("^sha256:[0-9a-f]{64}$")) and
      ($rustc | length > 0) and
      .analysis_plan.script.path == "../../examples/dreg_high_arity_analyze.rs" and
      .analysis_plan.script.sha256 == $analyzer_source
    ' "$PROTOCOL" >/dev/null || die "protocol instrument identity is incomplete or mismatched"

  (
    cd "$REPO"
    sha256sum -c "$SOURCE_MANIFEST"
    git cat-file -e "$SOURCE_COMMIT^{commit}"
    git merge-base --is-ancestor "$SOURCE_COMMIT" HEAD
    git diff --quiet
    git diff --cached --quiet
    git diff --quiet "$SOURCE_COMMIT" -- \
      Cargo.toml Cargo.lock build.rs src \
      examples/dreg_ladder.rs examples/dreg_closure.rs \
      examples/dreg_high_arity_analyze.rs \
      research/dreg_high_arity_20261101/run.sh \
      research/dreg_high_arity_20261101/sources.sha256
    [[ -z $(git status --porcelain --untracked-files=all -- \
      Cargo.toml Cargo.lock build.rs src \
      examples/dreg_ladder.rs examples/dreg_closure.rs \
      examples/dreg_high_arity_analyze.rs \
      research/dreg_high_arity_20261101/run.sh \
      research/dreg_high_arity_20261101/sources.sha256 \
      research/dreg_high_arity_20261101/binary.sha256 \
      research/dreg_high_arity_20261101/protocol.json) ]]
  ) || die "current transitive Rust source tree differs from the frozen commit"
}

require_durable_output() {
  [[ -n ${DREG_OUTPUT_ROOT:-} ]] ||
    die "scientific execution requires an explicit absolute DREG_OUTPUT_ROOT"
  [[ $DREG_OUTPUT_ROOT == /* ]] || die "DREG_OUTPUT_ROOT must be absolute"
  OUTPUT_ROOT=$(realpath -m "$DREG_OUTPUT_ROOT")
  [[ $OUTPUT_ROOT != / && $OUTPUT_ROOT != "$REPO" ]] || die "unsafe output root: $OUTPUT_ROOT"
  RUNS=$OUTPUT_ROOT/runs
  CHECKPOINTS=$OUTPUT_ROOT/checkpoints

  if [[ -n ${EPHEMERAL_OUT:-} ]]; then
    local durable
    durable=$(realpath -m "$EPHEMERAL_OUT")
    [[ $OUTPUT_ROOT == "$durable" ]] ||
      die "ephemeral DREG_OUTPUT_ROOT must equal EPHEMERAL_OUT"
  elif [[ -n ${EPHEMERAL_RESUME_DIR:-} ]]; then
    die "EPHEMERAL_RESUME_DIR is set without EPHEMERAL_OUT"
  fi
}

require_durable_draws_on_ephemeral() {
  if [[ -n ${EPHEMERAL_OUT:-} ]]; then
    local durable
    [[ -n ${DREG_DRAWS_DIR:-} && $DREG_DRAWS_DIR == /* ]] ||
      die "ephemeral generation requires an absolute DREG_DRAWS_DIR"
    durable=$(realpath -m "$EPHEMERAL_OUT")
    [[ $DRAWS == "$durable" || $DRAWS == "$durable/"* ]] ||
      die "ephemeral DREG_DRAWS_DIR must equal or descend from EPHEMERAL_OUT"
  fi
}

prepare_output() {
  require_durable_output
  if [[ -n ${EPHEMERAL_RESUME_DIR:-} ]]; then
    local resume durable stamp
    resume=$(realpath -e "$EPHEMERAL_RESUME_DIR") || die "resume directory does not exist"
    durable=$(realpath -m "$EPHEMERAL_OUT")
    [[ -d $resume && $resume != "$OUTPUT_ROOT" ]] || die "invalid resume directory"
    [[ $resume != "$OUTPUT_ROOT/"* && $OUTPUT_ROOT != "$resume/"* ]] ||
      die "resume and output roots may not contain one another"
    [[ $OUTPUT_ROOT == "$durable" ]] ||
      die "resume destination is not durable"
    if [[ -d $OUTPUT_ROOT && -n $(find "$OUTPUT_ROOT" -mindepth 1 -print -quit) ]]; then
      die "resume destination must be empty before import"
    fi
    [[ -z $(find "$resume" -type l -print -quit) ]] || die "resume tree contains a symlink"
    [[ -z $(find "$resume" ! -type d ! -type f -print -quit) ]] ||
      die "resume tree contains a special file"
    mkdir -p "$OUTPUT_ROOT"
    cp -a -- "$resume"/. "$OUTPUT_ROOT"/
    stamp=$(date -u +%Y%m%dT%H%M%SZ)
    local import_record
    import_record=$(mktemp "$OUTPUT_ROOT/resume-import-$stamp.XXXXXX.json")
    jq -n --arg source "$resume" --arg destination "$OUTPUT_ROOT" \
      --arg imported_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
      '{schema:"dreg.resume-import/v1",source:$source,destination:$destination,imported_at:$imported_at}' \
      >"$import_record"
    unset EPHEMERAL_RESUME_DIR
  fi
  mkdir -p "$RUNS" "$CHECKPOINTS"
  [[ -z $(find "$OUTPUT_ROOT" -type l -print -quit) ]] ||
    die "output tree contains a symlink"
  [[ -z $(find "$OUTPUT_ROOT" ! -type d ! -type f -print -quit) ]] ||
    die "output tree contains a special file"
  validate_output_bindings
}

validate_output_bindings() {
  local attempt meta logical number final
  while IFS= read -r -d '' attempt; do
    meta=$attempt/meta.json
    logical=${attempt%/attempts/*}
    logical=${logical##*/}
    number=${attempt##*/}
    if [[ ! -e $meta ]]; then
      [[ ! -e $attempt/out.jsonl && ! -e $attempt/status && ! -e $attempt/finished.json ]] ||
        die "resumed attempt has output but no binding metadata: $attempt"
      continue
    fi
    [[ -f $meta && ! -L $meta ]] || die "invalid attempt metadata file: $meta"
    jq -e --arg logical "$logical" --arg attempt "$number" \
      --arg protocol "$PROTOCOL_SHA" --arg commit "$SOURCE_COMMIT" \
      --arg ladder "$LADDER_SHA" --arg closure "$CLOSURE_SHA" \
      --arg analyzer "$ANALYZER_SHA" --arg container "$BUILD_CONTAINER_DIGEST" \
      --arg rustc "$FROZEN_RUSTC_VERSION" '
        .schema == "dreg.high-arity-attempt/v1" and
        .logical_name == $logical and .attempt == $attempt and
        .protocol_sha256 == $protocol and .source_commit == $commit and
        .binaries.ladder == $ladder and .binaries.closure == $closure and
        .binaries.analyzer == $analyzer and
        .build.container_image_digest == $container and .build.rustc_version == $rustc and
        .parent_environment_cleared == true and
        .sanitized_env == {PATH:"/usr/bin:/bin",F4_F2_MAX_ROWS:"50000000",
          F4_F2_MAX_COLS:"50000000",RAYON_NUM_THREADS:"4",DREG_CKPT_SECONDS:"300"}
      ' "$meta" >/dev/null || die "resumed attempt binding mismatch: $meta"
  done < <(find "$RUNS" -type d -path '*/attempts/[0-9][0-9][0-9][0-9]' -print0)
  while IFS= read -r -d '' final; do
    logical=${final%/final.json}
    logical=${logical##*/}
    jq -e --arg logical "$logical" --arg protocol "$PROTOCOL_SHA" \
      --arg analyzer "$ANALYZER_SHA" '
        .schema == "dreg.high-arity-final/v1" and .logical_name == $logical and
        .protocol_sha256 == $protocol and .analyzer_sha256 == $analyzer and
        (.terminal_exit_code == 0 or .terminal_exit_code == 65 or .terminal_exit_code == 76) and
        (.row_sha256 | type == "string" and test("^[0-9a-f]{64}$"))
      ' "$final" >/dev/null || die "resumed terminal pointer binding mismatch: $final"
  done < <(find "$RUNS" -mindepth 2 -maxdepth 2 -type f -name final.json -print0)
}

audit_draw_file() {
  local file=$1 m=$2 n=$3 ell=$4 shape n_vars n_eqs rows lines
  [[ -s $file ]] || { echo "missing draw file: $file" >&2; return 1; }
  rows=$(jq -s 'length' "$file") || return 1
  lines=$(wc -l <"$file")
  [[ $rows -eq $lines ]] || {
    echo "draw file must contain exactly one JSON object per nonempty line: $file" >&2
    return 1
  }
  shape=$(cell_shape "$m" "$n" "$ell") || return 1
  IFS=: read -r n_vars n_eqs <<<"$shape"
  jq -e -s --arg cell "n${n}l${ell}m${m}" \
    --argjson m "$m" --argjson n "$n" --argjson ell "$ell" \
    --argjson nv "$n_vars" --argjson ne "$n_eqs" '
      def sat: .solutions > 0 and .outcome.kind == "satisfiable";
      def unsat:
        .solutions == 0 and
        (.outcome.kind == "resolved" or .outcome.kind == "at_least" or
         .outcome.kind == "caps_hit");
      def outcome_valid:
        if .kind == "satisfiable" then (keys | length) == 1
        elif .kind == "resolved" then
          (.degree | type == "number" and floor == . and . >= 1 and . <= 3) and
          (.refuted | type == "boolean") and (keys | length) == 3
        elif .kind == "at_least" then
          .degree == 4 and (keys | length) == 2
        elif .kind == "caps_hit" then
          (.built == null or
            (.built | type == "number" and floor == . and . >= 1 and . <= 2)) and
          (keys | length) == 2
        else false end;
      . as $rows |
      length >= 6 and length <= 4096 and
      all(to_entries[];
        (.key as $i | .value |
          type == "object" and .draw == $i and
          .cell == $cell and .m == $m and .n == $n and .ell == $ell and
          .surplus == ($n - ($m * $ell)) and .d_max == 3 and
          .n_vars == $nv and .n_eqs == $ne and
          (.v_basis | type == "array" and length == $ell and
            all(.[]; type == "number" and floor == . and . >= 0 and . < pow(2; $n))) and
          (.x_r | type == "number" and floor == . and . >= 0 and . < pow(2; $n)) and
          (.solutions | type == "number" and floor == . and . >= 0) and
          (.outcome | type == "object" and outcome_valid) and
          ((.ffd == null) or
            (.solutions == 0 and (.ffd | type == "number" and floor == . and . >= 1 and . <= 3))) and
          (.secs | type == "number") and
          (sat or unsat)
        )
      ) and
      ([$rows[] | select(sat)] | length) >= 2 and
      ([$rows[] | select(unsat)] | length) >= 4 and
      ([range(0; ($rows | length) - 1) as $i |
        (([$rows[0:$i + 1][] | select(sat)] | length) < 2 or
         ([$rows[0:$i + 1][] | select(unsat)] | length) < 4)] | all)
    ' "$file" >/dev/null || {
      echo "draw file is not the exact registered full prefix: $file" >&2
      return 1
    }
}

audit_one() {
  local m=$1 n=$2 ell=$3
  audit_draw_file "$(cell_file "$m" "$n" "$ell")" "$m" "$n" "$ell"
}

validate_generation_provenance() {
  local draw=$1 m=$2 n=$3 ell=$4 metadata log
  metadata=${draw%.jsonl}.generation.json
  log=${draw%.jsonl}.log
  [[ -f $metadata && ! -L $metadata && -f $log && ! -L $log ]] || return 1
  jq -e --arg cell "m$m-n$n-l$ell" --arg protocol "$PROTOCOL_SHA" \
    --arg commit "$SOURCE_COMMIT" --arg ladder "$LADDER_SHA" \
    --arg container "$BUILD_CONTAINER_DIGEST" --arg rustc "$FROZEN_RUSTC_VERSION" '
      .schema == "dreg.high-arity-generation/v1" and .cell == $cell and
      .protocol_sha256 == $protocol and .source_commit == $commit and
      .binary_sha256 == $ladder and
      .build.container_image_digest == $container and .build.rustc_version == $rustc and
      .parent_environment_cleared == true and
      .sanitized_env == {PATH:"/usr/bin:/bin",F4_F2_MAX_ROWS:"50000000",
        F4_F2_MAX_COLS:"50000000",RAYON_NUM_THREADS:"4",DREG_CKPT_SECONDS:"300"}
    ' "$metadata" >/dev/null
}

verify_draw_custody() {
  local row m n ell ignored committed selected metadata log rel rel_metadata rel_log
  for row in "${CELLS[@]}"; do
    IFS=: read -r m n ell ignored _ <<<"$row"
    committed=$HERE/draws/m$m-n$n-l$ell.jsonl
    selected=$(cell_file "$m" "$n" "$ell")
    metadata=${committed%.jsonl}.generation.json
    log=${committed%.jsonl}.log
    [[ -f $committed && ! -L $committed && -f $selected && ! -L $selected ]] || {
      echo "draw input must be a regular non-symlink file: $selected" >&2
      return 1
    }
    rel=${committed#"$REPO/"}
    rel_metadata=${metadata#"$REPO/"}
    rel_log=${log#"$REPO/"}
    (
      cd "$REPO"
      git ls-files --error-unmatch "$rel" >/dev/null
      git ls-files --error-unmatch "$rel_metadata" >/dev/null
      git ls-files --error-unmatch "$rel_log" >/dev/null
      git diff --quiet HEAD -- "$rel" "$rel_metadata" "$rel_log"
      [[ -z $(git status --porcelain --untracked-files=all -- \
        "$rel" "$rel_metadata" "$rel_log") ]]
    ) || {
      echo "draw input is not a clean committed file: $committed" >&2
      return 1
    }
    validate_generation_provenance "$committed" "$m" "$n" "$ell" || {
      echo "draw generation provenance mismatch: $metadata" >&2
      return 1
    }
    [[ $(sha256sum "$committed" | awk '{print $1}') == \
      "$(sha256sum "$selected" | awk '{print $1}')" ]] || {
      echo "selected draw copy differs from committed input: $selected" >&2
      return 1
    }
  done
}

audit_all_draws() {
  local row m n ell ignored
  for row in "${CELLS[@]}"; do
    IFS=: read -r m n ell ignored _ <<<"$row"
    audit_one "$m" "$n" "$ell"
  done
  verify_draw_custody
  echo "all three deterministic draw prefixes passed"
}

selected_draws() {
  local file=$1 mode=$2 limit=$3
  if [[ $mode == sat ]]; then
    jq -r 'select(.solutions > 0 and .outcome.kind == "satisfiable") | .draw' "$file" |
      sort -n | sed -n "1,${limit}p"
  else
    jq -r 'select(.solutions == 0 and .outcome.kind != "satisfiable") | .draw' "$file" |
      sort -n | sed -n "1,${limit}p"
  fi
}

draw_is_selected() {
  local file=$1 mode=$2 draw=$3 limit
  [[ $draw =~ ^(0|[1-9][0-9]*)$ ]] || return 1
  [[ $mode == sat ]] && limit=2 || limit=4
  selected_draws "$file" "$mode" "$limit" |
    awk -v draw="$draw" '$0 == draw { found=1 } END { exit !found }'
}

analyzer_snapshot() {
  local required=${1:-} tmp rc
  mkdir -p "$OUTPUT_ROOT"
  tmp=$(mktemp "$OUTPUT_ROOT/.summary.tmp.XXXXXX")
  if [[ -n $required ]]; then
    if (
      cd "$REPO"
      "${SCIENTIFIC_ENV[@]}" "$ANALYZER" "$DRAWS" "$OUTPUT_ROOT" --require "$required"
    ) >"$tmp"; then
      rc=0
    else
      rc=$?
    fi
  else
    if (
      cd "$REPO"
      "${SCIENTIFIC_ENV[@]}" "$ANALYZER" "$DRAWS" "$OUTPUT_ROOT"
    ) >"$tmp"; then
      rc=0
    else
      rc=$?
    fi
  fi
  mv -f -- "$tmp" "$OUTPUT_ROOT/summary.json"
  return "$rc"
}

write_final_pointer() {
  local logical=$1 analysis=$2 terminal_rc=$3 run_dir final count row row_sha
  local classification selected tmp created
  [[ $terminal_rc -eq 0 || $terminal_rc -eq 65 || $terminal_rc -eq 76 ]] ||
    die "refusing a terminal pointer for nonterminal analyzer exit $terminal_rc"
  run_dir=$RUNS/$logical
  final=$run_dir/final.json
  mkdir -p "$run_dir"
  count=$(jq --arg logical "$logical" '[.rows[] | select(.run_id == $logical)] | length' "$analysis") ||
    die "cannot read analyzer output for $logical"
  [[ $count -eq 1 ]] || die "analyzer output has $count rows for $logical"
  # Attempt artifact paths contain the per-job EPHEMERAL_OUT root and change
  # after a valid resume. Parser-error reasons can quote those paths too. Hash
  # the logical evidence after dropping only those location-bearing strings,
  # so a copied, otherwise identical history keeps its terminal identity.
  row=$(jq -cS --arg logical "$logical" '
    .rows[] | select(.run_id == $logical) |
    del(.reason, .attempts[].reason, .attempts[].output,
      .attempts[].status, .attempts[].metadata)
  ' "$analysis")
  row_sha=$(printf '%s' "$row" | sha256sum | awk '{print $1}')
  classification=$(printf '%s' "$row" | jq -er '.classification') ||
    die "analyzer row lacks classification for $logical"
  selected=$(printf '%s' "$row" | jq -c '.selected_attempt') ||
    die "analyzer row lacks selected_attempt for $logical"

  if [[ -e $final ]]; then
    [[ -f $final && ! -L $final ]] || die "invalid terminal pointer: $final"
    jq -e --arg logical "$logical" --arg classification "$classification" \
      --argjson selected_attempt "$selected" \
      --argjson terminal_exit_code "$terminal_rc" --arg row_sha256 "$row_sha" \
      --arg protocol_sha256 "$PROTOCOL_SHA" --arg analyzer_sha256 "$ANALYZER_SHA" '
        .schema == "dreg.high-arity-final/v1" and .logical_name == $logical and
        .classification == $classification and .selected_attempt == $selected_attempt and
        .terminal_exit_code == $terminal_exit_code and
        .row_sha256 == $row_sha256 and .protocol_sha256 == $protocol_sha256 and
        .analyzer_sha256 == $analyzer_sha256
      ' "$final" >/dev/null || die "existing terminal pointer disagrees with analysis: $final"
    return 0
  fi

  created=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  tmp=$(mktemp "$run_dir/.final.tmp.XXXXXX")
  jq -n --arg logical_name "$logical" --arg classification "$classification" \
    --argjson selected_attempt "$selected" \
    --argjson terminal_exit_code "$terminal_rc" --arg row_sha256 "$row_sha" \
    --arg protocol_sha256 "$PROTOCOL_SHA" --arg analyzer_sha256 "$ANALYZER_SHA" \
    --arg created_at "$created" '
      {schema:"dreg.high-arity-final/v1",logical_name:$logical_name,
       classification:$classification,selected_attempt:$selected_attempt,
       terminal_exit_code:$terminal_exit_code,row_sha256:$row_sha256,
       protocol_sha256:$protocol_sha256,analyzer_sha256:$analyzer_sha256,
       created_at:$created_at}
    ' >"$tmp"
  chmod 644 "$tmp"
  if ln -- "$tmp" "$final" 2>/dev/null; then
    unlink "$tmp"
    echo "sealed write-once terminal pointer $final"
  else
    unlink "$tmp"
    [[ -e $final ]] || die "could not atomically create terminal pointer: $final"
    write_final_pointer "$logical" "$analysis" "$terminal_rc"
  fi
}

seal_terminal_rows_from_summary() {
  local summary=$1 final logical terminal_rc
  while IFS= read -r logical; do
    terminal_rc=$(jq -r --arg logical "$logical" '
      .rows[] | select(.run_id == $logical) |
      if (.classification == "passed" or .classification == "closure_by_four" or
          .classification == "above_four") then 0
      elif .classification == "unmeasured" then
        if (any(.attempts[]?; .exit_status == 0) or (.attempts | length) >= 4)
        then 76 else 75 end
      else 65 end
    ' "$summary") || die "cannot classify terminal state for $logical"
    if [[ $terminal_rc -ne 75 ]]; then
      write_final_pointer "$logical" "$summary" "$terminal_rc"
    fi
  done < <(jq -r '.rows[].run_id' "$summary")

  while IFS= read -r -d '' final; do
    logical=${final%/final.json}
    logical=${logical##*/}
    terminal_rc=$(jq -er '.terminal_exit_code' "$final") ||
      die "terminal pointer lacks an exit code: $final"
    write_final_pointer "$logical" "$summary" "$terminal_rc"
  done < <(find "$RUNS" -mindepth 2 -maxdepth 2 -type f -name final.json -print0)
}

require_existing() {
  local logical=$1 rc
  if analyzer_snapshot "$logical"; then rc=0; else rc=$?; fi
  if [[ $rc -eq 0 ]]; then
    write_final_pointer "$logical" "$OUTPUT_ROOT/summary.json" "$rc"
    return 0
  fi
  die "prerequisite $logical is not successful (analyzer exit $rc)"
}

gate_all_controls() {
  local row m n ell ignored file draw
  require_existing identity-m4-n12-l5-d24
  for row in "${CELLS[@]}"; do
    IFS=: read -r m n ell ignored _ <<<"$row"
    file=$(cell_file "$m" "$n" "$ell")
    while IFS= read -r draw; do
      require_existing "control-m$m-n$n-l$ell-d$draw"
    done < <(selected_draws "$file" sat 2)
  done
}

next_attempt_dir() {
  local logical=$1 base expected=1 path name number
  base=$RUNS/$logical/attempts
  mkdir -p "$base"
  shopt -s nullglob
  for path in "$base"/*; do
    [[ -d $path ]] || die "non-directory entry in attempt set: $path"
    name=${path##*/}
    [[ $name =~ ^[0-9]{4}$ ]] || die "non-canonical attempt directory: $path"
    number=$((10#$name))
    [[ $number -eq $expected ]] || die "attempt directory gap under $base"
    expected=$((expected + 1))
  done
  shopt -u nullglob
  [[ $expected -le $MAX_ATTEMPTS ]] || return 76
  printf -v name '%04d' "$expected"
  path=$base/$name
  mkdir "$path" || die "could not allocate append-only attempt $path"
  printf '%s\n' "$path"
}

run_closure_once() {
  local logical=$1 file=$2 draw=$3 mode=$4 max_cols=$5
  local ckpt attempt_dir attempt_no started rc analyzer_rc input_sha
  local kernel arch cpu_model logical_cpus mem_kib mem_bytes
  local -a flags=()

  if analyzer_snapshot "$logical"; then analyzer_rc=0; else analyzer_rc=$?; fi
  if [[ $analyzer_rc -eq 0 ]]; then
    write_final_pointer "$logical" "$OUTPUT_ROOT/summary.json" "$analyzer_rc"
    echo "$logical is already terminal-successful"
    return 0
  fi
  if [[ $analyzer_rc -eq 76 ]]; then
    write_final_pointer "$logical" "$OUTPUT_ROOT/summary.json" "$analyzer_rc"
    echo "$logical is already terminal-unmeasured" >&2
    return 76
  fi
  if [[ $analyzer_rc -eq 65 ]]; then
    write_final_pointer "$logical" "$OUTPUT_ROOT/summary.json" "$analyzer_rc"
    return 65
  fi
  [[ $analyzer_rc -eq 75 ]] || return "$analyzer_rc"

  if attempt_dir=$(next_attempt_dir "$logical"); then rc=0; else rc=$?; fi
  if [[ $rc -eq 76 ]]; then
    echo "$logical exhausted $MAX_ATTEMPTS attempts and remains unmeasured" >&2
    return 76
  fi
  [[ $rc -eq 0 ]] || return "$rc"

  attempt_no=${attempt_dir##*/}
  ckpt=$CHECKPOINTS/$logical
  mkdir -p "$ckpt"
  [[ $mode == sat ]] && flags+=(--satisfiable)
  input_sha=$(sha256sum "$file" | awk '{print $1}')
  started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  kernel=$(uname -sr)
  arch=$(uname -m)
  cpu_model=$(awk -F ': ' '/^model name[[:space:]]*:/{print $2; exit}' /proc/cpuinfo)
  logical_cpus=$(getconf _NPROCESSORS_ONLN)
  mem_kib=$(awk '/^MemTotal:/{print $2; exit}' /proc/meminfo)
  mem_bytes=$((mem_kib * 1024))
  local satisfiable=false
  [[ $mode == sat ]] && satisfiable=true
  jq -n --arg logical_name "$logical" --arg attempt "$attempt_no" \
    --arg input "$file" --arg input_sha256 "$input_sha" --argjson draw "$draw" \
    --arg mode "$mode" --arg started_at "$started" --arg protocol_sha256 "$PROTOCOL_SHA" \
    --arg source_commit "$SOURCE_COMMIT" --arg ladder_sha256 "$LADDER_SHA" \
    --arg closure_sha256 "$CLOSURE_SHA" --arg analyzer_sha256 "$ANALYZER_SHA" \
    --arg container_image_digest "$BUILD_CONTAINER_DIGEST" \
    --arg rustc_version "$FROZEN_RUSTC_VERSION" --arg kernel "$kernel" --arg arch "$arch" \
    --arg cpu_model "$cpu_model" --argjson logical_cpus "$logical_cpus" \
    --argjson memory_bytes "$mem_bytes" --arg checkpoint_dir "$ckpt" \
    --argjson max_cols "$max_cols" \
    --argjson satisfiable "$satisfiable" '
      {
        schema:"dreg.high-arity-attempt/v1",logical_name:$logical_name,
        attempt:$attempt,input:{path:$input,sha256:$input_sha256},draw:$draw,mode:$mode,
        started_at:$started_at,protocol_sha256:$protocol_sha256,source_commit:$source_commit,
        binaries:{ladder:$ladder_sha256,closure:$closure_sha256,analyzer:$analyzer_sha256},
        build:{container_image_digest:$container_image_digest,rustc_version:$rustc_version},
        host:{kernel:$kernel,arch:$arch,cpu_model:$cpu_model,
          logical_cpus:$logical_cpus,memory_bytes:$memory_bytes},
        limits:{address_space_bytes:32000000000,timeout_seconds:86400,
          term_grace_seconds:60,max_cols:$max_cols,degree_to:4},
        checkpoint_dir:$checkpoint_dir,parent_environment_cleared:true,
        sanitized_env:{PATH:"/usr/bin:/bin",F4_F2_MAX_ROWS:"50000000",
          F4_F2_MAX_COLS:"50000000",RAYON_NUM_THREADS:"4",DREG_CKPT_SECONDS:"300"},
        argv:((if $satisfiable then ["--satisfiable"] else [] end) +
          ["--to","4","--max-cols",($max_cols|tostring),"--draw",($draw|tostring),
           "--checkpoint-dir",$checkpoint_dir,$input])
      }
    ' >"$attempt_dir/meta.json"

  if (
    cd "$REPO"
    prlimit --as=32000000000 -- \
      timeout --signal=TERM --kill-after=60s 86400s \
      "${SCIENTIFIC_ENV[@]}" "$CLOSURE" "${flags[@]}" \
      --to 4 --max-cols "$max_cols" --draw "$draw" \
      --checkpoint-dir "$ckpt" "$file"
  ) >"$attempt_dir/out.jsonl" 2>"$attempt_dir/stderr.log"; then
    rc=0
  else
    rc=$?
  fi
  printf '%s\n' "$rc" >"$attempt_dir/status"
  jq -n --arg finished_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" --argjson exit_code "$rc" \
    '{finished_at:$finished_at,exit_code:$exit_code}' >"$attempt_dir/finished.json"

  if (
    cd "$REPO"
    "${SCIENTIFIC_ENV[@]}" "$ANALYZER" "$DRAWS" "$OUTPUT_ROOT" --require "$logical"
  ) >"$attempt_dir/analysis.json" 2>"$attempt_dir/analyzer.stderr"; then
    analyzer_rc=0
  else
    analyzer_rc=$?
  fi
  printf '%s\n' "$analyzer_rc" >"$attempt_dir/analyzer.status"
  cp -- "$attempt_dir/analysis.json" "$OUTPUT_ROOT/.summary.tmp.$$"
  mv -f -- "$OUTPUT_ROOT/.summary.tmp.$$" "$OUTPUT_ROOT/summary.json"

  if [[ $analyzer_rc -eq 0 || $analyzer_rc -eq 65 || $analyzer_rc -eq 76 ]]; then
    write_final_pointer "$logical" "$attempt_dir/analysis.json" "$analyzer_rc"
  fi

  case $analyzer_rc in
    0)
      echo "$logical passed its frozen terminal classifier"
      ;;
    75)
      echo "$logical attempt $attempt_no retained; an infrastructure retry remains" >&2
      ;;
    76)
      echo "$logical is terminal-unmeasured under the frozen cap or retry limit" >&2
      ;;
    *)
      echo "$logical is malformed, contradictory, or invalid (analyzer exit $analyzer_rc)" >&2
      ;;
  esac
  return "$analyzer_rc"
}

generate_cell() {
  local m=$1 n=$2 ell=$3 final tmp logtmp metatmp rc started finished
  local kernel arch cpu_model logical_cpus mem_kib mem_bytes
  registered_cell "$m" "$n" "$ell" || die "generator step names an unregistered cell"
  require_durable_draws_on_ephemeral
  mkdir -p "$DRAWS"
  final=$(cell_file "$m" "$n" "$ell")
  if [[ -e $final ]]; then
    audit_draw_file "$final" "$m" "$n" "$ell" ||
      die "existing draw file is invalid and will not be overwritten: $final"
    validate_generation_provenance "$final" "$m" "$n" "$ell" ||
      die "existing draw provenance is missing or mismatched: ${final%.jsonl}.generation.json"
    echo "validated existing $final"
    return 0
  fi
  [[ ! -e ${final%.jsonl}.log && ! -e ${final%.jsonl}.generation.json ]] ||
    die "orphaned generation sidecar exists; retain and reconcile it before retrying"
  tmp=$(mktemp "$DRAWS/.m$m-n$n-l$ell.jsonl.attempt.XXXXXX")
  logtmp=$tmp.log
  metatmp=$tmp.meta.json
  started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  if (
    cd "$REPO"
    "${SCIENTIFIC_ENV[@]}" "$LADDER" --cells "$n:$ell:3" --m "$m" \
      --unsat 4 --min-sat 2 --ffd-max 3 --seed "$SEED" --max-draws 4096
  ) >"$tmp" 2>"$logtmp"; then
    rc=0
  else
    rc=$?
  fi
  [[ $rc -eq 0 ]] || die "draw generation failed for m=$m n=$n ell=$ell; retained $tmp"
  audit_draw_file "$tmp" "$m" "$n" "$ell" ||
    die "generated draw prefix failed its gate; retained $tmp"
  finished=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  kernel=$(uname -sr)
  arch=$(uname -m)
  cpu_model=$(awk -F ': ' '/^model name[[:space:]]*:/{print $2; exit}' /proc/cpuinfo)
  logical_cpus=$(getconf _NPROCESSORS_ONLN)
  mem_kib=$(awk '/^MemTotal:/{print $2; exit}' /proc/meminfo)
  mem_bytes=$((mem_kib * 1024))
  jq -n --arg cell "m$m-n$n-l$ell" --arg started_at "$started" --arg finished_at "$finished" \
    --arg protocol_sha256 "$PROTOCOL_SHA" --arg source_commit "$SOURCE_COMMIT" \
    --arg ladder_sha256 "$LADDER_SHA" --arg container_image_digest "$BUILD_CONTAINER_DIGEST" \
    --arg rustc_version "$FROZEN_RUSTC_VERSION" --arg kernel "$kernel" --arg arch "$arch" \
    --arg cpu_model "$cpu_model" --argjson logical_cpus "$logical_cpus" \
    --argjson memory_bytes "$mem_bytes" --argjson m "$m" --argjson n "$n" \
    --argjson ell "$ell" --argjson seed "$SEED" '
      {schema:"dreg.high-arity-generation/v1",cell:$cell,started_at:$started_at,
       finished_at:$finished_at,protocol_sha256:$protocol_sha256,source_commit:$source_commit,
       binary_sha256:$ladder_sha256,
       build:{container_image_digest:$container_image_digest,rustc_version:$rustc_version},
       host:{kernel:$kernel,arch:$arch,cpu_model:$cpu_model,
         logical_cpus:$logical_cpus,memory_bytes:$memory_bytes},
       parent_environment_cleared:true,
       sanitized_env:{PATH:"/usr/bin:/bin",F4_F2_MAX_ROWS:"50000000",
         F4_F2_MAX_COLS:"50000000",RAYON_NUM_THREADS:"4",DREG_CKPT_SECONDS:"300"},
       argv:["--cells",(($n|tostring)+":"+($ell|tostring)+":3"),"--m",($m|tostring),
         "--unsat","4","--min-sat","2","--ffd-max","3","--seed",($seed|tostring),
         "--max-draws","4096"]}
    ' >"$metatmp"
  mv -- "$logtmp" "${final%.jsonl}.log"
  mv -- "$metatmp" "${final%.jsonl}.generation.json"
  mv -- "$tmp" "$final"
  echo "generated and validated $final"
}

run_named_step() {
  local logical=$1 kind m n ell draw file
  if [[ $logical == identity-m4-n12-l5-d24 ]]; then
    audit_all_draws
    file=$REPO/research/dreg_closure_driver_20261016/draws/m4-12-5.jsonl
    run_closure_once "$logical" "$file" 24 unsat 155000
    return
  fi

  if [[ $logical =~ ^(control|unsat)-m([0-9]+)-n([0-9]+)-l([0-9]+)-d(0|[1-9][0-9]*)$ ]]; then
    kind=${BASH_REMATCH[1]}
    m=${BASH_REMATCH[2]}
    n=${BASH_REMATCH[3]}
    ell=${BASH_REMATCH[4]}
    draw=${BASH_REMATCH[5]}
  else
    usage
  fi
  registered_cell "$m" "$n" "$ell" || die "step names an unregistered cell"
  audit_all_draws
  file=$(cell_file "$m" "$n" "$ell")
  if [[ $kind == control ]]; then
    draw_is_selected "$file" sat "$draw" || die "control is not one of the first two SAT draws"
    require_existing identity-m4-n12-l5-d24
    run_closure_once "$logical" "$file" "$draw" sat 400000
  else
    draw_is_selected "$file" unsat "$draw" || die "measurement is not one of the first four UNSAT draws"
    gate_all_controls
    run_closure_once "$logical" "$file" "$draw" unsat 400000
  fi
}

verify_bindings
case "$STEP" in
  gen-m5-n11-l2)
    generate_cell 5 11 2
    ;;
  gen-m5-n13-l3)
    generate_cell 5 13 3
    ;;
  gen-m6-n11-l2)
    generate_cell 6 11 2
    ;;
  audit-draws)
    audit_all_draws
    ;;
  analyze)
    audit_all_draws
    prepare_output
    if analyzer_snapshot; then rc=0; else rc=$?; fi
    seal_terminal_rows_from_summary "$OUTPUT_ROOT/summary.json"
    exit "$rc"
    ;;
  identity-m4-n12-l5-d24|control-*|unsat-*)
    prepare_output
    run_named_step "$STEP"
    ;;
  *)
    usage
    ;;
esac
