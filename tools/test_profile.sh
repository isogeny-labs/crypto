#!/usr/bin/env bash
# Checks tools/profile.sh: the GPU parsers and graphs against the test doubles
# in tools/testdata/profile/bin (synthetic output in the ncu, nsys and
# nvidia-smi formats; no GPU needed), the refusal paths (counter permission,
# an ncu too old for the GPU, no PMU for top-down), and the CPU paths on the
# smoke workload when perf, valgrind and a C compiler are present.
#
#   tools/test_profile.sh        exit 0 when every check that could run passed
set -uo pipefail
here=$(cd "$(dirname "$0")" && pwd)
P="$here/profile.sh"
fx="$here/testdata/profile"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
pass=0 fail=0 skip=0
ok() { pass=$((pass + 1)); echo "ok   $1"; }
bad() { fail=$((fail + 1)); echo "FAIL $1"; }
check() { # name jq-filter file
    if jq -e "$2" "$3" > /dev/null 2>&1; then ok "$1"; else bad "$1 ($2 on $3)"; fi
}
expect_rc() { # name want got
    if [ "$2" = "$3" ]; then ok "$1 exits $2"; else bad "$1 exits $3, want $2"; fi
}

gpu() { PATH="$fx/bin:$PATH" NCU="" NSYS="" "$P" "$@" > "$tmp/log" 2>&1; echo $?; }

# ncu: parse, summarize, graph.
rc=$(gpu ncu --out "$tmp/ncu" --label fixture --kernel eccWalk --launch-skip 2 -- true)
expect_rc "ncu fixture" 0 "$rc"
k="$tmp/ncu/kernels.json"
check "ncu: one launch" '.launches | length == 1' "$k"
check "ncu: kernel name with commas survives CSV quoting" '.launches[0].kernel == "eccWalkKernel<32, true>(unsigned int*, const unsigned int*, int)"' "$k"
check "ncu: thousands separators parsed" '.launches[0].duration_ns == 1234567 and .launches[0].instructions == 2101445632' "$k"
check "ncu: stalls sorted, top is math_pipe_throttle" '.launches[0].stalls_per_issue[0] == {reason: "math_pipe_throttle", value: 1.92}' "$k"
check "ncu: pipes" '.launches[0].pipe_utilization_pct | map(.pipe) == ["alu", "fma"]' "$k"
check "ncu: local memory sectors" '.launches[0].local_memory_sectors == {load: 1024, store: 2048}' "$k"
check "ncu: units row kept" '.units["gpu__time_duration.sum"] == "nsecond"' "$k"
check "ncu: manifest records clock control and launch selection" '.ncu.clock_control == "base" and .ncu.launch_skip == 2 and .ncu.kernel_regex == "eccWalk"' "$tmp/ncu/manifest.json"
check "ncu: manifest host lists the GPU" '.host.gpus[0].compute_cap == "12.0"' "$tmp/ncu/manifest.json"
for f in sol.svg stalls.svg summary.md report.typ; do [ -s "$tmp/ncu/$f" ] && ok "ncu: $f written" || bad "ncu: $f missing"; done
grep -q 'not_selected' "$tmp/ncu/stalls.svg" && ! grep -q '>selected<' "$tmp/ncu/stalls.svg" && ok "ncu: stall graph" || bad "ncu: stall graph"

rc=$(STUB_NCU_PERM=1 gpu ncu --out "$tmp/ncu-perm" -- true)
expect_rc "ncu without counter permission" 3 "$rc"
check "ncu: ERR_NVGPUCTRPERM reported as unsupported" '.status == "unsupported" and (.reason | test("ERR_NVGPUCTRPERM"))' "$tmp/ncu-perm/manifest.json"
rc=$(STUB_NCU_VERSION=2024.3.0.0 gpu ncu --out "$tmp/ncu-old" -- true)
expect_rc "ncu 2024 on compute capability 12.0" 3 "$rc"
check "ncu: version refusal names Blackwell" '.reason | test("Blackwell")' "$tmp/ncu-old/manifest.json"
rc=$(STUB_NCU_VERSION=2024.3.0.0 gpu ncu --force-version --out "$tmp/ncu-force" -- true)
expect_rc "ncu --force-version" 0 "$rc"

# nsys: report names, coverage arithmetic.
rc=$(gpu nsys --out "$tmp/nsys" --gpu-metrics -- true)
expect_rc "nsys fixture" 0 "$rc"
s="$tmp/nsys/summary.json"
check "nsys: kernels" '.kernels | length == 2 and .[0].Instances == 4' "$s"
# Fixture trace: kernels [1.0,2.2], [2.25,2.298], [2.4,3.6] ms, a memcpy after.
check "nsys: coverage = 2.448 / 2.6 ms, memcpy excluded" '.timeline.kernel_launches == 3 and .timeline.span_ns == 2600000 and .timeline.kernel_busy_ns == 2448000' "$s"
check "nsys: api table" '.cuda_api[0].Name == "cudaDeviceSynchronize"' "$s"
[ -s "$tmp/nsys/kernels.svg" ] && ok "nsys: kernels.svg" || bad "nsys: kernels.svg"

# smi: sampling during the command, limit reasons.
rc=$(gpu smi --out "$tmp/smi" --interval-ms 50 -- sleep 0.6)
expect_rc "smi fixture" 0 "$rc"
check "smi: samples and the limited one" '.samples >= 8 and .sm_clock_mhz.min == 2100 and .clock_reason_masks["0x0000000000000004"] == 1' "$tmp/smi/smi-summary.json"
grep -q '<polyline' "$tmp/smi/smi.svg" && grep -q '#cf222e' "$tmp/smi/smi.svg" && ok "smi: line graph marks the limited sample" || bad "smi: graph"
check "smi: defaults to physical GPU 0" '.smi.gpu_index == 0' "$tmp/smi/manifest.json"
rc=$(gpu smi --out "$tmp/smi-1" --gpu-index 1 --interval-ms 50 -- sleep 0.2)
expect_rc "smi selected GPU" 0 "$rc"
check "smi: selected GPU alone contributes" '.gpu_index == 1 and .sm_clock_mhz.min == 900 and .sm_clock_mhz.max == 900' "$tmp/smi-1/smi-summary.json"
grep -q 'GPU 1 SM clock' "$tmp/smi-1/smi.svg" && ok "smi: graph labels selected GPU" || bad "smi: graph GPU identity"
rc=$(STUB_SMI_EMPTY=1 gpu smi --out "$tmp/smi-empty" -- true)
expect_rc "smi empty sample stream" 6 "$rc"
check "smi: no samples cannot be successful" '.status == "profiler_failed"' "$tmp/smi-empty/manifest.json"

# Existing evidence must survive even if the caller chooses the same path.
before=$(sha256sum "$tmp/smi/manifest.json")
rc=$(gpu smi --out "$tmp/smi" -- touch "$tmp/should-not-run")
expect_rc "existing output directory" 2 "$rc"
[ ! -e "$tmp/should-not-run" ] && [ "$before" = "$(sha256sum "$tmp/smi/manifest.json")" ] && ok "existing evidence and command preserved" || bad "output directory reused"

# report re-renders from JSON alone.
rm "$tmp/ncu/summary.md" "$tmp/ncu/sol.svg"
"$P" report "$tmp/ncu" > /dev/null 2>&1 && [ -s "$tmp/ncu/sol.svg" ] && ok "report re-renders" || bad "report re-renders"

# perf with a PMU whose top-down group fails: a clean profiler failure, not a
# crash (the GitHub runner case).
rc=0; PERF="$fx/perf-topdown-fails" "$P" topdown --out "$tmp/td-fail" -- true > /dev/null 2>&1 || rc=$?
expect_rc "topdown when perf cannot schedule the group" 6 "$rc"
check "topdown: perf's own message in the reason" '.status == "profiler_failed" and (.reason | test("Cannot find metric"))' "$tmp/td-fail/manifest.json"

# A failed perf probe must never be mistaken for readable hardware counters.
PERF="$fx/perf-denied-counters" "$P" probe --out "$tmp/probe-denied" > /dev/null 2>&1
expect_rc "probe with denied hardware counters" 0 $?
check "probe: software timers remain available" '.capabilities.cpu_hardware_counters == false and .capabilities.cpu_sampling_software == true and .capabilities.topdown == false' "$tmp/probe-denied/probe.json"
rc=0; PERF="$fx/perf-denied-counters" "$P" topdown --out "$tmp/td-denied" -- true > /dev/null 2>&1 || rc=$?
expect_rc "topdown with denied hardware counters" 3 "$rc"
rc=0; PERF="$fx/perf-denied-counters" "$P" cpu --out "$tmp/cpu-denied" -- true > /dev/null 2>&1 || rc=$?
expect_rc "CPU fixture stops after selecting event" 6 "$rc"
grep -qx cpu-clock "$tmp/cpu-denied/command.stdout" && ok "cpu: fallback selects software timer" || bad "cpu: failed hardware probe chose cycles"

# Usage errors.
"$P" cpu -- true > /dev/null 2>&1; expect_rc "cpu without --out" 2 $?
"$P" ncu --out "$tmp/x" > /dev/null 2>&1; expect_rc "ncu without a command" 2 $?

# CPU paths, where the host allows.
cc=$(command -v cc || command -v gcc || true)
if [ -n "$cc" ] && "$cc" -O2 -g -fno-omit-frame-pointer "$fx/hot.c" -o "$tmp/hot" 2> /dev/null; then
    perf=""
    for p in "${PERF:-}" "$(command -v perf || true)" $(ls -d /usr/lib/linux-tools-*/perf 2>/dev/null | sort -rV); do
        [ -n "$p" ] && [ -x "$p" ] && "$p" stat -e task-clock -- true > /dev/null 2>&1 && { perf=$p; break; }
    done
    if [ -n "$perf" ]; then
        PERF=$perf "$P" cpu --out "$tmp/cpu" --call-graph dwarf -- "$tmp/hot" > /dev/null 2>&1; expect_rc "cpu sample" 0 $?
        h="$tmp/cpu/hotspots.json"
        check "cpu: the two hot functions hold >= 90% of samples" '[.hotspots[] | select(.symbol == "mix" or .symbol == "gf2_mul") | .self_pct] | add >= 90' "$h"
        check "cpu: s.e. present and consistent" '.hotspots[0] | .se_pct > 0 and .se_pct < .self_pct' "$h"
        grep -q 'sq_chain;gf2_mul' "$tmp/cpu/folded.txt" && ok "cpu: dwarf stacks keep the leaf's caller" || bad "cpu: dwarf stacks"
        grep -q '<rect' "$tmp/cpu/flame.svg" && ok "cpu: flame graph" || bad "cpu: flame graph"
        # Whether top-down works depends on the host's PMU (a VM may expose
        # cycles but not the top-down events), so the check is that every
        # outcome is a clean one: profiled, unsupported or profiler failed,
        # each with a manifest that says which.
        rc=0; PERF=$perf "$P" topdown --out "$tmp/td" -- true > /dev/null 2>&1 || rc=$?
        case $rc in
            0) check "topdown: profiled" '.status == "ok"' "$tmp/td/manifest.json" ;;
            3) check "topdown: unsupported, with a reason" '.status == "unsupported" and (.reason | length > 0)' "$tmp/td/manifest.json" ;;
            6) check "topdown: profiler failure, with a reason" '.status == "profiler_failed" and (.reason | length > 0)' "$tmp/td/manifest.json" ;;
            *) bad "topdown exits $rc, want 0, 3 or 6" ;;
        esac
    else
        skip=$((skip + 1)); echo "skip cpu sampling: no perf that can sample here"
    fi
    if command -v valgrind > /dev/null; then
        "$P" cpu --mode callgrind --out "$tmp/cg" -- "$tmp/hot" > /dev/null 2>&1; expect_rc "cpu callgrind" 0 $?
        check "callgrind: function table, not source annotation" '.hotspots | map(.symbol) | index("gf2_mul") != null and index("mix") != null' "$tmp/cg/hotspots.json"
        check "callgrind: gf2_mul executes more instructions than mix" '(.hotspots | map(select(.symbol == "gf2_mul"))[0].ir) > (.hotspots | map(select(.symbol == "mix"))[0].ir)' "$tmp/cg/hotspots.json"
    else
        skip=$((skip + 1)); echo "skip callgrind: no valgrind"
    fi
else
    skip=$((skip + 1)); echo "skip cpu: no C compiler"
fi

echo "test_profile: $pass passed, $fail failed, $skip skipped"
[ "$fail" = 0 ]
