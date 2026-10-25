#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
out=$1
mkdir -p "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]]
out=$(cd -- "$out" && pwd)
date -u +%FT%TZ > "$out/started.txt"
cc=${CC:-cc}
"$cc" --version > "$out/compiler-version.txt"
sha256sum "$here/timeout_print.c" "$here/run_controls.sh" > "$out/source-sha256.txt"
for site in 1 2; do
  set +e
  "$cc" -std=c11 -O0 -Wall -Wextra -Wformat=2 -Werror=format \
    -DFORMAT_SITE="$site" -DFIX_TIMEOUT_PRINT=0 "$here/timeout_print.c" \
    -o "$out/original-$site" > "$out/original-$site.compile.stdout" 2> "$out/original-$site.compile.stderr"
  rc=$?
  set -e
  printf '%s\n' "$rc" > "$out/original-$site.compile.exit"
  [[ $rc != 0 ]]
  "$cc" -std=c11 -O0 -Wall -Wextra -Wformat=2 -Werror=format \
    -DFORMAT_SITE="$site" -DFIX_TIMEOUT_PRINT=1 "$here/timeout_print.c" \
    -o "$out/fixed-$site" > "$out/fixed-$site.compile.stdout" 2> "$out/fixed-$site.compile.stderr"
  timeout 5 "$out/fixed-$site" 3600 > "$out/fixed-$site.stdout" 2> "$out/fixed-$site.stderr"
  printf '0\n' > "$out/fixed-$site.run.exit"
done
"$cc" -std=c11 -O0 -Wall -Wextra -Wformat=2 -DFORMAT_SITE=1 -DFIX_TIMEOUT_PRINT=0 \
  "$here/timeout_print.c" -o "$out/original-runtime" > "$out/original-runtime.compile.stdout" 2> "$out/original-runtime.compile.stderr"
set +e
(ulimit -c 0; timeout 5 "$out/original-runtime" 3600) > "$out/original-runtime.stdout" 2> "$out/original-runtime.stderr"
rc=$?
set -e
printf '%s\n' "$rc" > "$out/original-runtime.run.exit"
[[ $rc == 139 ]]
rg -q '3600 sec exceeded' "$out/fixed-1.stdout"
rg -q '3600 sec, exiting' "$out/fixed-2.stdout"
jq -n --argjson rc "$rc" '{original_sieving_statement_exit:$rc,
  original_format_sites_rejected_by_compiler:2,corrected_format_sites_compile_and_run:2,
  full_yafu_pipeline_reproduced:false,production_failure_instruction_pointer_known:false,
  factorization_search_executed:false}' > "$out/summary.json"
date -u +%FT%TZ > "$out/finished.txt"
cat "$out/summary.json"
