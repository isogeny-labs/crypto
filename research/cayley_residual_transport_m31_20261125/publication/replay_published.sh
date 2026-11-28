#!/usr/bin/env bash
# Publication byte-custody checks and an explicitly requested native replay.
# This wrapper never changes the frozen replay.sh or historical raw evidence.
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  bash publication/replay_published.sh [--verify]
  bash publication/replay_published.sh --run NEW_OUTPUT_DIRECTORY

The default/--verify path hydrates historical evidence from Git recovery and
checks publication hashes only. It does not compile or execute research code.

--run additionally invokes the unchanged native replay.sh and compares its
new JSONL/summary bytes with the hydrated historical records. The output path
must not exist and must be outside this study directory. Relative output paths
are resolved against the caller's working directory. Set RUSTC as needed.
The historical checker remains archive-only, not repository-verified.
USAGE
}

task_mode=verify
task_requested_output=
if [[ $# -eq 0 ]]; then
  :
elif [[ $# -eq 1 && "$1" == --verify ]]; then
  :
elif [[ $# -eq 1 && "$1" == --help ]]; then
  usage
  exit 0
elif [[ $# -eq 2 && "$1" == --run && -n "$2" ]]; then
  task_mode=run
  task_requested_output=$2
else
  usage >&2
  exit 2
fi

task_caller_dir=$PWD
task_support_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
task_study_root=$(cd -- "$task_support_dir/.." && pwd -P)
task_repo_root=$(git -C "$task_study_root" rev-parse --show-toplevel)
task_repo_root=$(cd -- "$task_repo_root" && pwd -P)
case "$task_study_root" in
  "$task_repo_root"/*) task_study_relative=${task_study_root#"$task_repo_root"/} ;;
  *) echo "Study directory is not inside its reported Git repository" >&2; exit 2 ;;
esac

if [[ "$task_mode" == run ]]; then
  case "$task_requested_output" in
    /*) task_output=$(realpath -m -- "$task_requested_output") ;;
    *) task_output=$(realpath -m -- "$task_caller_dir/$task_requested_output") ;;
  esac
  case "$task_output" in
    "$task_study_root"|"$task_study_root"/*)
      echo "Replay output must be outside the frozen study directory" >&2
      exit 2
      ;;
  esac
  if [[ -e "$task_output" || -L "$task_output" ]]; then
    echo "Refusing to overwrite existing output: $task_output" >&2
    exit 2
  fi
fi

cd -- "$task_repo_root"
python3 tools/artifacts/artifacts.py pull "$task_study_relative/evidence" --source git
python3 tools/artifacts/artifacts.py verify "$task_study_relative/evidence"
python3 "$task_support_dir/verify_publication.py" --package-root "$task_study_root" --json

if [[ "$task_mode" == run ]]; then
  bash "$task_study_root/replay.sh" "$task_output"
  python3 "$task_support_dir/verify_publication.py" --package-root "$task_study_root" \
    --replay-dir "$task_output" --json
fi
