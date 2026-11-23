#!/bin/sh
# Exact-string copying checks only. No curve arithmetic or mathematical verdict.
set -eu
case_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
jq -e --slurpfile protocol "$case_dir/protocol.json" \
  '.root_basis[1]==$protocol[0].transport_point and .root_basis[0]==$protocol[0].kernel_point and .p==$protocol[0].field_prime and .ell==$protocol[0].candidate_prime_conductor_ratio and .weil_pairing==$protocol[0].recorded_weil_pairing' \
  "$case_dir/sources/upper-96-torsion-certificate.jsonl" > /dev/null
jq -e --rawfile row "$case_dir/sources/upper-96-torsion.gpvec" --rawfile input "$case_dir/input.txt" \
  '. as $p | ($row|gsub("[\\[\\]\\s]";"")|split(",")) as $v | ($input|split("\n")) as $lines | ($lines[0]|split(" ")) as $h | $p.field_prime==$v[2] and $p.source_short_coefficients==[$v[5],$v[6]] and $p.kernel_point==[$v[7],$v[8]] and $h==[$p.field_prime,$p.source_short_coefficients[0],$p.source_short_coefficients[1],$p.candidate_prime_conductor_ratio,$p.kernel_point[0],$p.kernel_point[1],"3"] and ($lines[1]|split(" "))==$p.transport_point and ($lines[2]|split(" "))==$p.kernel_point and $lines[3]=="O"' \
  "$case_dir/protocol.json" > /dev/null
jq -e --slurpfile protocol "$case_dir/protocol.json" \
  '.a==$protocol[0].normalization.expected_scaled_codomain_a and .b==$protocol[0].normalization.expected_scaled_codomain_b and .floor_conductor==$protocol[0].candidate_prime_conductor_ratio' \
  "$case_dir/sources/upper-96-floor-certificate.jsonl" > /dev/null
printf '%s\n' '{"scope":"exact-string metadata checks only; no curve arithmetic","copied_basis_and_pairing_match":true,"input_and_tuple_match":true,"expected_floor_fields_match":true}'
