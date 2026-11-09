#!/usr/bin/env bash
set -euo pipefail
root=/var/tmp/independent-phase-accounting-review-20261101
original=/home/ec2-user/crypto-pr2304-persist
csv=research/four_followups_review_20261022/results/standard-targets.csv
native=research/four_followups_20261021/results/matched/fb1-j16-bits16.jsonl
mkdir -p "$root/base"
jq -r '.inputs|keys[]' "$root/result.json" | while IFS= read -r src;do
 rel=${src#"$original/"}; mkdir -p "$root/base/$(dirname "$rel")"; cp --reflink=auto "$src" "$root/base/$rel"
done
for name in duplicate_standard altered_bits wrong_workload duplicate_native unsigned_counter empty_phases omitted_phase empty_cohort;do
 dest="$root/controls/$name";mkdir -p "$dest"
 cp -a --reflink=auto "$root/base" "$dest/input"
 case "$name" in
 duplicate_standard) sed -n '2p' "$dest/input/$csv" >> "$dest/input/$csv";;
 altered_bits) jq -c 'if input_line_number==1 then .bits=999 else . end' "$dest/input/$native" > "$dest/temp"; mv "$dest/temp" "$dest/input/$native";;
 wrong_workload) jq -c 'if input_line_number==1 then .workload_id="REVIEW-MISMATCH" else . end' "$dest/input/$native" > "$dest/temp";mv "$dest/temp" "$dest/input/$native";;
 duplicate_native) head -1 "$dest/input/$native" >> "$dest/input/$native";;
 unsigned_counter) jq -c 'if input_line_number==1 then .phases[0].counts.m = -1 else . end' "$dest/input/$native" > "$dest/temp"; mv "$dest/temp" "$dest/input/$native";;
 empty_phases) jq -c 'if input_line_number==1 then .phases=[] else . end' "$dest/input/$native" > "$dest/temp";mv "$dest/temp" "$dest/input/$native";;
 omitted_phase) jq -c 'if input_line_number==1 then .phases |= .[1:] else . end' "$dest/input/$native" > "$dest/temp";mv "$dest/temp" "$dest/input/$native";;
 empty_cohort) head -1 "$dest/input/$csv" > "$dest/temp";mv "$dest/temp" "$dest/input/$csv";;
 esac
 sha256sum "$dest/input/$csv" "$dest/input/$native" > "$dest/mutated-bindings.sha256"
 set +e
 "$root/build/release/pencil-readonly-accounting" "$dest/input" > "$dest/result.json" 2> "$dest/stderr"
 rc=$?
 jq -e -f "$root/accept.jq" "$dest/result.json" > "$dest/acceptance.txt" 2> "$dest/acceptance.stderr"
 ac=$?
 set -e
 # Empty stdout must never count as an accepted report.
 if [ ! -s "$dest/result.json" ];then ac=2;fi
 jq -n --arg name "$name" --argjson rc "$rc" --argjson ac "$ac" '{name:$name,reporter_exit:$rc,review_gate_exit:$ac,expected_rejection:($ac!=0)}' > "$dest/receipt.json"
 test "$ac" -ne 0
done
jq -s '{schema:"readonly-negative-controls/v1",controls:.,all_rejected:all(.[];.expected_rejection)}' "$root"/controls/*/receipt.json > "$root/controls.json"
