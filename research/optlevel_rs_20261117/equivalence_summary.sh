#!/usr/bin/env bash
# Tabulate equivalence.sh output: per control and round, the two tools' records side by side, and for scale the
# same tool against itself across rounds.  "within" compares |ratio - 1| with the larger of the two A/A noise floors.
# usage: equivalence_summary.sh EQUIV_DIR
set -euo pipefail
O=$1
f() { jq -r "$2" "$O/$1.record.json"; }
echo -e "control\tround\tpoint\tpython\trust\trust/python-1\tnoise_py\tnoise_rs\twithin_noise\tloadavg_py\tloadavg_rs\tother_cores_py\tother_cores_rs"
for c in positive-simd-incache scalar-incache serial-dependency stream-daxpy pointer-chase; do for r in 1 2; do
  for pt in single full; do
    jq -rn --arg c $c --arg r $r --arg pt $pt --slurpfile p "$O/$c.r$r.py.record.json" --slurpfile s "$O/$c.r$r.rs.record.json" '
      ($p[0]) as $p | ($s[0]) as $s | ($p.attainment[$pt].value) as $a | ($s.attainment[$pt].value) as $b
      | ([$p.noise_floor.value, $s.noise_floor.value] | max) as $n
      | [$c, $r, "A_" + $pt, $a, $b, ($b / $a - 1), $p.noise_floor.value, $s.noise_floor.value, ((($b / $a - 1) | fabs) <= $n),
         $p.conditions.loadavg_1m.median, $s.conditions.loadavg_1m.median, $p.conditions.other_cpu_cores.median, $s.conditions.other_cpu_cores.median] | @tsv'
  done
done; done
echo
echo -e "control\ttool\tA_single_r1\tA_single_r2\tr2/r1-1\tgrade_r1\tgrade_r2\tlimiting_factor_same\tflags_r1\tflags_r2"
for c in positive-simd-incache scalar-incache serial-dependency stream-daxpy pointer-chase; do for t in py rs; do
  jq -rn --arg c $c --arg t $t --slurpfile a "$O/$c.r1.$t.record.json" --slurpfile b "$O/$c.r2.$t.record.json" '
    ($a[0]) as $a | ($b[0]) as $b | [$c, $t, $a.attainment.single.value, $b.attainment.single.value, ($b.attainment.single.value / $a.attainment.single.value - 1),
     $a.grade.letter_range, $b.grade.letter_range, (($a.grade.limiting_factor | sub("[0-9.]+"; "N"; "g")) == ($b.grade.limiting_factor | sub("[0-9.]+"; "N"; "g"))),
     ($a.flags | join(",")), ($b.flags | join(","))] | @tsv'
done; done
echo
echo "# structure: keys present in one tool's record and not the other's (round 1), and static values that must be equal"
for c in positive-simd-incache scalar-incache serial-dependency stream-daxpy pointer-chase; do
  jq -rn --arg c $c --slurpfile p "$O/$c.r1.py.record.json" --slurpfile s "$O/$c.r1.rs.record.json" '
    def keys_deep: [paths | map(tostring) | join(".")] | map(select(test("^(host|achieved|conditions|kernel\\.command)") | not)) | unique;
    ($p[0]) as $p | ($s[0]) as $s
    | [$c, "only_python=" + (($p | keys_deep) - ($s | keys_deep) | join(",")), "only_rust=" + (($s | keys_deep) - ($p | keys_deep) | join(",")),
       "host_keys_only_rust=" + (($s.host | keys) - ($p.host | keys) | join(",")),
       "same_fingerprint=" + ($p.host.fingerprint == $s.host.fingerprint | tostring), "same_roofline=" + ($p.roofline == $s.roofline | tostring),
       "same_intensity=" + ($p.intensity == $s.intensity | tostring), "same_vector_share=" + ($p.diagnosis.vector_share_static == $s.diagnosis.vector_share_static | tostring),
       "same_binding=" + ($p.roofline.single.binding == $s.roofline.single.binding | tostring)] | join("\t")'
done
