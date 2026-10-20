#!/usr/bin/env bash
# Profile one command and say where its time goes: CPU sampling with Linux
# `perf record` (or Callgrind where perf cannot sample), top-down CPU metrics
# with `perf stat -M`, NVIDIA Nsight Compute (`ncu`) per-kernel metrics,
# Nsight Systems (`nsys`) timelines, and `nvidia-smi` clock/power traces.
# Every subcommand writes a directory with a `profile/v1` manifest.json (host,
# tool versions, binary hash, commit, command, exit status), the profiler's
# raw output, a summary JSON and an SVG graph.  It never invents a number the
# host cannot produce: a missing tool, a host without a hardware PMU or GPU
# counters the driver withholds give "status": "unsupported" and exit 3.
#
#   tools/profile.sh probe  [--out DIR] [--gpu-control]
#   tools/profile.sh cpu    --out DIR [--mode auto|sample|callgrind] [--event E]
#                           [--freq HZ] [--call-graph fp|dwarf|lbr] [--top N]
#                           [--isolate] [--label L] -- COMMAND [ARGS...]
#   tools/profile.sh topdown --out DIR [--level 1|2] [--isolate] -- COMMAND...
#   tools/profile.sh ncu    --out DIR [--kernel REGEX] [--launch-skip N]
#                           [--launch-count N] [--set NAME] [--section S]...
#                           [--metrics LIST] [--clock-control base|none]
#                           [--source] [--force-version] -- COMMAND...
#   tools/profile.sh nsys   --out DIR [--trace LIST] [--cpu-sample]
#                           [--gpu-metrics] [--top N] -- COMMAND...
#   tools/profile.sh smi    --out DIR [--interval-ms MS] -- COMMAND...
#   tools/profile.sh report DIR        (re-render summary.md, SVG and report.typ)
#
# Environment: PERF, NCU, NSYS (binaries), ISOLATED_BENCH (for --isolate),
# MAX_OTHER_CPU (isolated_bench, default 0.1).
# Exit status: 0 profiled, 2 usage error, 3 unsupported on this host,
# 4 the command failed, 6 the profiler failed or profiled nothing.
set -euo pipefail

SCHEMA=profile/v1
die() { echo "profile: $*" >&2; exit 2; }
for t in jq gawk; do command -v "$t" > /dev/null || die "needs $t"; done
[ $# -gt 0 ] || die "usage: tools/profile.sh probe|cpu|topdown|ncu|nsys|smi|report ... (see the header of $0)"
sub=$1; shift

out="" label="" isolate=0 top=25
mode=auto event="" freq=999 callgraph=fp level=1
kernel="" lskip=0 lcount=1 nset="" sections=() metrics="" clock=base source=0 force_version=0
trace="cuda,nvtx,osrt" cpusample=0 gpumetrics=0 interval=100 gpucontrol=0
report_dir=""
while [ $# -gt 0 ]; do
    case $1 in
        --out) out=$2; shift 2 ;;
        --label) label=$2; shift 2 ;;
        --isolate) isolate=1; shift ;;
        --top) top=$2; shift 2 ;;
        --mode) mode=$2; shift 2 ;;
        --event) event=$2; shift 2 ;;
        --freq) freq=$2; shift 2 ;;
        --call-graph) callgraph=$2; shift 2 ;;
        --level) level=$2; shift 2 ;;
        --kernel) kernel=$2; shift 2 ;;
        --launch-skip) lskip=$2; shift 2 ;;
        --launch-count) lcount=$2; shift 2 ;;
        --set) nset=$2; shift 2 ;;
        --section) sections+=("$2"); shift 2 ;;
        --metrics) metrics=$2; shift 2 ;;
        --clock-control) clock=$2; shift 2 ;;
        --source) source=1; shift ;;
        --force-version) force_version=1; shift ;;
        --trace) trace=$2; shift 2 ;;
        --cpu-sample) cpusample=1; shift ;;
        --gpu-metrics) gpumetrics=1; shift ;;
        --interval-ms) interval=$2; shift 2 ;;
        --gpu-control) gpucontrol=1; shift ;;
        --) shift; break ;;
        -*) die "unknown option $1" ;;
        *) if [ "$sub" = report ] && [ -z "$report_dir" ]; then report_dir=$1; shift; else die "unexpected argument $1 (put the command after --)"; fi ;;
    esac
done
cmd=("$@")
NCU_DEFAULT_SECTIONS=(SpeedOfLight LaunchStats Occupancy ComputeWorkloadAnalysis MemoryWorkloadAnalysis SchedulerStats WarpStateStats InstructionStats SourceCounters)
[ ${#sections[@]} -gt 0 ] || sections=("${NCU_DEFAULT_SECTIONS[@]}")

# ---------------------------------------------------------------- identity

find_perf() {
    local p
    for p in "${PERF:-}" "$(command -v perf || true)" $(ls -d /usr/lib/linux-tools-*/perf /usr/lib/linux-tools/*/perf 2>/dev/null | sort -rV); do
        [ -n "$p" ] && [ -x "$p" ] && "$p" --version > /dev/null 2>&1 && { echo "$p"; return 0; }
    done
    return 1
}
# Newest Nsight binary: $VAR, PATH, then NVIDIA's versioned install trees.
# Ubuntu's unversioned nsight-compute package is 2022.4, which predates
# Blackwell (ecc2k130/TUNING.md); the versioned trees come first after PATH.
find_nsight() { # var-value name glob...
    local want=$1 name=$2 p; shift 2
    for p in "$want" "$(command -v "$name" || true)" $(ls -d "$@" 2>/dev/null | sort -rV); do
        [ -n "$p" ] && [ -x "$p" ] && "$p" --version > /dev/null 2>&1 && { echo "$p"; return 0; }
    done
    return 1
}
find_ncu() { find_nsight "${NCU:-}" ncu /opt/nvidia/nsight-compute/*/ncu /usr/local/cuda*/bin/ncu /usr/local/cuda*/nsight-compute*/ncu; }
find_nsys() { find_nsight "${NSYS:-}" nsys /opt/nvidia/nsight-systems/*/bin/nsys /usr/local/cuda*/bin/nsys /usr/local/cuda*/nsight-systems*/bin/nsys; }

gpus_json() {
    command -v nvidia-smi > /dev/null || { echo '[]'; return; }
    nvidia-smi --query-gpu=index,name,driver_version,compute_cap,clocks.max.sm,clocks.max.mem,power.limit,memory.total,persistence_mode \
        --format=csv,noheader,nounits 2>/dev/null | jq -Rsc '
        split("\n") | map(select(length > 0) | split(", ") |
          {index: (.[0] | tonumber? // .[0]), name: .[1], driver: .[2], compute_cap: .[3],
           max_sm_clock_mhz: (.[4] | tonumber? // null), max_mem_clock_mhz: (.[5] | tonumber? // null),
           power_limit_w: (.[6] | tonumber? // null), memory_mib: (.[7] | tonumber? // null), persistence: .[8]})' 2>/dev/null || echo '[]'
}

host_json() {
    local cpu flags pmus
    cpu=$(grep -m1 'model name' /proc/cpuinfo 2>/dev/null | cut -d: -f2- | sed 's/^ *//' || true)
    [ -n "$cpu" ] || cpu=$(grep -m1 -i 'CPU part' /proc/cpuinfo 2>/dev/null | cut -d: -f2- | sed 's/^ *//' || echo unknown)
    flags=$(grep -m1 -E '^(flags|Features)' /proc/cpuinfo 2>/dev/null | cut -d: -f2- | tr ' ' '\n' \
        | grep -xE 'popcnt|avx2|avx512f|avx512ifma|vpclmulqdq|pclmulqdq|bmi2|adx|sha_ni|gfni|aes|pmull|sha2|asimd|sve|sve2' | sort -u | tr '\n' ' ' || true)
    pmus=$(ls /sys/bus/event_source/devices 2>/dev/null | tr '\n' ' ')
    jq -nc --arg cpu "$cpu" --arg flags "$flags" --arg kernel "$(uname -r)" --arg arch "$(uname -m)" \
        --arg pmus "$pmus" --arg paranoid "$(cat /proc/sys/kernel/perf_event_paranoid 2>/dev/null || echo unknown)" \
        --arg kptr "$(cat /proc/sys/kernel/kptr_restrict 2>/dev/null || echo unknown)" \
        --argjson cpus "$(nproc)" --arg mem "$(awk '/MemTotal/ {print $2}' /proc/meminfo 2>/dev/null)" \
        --arg hostname "$(hostname 2>/dev/null || echo unknown)" --argjson gpus "$(gpus_json)" \
        '{hostname: $hostname, cpu_model: $cpu, cpu_features: ($flags | split(" ") | map(select(. != ""))),
          kernel: $kernel, arch: $arch, logical_cpus: $cpus, mem_kib: ($mem | tonumber? // null),
          event_sources: ($pmus | split(" ") | map(select(. != ""))),
          hardware_pmu: ($pmus | test("(^| )(cpu|cpu_core|cpu_atom|armv8_pmuv3[^ ]*|armv9[^ ]*)( |$)")),
          perf_event_paranoid: $paranoid, kptr_restrict: $kptr, gpus: $gpus}'
}

# The binary the command runs (skipping `env VAR=...` and `taskset ...`
# prefixes), with its SHA-256, so a profile names exactly what it profiled.
binary_json() {
    local i=0 a p=""
    while [ $i -lt ${#cmd[@]} ]; do
        a=${cmd[$i]}
        case $a in
            env|nice|taskset|numactl|stdbuf) i=$((i + 1)); [ "$a" = taskset ] && i=$((i + 1)); continue ;;
            -*|*=*) i=$((i + 1)); continue ;;
        esac
        p=$(command -v "$a" 2>/dev/null || true); break
    done
    if [ -n "$p" ] && [ -f "$p" ]; then
        jq -nc --arg path "$(readlink -f "$p")" --arg sha "$(sha256sum "$p" | cut -d' ' -f1)" \
            --arg bytes "$(stat -c %s "$p")" '{path: $path, sha256: $sha, bytes: ($bytes | tonumber)}'
    else
        echo null
    fi
}

git_json() {
    git rev-parse --is-inside-work-tree > /dev/null 2>&1 || { echo null; return; }
    jq -nc --arg c "$(git rev-parse HEAD 2>/dev/null)" --arg d "$(git status --porcelain --untracked-files=no 2>/dev/null | wc -l)" \
        '{commit: $c, dirty_tracked_files: ($d | tonumber)}'
}

started="" wall=""
tools='{}'
add_tool() { tools=$(jq -c --arg k "$1" --arg p "$2" --arg v "$3" '. + {($k): {path: $p, version: $v}}' <<< "$tools"); }

write_manifest() { # status reason [extra-json]
    [ -n "$out" ] || return 0
    jq -n --arg schema "$SCHEMA" --arg sub "$sub" --arg label "$label" --arg status "$1" --arg reason "$2" \
        --arg started "$started" --arg wall "$wall" --arg cwd "$PWD" --argjson extra "${3:-{\}}" \
        --argjson host "$(host_json)" --argjson tools "$tools" --argjson binary "$(binary_json)" \
        --argjson git "$(git_json)" --argjson isolate "$isolate" \
        --args '{schema: $schema, kind: "manifest", subcommand: $sub, label: $label, status: $status,
                 reason: (if $reason == "" then null else $reason end),
                 started_utc: (if $started == "" then null else $started end),
                 wall_seconds: ($wall | tonumber? // null), cwd: $cwd, command: $ARGS.positional,
                 isolation: (if $isolate == 1 then "isolated_bench" else "none" end),
                 binary: $binary, git: $git, tools: $tools, host: $host} + $extra' -- "${cmd[@]}" > "$out/manifest.json"
}
unsupported() { # reason [extra]
    write_manifest unsupported "$1" "${2:-{\}}"
    echo "profile: unsupported on this host: $1" >&2
    exit 3
}
profiler_failed() { # reason [extra]
    write_manifest profiler_failed "$1" "${2:-{\}}"
    echo "profile: profiler failed: $1" >&2
    exit 6
}

setup_out() {
    [ -n "$out" ] || die "$sub needs --out DIR"
    [ ${#cmd[@]} -gt 0 ] || die "$sub needs a command after --"
    mkdir -p "$out"
    out=$(cd "$out" && pwd)
}

# Run the profiler command line, under isolated_bench when asked (AGENTS.md
# section 10).  The command's stdout and stderr land in the output directory.
rc=0
run_profiled() {
    local bench=""
    if [ "$isolate" = 1 ]; then
        bench=${ISOLATED_BENCH:-target/release/isolated_bench}
        [ -x "$bench" ] || die "--isolate needs \$ISOLATED_BENCH or $bench"
    fi
    started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    local t0 t1
    t0=$(date +%s.%N)
    set +e
    if [ -n "$bench" ]; then
        "$bench" run --wait --cpus 3 --max-other-cpu "${MAX_OTHER_CPU:-0.1}" --label "${label:-profile-$sub}" \
            --out "$out/isolation.jsonl" -- "$@" > "$out/command.stdout" 2> "$out/command.stderr"
    else
        "$@" > "$out/command.stdout" 2> "$out/command.stderr"
    fi
    rc=$?
    set -e
    t1=$(date +%s.%N)
    wall=$(awk -v a="$t0" -v b="$t1" 'BEGIN {printf "%.3f", b - a}')
}

# CSV (RFC 4180 quoting, commas inside kernel names) to a JSON array of
# objects keyed by the header row; numbers stay strings until jq converts them.
csv2json() { # file [skip-regex]
    gawk -v skip="${2:-^==}" '
        BEGIN { FPAT = "([^,]*)|(\"([^\"]|\"\")*\")"; n = 0; printf "[" }
        function unq(s) { if (s ~ /^".*"$/) { s = substr(s, 2, length(s) - 2); gsub(/""/, "\"", s) } return s }
        function js(s) { gsub(/\\/, "\\\\", s); gsub(/"/, "\\\"", s); gsub(/\t/, "\\t", s); return "\"" s "\"" }
        { sub(/\r$/, "") }
        $0 ~ skip || $0 == "" { next }
        !h { for (i = 1; i <= NF; i++) hd[i] = unq($i); nh = NF; h = 1; next }
        { printf "%s{", (n++ ? "," : ""); for (i = 1; i <= nh; i++) printf "%s%s:%s", (i > 1 ? "," : ""), js(hd[i]), js(unq($i)); printf "}" }
        END { print "]" }' "$1"
}
NUM='def num: if type == "string" and test("^-?[0-9][0-9,]*(\\.[0-9]+)?([eE][-+]?[0-9]+)?$") then gsub(","; "") | tonumber else . end;'

# ---------------------------------------------------------------- graphs

# Horizontal bar chart (SVG) from JSON [{name, value, err?}], recorded values only.
svg_bars() { # json title unit caption [fixed-axis-max]
    jq -r --arg title "$2" --arg unit "$3" --arg cap "$4" --arg fixed "${5:-}" '
        def esc: tostring | gsub("&"; "&amp;") | gsub("<"; "&lt;") | gsub(">"; "&gt;") | gsub("\""; "&quot;");
        def short: if length > 64 then .[0:61] + "..." else . end;
        (map(.value) | max // 1) as $max | (if $fixed != "" then ($fixed | tonumber) elif $max <= 0 then 1 else $max end) as $m |
        length as $n | (70 + $n * 22 + 40) as $h | 420 as $x0 | 340 as $w |
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"900\" height=\"\($h)\" viewBox=\"0 0 900 \($h)\" font-family=\"DejaVu Sans, Helvetica, Arial, sans-serif\" font-size=\"12\">",
        "<rect width=\"900\" height=\"\($h)\" fill=\"#ffffff\"/>",
        "<text x=\"16\" y=\"26\" font-size=\"15\" font-weight=\"bold\" fill=\"#1f2328\">\($title | esc)</text>",
        "<text x=\"\($x0)\" y=\"50\" fill=\"#59636e\">\($unit | esc)</text>",
        (to_entries[] | .key as $i | .value as $r | (60 + $i * 22) as $y | ($r.value / $m * $w) as $bw |
          "<text x=\"\($x0 - 8)\" y=\"\($y + 14)\" text-anchor=\"end\" fill=\"#1f2328\"><title>\($r.name | esc)</title>\($r.name | short | esc)</text>",
          "<rect x=\"\($x0)\" y=\"\($y + 3)\" width=\"\([$bw, 0.5] | max)\" height=\"15\" fill=\"#2f6fb0\"/>",
          (if ($r.err // null) != null and $r.err > 0 then
             "<line x1=\"\($x0 + ([$r.value - $r.err, 0] | max) / $m * $w)\" x2=\"\($x0 + ($r.value + $r.err) / $m * $w)\" y1=\"\($y + 10.5)\" y2=\"\($y + 10.5)\" stroke=\"#1f2328\" stroke-width=\"1.2\"/>"
           else empty end),
          "<text x=\"\($x0 + ([$bw, (($r.value + ($r.err // 0)) / $m * $w)] | max) + 6)\" y=\"\($y + 14)\" fill=\"#1f2328\">\($r.label // ($r.value | tostring) | esc)</text>"),
        "<text x=\"16\" y=\"\($h - 14)\" fill=\"#59636e\" font-size=\"11\">\($cap | esc)</text>",
        "</svg>"' <<< "$1"
}

# Flame graph (SVG) from folded stacks "a;b;c COUNT": width is the share of
# samples, frames ordered alphabetically per level, root at the bottom.
svg_flame() { # folded-file title caption
    gawk -v title="$2" -v cap="$3" '
        function esc(s) { gsub(/&/, "\\&amp;", s); gsub(/</, "\\&lt;", s); gsub(/>/, "\\&gt;", s); gsub(/"/, "\\&quot;", s); return s }
        function color(s,   h, i) { h = 0; for (i = 1; i <= length(s) && i <= 12; i++) h = (h * 31 + index("abcdefghijklmnopqrstuvwxyz_0123456789", substr(tolower(s), i, 1))) % 997
            return sprintf("rgb(%d,%d,%d)", 205 + h % 50, 80 + (h * 7) % 130, 40 + (h * 13) % 50) }
        function draw(key, x, d,   w, y, nm, nk, ks, i, c, cx, lab, srt, ord) {
            w = tot[key] / total * W; if (w < 0.3) return
            if (d > maxd) maxd = d
            y = H - 40 - (d + 1) * 17; nm = name[key]
            lab = nm; if (length(lab) * 6.5 > w - 6) lab = (w > 30 ? substr(lab, 1, int((w - 6) / 6.5) - 2) ".." : "")
            body = body sprintf("<g><title>%s (%d samples, %.2f%%)</title><rect x=\"%.2f\" y=\"%d\" width=\"%.2f\" height=\"16\" fill=\"%s\" rx=\"1\"/><text x=\"%.2f\" y=\"%d\">%s</text></g>\n",
                esc(nm), tot[key], 100 * tot[key] / total, x, y, w, color(nm), x + 3, y + 12, esc(lab))
            nk = split(kids[key], ks, "\034"); c = 0
            for (i = 1; i <= nk; i++) if (ks[i] != "") srt[name[ks[i]] "\035" ks[i]] = ks[i]
            c = asorti(srt, ord); cx = x
            for (i = 1; i <= c; i++) { draw(srt[ord[i]], cx, d + 1); cx += tot[srt[ord[i]]] / total * W }
        }
        {
            n = $NF; $NF = ""; sub(/ +$/, ""); k = split($0, fr, ";"); total += n; key = "ROOT"
            for (i = 1; i <= k; i++) { pk = key; key = key "\036" fr[i]
                if (!(key in tot)) { name[key] = fr[i]; kids[pk] = kids[pk] "\034" key }
                tot[key] += n }
            tot["ROOT"] += n
        }
        END {
            if (total == 0) { print "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"600\" height=\"60\"><text x=\"10\" y=\"30\">no samples</text></svg>"; exit }
            W = 1180; H = 4000; name["ROOT"] = "all"; maxd = 0; body = ""
            draw("ROOT", 10, 0)
            top = H - 40 - (maxd + 2) * 17 - 30; hh = H - top
            printf "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"%d\" viewBox=\"0 %d 1200 %d\" font-family=\"DejaVu Sans Mono, Menlo, monospace\" font-size=\"11\">\n", hh, top, hh
            printf "<rect x=\"0\" y=\"%d\" width=\"1200\" height=\"%d\" fill=\"#ffffff\"/>\n", top, hh
            printf "<text x=\"10\" y=\"%d\" font-size=\"15\" font-weight=\"bold\" font-family=\"DejaVu Sans, Helvetica, sans-serif\">%s</text>\n", top + 22, esc(title)
            printf "%s", body
            printf "<text x=\"10\" y=\"%d\" fill=\"#59636e\" font-family=\"DejaVu Sans, Helvetica, sans-serif\">%s (%d samples; hover a frame for its count)</text>\n</svg>\n", H - 14, esc(cap), total
        }' "$1"
}

# SM clock and power over the run (SVG line chart) from the smi CSV; samples
# with a clock-limit reason other than none/idle are marked red.
svg_smi() { # smi.csv interval-ms caption
    gawk -F', ' -v iv="$2" -v cap="$3" '
        function esc(s) { gsub(/&/, "\\&amp;", s); gsub(/</, "\\&lt;", s); return s }
        NF >= 10 && $3 ~ /^[0-9]/ { n++; c[n] = $3 + 0; p[n] = $5 + 0; r[n] = $10; gsub(/ /, "", r[n])
            if (c[n] > cmax) cmax = c[n]; if (p[n] > pmax) pmax = p[n] }
        END {
            W = 820; H = 200; x0 = 60; printf "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"900\" height=\"590\" viewBox=\"0 0 900 590\" font-family=\"DejaVu Sans, Helvetica, Arial, sans-serif\" font-size=\"12\">\n<rect width=\"900\" height=\"590\" fill=\"#ffffff\"/>\n"
            printf "<text x=\"16\" y=\"26\" font-size=\"15\" font-weight=\"bold\">GPU 0 SM clock and power during the run</text>\n"
            if (n < 2) { printf "<text x=\"16\" y=\"60\">fewer than two samples</text></svg>\n"; exit }
            panel(c, cmax, 50, "SM clock (MHz)", "#2f6fb0"); panel(p, pmax, 300, "power (W)", "#b0502f")
            printf "<text x=\"%d\" y=\"560\" text-anchor=\"middle\" fill=\"#59636e\">time (s), %d samples</text>\n", x0 + W / 2, n
            printf "<text x=\"16\" y=\"580\" fill=\"#59636e\" font-size=\"11\">%s</text>\n</svg>\n", esc(cap)
        }
        function panel(v, vmax, top, ylab, col,   i, x, y, pts, t, ymax) {
            ymax = vmax * 1.1; if (ymax <= 0) ymax = 1
            printf "<text x=\"%d\" y=\"%d\" fill=\"#59636e\">%s</text>\n", x0, top + 12, ylab
            printf "<line x1=\"%d\" x2=\"%d\" y1=\"%d\" y2=\"%d\" stroke=\"#d0d7de\"/>\n", x0, x0 + W, top + 20 + H, top + 20 + H
            for (t = 0; t <= 4; t++) { y = top + 20 + H - t / 4 * H
                printf "<line x1=\"%d\" x2=\"%d\" y1=\"%.1f\" y2=\"%.1f\" stroke=\"#eef1f4\"/><text x=\"%d\" y=\"%.1f\" text-anchor=\"end\" fill=\"#59636e\" font-size=\"10\">%.0f</text>\n", x0, x0 + W, y, y, x0 - 6, y + 3, ymax * t / 4 }
            for (t = 0; t <= 4; t++) printf "<text x=\"%.1f\" y=\"%d\" text-anchor=\"middle\" fill=\"#59636e\" font-size=\"10\">%.2f</text>\n", x0 + t / 4 * W, top + 34 + H, (n - 1) * iv / 1000 * t / 4
            pts = ""; for (i = 1; i <= n; i++) { x = x0 + (i - 1) / (n - 1) * W; y = top + 20 + H - v[i] / ymax * H; pts = pts sprintf("%.1f,%.1f ", x, y) }
            printf "<polyline points=\"%s\" fill=\"none\" stroke=\"%s\" stroke-width=\"1.6\"/>\n", pts, col
            for (i = 1; i <= n; i++) if (r[i] != "0x0000000000000000" && r[i] != "0x0000000000000001") {
                x = x0 + (i - 1) / (n - 1) * W; y = top + 20 + H - v[i] / ymax * H
                printf "<circle cx=\"%.1f\" cy=\"%.1f\" r=\"3.5\" fill=\"#cf222e\"><title>%s</title></circle>\n", x, y, r[i] }
        }' "$1"
}

# ---------------------------------------------------------------- probe

do_probe() {
    local perf pv="" ncu nv="" nsys sv="" vg="" sw=null hw=null ctl=null
    if perf=$(find_perf); then
        pv=$("$perf" --version 2>/dev/null)
        "$perf" stat -e task-clock -- true > /dev/null 2>&1 && sw=true || sw=false
        if "$perf" stat -x, -e cycles:u,instructions:u -- true 2>&1 | grep -qE '<not supported>|<not counted>|rror'; then hw=false; else hw=true; fi
        add_tool perf "$perf" "$pv"
    fi
    if command -v valgrind > /dev/null; then vg=$(valgrind --version); add_tool valgrind "$(command -v valgrind)" "$vg"; fi
    if ncu=$(find_ncu); then nv=$("$ncu" --version 2>/dev/null | grep -m1 -i version); add_tool ncu "$ncu" "$nv"; fi
    if nsys=$(find_nsys); then sv=$("$nsys" --version 2>/dev/null | head -1); add_tool nsys "$nsys" "$sv"; fi
    local admin
    admin=$(grep -h RmProfilingAdminOnly /proc/driver/nvidia/params 2>/dev/null | awk '{print $2}' || true)
    if [ "$gpucontrol" = 1 ]; then ctl=$(gpu_control "${ncu:-}"); fi
    local rec
    rec=$(jq -nc --arg schema "$SCHEMA" --argjson host "$(host_json)" --argjson tools "$tools" \
        --argjson sw "$sw" --argjson hw "$hw" --arg admin "$admin" --argjson ctl "$ctl" \
        --arg nvcc "$(command -v nvcc || true)" --arg inferno "$(command -v inferno-flamegraph || true)" '
        {schema: $schema, kind: "probe", host: $host, tools: $tools,
         capabilities: {
           cpu_sampling_software: $sw,
           cpu_hardware_counters: $hw,
           cpu_sampling: (if $hw == true then "hardware (cycles)" elif $sw == true then "software timer (cpu-clock): where time goes, no cycle or cache attribution" else "none" end),
           callgrind: ($tools.valgrind != null),
           topdown: ($hw == true),
           gpu_present: ($host.gpus | length > 0),
           ncu: ($tools.ncu != null), nsys: ($tools.nsys != null), nvcc: ($nvcc != ""),
           nvidia_profiling_admin_only: (if $admin == "" then null else ($admin == "1") end),
           ncu_control_kernel: $ctl,
           inferno_flamegraph: ($inferno != "")}}')
    if [ -n "$out" ]; then mkdir -p "$out"; printf '%s\n' "$rec" | jq . > "$out/probe.json"; fi
    jq . <<< "$rec"
    jq -r '.capabilities | "profile: cpu sampling: \(.cpu_sampling); topdown: \(.topdown); callgrind: \(.callgrind); gpu: \(.gpu_present); ncu: \(.ncu); nsys: \(.nsys)"' <<< "$rec" >&2
}

# Can ncu read counters here at all?  A one-block control kernel, run plainly
# (checking its output) and under ncu with one metric.  Separates "this host
# withholds counters" from "this kernel will not profile" (ecc2k130/TUNING.md).
gpu_control() { # ncu-path
    local ncu=$1 d
    [ -n "$ncu" ] && command -v nvcc > /dev/null && command -v nvidia-smi > /dev/null || { echo null; return; }
    d=$(mktemp -d)
    cat > "$d/control.cu" << 'EOF'
#include <cstdio>
__global__ void profileControlKernel(unsigned *o) { unsigned t = threadIdx.x; o[t] = t * 2654435761u; }
int main() {
    unsigned *d, h[256]; cudaMalloc(&d, sizeof h);
    profileControlKernel<<<1, 256>>>(d); cudaMemcpy(h, d, sizeof h, cudaMemcpyDeviceToHost);
    for (unsigned t = 0; t < 256; t++) if (h[t] != t * 2654435761u) { printf("control: wrong output at %u\n", t); return 1; }
    printf("control: ok\n"); return 0;
}
EOF
    local b=1 r=1 p=1 log=""
    nvcc -O2 -o "$d/control" "$d/control.cu" > "$d/build.log" 2>&1 && b=0
    [ "$b" = 0 ] && { "$d/control" > "$d/run.log" 2>&1 && r=0; }
    [ "$r" = 0 ] && { timeout 300 "$ncu" --kernel-name profileControlKernel --metrics gpu__time_duration.sum "$d/control" > "$d/ncu.log" 2>&1 && grep -q profileControlKernel "$d/ncu.log" && p=0; }
    log=$(cat "$d/build.log" "$d/run.log" "$d/ncu.log" 2>/dev/null | tail -40)
    jq -nc --argjson b "$b" --argjson r "$r" --argjson p "$p" --arg log "$log" \
        '{built: ($b == 0), ran_and_checked: ($r == 0), ncu_profiled: ($p == 0),
          counter_permission_denied: ($log | test("ERR_NVGPUCTRPERM")),
          failed_to_prepare: ($log | test("Failed to prepare kernel")), log_tail: $log}'
    rm -rf "$d"
}

# ---------------------------------------------------------------- cpu

fold_perf_script() { # perf data -> folded stacks on stdout
    "$1" script -i "$2" -F comm,ip,sym,dso 2> /dev/null | gawk '
        function flush() { if (n > 0) { s = comm; for (i = n; i >= 1; i--) s = s ";" st[i]; cnt[s]++ } n = 0 }
        /^[^ \t]/ { flush(); comm = $1; next }
        /^[ \t]*$/ { flush(); next }
        {
            line = $0; sub(/^[ \t]*[0-9a-f]+ /, "", line)
            dso = ""; if (match(line, / \(([^()]*)\)$/)) { dso = substr(line, RSTART + 2, RLENGTH - 3); line = substr(line, 1, RSTART - 1) }
            sub(/\+0x[0-9a-f]+$/, "", line)
            if (line == "" || line == "[unknown]") { b = dso; sub(/.*\//, "", b); line = "[" (b == "" ? "unknown" : b) "]" }
            gsub(/;/, ":", line); st[++n] = line
        }
        END { flush(); for (s in cnt) print s, cnt[s] }' | sort
}

cpu_sample() { # perf
    local perf=$1 hw ev cg
    hw=1; "$perf" stat -x, -e cycles:u -- true 2>&1 | grep -qE '<not supported>|<not counted>' && hw=0
    ev=${event:-$([ "$hw" = 1 ] && echo cycles:u || echo cpu-clock)}
    case $callgraph in fp) cg=fp ;; dwarf) cg=dwarf,16384 ;; lbr) cg=lbr ;; *) die "--call-graph fp|dwarf|lbr" ;; esac
    run_profiled "$perf" record -F "$freq" -e "$ev" --call-graph "$cg" -o "$out/perf.data" -- "${cmd[@]}"
    grep -q 'perf record: Captured' "$out/command.stderr" || {
        grep -qiE 'perf_event_paranoid|Permission|access to performance monitoring' "$out/command.stderr" &&
            unsupported "perf record denied: perf_event_paranoid=$(cat /proc/sys/kernel/perf_event_paranoid) (set it to 2 or lower)"
        profiler_failed "perf record wrote no data (exit $rc); see command.stderr"; }
    "$perf" report -i "$out/perf.data" --stdio --no-children --sort dso,sym --percent-limit 0.5 > "$out/report.txt" 2> /dev/null || true
    "$perf" report -i "$out/perf.data" --stdio --no-children -g none --sort dso,sym -F overhead,sample,dso,sym \
        -t $'\t' --percent-limit 0 2> /dev/null | grep -v '^#' | grep -v '^[[:space:]]*$' > "$out/hotspots.tsv" || true
    fold_perf_script "$perf" "$out/perf.data" > "$out/folded.txt"
    local lost
    lost=$("$perf" report -i "$out/perf.data" --stats 2> /dev/null | awk '/LOST/ && $NF ~ /^[0-9]+$/ {s += $NF} END {print s + 0}' || echo 0)
    gawk -F'\t' '{ gsub(/^ +| +$/, "", $2); gsub(/^ +| +$/, "", $3); gsub(/^ +| +$/, "", $4); sub(/^\[[.k]\] /, "", $4); print $2 "\t" $3 "\t" $4 }' "$out/hotspots.tsv" \
        | jq -Rsc --argjson top "$top" --arg ev "$ev" --argjson hw "$hw" --arg freq "$freq" --arg cg "$callgraph" --argjson lost "$lost" '
        split("\n") | map(select(length > 0) | split("\t") | {samples: (.[0] | tonumber), dso: .[1], symbol: .[2]}) as $rows |
        ($rows | map(.samples) | add // 0) as $n |
        {schema: "profile/v1", kind: "cpu-hotspots", method: "perf record", event: $ev,
         event_class: (if $hw == 1 and ($ev | test("cpu-clock|task-clock") | not) then "hardware" else "software timer" end),
         frequency_hz: ($freq | tonumber), call_graph: $cg, total_samples: $n, lost_events: $lost,
         hotspots: ($rows | sort_by(-.samples) | .[:$top] | map(. + {
             self_pct: (100 * .samples / $n),
             se_pct: (100 * ((.samples / $n) * (1 - .samples / $n) / $n | sqrt))}))}' > "$out/hotspots.json"
    add_tool perf "$perf" "$("$perf" --version)"
}

cpu_callgrind() {
    command -v valgrind > /dev/null || unsupported "no perf sampling and no valgrind"
    add_tool valgrind "$(command -v valgrind)" "$(valgrind --version)"
    run_profiled valgrind --tool=callgrind --callgrind-out-file="$out/callgrind.out" -- "${cmd[@]}"
    [ -s "$out/callgrind.out" ] || profiler_failed "callgrind wrote no profile (exit $rc); see command.stderr"
    callgrind_annotate --auto=no --inclusive=no --threshold=100 "$out/callgrind.out" > "$out/annotate.txt" 2> /dev/null || true
    callgrind_annotate --auto=no --inclusive=yes --threshold=99 "$out/callgrind.out" > "$out/annotate-inclusive.txt" 2> /dev/null || true
    gawk 'BEGIN { OFS = "\t" }
        /PROGRAM TOTALS/ { t = $1; gsub(/,/, "", t); print "TOTAL", t, "", ""; next }
        /file:function/ { tbl = 1; next }
        tbl && /^-+$|^$/ { if (seen) tbl = 0; next }
        tbl && match($0, /^ *([0-9,]+) +\( *[0-9.]+%\) +(.*)$/, m) { seen = 1
            ir = m[1]; gsub(/,/, "", ir); rest = m[2]; obj = ""
            if (match(rest, / \[([^]]*)\]$/, o)) { obj = o[1]; rest = substr(rest, 1, RSTART - 1) }
            p = index(rest, ":"); file = substr(rest, 1, p - 1); fn = substr(rest, p + 1)
            print ir, obj, fn, file }' "$out/annotate.txt" \
        | jq -Rsc --argjson top "$top" '
        split("\n") | map(select(length > 0) | split("\t")) as $rows |
        ($rows | map(select(.[0] == "TOTAL")) | .[0][1] | tonumber? // null) as $tot |
        ($rows | map(select(.[0] != "TOTAL") | {ir: (.[0] | tonumber), dso: .[1], symbol: .[2], file: .[3]})) as $h |
        {schema: "profile/v1", kind: "cpu-hotspots", method: "callgrind", event: "Ir (instructions executed, simulated)",
         event_class: "deterministic instruction count", total_ir: $tot,
         hotspots: ($h | sort_by(-.ir) | .[:$top] | map(. + {self_pct: (if $tot then 100 * .ir / $tot else null end)}))}' > "$out/hotspots.json"
}

do_cpu() {
    setup_out
    local perf=""
    perf=$(find_perf || true)
    case $mode in
        auto) if [ -n "$perf" ] && "$perf" stat -e task-clock -- true > /dev/null 2>&1; then cpu_sample "$perf"; else cpu_callgrind; fi ;;
        sample) [ -n "$perf" ] || unsupported "no working perf binary (Ubuntu: apt-get install linux-tools-common linux-tools-generic)"; cpu_sample "$perf" ;;
        callgrind) cpu_callgrind ;;
        *) die "--mode auto|sample|callgrind" ;;
    esac
    finish_cpu
}

finish_cpu() {
    local st=ok
    [ "$rc" = 0 ] || st=command_failed
    write_manifest "$st" "$([ "$rc" = 0 ] || echo "command exited $rc")" "$(jq -c '{hotspots_method: .method, event: .event}' "$out/hotspots.json")"
    render "$out"
    [ "$rc" = 0 ] || exit 4
}

# ---------------------------------------------------------------- topdown

do_topdown() {
    setup_out
    local perf
    perf=$(find_perf) || unsupported "no working perf binary"
    add_tool perf "$perf" "$("$perf" --version)"
    "$perf" stat -x, -e cycles:u -- true 2>&1 | grep -qE '<not supported>|<not counted>' &&
        unsupported "no hardware PMU: top-down needs core counters (event sources: $(ls /sys/bus/event_source/devices | tr '\n' ' '))"
    local grp="TopdownL$level"
    "$perf" list metricgroup 2> /dev/null | grep -q "$grp" || grp=""
    if [ -n "$grp" ]; then
        run_profiled "$perf" stat -x, --no-big-num -M "$grp" -o "$out/topdown.csv" -- "${cmd[@]}"
    else
        run_profiled "$perf" stat -x, --no-big-num --topdown -o "$out/topdown.csv" -- "${cmd[@]}"
    fi
    [ -s "$out/topdown.csv" ] || profiler_failed "perf stat wrote nothing; see command.stderr"
    # -x, rows: value,unit,event,run-time,pct-running,metric-value,metric-unit
    { grep -v '^#' "$out/topdown.csv" | grep -v '^$' || true; } | jq -Rsc --arg grp "${grp:---topdown}" '
        split("\n") | map(select(length > 0) | split(",")) |
        {schema: "profile/v1", kind: "topdown", group: $grp,
         events: map({event: .[2], value: (.[0] | tonumber? // .[0]), running_pct: (.[4] | tonumber? // null)}),
         metrics: map(select((.[6] // "") != "") | {metric: .[6], value: (.[5] | tonumber? // null)}),
         multiplexed: map(select((.[4] | tonumber? // 100) < 100) | .[2])}' > "$out/topdown.json"
    # perf writes its "# started on" header even when the metric group cannot
    # be scheduled, so an empty table means perf failed, not the command.
    [ "$(jq '.events | length' "$out/topdown.json")" -gt 0 ] ||
        profiler_failed "perf stat counted nothing for ${grp:---topdown}: $(grep -m1 -v '^[[:space:]]*$' "$out/command.stderr" || echo 'no message')"
    write_manifest "$([ "$rc" = 0 ] && echo ok || echo command_failed)" "" '{}'
    render "$out"
    [ "$rc" = 0 ] || exit 4
}

# ---------------------------------------------------------------- ncu

# Metrics every ncu profile records beside its sections, so summaries compare
# across runs.  Missing ones (older ncu, other architectures) come out null.
NCU_METRICS="gpu__time_duration.sum,sm__throughput.avg.pct_of_peak_sustained_elapsed,gpu__compute_memory_throughput.avg.pct_of_peak_sustained_elapsed,dram__throughput.avg.pct_of_peak_sustained_elapsed,sm__warps_active.avg.pct_of_peak_sustained_active,sm__maximum_warps_per_active_cycle_pct,smsp__issue_active.avg.pct_of_peak_sustained_active,smsp__inst_executed.sum,launch__registers_per_thread,launch__block_size,launch__grid_size,launch__occupancy_limit_registers,launch__occupancy_limit_shared_mem,launch__occupancy_limit_warps,l1tex__t_sectors_pipe_lsu_mem_local_op_ld.sum,l1tex__t_sectors_pipe_lsu_mem_local_op_st.sum,sm__cycles_elapsed.avg.per_second"
do_ncu() {
    setup_out
    local ncu nv year cc major
    ncu=$(find_ncu) || unsupported "no Nsight Compute (ncu) binary: install NVIDIA's versioned nsight-compute-<year>.<n> package, or set NCU"
    nv=$("$ncu" --version 2>/dev/null | grep -m1 -i version)
    add_tool ncu "$ncu" "$nv"
    command -v nvidia-smi > /dev/null && [ "$(gpus_json | jq length)" -gt 0 ] || unsupported "no NVIDIA GPU visible (nvidia-smi absent or lists none)"
    # Blackwell (compute capability 10.x/12.x) needs Nsight Compute 2025.1 or
    # newer; 2022.4 failed with "Failed to prepare kernel" (ecc2k130/TUNING.md).
    year=$(grep -oE '20[0-9]{2}\.[0-9]+' <<< "$nv" | head -1 | cut -d. -f1)
    cc=$(gpus_json | jq -r '.[0].compute_cap'); major=${cc%%.*}
    if [ "$force_version" = 0 ] && [ -n "$year" ] && [ "${major:-0}" -ge 10 ] && [ "$year" -lt 2025 ]; then
        unsupported "ncu $nv predates compute capability $cc (Blackwell needs 2025.1+); install a versioned nsight-compute package or pass --force-version"
    fi
    local args=(--target-processes all --clock-control "$clock" --cache-control all -f -o "$out/profile")
    [ -n "$kernel" ] && args+=(--kernel-name "regex:$kernel")
    args+=(--launch-skip "$lskip" --launch-count "$lcount")
    if [ -n "$nset" ]; then args+=(--set "$nset"); else
        local s; for s in "${sections[@]}"; do args+=(--section "$s"); done
    fi
    args+=(--metrics "$NCU_METRICS${metrics:+,$metrics}")
    [ "$source" = 1 ] && args+=(--import-source yes)
    run_profiled "$ncu" "${args[@]}" "${cmd[@]}"
    local log="$out/command.stderr" extra
    extra=$(jq -nc --arg k "$kernel" --argjson s "$lskip" --argjson c "$lcount" --arg set "$nset" --arg clock "$clock" \
        --args '{ncu: {kernel_regex: $k, launch_skip: $s, launch_count: $c, set: $set, sections: $ARGS.positional,
                 clock_control: $clock, cache_control: "all", replay: "kernel (ncu default): every profiled launch is replayed, so its duration is not a throughput figure"}}' \
        -- "${sections[@]}")
    cat "$out/command.stdout" >> "$log"
    grep -q ERR_NVGPUCTRPERM "$log" && unsupported "ERR_NVGPUCTRPERM: the driver withholds GPU performance counters from this user (NVreg_RestrictProfilingToAdminUsers=0, or run with the provider's profiling capability)" "$extra"
    grep -q 'Failed to prepare kernel' "$log" && profiler_failed "ncu: Failed to prepare kernel for profiling (check the ncu version against the GPU; run 'probe --gpu-control' to separate host from kernel; ecc2k130/TUNING.md)" "$extra"
    [ -s "$out/profile.ncu-rep" ] || profiler_failed "ncu wrote no report (exit $rc): $(grep -m1 -E '==ERROR==|==WARNING== No kernels' "$log" || echo 'no kernel matched the filter?')" "$extra"
    "$ncu" -i "$out/profile.ncu-rep" --csv --page raw --print-units base > "$out/raw.csv" 2> /dev/null ||
        "$ncu" -i "$out/profile.ncu-rep" --csv --page raw > "$out/raw.csv"
    "$ncu" -i "$out/profile.ncu-rep" --page details --print-details all > "$out/details.txt" 2> /dev/null ||
        "$ncu" -i "$out/profile.ncu-rep" --page details > "$out/details.txt" 2> /dev/null || true
    ncu_summarize "$out/raw.csv" > "$out/kernels.json"
    [ "$(jq '.launches | length' "$out/kernels.json")" -gt 0 ] || profiler_failed "ncu report holds no kernel launches" "$extra"
    write_manifest "$([ "$rc" = 0 ] && echo ok || echo command_failed)" "" "$extra"
    render "$out"
    [ "$rc" = 0 ] || exit 4
}

ncu_summarize() { # raw.csv
    csv2json "$1" | jq -c "$NUM"'
        (if length > 0 and (.[0].ID // "x") == "" then .[0] else {} end) as $units |
        map(select((.ID // "") != "")) as $rows |
        {schema: "profile/v1", kind: "ncu-kernels", units: $units,
         launches: ($rows | map(. as $r | {
            id: (.ID | num), kernel: (.["Kernel Name"] // .["Function Name"] // null),
            block: (.["Block Size"] // null), grid: (.["Grid Size"] // null), device: (.Device // null), cc: (.CC // null),
            duration_ns: (.["gpu__time_duration.sum"] // null | num),
            sm_throughput_pct: (.["sm__throughput.avg.pct_of_peak_sustained_elapsed"] // null | num),
            memory_throughput_pct: (.["gpu__compute_memory_throughput.avg.pct_of_peak_sustained_elapsed"] // null | num),
            dram_throughput_pct: (.["dram__throughput.avg.pct_of_peak_sustained_elapsed"] // null | num),
            achieved_occupancy_pct: (.["sm__warps_active.avg.pct_of_peak_sustained_active"] // null | num),
            theoretical_occupancy_pct: (.["sm__maximum_warps_per_active_cycle_pct"] // null | num),
            issue_active_pct: (.["smsp__issue_active.avg.pct_of_peak_sustained_active"] // null | num),
            instructions: (.["smsp__inst_executed.sum"] // null | num),
            registers_per_thread: (.["launch__registers_per_thread"] // null | num),
            occupancy_limits: {registers: (.["launch__occupancy_limit_registers"] // null | num),
                               shared_mem: (.["launch__occupancy_limit_shared_mem"] // null | num),
                               warps: (.["launch__occupancy_limit_warps"] // null | num)},
            local_memory_sectors: {load: (.["l1tex__t_sectors_pipe_lsu_mem_local_op_ld.sum"] // null | num),
                                   store: (.["l1tex__t_sectors_pipe_lsu_mem_local_op_st.sum"] // null | num)},
            sm_clock_hz: (.["sm__cycles_elapsed.avg.per_second"] // null | num),
            stalls_per_issue: ([to_entries[] | select(.key | test("^smsp__average_warps?_issue_stalled_.*_per_issue_active\\.ratio$")) |
                {reason: (.key | sub("^smsp__average_warps?_issue_stalled_"; "") | sub("_per_issue_active\\.ratio$"; "")), value: (.value | num)} |
                select(.value | type == "number")] | sort_by(-.value)),
            pipe_utilization_pct: ([to_entries[] | select(.key | test("^sm__inst_executed_pipe_[a-z0-9_]+\\.avg\\.pct_of_peak_sustained_active$")) |
                {pipe: (.key | sub("^sm__inst_executed_pipe_"; "") | sub("\\.avg\\.pct_of_peak_sustained_active$"; "")), value: (.value | num)} |
                select(.value | type == "number" and . > 0)] | sort_by(-.value))}))}'
}

# ---------------------------------------------------------------- nsys

do_nsys() {
    setup_out
    local nsys sv
    nsys=$(find_nsys) || unsupported "no Nsight Systems (nsys) binary: install a versioned nsight-systems package, or set NSYS"
    sv=$("$nsys" --version 2>/dev/null | head -1)
    add_tool nsys "$nsys" "$sv"
    local args=(profile --trace="$trace" --force-overwrite=true -o "$out/profile")
    if [ "$cpusample" = 1 ]; then args+=(--sample=cpu); else args+=(--sample=none --cpuctxsw=none); fi
    if [ "$gpumetrics" = 1 ]; then
        "$nsys" profile --help 2>&1 | grep -q -- '--gpu-metrics-devices' && args+=(--gpu-metrics-devices=all) || args+=(--gpu-metrics-device=all)
    fi
    run_profiled "$nsys" "${args[@]}" "${cmd[@]}"
    [ -s "$out/profile.nsys-rep" ] || [ -s "$out/profile.qdrep" ] ||
        profiler_failed "nsys wrote no report (exit $rc); see command.stderr"
    local rep="$out/profile.nsys-rep"; [ -s "$rep" ] || rep="$out/profile.qdrep"
    # Report names changed in 2023 (gpukernsum -> cuda_gpu_kern_sum, ...).
    nsys_stat() { # new old dest
        "$nsys" stats --force-export=true -r "$1" -f csv -o "$out/stats" "$rep" > /dev/null 2>&1 && [ -s "$out/stats_$1.csv" ] && { mv "$out/stats_$1.csv" "$out/$3.csv"; return 0; }
        "$nsys" stats --force-export=true -r "$2" -f csv -o "$out/stats" "$rep" > /dev/null 2>&1 && [ -s "$out/stats_$2.csv" ] && { mv "$out/stats_$2.csv" "$out/$3.csv"; return 0; }
        return 1
    }
    nsys_stat cuda_gpu_kern_sum gpukernsum kern_sum || true
    nsys_stat cuda_api_sum cudaapisum api_sum || true
    nsys_stat cuda_gpu_mem_time_sum gpumemtimesum memtime_sum || true
    nsys_stat cuda_gpu_trace gputrace gpu_trace || true
    nsys_stat nvtx_sum nvtxsum nvtx_sum || true
    local k='[]' a='[]' m='[]' cov=null n=null
    [ -s "$out/kern_sum.csv" ] && k=$(csv2json "$out/kern_sum.csv" | jq -c "$NUM map(with_entries(.value |= num))")
    [ -s "$out/api_sum.csv" ] && a=$(csv2json "$out/api_sum.csv" | jq -c "$NUM map(with_entries(.value |= num))")
    [ -s "$out/memtime_sum.csv" ] && m=$(csv2json "$out/memtime_sum.csv" | jq -c "$NUM map(with_entries(.value |= num))")
    [ -s "$out/nvtx_sum.csv" ] && n=$(csv2json "$out/nvtx_sum.csv" | jq -c "$NUM map(with_entries(.value |= num))")
    # Kernel coverage: union of kernel intervals over the span from the first
    # kernel start to the last kernel end.  Low coverage means the GPU waits on
    # the host (launch overhead, synchronisation, copies), not on arithmetic.
    if [ -s "$out/gpu_trace.csv" ]; then
        cov=$(csv2json "$out/gpu_trace.csv" | jq -c "$NUM"'
            map({s: (.["Start (ns)"] // .Start | num), d: (.["Duration (ns)"] // .Duration | num), name: (.Name // "")}) |
            map(select((.s | type) == "number" and (.d | type) == "number" and (.name | test("^\\[CUDA mem") | not))) | sort_by(.s) |
            if length == 0 then null else
            (reduce .[] as $k ({busy: 0, cur_s: null, cur_e: null};
                if .cur_e == null then .cur_s = $k.s | .cur_e = ($k.s + $k.d)
                elif $k.s <= .cur_e then .cur_e = ([.cur_e, $k.s + $k.d] | max)
                else .busy += (.cur_e - .cur_s) | .cur_s = $k.s | .cur_e = ($k.s + $k.d) end)
             | .busy + (.cur_e - .cur_s)) as $busy |
            (.[0].s) as $t0 | (map(.s + .d) | max) as $t1 |
            {kernel_launches: length, span_ns: ($t1 - $t0), kernel_busy_ns: $busy,
             kernel_coverage: (if $t1 > $t0 then $busy / ($t1 - $t0) else null end)} end')
    fi
    jq -n --argjson k "$k" --argjson a "$a" --argjson m "$m" --argjson n "$n" --argjson cov "$cov" --argjson top "$top" \
        --arg trace "$trace" '{schema: "profile/v1", kind: "nsys-summary", trace: $trace,
         kernels: ($k | .[:$top]), cuda_api: ($a | .[:$top]), memory_ops: $m, nvtx: $n, timeline: $cov}' > "$out/summary.json"
    write_manifest "$([ "$rc" = 0 ] && echo ok || echo command_failed)" "" "$(jq -nc --arg t "$trace" --argjson c "$cpusample" --argjson g "$gpumetrics" '{nsys: {trace: $t, cpu_sample: ($c == 1), gpu_metrics: ($g == 1)}}')"
    render "$out"
    [ "$rc" = 0 ] || exit 4
}

# ---------------------------------------------------------------- smi

do_smi() {
    setup_out
    command -v nvidia-smi > /dev/null || unsupported "no nvidia-smi"
    add_tool nvidia-smi "$(command -v nvidia-smi)" "$(nvidia-smi --version 2>/dev/null | grep -m1 -i 'smi version' || true)"
    local q="timestamp,index,clocks.sm,clocks.mem,power.draw,temperature.gpu,utilization.gpu,utilization.memory,pstate"
    local reasons=clocks_event_reasons.active
    nvidia-smi --query-gpu=$reasons --format=csv,noheader > /dev/null 2>&1 || reasons=clocks_throttle_reasons.active
    nvidia-smi --query-gpu="$q,$reasons" --format=csv,noheader,nounits -lms "$interval" > "$out/smi.csv" 2> "$out/smi.stderr" &
    local spid=$!
    sleep 0.3
    run_profiled "${cmd[@]}"
    kill "$spid" 2> /dev/null || true; wait "$spid" 2> /dev/null || true
    gawk -F', ' -v iv="$interval" '
        function med(a, n,   i, j, t) { for (i = 2; i <= n; i++) { t = a[i]; for (j = i - 1; j >= 1 && a[j] > t; j--) a[j + 1] = a[j]; a[j + 1] = t }
            return n == 0 ? "null" : (n % 2 ? a[(n + 1) / 2] : (a[n / 2] + a[n / 2 + 1]) / 2) }
        NF >= 10 && $3 ~ /^[0-9]/ { n++; sm[n] = $3; pw[n] = $5; ut[n] = $7
            if (min == "" || $3 < min) min = $3; if ($3 > max) max = $3; if ($6 > tmax) tmax = $6; if ($5 > pmax) pmax = $5
            r = $10; gsub(/ /, "", r); seen[r]++ }
        END { printf "{\"schema\":\"profile/v1\",\"kind\":\"smi-summary\",\"interval_ms\":%d,\"samples\":%d,", iv, n
              printf "\"sm_clock_mhz\":{\"median\":%s,\"min\":%s,\"max\":%s},", med(sm, n), (min == "" ? "null" : min), (n ? max : "null")
              printf "\"power_w\":{\"median\":%s,\"max\":%s},\"gpu_util_pct_median\":%s,\"temperature_c_max\":%s,", med(pw, n), (n ? pmax : "null"), med(ut, n), (n ? tmax : "null")
              printf "\"clock_reason_masks\":{"; c = 0; for (r in seen) printf "%s\"%s\":%d", (c++ ? "," : ""), r, seen[r]; print "}}" }' "$out/smi.csv" > "$out/smi-summary.json"
    write_manifest "$([ "$rc" = 0 ] && echo ok || echo command_failed)" "" "$(jq -nc --arg r "$reasons" --argjson i "$interval" '{smi: {interval_ms: $i, reason_field: $r}}')"
    render "$out"
    [ "$rc" = 0 ] || exit 4
}

# ---------------------------------------------------------------- report

# The Markdown subset summary.md uses (headings, bullets, bold, code spans,
# links, pipe tables) as Typst markup.
md2typ() { # summary.md
    gawk '
        function esc(t) { gsub(/\\/, "\\\\", t); gsub(/[#$@<>_*~=\[\]]/, "\\\\&", t); gsub(/--/, "\\-\\-", t); return t }
        function inl(t,   out, n, parts, i, seg) {
            gsub(/\*\*/, "\001", t)
            while (match(t, /\[[^]]*\]\([^)]*\)/)) {
                seg = substr(t, RSTART, RLENGTH); i = index(seg, "](")
                t = substr(t, 1, RSTART - 1) "\002" substr(seg, 2, i - 2) "\003" substr(seg, i + 2, length(seg) - i - 2) "\004" substr(t, RSTART + RLENGTH) }
            n = split(t, parts, "`"); out = ""
            for (i = 1; i <= n; i++) {
                if (i % 2 == 0) { seg = parts[i]; out = out "#raw(\"" gensub(/(["\\])/, "\\\\\\1", "g", seg) "\")" }
                else { seg = esc(parts[i]); gsub(/\001/, "*", seg)
                    while (match(seg, /\002[^\003]*\003[^\004]*\004/)) {
                        lk = substr(seg, RSTART + 1, RLENGTH - 2); split(lk, lp, "\003")
                        seg = substr(seg, 1, RSTART - 1) "#link(\"" lp[2] "\")[" lp[1] "]" substr(seg, RSTART + RLENGTH) }
                    out = out seg } }
            return out }
        function flush(   i, j, c) {
            if (!nt) return
            printf "#table(columns: %d,\n", tc
            for (i = 1; i <= nt; i++) { c = split(rows[i], cell, "\005")
                for (j = 2; j < c; j++) { v = cell[j]; gsub(/^ +| +$/, "", v); printf "  [%s%s%s],", (i == 1 ? "*" : ""), inl(v), (i == 1 ? "*" : "") }
                printf "\n" }
            print ")"; nt = 0 }
        /^\|/ { line = $0; gsub(/\\\|/, "\006", line); gsub(/\|/, "\005", line); gsub(/\006/, "|", line)
            if (line ~ /^\005[-: \005]+$/) next
            rows[++nt] = line; if (nt == 1) tc = split(line, tmp, "\005") - 2; next }
        { flush() }
        /^## / { print "== " inl(substr($0, 4)); next }
        /^# / { print "= " inl(substr($0, 3)); next }
        /^- / { print "- " inl(substr($0, 3)); next }
        { print inl($0) }
        END { flush() }' "$1"
}

# summary.md, the graphs and a Typst source for the PDF, from the JSON in DIR.
# Layout only: every number comes from a file the profiler wrote.
render() { # dir
    local d=$1 m title id graphs=()
    m="$d/manifest.json"
    [ -s "$m" ] || die "no manifest.json in $d"
    id=$(basename "$d")
    title="$(jq -r '.subcommand' "$m") profile: $(jq -r '.label // "" | if . == "" then "unlabelled" else . end' "$m")"
    {
        echo "# $title"
        echo
        jq -r '"- status: **\(.status)**\(if .reason then " (\(.reason))" else "" end)",
               "- command: `\(.command | map(if test("[ \"]") then @json else . end) | join(" "))`",
               "- binary: \(if .binary then "`\(.binary.path)` sha256 `\(.binary.sha256[0:16])…`" else "not resolved" end)",
               "- commit: \(if .git then "`\(.git.commit[0:12])` (\(.git.dirty_tracked_files) dirty tracked files)" else "not a git checkout" end)",
               "- host: \(.host.cpu_model), \(.host.logical_cpus) logical CPUs, kernel \(.host.kernel), PMU \(if .host.hardware_pmu then "present" else "absent" end), perf_event_paranoid \(.host.perf_event_paranoid)",
               (.host.gpus[] | "- GPU \(.index): \(.name), cc \(.compute_cap), driver \(.driver), max SM clock \(.max_sm_clock_mhz) MHz, power limit \(.power_limit_w) W"),
               (.tools | to_entries[] | "- \(.key): \(.value.version)"),
               "- isolation: \(.isolation); wall \(.wall_seconds // "n/a") s (profiled run: not a timing measurement)",
               "- started: \(.started_utc // "n/a")"' "$m"
        echo
    } > "$d/summary.md"
    if [ -s "$d/hotspots.json" ]; then
        local h="$d/hotspots.json" cap
        if [ "$(jq -r .method "$h")" = callgrind ]; then
            cap="callgrind Ir, self; total $(jq -r .total_ir "$h") Ir; run $id"
            svg_bars "$(jq -c '.hotspots | .[:20] | map({name: .symbol, value: .self_pct, label: "\(.self_pct * 100 | round / 100)%"})' "$h")" \
                "Self instructions by function (callgrind)" "% of program Ir (self)" "$cap" > "$d/hotspots.svg"
            jq -r '"## Hotspots (callgrind, self Ir)\n\nTotal: \(.total_ir) Ir (deterministic, simulated; not cycles).\n\n| # | self % | Ir | function | object |\n|---|---:|---:|---|---|",
                   (.hotspots | to_entries[] | "| \(.key + 1) | \(.value.self_pct * 100 | round / 100) | \(.value.ir) | `\(.value.symbol | gsub("\\|"; "\\|"))` | \(.value.dso | split("/") | last) |")' "$h" >> "$d/summary.md"
        else
            cap="perf record -e $(jq -r .event "$h") at $(jq -r .frequency_hz "$h") Hz, $(jq -r .total_samples "$h") samples; bars ±1 binomial s.e.; run $id"
            svg_bars "$(jq -c '.hotspots | .[:20] | map({name: .symbol, value: .self_pct, err: .se_pct, label: "\(.self_pct * 10 | round / 10)% (\(.samples))"})' "$h")" \
                "Self samples by function (perf, $(jq -r .event_class "$h"))" "% of samples (self), ±1 s.e." "$cap" > "$d/hotspots.svg"
            jq -r '"## Hotspots (perf, self samples)\n\nEvent `\(.event)` (\(.event_class)), \(.frequency_hz) Hz, call graph `\(.call_graph)`, \(.total_samples) samples, \(.lost_events) lost events. s.e. is the binomial standard error of the share; shares within ~2 s.e. of each other are not ordered by this profile.\(if .total_samples < 1000 then "\n\n**Fewer than 1000 samples: lengthen the run or raise --freq before reading small shares.**" else "" end)\n\n| # | self % | ±s.e. | samples | symbol | object |\n|---|---:|---:|---:|---|---|",
                   (.hotspots | to_entries[] | "| \(.key + 1) | \(.value.self_pct * 100 | round / 100) | \(.value.se_pct * 100 | round / 100) | \(.value.samples) | `\(.value.symbol | gsub("\\|"; "\\|"))` | \(.value.dso | split("/") | last) |")' "$h" >> "$d/summary.md"
            if [ -s "$d/folded.txt" ]; then
                if command -v inferno-flamegraph > /dev/null; then
                    inferno-flamegraph --title "$title" < "$d/folded.txt" > "$d/flame.svg" 2> /dev/null || svg_flame "$d/folded.txt" "$title" "run $id" > "$d/flame.svg"
                else
                    svg_flame "$d/folded.txt" "$title" "perf $(jq -r .event "$h"), run $id" > "$d/flame.svg"
                fi
                graphs+=(flame.svg)
                printf '\nFlame graph: [flame.svg](flame.svg) (folded stacks in `folded.txt`).\n' >> "$d/summary.md"
            fi
        fi
        graphs=(hotspots.svg "${graphs[@]}")
    fi
    if [ -s "$d/topdown.json" ]; then
        svg_bars "$(jq -c '.metrics | map({name: .metric, value: .value, label: (.value | tostring)})' "$d/topdown.json")" \
            "Top-down breakdown ($(jq -r .group "$d/topdown.json"))" "metric value (perf units)" "perf stat -M, run $id" > "$d/topdown.svg"
        graphs+=(topdown.svg)
        jq -r '"## Top-down (\(.group))\n\n| metric | value |\n|---|---:|", (.metrics[] | "| \(.metric) | \(.value) |"),
               (if (.multiplexed | length) > 0 then "\n**Multiplexed events: \(.multiplexed | join(", ")); the values are scaled estimates.**" else empty end)' "$d/topdown.json" >> "$d/summary.md"
    fi
    if [ -s "$d/kernels.json" ]; then
        local k="$d/kernels.json"
        svg_bars "$(jq -c '[.launches[0].stalls_per_issue[]? | select(.value > 0.01 and .reason != "selected")] | .[:16] | map({name: .reason, value: .value, label: (.value * 100 | round / 100 | tostring)})' "$k")" \
            "Warp stall reasons, $(jq -r '.launches[0].kernel // "kernel" | .[0:60]' "$k")" "warps stalled per issued instruction" "ncu WarpStateStats, launch id $(jq -r '.launches[0].id' "$k"), clock-control $(jq -r '.ncu.clock_control' "$m"); run $id" > "$d/stalls.svg"
        svg_bars "$(jq -c '.launches[0] | [{name: "SM throughput", value: .sm_throughput_pct}, {name: "memory throughput", value: .memory_throughput_pct}, {name: "DRAM throughput", value: .dram_throughput_pct}, {name: "issue slots busy", value: .issue_active_pct}, {name: "achieved occupancy", value: .achieved_occupancy_pct}, {name: "theoretical occupancy", value: .theoretical_occupancy_pct}] + (.pipe_utilization_pct[:8] | map({name: "pipe \(.pipe)", value: .value})) | map(select(.value != null) | . + {label: "\(.value * 10 | round / 10)%"})' "$k")" \
            "Speed of light, $(jq -r '.launches[0].kernel // "kernel" | .[0:60]' "$k")" "% of peak (axis 0-100)" "ncu, launch id $(jq -r '.launches[0].id' "$k"); run $id" 100 > "$d/sol.svg"
        graphs+=(sol.svg stalls.svg)
        jq -r '"## Nsight Compute: per launch\n\nReplayed launches with clocks locked by ncu: durations here explain a kernel, they do not time it.\n\n| id | kernel | grid × block | regs/thread | duration (µs) | instructions |\n|---|---|---|---:|---:|---:|",
               (.launches[] | "| \(.id) | `\(.kernel // "?" | .[0:56])` | \(.grid) × \(.block) | \(.registers_per_thread) | \(if .duration_ns then (.duration_ns / 10 | round / 100) else "n/a" end) | \(.instructions) |"),
               "\n| id | SM % | memory % | DRAM % | occupancy achieved/theoretical % | issue busy % | local ld/st sectors | top stall (per issue) |\n|---|---:|---:|---:|---|---:|---|---|",
               (.launches[] | "| \(.id) | \(.sm_throughput_pct) | \(.memory_throughput_pct) | \(.dram_throughput_pct) | \(.achieved_occupancy_pct)/\(.theoretical_occupancy_pct) | \(.issue_active_pct) | \(.local_memory_sectors.load)/\(.local_memory_sectors.store) | \([.stalls_per_issue[] | select(.reason != "selected")][0] | if . then "\(.reason) \(.value * 100 | round / 100)" else "n/a" end) |")' "$k" >> "$d/summary.md"
    fi
    if [ -s "$d/summary.json" ]; then
        local s="$d/summary.json"
        svg_bars "$(jq -c '.kernels | .[:15] | map({name: .Name, value: (.["Time (%)"] // .["Time(%)"] // 0), label: "\((.["Time (%)"] // .["Time(%)"] // 0) * 10 | round / 10)% ×\(.Instances // .Calls // "?")"})' "$s")" \
            "GPU time by kernel (nsys)" "% of GPU kernel time" "nsys cuda_gpu_kern_sum; run $id" > "$d/kernels.svg"
        graphs+=(kernels.svg)
        jq -r '"## Nsight Systems\n",
               (if .timeline then "Kernel coverage of the kernel span: **\(.timeline.kernel_coverage * 1000 | round / 10)%** (\(.timeline.kernel_launches) launches, span \(.timeline.span_ns / 1e6) ms). Gaps are time the GPU had no kernel running.\n" else "No GPU trace (no kernels, or cuda tracing off).\n" end),
               "| kernel | time % | total (ms) | instances | median (µs) |\n|---|---:|---:|---:|---:|",
               (.kernels[] | "| `\(.Name | .[0:60])` | \(.["Time (%)"] // .["Time(%)"]) | \((.["Total Time (ns)"] // .["Total Time"] // 0) / 1e6) | \(.Instances // "") | \((.["Med (ns)"] // .Median // 0) / 1e3) |"),
               "\n| CUDA API | time % | calls | total (ms) |\n|---|---:|---:|---:|",
               (.cuda_api[:10][] | "| `\(.Name)` | \(.["Time (%)"] // .["Time(%)"]) | \(.["Num Calls"] // .Calls // "") | \((.["Total Time (ns)"] // .["Total Time"] // 0) / 1e6) |")' "$s" >> "$d/summary.md"
    fi
    if [ -s "$d/smi-summary.json" ]; then
        jq -r '"## nvidia-smi during the run\n\n\(.samples) samples every \(.interval_ms) ms: SM clock median \(.sm_clock_mhz.median) MHz (min \(.sm_clock_mhz.min), max \(.sm_clock_mhz.max)), power median \(.power_w.median) W (max \(.power_w.max)), GPU utilisation median \(.gpu_util_pct_median)%, max temperature \(.temperature_c_max) °C. Clock-limit reason masks seen (sample counts): \(.clock_reason_masks | to_entries | map("\(.key)×\(.value)") | join(", ")). A mask other than 0x0000000000000000 or 0x0000000000000001 (idle) means the clock was limited (power, thermal, or application clocks)."' "$d/smi-summary.json" >> "$d/summary.md"
        svg_smi "$d/smi.csv" "$(jq .interval_ms "$d/smi-summary.json")" "nvidia-smi every $(jq .interval_ms "$d/smi-summary.json") ms; red marks: a clock-limit reason was active; run $id" > "$d/smi.svg"
        graphs+=(smi.svg)
    fi
    {
        echo
        echo "## Files"
        echo
        (cd "$d" && ls -1 | grep -vE '^(summary\.md|report\.typ)$' | sed 's/^/- `/; s/$/`/')
        echo
        echo "Raw profiler output (perf.data, *.ncu-rep, *.nsys-rep, callgrind.out) is large: push it with \`crypto store push\` (AGENTS.md section 13) rather than committing it."
    } >> "$d/summary.md"
    # Typst source for the PDF (research-visuals skill, "Rendering").
    {
        printf '#set page(paper: "a4", margin: 1.5cm)\n#set text(size: 9pt)\n#show raw: set text(size: 8pt)\n#set table(stroke: 0.4pt + luma(180), inset: 3.5pt)\n#show table: set text(size: 7.5pt)\n\n'
        md2typ "$d/summary.md"
        printf '\n== Graphs\n\n'
        local g; for g in "${graphs[@]}"; do printf '#figure(image("%s", width: 100%%))\n\n' "$g"; done
    } > "$d/report.typ"
    echo "profile: wrote $d/summary.md$(for g in "${graphs[@]}"; do printf ' %s' "$g"; done)" >&2
}

case $sub in
    probe) do_probe ;;
    cpu) do_cpu ;;
    topdown) do_topdown ;;
    ncu) do_ncu ;;
    nsys) do_nsys ;;
    smi) do_smi ;;
    report) [ -n "$report_dir" ] || die "report DIR"; render "$(cd "$report_dir" && pwd)" ;;
    -h|--help|help) sed -n '2,28p' "$0" ;;
    *) die "unknown subcommand $sub (probe|cpu|topdown|ncu|nsys|smi|report)" ;;
esac
