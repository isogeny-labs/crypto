#!/usr/bin/env bash
# Serialize existing public unit programs without changing any polynomial.
set -euo pipefail
(( $# >= 2 )) || { echo 'Usage: make_batch.sh FRESH_OUTPUT.m ARM=INPUT.m ...' >&2; exit 2; }
batch_file=$1
shift
[[ ! -e "$batch_file" ]] || { echo 'Output must be fresh' >&2; exit 2; }
declare -A seen=()
printf 'SetQuitOnError(true);\n' > "$batch_file"
for spec in "$@"; do
  arm=${spec%%=*}
  input_file=${spec#*=}
  [[ $arm =~ ^(dense|tower|paired|oriented)$ && -f "$input_file" && -z ${seen[$arm]:-} ]] || { echo 'Invalid/duplicate arm or missing input' >&2; exit 2; }
  seen[$arm]=1
  printf 'printf "BEGIN_ARM %s\\n";\n' "$arm" >> "$batch_file"
  sed '/^SetNthreads(1);$/d; /^quit;$/d; s/^SetVerbose("Groebner",1);$/SetVerbose("Groebner",0);/' "$input_file" \
    | perl -0777 -pe 's/\n>;\n/ >;\n/g' >> "$batch_file"
  printf '\nprintf "END_ARM %s\\n";\n' "$arm" >> "$batch_file"
done
printf 'quit;\n' >> "$batch_file"
(( $(wc -c < "$batch_file") <= 51200 )) || { echo 'Batch exceeds service input cap' >&2; exit 2; }
