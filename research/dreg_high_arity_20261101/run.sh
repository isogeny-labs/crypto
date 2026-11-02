#!/usr/bin/env bash
# Frozen, append-only orchestration for protocol.json. Scientific
# classification belongs to dreg_high_arity_analyze, never to this shell.
set -euo pipefail

usage() {
  cat >&2 <<'EOF'
usage: run.sh LADDER_BIN CLOSURE_BIN ANALYZER_BIN REPORT_BIN STEP
       run.sh --verify-source-tree-only REPO SOURCE_COMMIT SOURCE_MANIFEST
       run.sh --verify-dep-info-only REPO DEP_INFO [DEP_INFO ...]
       run.sh --verify-binary-manifest-only MANIFEST LADDER_SHA CLOSURE_SHA ANALYZER_SHA REPORT_SHA

STEP is one of:
  gen-m5-n11-l2
  gen-m5-n13-l3
  gen-m6-n11-l2
  audit-draws
  seal-resume-anchor
  analyze
  identity-m4-n12-l5-d24
  control-m{5,6}-n{11,13}-l{2,3}-dDRAW
  unsat-m{5,6}-n{11,13}-l{2,3}-dDRAW

Only the three registered cells m5-n11-l2, m5-n13-l3, and m6-n11-l2
are accepted. A closure STEP runs exactly one closure subprocess.
EOF
  exit 2
}

SOURCE_TREE_ONLY=false
DEP_INFO_ONLY=false
BINARY_MANIFEST_ONLY=false
if [[ ${1:-} == --verify-source-tree-only ]]; then
  [[ $# -eq 4 ]] || usage
  SOURCE_TREE_REPO=$2
  SOURCE_TREE_COMMIT=$3
  SOURCE_TREE_MANIFEST=$4
  SOURCE_TREE_ONLY=true
elif [[ ${1:-} == --verify-dep-info-only ]]; then
  [[ $# -ge 3 ]] || usage
  DEP_INFO_REPO=$2
  DEP_INFO_FILES=("${@:3}")
  DEP_INFO_ONLY=true
elif [[ ${1:-} == --verify-binary-manifest-only ]]; then
  [[ $# -eq 6 ]] || usage
  BINARY_MANIFEST_OVERRIDE=$2
  BINARY_MANIFEST_HASHES=("${@:3}")
  BINARY_MANIFEST_ONLY=true
else
  [[ $# -eq 5 ]] || usage
  LADDER_ARG=$1
  CLOSURE_ARG=$2
  ANALYZER_ARG=$3
  REPORT_ARG=$4
  STEP=$5
fi

HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
PROTOCOL=$HERE/protocol.json
SOURCE_MANIFEST=$HERE/sources.sha256
if [[ $BINARY_MANIFEST_ONLY == true ]]; then
  BINARY_MANIFEST=$BINARY_MANIFEST_OVERRIDE
else
  BINARY_MANIFEST=$HERE/binary.sha256
fi
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

# Every repository-local Rust package that the four frozen examples compile is
# part of the scientific source identity. Directory pathspecs deliberately
# reject both changes to existing files and newly added source files.
FROZEN_SOURCE_PATHS=(
  Cargo.toml
  Cargo.lock
  build.rs
  src
  crates/ecdl-tensor-ic/Cargo.toml
  crates/ecdl-tensor-ic/src
  crates/hashed-ecdl-tensor/Cargo.toml
  crates/hashed-ecdl-tensor/src
  tools/factor-base/Cargo.toml
  tools/factor-base/Cargo.lock
  tools/factor-base/src
  isogeny_algos/Cargo.toml
  isogeny_algos/src
  research/pencil_large_prime_20261029/Cargo.toml
  research/pencil_large_prime_20261029/src
  tools/volcano-cli/Cargo.toml
  tools/volcano-cli/src
  docs/curves/registry.json
  docs/curves/standards/parameters.json
  docs/ecbench/schema.sql
  docs/ic/boundary_targets.json
  docs/ic/calibration.json
  examples/dreg_ladder.rs
  examples/dreg_closure.rs
  examples/dreg_high_arity_analyze.rs
  examples/dreg_high_arity_report.rs
  research/dreg_high_arity_20261101/run.sh
  research/dreg_high_arity_20261101/test-source-bindings.sh
  research/dreg_high_arity_20261101/REPORT_EXPORT_CONTRACT.md
  research/dreg_high_arity_20261101/sources.sha256
)

# These files are populated only after the immutable source commit and binary
# build exist. They must be clean at execution time, but cannot equal the
# earlier source commit by construction.
FROZEN_POST_SOURCE_PATHS=(
  research/dreg_high_arity_20261101/binary.sha256
  research/dreg_high_arity_20261101/protocol.json
)

die() {
  echo "error: $*" >&2
  exit 1
}

atomic_install() {
  local source=$1 destination=$2 mode=${3:-644} directory base temporary
  directory=${destination%/*}
  base=${destination##*/}
  mkdir -p "$directory"
  temporary=$(mktemp "$directory/.$base.tmp.XXXXXX")
  if ! cp -- "$source" "$temporary"; then
    unlink "$temporary"
    return 1
  fi
  chmod "$mode" "$temporary"
  if ln -- "$temporary" "$destination" 2>/dev/null; then
    unlink "$temporary"
  else
    unlink "$temporary"
    return 1
  fi
}

link_install() {
  local source=$1 destination=$2 mode=${3:-644}
  chmod "$mode" "$source"
  if ln -- "$source" "$destination" 2>/dev/null; then
    unlink "$source"
  else
    return 1
  fi
}

write_tree_manifest() {
  local root=$1 output=$2 scope=$3 relative digest base
  (
    cd "$root"
    while IFS= read -r -d '' relative; do
      relative=${relative#./}
      [[ $relative != *$'\n'* && $relative != *$'\t'* && $relative != *'\\'* &&
         $relative != ' '* && $relative != *' ' ]] || {
        echo "unsafe filename in resume tree: $relative" >&2
        exit 1
      }
      if [[ $scope == immutable ]]; then
        case "$relative" in
          runs/*/attempts/[0-9][0-9][0-9][0-9]/*|runs/*/final.json|draws/*|resume-import-*) ;;
          *) continue ;;
        esac
        base=${relative##*/}
        [[ $base != .*.tmp.* ]] || continue
      fi
      digest=$(sha256sum -- "$relative" | awk '{print $1}') || exit 1
      printf '%s  %s\n' "$digest" "$relative"
    done < <(find . -type f -print0 | LC_ALL=C sort -z)
  ) >"$output"
}

write_attempt_evidence_manifest() {
  local run_dir=$1 output=$2 relative digest base
  (
    cd "$run_dir"
    if [[ ! -d attempts ]]; then
      exit 0
    fi
    while IFS= read -r -d '' relative; do
      relative=${relative#./}
      base=${relative##*/}
      [[ $base != .*.tmp.* ]] || continue
      [[ $relative != *$'\n'* && $relative != *$'\t'* && $relative != *'\\'* &&
         $relative != ' '* && $relative != *' ' ]] || exit 1
      digest=$(sha256sum -- "$relative" | awk '{print $1}') || exit 1
      printf '%s  %s\n' "$digest" "$relative"
    done < <(find attempts -type f -print0 | LC_ALL=C sort -z)
  ) >"$output"
}

termination_for_status() {
  local status=$1 stderr_path=${2:-}
  if [[ $status -eq 0 ]]; then
    printf '%s\n' success
  elif [[ $status -eq 125 ]]; then
    printf '%s\n' checkpoint_invalid
  elif [[ $status -eq 124 ]]; then
    printf '%s\n' timeout
  elif [[ $status -eq 137 ]]; then
    printf '%s\n' signal_kill
  elif [[ $status -eq 143 ]]; then
    printf '%s\n' signal_term
  elif [[ $status -eq 134 && -f $stderr_path ]] &&
    grep -Eq '^[[:space:]]*memory allocation of [0-9]+ bytes failed[[:space:]]*$' "$stderr_path"; then
    printf '%s\n' address_space
  else
    printf '%s\n' nonzero_exit
  fi
}

verify_checkpoint_generation() {
  local manifest=$1 directory generation=$2 entry path expected_sha expected_bytes
  local base listed manifest_spec catalog_spec actual=0
  directory=${manifest%/*}
  [[ -f $manifest && ! -L $manifest ]] || return 1
  jq -e --argjson generation "$generation" '
    (keys | sort) == (["committed","files","generation","job_id","schema","spec_sha256"] | sort) and
    .schema == "spot.checkpoint/v1" and .job_id == "dreg-closure" and
    .committed == true and .generation == $generation and .generation >= 1 and
    (.spec_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
    (.files | type == "array" and length >= 1) and
    ((.files | map(.path)) as $paths |
      ($paths | length) == ($paths | unique | length) and
      ($paths | index("catalog.json")) != null) and
    all(.files[];
      (keys | sort) == (["bytes","path","sha256"] | sort) and
      (.path | type == "string" and test("^[A-Za-z0-9][A-Za-z0-9._-]*$") and
        (contains("..") | not)) and
      (.sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
      (.bytes | type == "number" and floor == . and . >= 0))
  ' "$manifest" >/dev/null || return 1
  while IFS=$'\t' read -r path expected_sha expected_bytes; do
    [[ -n $path && -n $expected_sha && -n $expected_bytes ]] || return 1
    entry=$directory/$path
    [[ -f $entry && ! -L $entry ]] || return 1
    [[ $(stat -c %s -- "$entry") == "$expected_bytes" ]] || return 1
    [[ $(sha256sum -- "$entry" | awk '{print $1}') == "$expected_sha" ]] || return 1
  done < <(jq -r '.files[] | [.path,.sha256,(.bytes|tostring)] | @tsv' "$manifest")
  manifest_spec=$(jq -er '.spec_sha256' "$manifest") || return 1
  catalog_spec=$(jq -er '.system_sha256 | select(type == "string")' \
    "$directory/catalog.json") || return 1
  [[ $catalog_spec == "$manifest_spec" ]] || return 1
  listed=$(jq -er '.files | length' "$manifest") || return 1
  while IFS= read -r -d '' entry; do
    base=${entry##*/}
    [[ -f $entry && ! -L $entry ]] || return 1
    jq -e --arg path "$base" 'any(.files[]; .path == $path)' "$manifest" >/dev/null || return 1
    actual=$((actual + 1))
  done < <(find "$directory" -mindepth 1 -maxdepth 1 \
    ! -name checkpoint-manifest.json -print0)
  [[ $actual -eq $listed ]]
}

select_checkpoint_generation() {
  local checkpoint_root=$1 candidate directory name generation saw_committed=false
  CHECKPOINT_RESTORE_MANIFEST=
  CHECKPOINT_RESTORE_GENERATION=
  while IFS= read -r -d '' candidate; do
    saw_committed=true
    directory=${candidate%/*}
    name=${directory##*/}
    if [[ ! $name =~ ^gen-([0-9]{20})$ ]]; then
      echo "checkpoint: ignoring malformed committed-generation path $candidate" >&2
      continue
    fi
    # Keep this as an exact canonical decimal string. Bash arithmetic is
    # signed and would wrap valid native u64 generations above i64::MAX.
    # Removing the fixed-width zero padding makes the value valid JSON while
    # preserving every u64 digit for jq and the append-only evidence record.
    generation=${BASH_REMATCH[1]}
    while [[ $generation == 0* ]]; do
      generation=${generation#0}
    done
    [[ -n $generation ]] || continue
    if verify_checkpoint_generation "$candidate" "$generation"; then
      CHECKPOINT_RESTORE_MANIFEST=$candidate
      CHECKPOINT_RESTORE_GENERATION=$generation
      return 0
    fi
  done < <(find "$checkpoint_root" -mindepth 2 -maxdepth 2 -type f \
    -name checkpoint-manifest.json -print0 | LC_ALL=C sort -zr)
  if [[ $saw_committed == true ]]; then
    # The native closure owns the system-identity check and the definitive
    # checkpoint parser.  Let it inspect the retained store and return the
    # registered resource exit 125.  That exit is captured as an append-only
    # attempt and can never become invalid scientific evidence or suppress a
    # later candidate merely because checkpoint custody was damaged.
    echo "checkpoint: no committed generation passed the shell custody preflight; retaining the store for native exit 125" >&2
  fi
  return 0
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

verify_binary_manifest() {
  local expected_hashes=("$@")
  local expected_labels=(
    dreg_ladder
    dreg_closure
    dreg_high_arity_analyze
    dreg_high_arity_report
  )
  local index size
  [[ ${#expected_hashes[@]} -eq ${#expected_labels[@]} ]] || return 1
  [[ -f $BINARY_MANIFEST && ! -L $BINARY_MANIFEST ]] || return 1
  size=$(stat -c %s -- "$BINARY_MANIFEST") || return 1
  ((size <= 4096)) || return 1

  for index in "${!expected_labels[@]}"; do
    is_hex 64 "${expected_hashes[$index]}" || return 1
  done
  cmp -s -- "$BINARY_MANIFEST" <(
    for index in "${!expected_labels[@]}"; do
      printf '%s  %s\n' "${expected_hashes[$index]}" "${expected_labels[$index]}"
    done
  )
}

protocol_body_hash() {
  local canonical
  canonical=$(jq -cS 'del(.freeze)' "$PROTOCOL") || return 1
  printf '%s' "$canonical" | sha256sum | awk '{print $1}'
}

verify_source_tree() {
  local repository=$1 source_commit=$2 source_manifest=$3
  repository=$(realpath -e "$repository") || return 1
  source_manifest=$(realpath -e "$source_manifest") || return 1
  is_hex 40 "$source_commit" || return 1

  (
    cd "$repository"
    sha256sum -c "$source_manifest" || exit 1
    git cat-file -e "$source_commit^{commit}" || exit 1
    git merge-base --is-ancestor "$source_commit" HEAD || exit 1
    git diff --quiet || exit 1
    git diff --cached --quiet || exit 1
    git diff --quiet "$source_commit" -- "${FROZEN_SOURCE_PATHS[@]}" || exit 1
    [[ -z $(git status --porcelain --untracked-files=all -- \
      "${FROZEN_SOURCE_PATHS[@]}" "${FROZEN_POST_SOURCE_PATHS[@]}") ]] || exit 1
  )
}

source_path_registered() {
  local candidate=$1 registered
  for registered in "${FROZEN_SOURCE_PATHS[@]}"; do
    if [[ $candidate == "$registered" || $candidate == "$registered/"* ]]; then
      return 0
    fi
  done
  return 1
}

verify_dep_info_coverage() {
  local repository=$1 dep_info token relative missing=0
  shift
  repository=$(realpath -e "$repository") || return 1
  for dep_info in "$@"; do
    dep_info=$(realpath -e "$dep_info") || return 1
    while IFS= read -r token; do
      [[ $token == "$repository/"* ]] || continue
      relative=${token#"$repository/"}
      if ! source_path_registered "$relative"; then
        echo "unregistered repository input in $dep_info: $relative" >&2
        missing=1
      fi
    done < <(tr '[:space:]' '\n' <"$dep_info")
  done
  ((missing == 0))
}

verify_bindings() {
  local remote_oid remote_lines
  [[ -s $PROTOCOL && -s $SOURCE_MANIFEST && -s $BINARY_MANIFEST ]] ||
    die "protocol, sources.sha256, and binary.sha256 must all exist"

  LADDER=$(realpath -e "$LADDER_ARG") || die "missing ladder binary: $LADDER_ARG"
  CLOSURE=$(realpath -e "$CLOSURE_ARG") || die "missing closure binary: $CLOSURE_ARG"
  ANALYZER=$(realpath -e "$ANALYZER_ARG") || die "missing analyzer binary: $ANALYZER_ARG"
  REPORT=$(realpath -e "$REPORT_ARG") || die "missing report binary: $REPORT_ARG"
  [[ -x $LADDER && -x $CLOSURE && -x $ANALYZER && -x $REPORT ]] ||
    die "all four frozen native binaries must be executable"

  LADDER_SHA=$(sha256sum "$LADDER" | awk '{print $1}')
  CLOSURE_SHA=$(sha256sum "$CLOSURE" | awk '{print $1}')
  ANALYZER_SHA=$(sha256sum "$ANALYZER" | awk '{print $1}')
  REPORT_SHA=$(sha256sum "$REPORT" | awk '{print $1}')
  LADDER_SOURCE_SHA=$(sha256sum "$REPO/examples/dreg_ladder.rs" | awk '{print $1}')
  CLOSURE_SOURCE_SHA=$(sha256sum "$REPO/examples/dreg_closure.rs" | awk '{print $1}')
  ANALYZER_SOURCE_SHA=$(sha256sum "$REPO/examples/dreg_high_arity_analyze.rs" | awk '{print $1}')
  REPORT_SOURCE_SHA=$(sha256sum "$REPO/examples/dreg_high_arity_report.rs" | awk '{print $1}')
  verify_binary_manifest "$LADDER_SHA" "$CLOSURE_SHA" "$ANALYZER_SHA" "$REPORT_SHA" ||
    die "binary manifest is not the exact canonical ordered four-row identity"

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
    --arg closure "$CLOSURE_SHA" --arg analyzer "$ANALYZER_SHA" --arg report "$REPORT_SHA" \
    --arg ladder_source "$LADDER_SOURCE_SHA" --arg closure_source "$CLOSURE_SOURCE_SHA" \
    --arg analyzer_source "$ANALYZER_SOURCE_SHA" --arg report_source "$REPORT_SOURCE_SHA" \
    --arg container "$BUILD_CONTAINER_DIGEST" --arg rustc "$FROZEN_RUSTC_VERSION" '
      (.arms | length == 5) and
      all(.arms[];
        .instrument.commit == $commit and
        .instrument.binary_sha256 == $closure and
        .instrument.flags.ladder_binary_sha256 == $ladder and
        .instrument.flags.analyzer_binary_sha256 == $analyzer and
        .instrument.flags.report_binary_sha256 == $report and
        .instrument.flags.ladder_source_sha256 == $ladder_source and
        .instrument.flags.closure_source_sha256 == $closure_source and
        .instrument.flags.analyzer_source_sha256 == $analyzer_source and
        .instrument.flags.report_source_sha256 == $report_source and
        .instrument.flags.container_image_digest == $container and
        .instrument.flags.rustc_version == $rustc
      ) and
      ($container | test("^sha256:[0-9a-f]{64}$")) and
      ($rustc | length > 0) and
      .analysis_plan.script.path == "../../examples/dreg_high_arity_analyze.rs" and
      .analysis_plan.script.sha256 == $analyzer_source
    ' "$PROTOCOL" >/dev/null || die "protocol instrument identity is incomplete or mismatched"

  verify_source_tree "$REPO" "$SOURCE_COMMIT" "$SOURCE_MANIFEST" ||
    die "current transitive Rust source tree differs from the frozen commit"

  [[ -n ${DREG_EXECUTION_REF:-} ]] ||
    die "scientific execution requires DREG_EXECUTION_REF=refs/heads/<pushed-branch>"
  [[ $DREG_EXECUTION_REF == refs/heads/* ]] ||
    die "DREG_EXECUTION_REF must be a fully qualified refs/heads/... ref"
  git -C "$REPO" check-ref-format "$DREG_EXECUTION_REF" >/dev/null ||
    die "invalid DREG_EXECUTION_REF: $DREG_EXECUTION_REF"
  EXECUTION_COMMIT=$(git -C "$REPO" rev-parse HEAD) || die "cannot resolve execution HEAD"
  remote_lines=$(git -C "$REPO" ls-remote --refs origin "$DREG_EXECUTION_REF") ||
    die "cannot verify live pushed execution ref $DREG_EXECUTION_REF"
  [[ $(printf '%s\n' "$remote_lines" | sed '/^$/d' | wc -l) -eq 1 ]] ||
    die "execution ref does not uniquely exist on origin: $DREG_EXECUTION_REF"
  remote_oid=$(printf '%s\n' "$remote_lines" | awk 'NR == 1 {print $1}')
  [[ $remote_oid == "$EXECUTION_COMMIT" ]] ||
    die "origin $DREG_EXECUTION_REF is $remote_oid, not execution HEAD $EXECUTION_COMMIT"
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
    local resume durable anchor stamp prefix copy_source immutable_source copy_destination
    local copy_name immutable_name anchor_name record_name copy_sha immutable_sha anchor_sha
    local copy_count immutable_count anchor_count record_tmp
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
    [[ -n ${DREG_RESUME_ANCHOR:-} ]] ||
      die "resume import requires an external DREG_RESUME_ANCHOR immutable manifest"
    anchor=$(realpath -e "$DREG_RESUME_ANCHOR") || die "resume anchor does not exist"
    [[ -f $anchor && ! -L $anchor ]] || die "resume anchor is not an ordinary file"
    [[ $anchor != "$resume" && $anchor != "$resume/"* &&
       $anchor != "$OUTPUT_ROOT" && $anchor != "$OUTPUT_ROOT/"* &&
       $anchor != "$REPO" && $anchor != "$REPO/"* ]] ||
      die "resume anchor must be external to resume, destination, and repository trees"
    copy_source=$(mktemp)
    immutable_source=$(mktemp)
    copy_destination=$(mktemp)
    write_tree_manifest "$resume" "$copy_source" all || die "cannot hash resume source tree"
    write_tree_manifest "$resume" "$immutable_source" immutable ||
      die "cannot hash immutable resume evidence"
    cmp -s "$anchor" "$immutable_source" ||
      die "external resume anchor differs from the immutable source evidence tree"
    mkdir -p "$OUTPUT_ROOT"
    cp -a -- "$resume"/. "$OUTPUT_ROOT"/
    write_tree_manifest "$OUTPUT_ROOT" "$copy_destination" all ||
      die "cannot hash imported destination tree"
    cmp -s "$copy_source" "$copy_destination" ||
      die "resume copy differs from its source manifest"

    stamp=$(date -u +%Y%m%dT%H%M%S%NZ)
    prefix=resume-import-$stamp
    copy_name=$prefix.copy.sha256
    immutable_name=$prefix.immutable.sha256
    anchor_name=$prefix.anchor.sha256
    record_name=$prefix.json
    [[ ! -e $OUTPUT_ROOT/$copy_name && ! -e $OUTPUT_ROOT/$immutable_name &&
       ! -e $OUTPUT_ROOT/$anchor_name && ! -e $OUTPUT_ROOT/$record_name ]] ||
      die "resume provenance name collision"
    copy_sha=$(sha256sum "$copy_source" | awk '{print $1}')
    immutable_sha=$(sha256sum "$immutable_source" | awk '{print $1}')
    anchor_sha=$(sha256sum "$anchor" | awk '{print $1}')
    copy_count=$(wc -l <"$copy_source")
    immutable_count=$(wc -l <"$immutable_source")
    anchor_count=$(wc -l <"$anchor")
    [[ $anchor_sha == "$immutable_sha" && $anchor_count -eq $immutable_count ]] ||
      die "external resume anchor identity differs from the immutable source manifest"
    atomic_install "$copy_source" "$OUTPUT_ROOT/$copy_name" ||
      die "cannot install resume copy manifest"
    atomic_install "$immutable_source" "$OUTPUT_ROOT/$immutable_name" ||
      die "cannot install immutable resume manifest"
    atomic_install "$anchor" "$OUTPUT_ROOT/$anchor_name" ||
      die "cannot install external resume anchor"
    record_tmp=$(mktemp)
    jq -n --arg source "$resume" --arg destination "$OUTPUT_ROOT" \
      --arg imported_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
      --arg copy_path "$copy_name" --arg copy_sha256 "$copy_sha" \
      --argjson copy_file_count "$copy_count" \
      --arg immutable_path "$immutable_name" --arg immutable_sha256 "$immutable_sha" \
      --argjson immutable_file_count "$immutable_count" \
      --arg anchor_path "$anchor_name" --arg anchor_sha256 "$anchor_sha" \
      --argjson anchor_file_count "$anchor_count" '
        {schema:"dreg.resume-import/v3",source:$source,destination:$destination,
         imported_at:$imported_at,
         copy_manifest:{path:$copy_path,sha256:$copy_sha256,file_count:$copy_file_count},
         immutable_manifest:{path:$immutable_path,sha256:$immutable_sha256,
           file_count:$immutable_file_count},
         anchor_manifest:{path:$anchor_path,sha256:$anchor_sha256,
           file_count:$anchor_file_count}}
      ' >"$record_tmp"
    atomic_install "$record_tmp" "$OUTPUT_ROOT/$record_name" ||
      die "cannot install resume provenance record"
    unlink "$copy_source"
    unlink "$immutable_source"
    unlink "$copy_destination"
    unlink "$record_tmp"
    unset EPHEMERAL_RESUME_DIR
    unset DREG_RESUME_ANCHOR
  fi
  mkdir -p "$RUNS" "$CHECKPOINTS"
  [[ -z $(find "$OUTPUT_ROOT" -type l -print -quit) ]] ||
    die "output tree contains a symlink"
  [[ -z $(find "$OUTPUT_ROOT" ! -type d ! -type f -print -quit) ]] ||
    die "output tree contains a special file"
  validate_output_bindings
}

seal_resume_anchor() {
  local requested output parent temporary
  require_durable_output
  [[ -d $OUTPUT_ROOT ]] || die "cannot anchor a missing output root: $OUTPUT_ROOT"
  [[ -z $(find "$OUTPUT_ROOT" -type l -print -quit) ]] ||
    die "output tree contains a symlink"
  [[ -z $(find "$OUTPUT_ROOT" ! -type d ! -type f -print -quit) ]] ||
    die "output tree contains a special file"
  validate_output_bindings
  [[ -n ${DREG_RESUME_ANCHOR_OUT:-} && $DREG_RESUME_ANCHOR_OUT == /* ]] ||
    die "seal-resume-anchor requires an absolute DREG_RESUME_ANCHOR_OUT"
  requested=$DREG_RESUME_ANCHOR_OUT
  output=$(realpath -m "$requested")
  [[ $output != "$OUTPUT_ROOT" && $output != "$OUTPUT_ROOT/"* &&
     $output != "$REPO" && $output != "$REPO/"* ]] ||
    die "resume anchor output must be external to DREG_OUTPUT_ROOT and the repository"
  [[ ! -e $output ]] || die "refusing to overwrite resume anchor: $output"
  parent=$(dirname -- "$output")
  [[ -d $parent ]] || die "resume anchor parent does not exist: $parent"
  temporary=$(mktemp "$parent/.dreg-resume-anchor.tmp.XXXXXX")
  write_tree_manifest "$OUTPUT_ROOT" "$temporary" immutable || {
    unlink "$temporary"
    die "cannot construct immutable resume anchor"
  }
  chmod 644 "$temporary"
  if ln -- "$temporary" "$output" 2>/dev/null; then
    unlink "$temporary"
  else
    unlink "$temporary"
    die "could not atomically create resume anchor: $output"
  fi
  echo "sealed external resume anchor $output ($(sha256sum "$output" | awk '{print $1}'))"
}

validate_output_bindings() {
  local attempt meta logical number final receipt status_value expected_termination
  local import_record record_base record_stem copy_path immutable_path anchor_path
  local expected_hash expected_count actual_count attempt_commit checkpoint_restored
  local checkpoint_manifest_sha checkpoint_generation
  local evidence_manifest evidence_sha evidence_count
  while IFS= read -r -d '' import_record; do
    [[ -f $import_record && ! -L $import_record ]] || die "invalid resume record: $import_record"
    record_base=${import_record##*/}
    record_stem=${record_base%.json}
    jq -e '
      (keys | sort) == (["anchor_manifest","copy_manifest","destination","immutable_manifest","imported_at","schema","source"] | sort) and
      .schema == "dreg.resume-import/v3" and
      (.source | type == "string" and startswith("/")) and
      (.destination | type == "string" and startswith("/")) and
      (.imported_at | type == "string") and
      (.copy_manifest | keys | sort) == (["file_count","path","sha256"] | sort) and
      (.immutable_manifest | keys | sort) == (["file_count","path","sha256"] | sort) and
      (.anchor_manifest | keys | sort) == (["file_count","path","sha256"] | sort) and
      all(.copy_manifest,.immutable_manifest,.anchor_manifest;
        (.path | type == "string" and test("^resume-import-[0-9]{8}T[0-9]{6}[0-9]+Z\\.(copy|immutable|anchor)\\.sha256$")) and
        (.sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
        (.file_count | type == "number" and floor == . and . >= 0))
    ' "$import_record" >/dev/null || die "malformed resume provenance record: $import_record"
    [[ $(jq -er '.copy_manifest.path' "$import_record") == "$record_stem.copy.sha256" &&
       $(jq -er '.immutable_manifest.path' "$import_record") == "$record_stem.immutable.sha256" &&
       $(jq -er '.anchor_manifest.path' "$import_record") == "$record_stem.anchor.sha256" ]] ||
      die "resume manifest names do not match their record: $import_record"
    for receipt in copy_manifest immutable_manifest anchor_manifest; do
      copy_path=$(jq -er --arg receipt "$receipt" '.[$receipt].path' "$import_record")
      expected_hash=$(jq -er --arg receipt "$receipt" '.[$receipt].sha256' "$import_record")
      expected_count=$(jq -er --arg receipt "$receipt" '.[$receipt].file_count' "$import_record")
      [[ -f $OUTPUT_ROOT/$copy_path && ! -L $OUTPUT_ROOT/$copy_path ]] ||
        die "missing resume manifest bound by $import_record: $copy_path"
      [[ $(sha256sum "$OUTPUT_ROOT/$copy_path" | awk '{print $1}') == "$expected_hash" ]] ||
        die "resume manifest hash mismatch: $copy_path"
      actual_count=$(wc -l <"$OUTPUT_ROOT/$copy_path")
      [[ $actual_count -eq $expected_count ]] || die "resume manifest count mismatch: $copy_path"
    done
    immutable_path=$(jq -er '.immutable_manifest.path' "$import_record")
    anchor_path=$(jq -er '.anchor_manifest.path' "$import_record")
    cmp -s "$OUTPUT_ROOT/$immutable_path" "$OUTPUT_ROOT/$anchor_path" ||
      die "external resume anchor differs from retained immutable manifest: $import_record"
    (
      cd "$OUTPUT_ROOT"
      sha256sum -c --status "$immutable_path"
    ) || die "immutable imported evidence changed: $immutable_path"
  done < <(find "$OUTPUT_ROOT" -maxdepth 1 -type f -name 'resume-import-*.json' -print0)

  while IFS= read -r -d '' attempt; do
    meta=$attempt/meta.json
    logical=${attempt%/attempts/*}
    logical=${logical##*/}
    number=${attempt##*/}
    if [[ ! -e $meta ]]; then
      if [[ -e $attempt/allocated.json ]]; then
        jq -e --arg logical "$logical" --arg attempt "$number" '
          (keys | sort) == (["allocated_at","attempt","logical_name","schema"] | sort) and
          .schema == "dreg.high-arity-allocation/v1" and
          .logical_name == $logical and .attempt == $attempt and
          (.allocated_at | type == "string" and length > 0)
        ' "$attempt/allocated.json" >/dev/null ||
          die "invalid append-only attempt allocation marker: $attempt/allocated.json"
      fi
      [[ -z $(find "$attempt" -maxdepth 1 -type f ! -name '.*.tmp.*' \
        ! -name allocated.json -print -quit) ]] ||
        die "resumed attempt has output but no binding metadata: $attempt"
      continue
    fi
    [[ -f $attempt/allocated.json && ! -L $attempt/allocated.json ]] ||
      die "attempt metadata lacks its append-only allocation marker: $attempt"
    jq -e --arg logical "$logical" --arg attempt "$number" '
      (keys | sort) == (["allocated_at","attempt","logical_name","schema"] | sort) and
      .schema == "dreg.high-arity-allocation/v1" and
      .logical_name == $logical and .attempt == $attempt and
      (.allocated_at | type == "string" and length > 0)
    ' "$attempt/allocated.json" >/dev/null ||
      die "invalid append-only attempt allocation marker: $attempt/allocated.json"
    [[ -f $meta && ! -L $meta ]] || die "invalid attempt metadata file: $meta"
    jq -e --arg logical "$logical" --arg attempt "$number" \
      --arg protocol "$PROTOCOL_SHA" --arg commit "$SOURCE_COMMIT" \
      --arg ladder "$LADDER_SHA" --arg closure "$CLOSURE_SHA" \
      --arg analyzer "$ANALYZER_SHA" --arg report "$REPORT_SHA" \
      --arg container "$BUILD_CONTAINER_DIGEST" \
      --arg rustc "$FROZEN_RUSTC_VERSION" '
        .schema == "dreg.high-arity-attempt/v1" and
        .logical_name == $logical and .attempt == $attempt and
        .protocol_sha256 == $protocol and .source_commit == $commit and
        .binaries.ladder == $ladder and .binaries.closure == $closure and
        .binaries.analyzer == $analyzer and .binaries.report == $report and
        .build.container_image_digest == $container and .build.rustc_version == $rustc and
        (.execution.ref | type == "string" and startswith("refs/heads/")) and
        (.execution.commit | type == "string" and test("^[0-9a-f]{40}$")) and
        .parent_environment_cleared == true and
        .sanitized_env == {PATH:"/usr/bin:/bin",F4_F2_MAX_ROWS:"50000000",
          F4_F2_MAX_COLS:"50000000",RAYON_NUM_THREADS:"4",DREG_CKPT_SECONDS:"300"}
      ' "$meta" >/dev/null || die "resumed attempt binding mismatch: $meta"
    jq -e '
      (.checkpoint_restore | keys | sort) ==
        (["generation","manifest_json","manifest_sha256","restored"] | sort) and
      if .checkpoint_restore.restored == true then
        (.checkpoint_restore.generation | type == "number" and floor == . and . >= 1) and
        (.checkpoint_restore.manifest_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
        (.checkpoint_restore.manifest_json | type == "string" and length > 0)
      elif .checkpoint_restore.restored == false then
        .checkpoint_restore.generation == null and
        .checkpoint_restore.manifest_sha256 == null and
        .checkpoint_restore.manifest_json == null
      else false end
    ' "$meta" >/dev/null || die "invalid checkpoint restore binding: $meta"
    checkpoint_restored=$(jq -er '.checkpoint_restore.restored' "$meta")
    if [[ $checkpoint_restored == true ]]; then
      checkpoint_manifest_sha=$(jq -j '.checkpoint_restore.manifest_json' "$meta" |
        sha256sum | awk '{print $1}')
      [[ $checkpoint_manifest_sha == \
        "$(jq -er '.checkpoint_restore.manifest_sha256' "$meta")" ]] ||
        die "embedded checkpoint restore manifest hash mismatch: $meta"
      checkpoint_generation=$(jq -er '.checkpoint_restore.generation' "$meta")
      jq -e --argjson generation "$checkpoint_generation" '
        (.checkpoint_restore.manifest_json | fromjson) as $manifest |
        $manifest.schema == "spot.checkpoint/v1" and
        $manifest.job_id == "dreg-closure" and $manifest.committed == true and
        $manifest.generation == $generation
      ' "$meta" >/dev/null || die "embedded checkpoint restore manifest identity mismatch: $meta"
    fi
    attempt_commit=$(jq -er '.execution.commit' "$meta") ||
      die "resumed attempt lacks an execution commit: $meta"
    git -C "$REPO" cat-file -e "$attempt_commit^{commit}" &&
      git -C "$REPO" merge-base --is-ancestor "$SOURCE_COMMIT" "$attempt_commit" &&
      git -C "$REPO" merge-base --is-ancestor "$attempt_commit" "$EXECUTION_COMMIT" ||
      die "resumed attempt execution commit is not a retained pushed descendant: $meta"

    if [[ -e $attempt/status && ! -e $attempt/finished.json ]]; then
      die "attempt has status without its atomic completion receipt: $attempt"
    fi
    if [[ -e $attempt/finished.json ]]; then
      [[ -f $attempt/finished.json && ! -L $attempt/finished.json ]] ||
        die "invalid completion receipt file: $attempt/finished.json"
      [[ -f $attempt/out.jsonl && ! -L $attempt/out.jsonl &&
         -f $attempt/stderr.log && ! -L $attempt/stderr.log ]] ||
        die "completed attempt is missing retained stdout or stderr: $attempt"
      status_value=$(jq -er '.exit_code' "$attempt/finished.json") ||
        die "completion receipt lacks exit code: $attempt/finished.json"
      [[ $status_value =~ ^(0|[1-9][0-9]{0,2})$ && $status_value -le 255 ]] ||
        die "invalid completion receipt exit code: $attempt/finished.json"
      expected_termination=$(termination_for_status "$status_value" "$attempt/stderr.log")
      jq -e --argjson status "$status_value" --arg termination "$expected_termination" '
        (keys | sort) == (["exit_code","finished_at","schema","termination"] | sort) and
        .schema == "dreg.high-arity-finished/v1" and
        .exit_code == $status and .termination == $termination and
        (.finished_at | type == "string")
      ' "$attempt/finished.json" >/dev/null || die "invalid completion receipt: $attempt/finished.json"
    fi
    if [[ -e $attempt/status ]]; then
      [[ -f $attempt/status && ! -L $attempt/status ]] || die "invalid status file: $attempt/status"
      [[ $(wc -l <"$attempt/status") -eq 1 ]] || die "noncanonical status file: $attempt/status"
      [[ $(<"$attempt/status") == "$status_value" ]] ||
        die "invalid status value: $attempt/status"
    fi
  done < <(find "$RUNS" -type d -path '*/attempts/[0-9][0-9][0-9][0-9]' -print0)
  while IFS= read -r -d '' final; do
    logical=${final%/final.json}
    logical=${logical##*/}
    evidence_manifest=$(mktemp)
    write_attempt_evidence_manifest "${final%/final.json}" "$evidence_manifest" ||
      die "cannot hash terminal attempt evidence for $logical"
    evidence_sha=$(sha256sum "$evidence_manifest" | awk '{print $1}')
    evidence_count=$(wc -l <"$evidence_manifest")
    unlink "$evidence_manifest"
    jq -e --arg logical "$logical" --arg protocol "$PROTOCOL_SHA" \
      --arg analyzer "$ANALYZER_SHA" --arg evidence_sha256 "$evidence_sha" \
      --argjson evidence_files "$evidence_count" '
        .schema == "dreg.high-arity-final/v1" and .logical_name == $logical and
        .protocol_sha256 == $protocol and .analyzer_sha256 == $analyzer and
        (.terminal_exit_code == 0 or .terminal_exit_code == 65 or .terminal_exit_code == 76) and
        (.row_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
        .evidence_sha256 == $evidence_sha256 and .evidence_files == $evidence_files
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

native_audit_draw_file() {
  local file=$1 m=$2 n=$3 ell=$4 count
  count=$(jq -s 'length' "$file") || return 1
  (
    cd "$REPO"
    "${SCIENTIFIC_ENV[@]}" "$LADDER" --audit-jsonl "$file" \
      --expect-m "$m" --expect-n "$n" --expect-lambda "$ell" \
      --expect-seed "$SEED" --expect-count "$count" \
      --expect-d-max 3 --expect-ffd-max 3
  ) || {
    echo "native deterministic replay rejected draw file: $file" >&2
    return 1
  }
}

audit_one() {
  local m=$1 n=$2 ell=$3
  local file
  file=$(cell_file "$m" "$n" "$ell")
  audit_draw_file "$file" "$m" "$n" "$ell" && native_audit_draw_file "$file" "$m" "$n" "$ell"
}

validate_generation_provenance() {
  local draw=$1 m=$2 n=$3 ell=$4 metadata log draw_sha log_sha draw_bytes log_bytes records
  local generation_commit
  metadata=${draw%.jsonl}.generation.json
  log=${draw%.jsonl}.log
  [[ -f $metadata && ! -L $metadata && -f $log && ! -L $log ]] || return 1
  draw_sha=$(sha256sum "$draw" | awk '{print $1}') || return 1
  log_sha=$(sha256sum "$log" | awk '{print $1}') || return 1
  draw_bytes=$(stat -c %s "$draw") || return 1
  log_bytes=$(stat -c %s "$log") || return 1
  records=$(jq -s 'length' "$draw") || return 1
  jq -e --arg cell "m$m-n$n-l$ell" --arg protocol "$PROTOCOL_SHA" \
    --arg commit "$SOURCE_COMMIT" --arg ladder "$LADDER_SHA" \
    --arg container "$BUILD_CONTAINER_DIGEST" --arg rustc "$FROZEN_RUSTC_VERSION" \
    --arg execution_ref "$DREG_EXECUTION_REF" \
    --arg draw_path "${draw##*/}" --arg draw_sha "$draw_sha" --argjson draw_bytes "$draw_bytes" \
    --argjson records "$records" --arg log_path "${log##*/}" --arg log_sha "$log_sha" \
    --argjson log_bytes "$log_bytes" --argjson m "$m" --argjson n "$n" \
    --argjson ell "$ell" --argjson seed "$SEED" --arg stage_marker ".m$m-n$n-l$ell.jsonl.attempt." '
      .schema == "dreg.high-arity-generation/v1" and .cell == $cell and
      .protocol_sha256 == $protocol and .source_commit == $commit and
      .binary_sha256 == $ladder and
      .build.container_image_digest == $container and .build.rustc_version == $rustc and
      .execution.ref == $execution_ref and
      (.execution.commit | type == "string" and test("^[0-9a-f]{40}$")) and
      .draw_jsonl == {path:$draw_path,sha256:$draw_sha,bytes:$draw_bytes,records:$records} and
      .generator_log == {path:$log_path,sha256:$log_sha,bytes:$log_bytes} and
      .replay_audit == {result:"passed",expect_m:$m,expect_n:$n,expect_lambda:$ell,
        expect_seed:$seed,expect_count:$records,expect_d_max:3,expect_ffd_max:3,
        ignored_value_fields:["secs"]} and
      (.audit_argv[1] as $audit_input |
        ($audit_input | type == "string" and contains($stage_marker)) and
        .audit_argv == ["--audit-jsonl",$audit_input,
          "--expect-m",($m|tostring),"--expect-n",($n|tostring),
          "--expect-lambda",($ell|tostring),"--expect-seed",($seed|tostring),
          "--expect-count",($records|tostring),"--expect-d-max","3",
          "--expect-ffd-max","3"]) and
      .parent_environment_cleared == true and
      .sanitized_env == {PATH:"/usr/bin:/bin",F4_F2_MAX_ROWS:"50000000",
        F4_F2_MAX_COLS:"50000000",RAYON_NUM_THREADS:"4",DREG_CKPT_SECONDS:"300"}
  ' "$metadata" >/dev/null || return 1
  generation_commit=$(jq -er '.execution.commit' "$metadata") || return 1
  git -C "$REPO" cat-file -e "$generation_commit^{commit}" &&
    git -C "$REPO" merge-base --is-ancestor "$SOURCE_COMMIT" "$generation_commit" &&
    git -C "$REPO" merge-base --is-ancestor "$generation_commit" "$EXECUTION_COMMIT"
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
  local classification selected tmp created evidence_manifest evidence_sha evidence_count
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
      .attempts[].status, .attempts[].finished, .attempts[].metadata)
  ' "$analysis")
  row_sha=$(printf '%s' "$row" | sha256sum | awk '{print $1}')
  evidence_manifest=$(mktemp)
  write_attempt_evidence_manifest "$run_dir" "$evidence_manifest" ||
    die "cannot hash retained attempt evidence for $logical"
  evidence_sha=$(sha256sum "$evidence_manifest" | awk '{print $1}')
  evidence_count=$(wc -l <"$evidence_manifest")
  unlink "$evidence_manifest"
  classification=$(printf '%s' "$row" | jq -er '.classification') ||
    die "analyzer row lacks classification for $logical"
  selected=$(printf '%s' "$row" | jq -c '.selected_attempt') ||
    die "analyzer row lacks selected_attempt for $logical"

  if [[ -e $final ]]; then
    [[ -f $final && ! -L $final ]] || die "invalid terminal pointer: $final"
    jq -e --arg logical "$logical" --arg classification "$classification" \
      --argjson selected_attempt "$selected" \
      --argjson terminal_exit_code "$terminal_rc" --arg row_sha256 "$row_sha" \
      --arg protocol_sha256 "$PROTOCOL_SHA" --arg analyzer_sha256 "$ANALYZER_SHA" \
      --arg evidence_sha256 "$evidence_sha" --argjson evidence_files "$evidence_count" '
        .schema == "dreg.high-arity-final/v1" and .logical_name == $logical and
        .classification == $classification and .selected_attempt == $selected_attempt and
        .terminal_exit_code == $terminal_exit_code and
        .row_sha256 == $row_sha256 and .protocol_sha256 == $protocol_sha256 and
        .analyzer_sha256 == $analyzer_sha256 and .evidence_sha256 == $evidence_sha256 and
        .evidence_files == $evidence_files
      ' "$final" >/dev/null || die "existing terminal pointer disagrees with analysis: $final"
    return 0
  fi

  created=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  tmp=$(mktemp "$run_dir/.final.tmp.XXXXXX")
  jq -n --arg logical_name "$logical" --arg classification "$classification" \
    --argjson selected_attempt "$selected" \
    --argjson terminal_exit_code "$terminal_rc" --arg row_sha256 "$row_sha" \
    --arg protocol_sha256 "$PROTOCOL_SHA" --arg analyzer_sha256 "$ANALYZER_SHA" \
    --arg evidence_sha256 "$evidence_sha" --argjson evidence_files "$evidence_count" \
    --arg created_at "$created" '
      {schema:"dreg.high-arity-final/v1",logical_name:$logical_name,
       classification:$classification,selected_attempt:$selected_attempt,
       terminal_exit_code:$terminal_exit_code,row_sha256:$row_sha256,
       protocol_sha256:$protocol_sha256,analyzer_sha256:$analyzer_sha256,
       evidence_sha256:$evidence_sha256,evidence_files:$evidence_files,
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

publication_ready() {
  local summary=$1 logical
  if [[ $(jq -r '.global_classification' "$summary") == invalid_instrument ]]; then
    return 0
  fi
  while IFS= read -r logical; do
    [[ -f $RUNS/$logical/final.json && ! -L $RUNS/$logical/final.json ]] || return 1
  done < <(jq -r '.rows[].run_id' "$summary")
}

validate_publication_layout() {
  local publication=$1 entry base count=0
  [[ -d $publication && ! -L $publication ]] || return 1
  while IFS= read -r -d '' entry; do
    [[ -f $entry && ! -L $entry ]] || return 1
    base=${entry##*/}
    case "$base" in
      REPORT.md|closure-outcomes.svg|report.typ|report.pdf|experiment.export.json) ;;
      *) return 1 ;;
    esac
    count=$((count + 1))
  done < <(find "$publication" -mindepth 1 -maxdepth 1 -print0)
  [[ $count -eq 5 ]]
}

build_or_check_publication() {
  local summary=$1 summary_sha export_id publication unexpected reviews fresh analyzer_rc
  # The report publisher validates the summary's internal grammar, while this
  # fresh frozen-analyzer replay binds those classifications to the retained
  # raw evidence.  A coherent edit of rows, packages, cells, and the global
  # label therefore cannot manufacture a scoped counterexample report.
  fresh=$(mktemp "$OUTPUT_ROOT/.publication-summary.tmp.XXXXXX")
  if (
    cd "$REPO"
    "${SCIENTIFIC_ENV[@]}" "$ANALYZER" "$DRAWS" "$OUTPUT_ROOT"
  ) >"$fresh"; then
    analyzer_rc=0
  else
    analyzer_rc=$?
  fi
  case $analyzer_rc in
    0|65|76) ;;
    *)
      unlink "$fresh"
      echo "fresh frozen-analyzer publication replay returned $analyzer_rc" >&2
      return 65
      ;;
  esac
  if ! cmp -s -- "$summary" "$fresh"; then
    unlink "$fresh"
    echo "publication summary differs from a fresh frozen-analyzer replay" >&2
    return 65
  fi
  unlink "$fresh"

  summary_sha=$(sha256sum "$summary" | awk '{print $1}')
  export_id=DREG-HIGH-ARITY-${summary_sha:0:16}
  publication=$OUTPUT_ROOT/reports/run-$export_id
  mkdir -p "$OUTPUT_ROOT/reports"
  unexpected=$(find "$OUTPUT_ROOT/reports" -mindepth 1 -maxdepth 1 \
    ! -name "run-$export_id" ! -name reviews -print -quit)
  [[ -z $unexpected ]] || die "unexpected or interrupted publication artifact: $unexpected"
  reviews=$OUTPUT_ROOT/reports/reviews
  if [[ -e $reviews ]]; then
    [[ -d $reviews && ! -L $reviews ]] || die "invalid publication review directory: $reviews"
    [[ -z $(find "$reviews" \( -type l -o \( ! -type d ! -type f \) \) \
      -print -quit) ]] ||
      die "publication review directory contains a symlink or special file: $reviews"
  fi
  if [[ -e $publication ]]; then
    validate_publication_layout "$publication" || die "invalid publication output: $publication"
    (
      cd "$REPO"
      "${SCIENTIFIC_ENV[@]}" "$REPORT" --summary "$summary" --protocol "$PROTOCOL" \
        --out-dir "$publication" --check
    ) || return 65
  else
    (
      cd "$REPO"
      "${SCIENTIFIC_ENV[@]}" "$REPORT" --summary "$summary" --protocol "$PROTOCOL" \
        --out-dir "$publication"
      validate_publication_layout "$publication"
      "${SCIENTIFIC_ENV[@]}" "$REPORT" --summary "$summary" --protocol "$PROTOCOL" \
        --out-dir "$publication" --check
    ) || return 65
  fi
}

require_terminal_prerequisite() {
  local logical=$1 rc
  if analyzer_snapshot "$logical"; then rc=0; else rc=$?; fi
  case $rc in
    0)
      write_final_pointer "$logical" "$OUTPUT_ROOT/summary.json" "$rc"
      return 0
      ;;
    76)
      write_final_pointer "$logical" "$OUTPUT_ROOT/summary.json" "$rc"
      echo "prerequisite $logical is resource-censored; pursuit continues without a package conclusion" >&2
      return 0
      ;;
    65)
      write_final_pointer "$logical" "$OUTPUT_ROOT/summary.json" "$rc"
      die "prerequisite $logical contains invalid or contradictory evidence"
      ;;
    75)
      die "prerequisite $logical is still retryable; finish its registered attempts before the next ordered step"
      ;;
    *)
      die "prerequisite $logical returned unexpected analyzer exit $rc"
      ;;
  esac
}

gate_all_controls() {
  local row m n ell ignored file draw
  require_terminal_prerequisite identity-m4-n12-l5-d24
  for row in "${CELLS[@]}"; do
    IFS=: read -r m n ell ignored _ <<<"$row"
    file=$(cell_file "$m" "$n" "$ell")
    while IFS= read -r draw; do
      require_terminal_prerequisite "control-m$m-n$n-l$ell-d$draw"
    done < <(selected_draws "$file" sat 2)
  done
}

gate_control_predecessors() {
  local target=$1 row m n ell ignored file draw logical found=false
  require_terminal_prerequisite identity-m4-n12-l5-d24
  for row in "${CELLS[@]}"; do
    IFS=: read -r m n ell ignored _ <<<"$row"
    file=$(cell_file "$m" "$n" "$ell")
    while IFS= read -r draw; do
      logical=control-m$m-n$n-l$ell-d$draw
      if [[ $logical == "$target" ]]; then
        found=true
        break 2
      fi
      require_terminal_prerequisite "$logical"
    done < <(selected_draws "$file" sat 2)
  done
  [[ $found == true ]] || die "control is absent from the frozen execution order: $target"
}

gate_unsat_predecessors() {
  local target=$1 row m n ell ignored file draw logical found=false
  gate_all_controls
  for row in "${CELLS[@]}"; do
    IFS=: read -r m n ell ignored _ <<<"$row"
    file=$(cell_file "$m" "$n" "$ell")
    while IFS= read -r draw; do
      logical=unsat-m$m-n$n-l$ell-d$draw
      if [[ $logical == "$target" ]]; then
        found=true
        break 2
      fi
      require_terminal_prerequisite "$logical"
    done < <(selected_draws "$file" unsat 4)
  done
  [[ $found == true ]] || die "measurement is absent from the frozen execution order: $target"
}

next_attempt_dir() {
  local logical=$1 base expected=1 path name number durable uncommitted= allocation_tmp
  base=$RUNS/$logical/attempts
  mkdir -p "$base"
  shopt -s nullglob
  for path in "$base"/*; do
    [[ -d $path ]] || die "non-directory entry in attempt set: $path"
    name=${path##*/}
    [[ $name =~ ^[0-9]{4}$ ]] || die "non-canonical attempt directory: $path"
    number=$((10#$name))
    [[ $number -eq $expected ]] || die "attempt directory gap under $base"
    durable=$(find "$path" -maxdepth 1 -type f ! -name '.*.tmp.*' \
      ! -name allocated.json -print -quit)
    if [[ -z $durable ]]; then
      [[ -z $uncommitted ]] || die "multiple uncommitted attempt allocations under $base"
      uncommitted=$path
    elif [[ -n $uncommitted ]]; then
      die "committed attempt follows an uncommitted allocation under $base"
    fi
    expected=$((expected + 1))
  done
  shopt -u nullglob
  if [[ -n $uncommitted ]]; then
    path=$uncommitted
    name=${path##*/}
  else
    [[ $expected -le $MAX_ATTEMPTS ]] || return 76
    printf -v name '%04d' "$expected"
    path=$base/$name
    mkdir "$path" || die "could not allocate append-only attempt $path"
  fi
  if [[ ! -e $path/allocated.json ]]; then
    allocation_tmp=$(mktemp "$path/.allocation.tmp.XXXXXX")
    jq -n --arg logical_name "$logical" --arg attempt "$name" \
      --arg allocated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" '
        {schema:"dreg.high-arity-allocation/v1",logical_name:$logical_name,
         attempt:$attempt,allocated_at:$allocated_at}
      ' >"$allocation_tmp"
    chmod 644 "$allocation_tmp"
    link_install "$allocation_tmp" "$path/allocated.json" || {
      unlink "$allocation_tmp"
      [[ -f $path/allocated.json && ! -L $path/allocated.json ]] ||
        die "could not atomically allocate append-only attempt $path"
    }
  fi
  printf '%s\n' "$path"
}

run_closure_once() {
  local logical=$1 file=$2 draw=$3 mode=$4 max_cols=$5
  local ckpt attempt_dir attempt_no started rc analyzer_rc input_sha
  local kernel arch cpu_model logical_cpus mem_kib mem_bytes
  local meta_tmp status_tmp finished_tmp analyzer_status_tmp summary_tmp termination
  local checkpoint_restore_json checkpoint_manifest_sha
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

  ckpt=$CHECKPOINTS/$logical
  mkdir -p "$ckpt"
  if select_checkpoint_generation "$ckpt"; then
    rc=0
  else
    rc=$?
  fi
  [[ $rc -eq 0 ]] ||
    die "checkpoint store for $logical contains committed generations but none verifies"

  if attempt_dir=$(next_attempt_dir "$logical"); then rc=0; else rc=$?; fi
  if [[ $rc -eq 76 ]]; then
    echo "$logical exhausted $MAX_ATTEMPTS attempts and remains unmeasured" >&2
    return 76
  fi
  [[ $rc -eq 0 ]] || return "$rc"

  attempt_no=${attempt_dir##*/}
  if [[ -n $CHECKPOINT_RESTORE_MANIFEST ]]; then
    checkpoint_manifest_sha=$(sha256sum "$CHECKPOINT_RESTORE_MANIFEST" | awk '{print $1}')
    checkpoint_restore_json=$(jq -cn --argjson restored true \
      --argjson generation "$CHECKPOINT_RESTORE_GENERATION" \
      --arg manifest_sha256 "$checkpoint_manifest_sha" \
      --rawfile manifest_json "$CHECKPOINT_RESTORE_MANIFEST" \
      '{restored:$restored,generation:$generation,
        manifest_sha256:$manifest_sha256,manifest_json:$manifest_json}')
  else
    checkpoint_restore_json='{"restored":false,"generation":null,"manifest_sha256":null,"manifest_json":null}'
  fi
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
  meta_tmp=$(mktemp "$attempt_dir/.meta.tmp.XXXXXX")
  jq -n --arg logical_name "$logical" --arg attempt "$attempt_no" \
    --arg input "$file" --arg input_sha256 "$input_sha" --argjson draw "$draw" \
    --arg mode "$mode" --arg started_at "$started" --arg protocol_sha256 "$PROTOCOL_SHA" \
    --arg source_commit "$SOURCE_COMMIT" --arg ladder_sha256 "$LADDER_SHA" \
    --arg closure_sha256 "$CLOSURE_SHA" --arg analyzer_sha256 "$ANALYZER_SHA" \
    --arg report_sha256 "$REPORT_SHA" \
    --arg container_image_digest "$BUILD_CONTAINER_DIGEST" \
    --arg rustc_version "$FROZEN_RUSTC_VERSION" --arg kernel "$kernel" --arg arch "$arch" \
    --arg cpu_model "$cpu_model" --argjson logical_cpus "$logical_cpus" \
    --argjson memory_bytes "$mem_bytes" --arg checkpoint_dir "$ckpt" \
    --argjson checkpoint_restore "$checkpoint_restore_json" \
    --argjson max_cols "$max_cols" \
    --argjson satisfiable "$satisfiable" --arg execution_ref "$DREG_EXECUTION_REF" \
    --arg execution_commit "$EXECUTION_COMMIT" '
      {
        schema:"dreg.high-arity-attempt/v1",logical_name:$logical_name,
        attempt:$attempt,input:{path:$input,sha256:$input_sha256},draw:$draw,mode:$mode,
        started_at:$started_at,protocol_sha256:$protocol_sha256,source_commit:$source_commit,
        binaries:{ladder:$ladder_sha256,closure:$closure_sha256,analyzer:$analyzer_sha256,
          report:$report_sha256},
        build:{container_image_digest:$container_image_digest,rustc_version:$rustc_version},
        execution:{ref:$execution_ref,commit:$execution_commit},
        host:{kernel:$kernel,arch:$arch,cpu_model:$cpu_model,
          logical_cpus:$logical_cpus,memory_bytes:$memory_bytes},
        limits:{address_space_bytes:32000000000,timeout_seconds:86400,
          term_grace_seconds:60,max_cols:$max_cols,degree_to:4},
        checkpoint_dir:$checkpoint_dir,checkpoint_restore:$checkpoint_restore,
        parent_environment_cleared:true,
        sanitized_env:{PATH:"/usr/bin:/bin",F4_F2_MAX_ROWS:"50000000",
          F4_F2_MAX_COLS:"50000000",RAYON_NUM_THREADS:"4",DREG_CKPT_SECONDS:"300"},
        argv:((if $satisfiable then ["--satisfiable"] else [] end) +
          ["--to","4","--max-cols",($max_cols|tostring),"--draw",($draw|tostring),
           "--checkpoint-dir",$checkpoint_dir,$input])
      }
    ' >"$meta_tmp"
  link_install "$meta_tmp" "$attempt_dir/meta.json" || {
    unlink "$meta_tmp"
    die "attempt metadata already exists; refusing concurrent overwrite: $attempt_dir"
  }

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
  status_tmp=$(mktemp "$attempt_dir/.status.tmp.XXXXXX")
  printf '%s\n' "$rc" >"$status_tmp"
  chmod 644 "$status_tmp"
  termination=$(termination_for_status "$rc" "$attempt_dir/stderr.log")
  finished_tmp=$(mktemp "$attempt_dir/.finished.tmp.XXXXXX")
  jq -n --arg finished_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" --argjson exit_code "$rc" \
    --arg termination "$termination" \
    '{schema:"dreg.high-arity-finished/v1",finished_at:$finished_at,
      exit_code:$exit_code,termination:$termination}' >"$finished_tmp"
  chmod 644 "$finished_tmp"
  # Install the self-contained receipt first. If the node is preempted before
  # the legacy status scalar is installed, analyzers treat the attempt as
  # incomplete/retryable; the reverse order could strand an unbound status.
  link_install "$finished_tmp" "$attempt_dir/finished.json" || {
    unlink "$finished_tmp" "$status_tmp"
    die "completion receipt already exists; refusing overwrite: $attempt_dir"
  }
  link_install "$status_tmp" "$attempt_dir/status" || {
    unlink "$status_tmp"
    die "status already exists after completion receipt install: $attempt_dir"
  }

  if (
    cd "$REPO"
    "${SCIENTIFIC_ENV[@]}" "$ANALYZER" "$DRAWS" "$OUTPUT_ROOT" --require "$logical"
  ) >"$attempt_dir/analysis.json" 2>"$attempt_dir/analyzer.stderr"; then
    analyzer_rc=0
  else
    analyzer_rc=$?
  fi
  analyzer_status_tmp=$(mktemp "$attempt_dir/.analyzer-status.tmp.XXXXXX")
  printf '%s\n' "$analyzer_rc" >"$analyzer_status_tmp"
  link_install "$analyzer_status_tmp" "$attempt_dir/analyzer.status" || {
    unlink "$analyzer_status_tmp"
    die "analyzer status already exists; refusing overwrite: $attempt_dir"
  }
  summary_tmp=$(mktemp "$OUTPUT_ROOT/.summary.tmp.XXXXXX")
  cp -- "$attempt_dir/analysis.json" "$summary_tmp"
  mv -f -- "$summary_tmp" "$OUTPUT_ROOT/summary.json"

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
  local draw_sha log_sha draw_bytes log_bytes records
  registered_cell "$m" "$n" "$ell" || die "generator step names an unregistered cell"
  require_durable_draws_on_ephemeral
  mkdir -p "$DRAWS"
  final=$(cell_file "$m" "$n" "$ell")
  if [[ -e $final ]]; then
    audit_draw_file "$final" "$m" "$n" "$ell" ||
      die "existing draw file is invalid and will not be overwritten: $final"
    native_audit_draw_file "$final" "$m" "$n" "$ell" ||
      die "existing draw file fails deterministic native replay: $final"
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
  native_audit_draw_file "$tmp" "$m" "$n" "$ell" ||
    die "generated draw prefix failed deterministic native replay; retained $tmp"
  finished=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  kernel=$(uname -sr)
  arch=$(uname -m)
  cpu_model=$(awk -F ': ' '/^model name[[:space:]]*:/{print $2; exit}' /proc/cpuinfo)
  logical_cpus=$(getconf _NPROCESSORS_ONLN)
  mem_kib=$(awk '/^MemTotal:/{print $2; exit}' /proc/meminfo)
  mem_bytes=$((mem_kib * 1024))
  draw_sha=$(sha256sum "$tmp" | awk '{print $1}')
  log_sha=$(sha256sum "$logtmp" | awk '{print $1}')
  draw_bytes=$(stat -c %s "$tmp")
  log_bytes=$(stat -c %s "$logtmp")
  records=$(jq -s 'length' "$tmp")
  jq -n --arg cell "m$m-n$n-l$ell" --arg started_at "$started" --arg finished_at "$finished" \
    --arg protocol_sha256 "$PROTOCOL_SHA" --arg source_commit "$SOURCE_COMMIT" \
    --arg ladder_sha256 "$LADDER_SHA" --arg container_image_digest "$BUILD_CONTAINER_DIGEST" \
    --arg rustc_version "$FROZEN_RUSTC_VERSION" --arg kernel "$kernel" --arg arch "$arch" \
    --arg cpu_model "$cpu_model" --argjson logical_cpus "$logical_cpus" \
    --argjson memory_bytes "$mem_bytes" --argjson m "$m" --argjson n "$n" \
    --argjson ell "$ell" --argjson seed "$SEED" \
    --arg execution_ref "$DREG_EXECUTION_REF" --arg execution_commit "$EXECUTION_COMMIT" \
    --arg draw_path "${final##*/}" --arg draw_sha256 "$draw_sha" \
    --argjson draw_bytes "$draw_bytes" --argjson records "$records" \
    --arg log_path "${final%.jsonl}.log" --arg log_sha256 "$log_sha" \
    --argjson log_bytes "$log_bytes" --arg audit_input "$tmp" '
      {schema:"dreg.high-arity-generation/v1",cell:$cell,started_at:$started_at,
       finished_at:$finished_at,protocol_sha256:$protocol_sha256,source_commit:$source_commit,
       binary_sha256:$ladder_sha256,
       build:{container_image_digest:$container_image_digest,rustc_version:$rustc_version},
       execution:{ref:$execution_ref,commit:$execution_commit},
       host:{kernel:$kernel,arch:$arch,cpu_model:$cpu_model,
         logical_cpus:$logical_cpus,memory_bytes:$memory_bytes},
       draw_jsonl:{path:$draw_path,sha256:$draw_sha256,bytes:$draw_bytes,records:$records},
       generator_log:{path:($log_path | split("/") | last),sha256:$log_sha256,bytes:$log_bytes},
       replay_audit:{result:"passed",expect_m:$m,expect_n:$n,expect_lambda:$ell,
         expect_seed:$seed,expect_count:$records,expect_d_max:3,expect_ffd_max:3,
         ignored_value_fields:["secs"]},
       audit_argv:["--audit-jsonl",$audit_input,
         "--expect-m",($m|tostring),"--expect-n",($n|tostring),
         "--expect-lambda",($ell|tostring),"--expect-seed",($seed|tostring),
         "--expect-count",($records|tostring),"--expect-d-max","3",
         "--expect-ffd-max","3"],
       parent_environment_cleared:true,
       sanitized_env:{PATH:"/usr/bin:/bin",F4_F2_MAX_ROWS:"50000000",
         F4_F2_MAX_COLS:"50000000",RAYON_NUM_THREADS:"4",DREG_CKPT_SECONDS:"300"},
       argv:["--cells",(($n|tostring)+":"+($ell|tostring)+":3"),"--m",($m|tostring),
         "--unsat","4","--min-sat","2","--ffd-max","3","--seed",($seed|tostring),
         "--max-draws","4096"]}
    ' >"$metatmp"
  link_install "$logtmp" "${final%.jsonl}.log" ||
    die "generator log appeared concurrently; retained staged draw $tmp"
  link_install "$metatmp" "${final%.jsonl}.generation.json" ||
    die "generation receipt appeared concurrently; retained staged draw $tmp"
  link_install "$tmp" "$final" ||
    die "draw appeared concurrently; refusing overwrite: $final"
  echo "generated and validated $final"
}

require_committed_generated_cell() {
  local m=$1 n=$2 ell=$3 draw rel metadata log
  draw=$HERE/draws/m$m-n$n-l$ell.jsonl
  metadata=${draw%.jsonl}.generation.json
  log=${draw%.jsonl}.log
  [[ -f $draw && ! -L $draw && -f $metadata && ! -L $metadata && -f $log && ! -L $log ]] ||
    die "earlier registered generator cell is not committed: m$m-n$n-l$ell"
  for rel in "${draw#"$REPO/"}" "${metadata#"$REPO/"}" "${log#"$REPO/"}"; do
    git -C "$REPO" ls-files --error-unmatch "$rel" >/dev/null ||
      die "earlier generator artifact is not tracked: $rel"
  done
  audit_draw_file "$draw" "$m" "$n" "$ell" &&
    native_audit_draw_file "$draw" "$m" "$n" "$ell" &&
    validate_generation_provenance "$draw" "$m" "$n" "$ell" ||
    die "earlier generator cell failed custody: m$m-n$n-l$ell"
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
    gate_control_predecessors "$logical"
    run_closure_once "$logical" "$file" "$draw" sat 400000
  else
    draw_is_selected "$file" unsat "$draw" || die "measurement is not one of the first four UNSAT draws"
    gate_unsat_predecessors "$logical"
    run_closure_once "$logical" "$file" "$draw" unsat 400000
  fi
}

if [[ $BINARY_MANIFEST_ONLY == true ]]; then
  verify_binary_manifest "${BINARY_MANIFEST_HASHES[@]}" ||
    die "binary manifest is not the exact canonical ordered four-row identity"
  exit 0
fi

if [[ $DEP_INFO_ONLY == true ]]; then
  verify_dep_info_coverage "$DEP_INFO_REPO" "${DEP_INFO_FILES[@]}" ||
    die "release dep-info contains an unregistered repository input"
  exit 0
fi

if [[ $SOURCE_TREE_ONLY == true ]]; then
  verify_source_tree "$SOURCE_TREE_REPO" "$SOURCE_TREE_COMMIT" "$SOURCE_TREE_MANIFEST" ||
    die "current transitive Rust source tree differs from the frozen commit"
  exit 0
fi

verify_bindings
case "$STEP" in
  gen-m5-n11-l2)
    generate_cell 5 11 2
    ;;
  gen-m5-n13-l3)
    require_committed_generated_cell 5 11 2
    generate_cell 5 13 3
    ;;
  gen-m6-n11-l2)
    require_committed_generated_cell 5 11 2
    require_committed_generated_cell 5 13 3
    generate_cell 6 11 2
    ;;
  audit-draws)
    audit_all_draws
    ;;
  seal-resume-anchor)
    seal_resume_anchor
    ;;
  analyze)
    audit_all_draws
    prepare_output
    if analyzer_snapshot; then rc=0; else rc=$?; fi
    seal_terminal_rows_from_summary "$OUTPUT_ROOT/summary.json"
    if publication_ready "$OUTPUT_ROOT/summary.json"; then
      if ! build_or_check_publication "$OUTPUT_ROOT/summary.json"; then
        echo "native publication build or exact replay failed" >&2
        exit 65
      fi
    else
      echo "publication deferred: at least one non-invalid row remains retryable" >&2
    fi
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
