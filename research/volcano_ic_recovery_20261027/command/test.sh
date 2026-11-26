#!/usr/bin/env bash
# Development checks only. Never calls the full-width run entrypoint successfully.
set -euo pipefail
umask 077
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
[[ $# == 2 && -f $1/READY_FOR_DIAGNOSTIC && $2 == /* && ! -e $2 ]] || { echo 'usage: test.sh PASSED_NATIVE_RECEIPT NEW_ABSOLUTE_PRIVATE_DIRECTORY' >&2; exit 64; }
receipt=$(cd -- "$1" && pwd -P)
output=$2
mkdir -- "$output"
(cd "$receipt" && sha256sum -c binaries.sha256) > "$output/native-binary-check.stdout"
bash "$here/prepare-context.sh" "$output/context" > "$output/context.stdout" 2> "$output/context.stderr"
context=$output/context
mkdir "$context/bin"
cp "$receipt/build/runner" "$receipt/build/verify" "$context/bin/"
(cd "$context" && sha256sum bin/runner bin/verify > binaries.sha256)
bash "$context/launch.sh" plan > "$output/plan.json" 2> "$output/plan.stderr"
jq -e '.payload_bytes==9566497092 and .side_entries==244140625 and .slots==536870912 and .radix==25' "$output/plan.json" > "$output/plan-check.stdout"
if env -u ISOLAB_CRIU bash "$context/launch.sh" run > "$output/no-supervisor.stdout" 2> "$output/no-supervisor.stderr"; then echo 'unguarded run accepted' >&2; exit 1; fi
if ISOLAB_CRIU=1 ISOLAB_OUTPUT_DIR="$output" bash "$context/launch.sh" run > "$output/outside-pvc.stdout" 2> "$output/outside-pvc.stderr"; then echo 'outside-PVC run accepted' >&2; exit 1; fi
input=$context/repo/research/volcano_ic_recovery_20261027/public_source_case.json
cp "$input" "$output/input.original.json"
printf '\n' >> "$input"
if bash "$context/launch.sh" plan > "$output/changed-input.stdout" 2> "$output/changed-input.stderr"; then echo 'changed input accepted' >&2; exit 1; fi
cp "$output/input.original.json" "$input"
printf x >> "$context/bin/runner"
if bash "$context/launch.sh" plan > "$output/changed-binary.stdout" 2> "$output/changed-binary.stderr"; then echo 'changed binary accepted' >&2; exit 1; fi
cp "$receipt/build/runner" "$context/bin/runner"
if bash "$here/prepare-context.sh" "$output/context" > "$output/reused-context.stdout" 2> "$output/reused-context.stderr"; then echo 'reused context accepted' >&2; exit 1; fi
printf '{"schema":"volcano.ic.command-development/v1","plan_payload_bytes":9566497092,"full_width_table_allocated":false,"negative_controls":5,"passed":true,"image_built":false,"live_criu_qualified":false}\n' > "$output/result.json"
cat "$output/result.json"
