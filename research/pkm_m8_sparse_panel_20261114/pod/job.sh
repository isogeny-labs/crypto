#!/usr/bin/env bash
# Pod-side (or dry-run local) job runner for the m = 8 sparse PKM panel A.   [pcc:unverified]
# Usage: job.sh BUNDLE.tgz WORKDIR [--dry-run]
#   BUNDLE.tgz   plan.json + files it references (see PLAN_SCHEMA.md). Extracted to WORKDIR/in.
#   WORKDIR/out  everything produced; results.tgz is written last, then done.json (the poll signal).
#   --dry-run    do not apt-get; use the msolve and time already installed (local development only).
# One solve at a time, in plan `order`. Never retries, never skips a planned row, a failed row is a row.
# No `set -e`: a failing row must not stop the plan.
set -uo pipefail
BUNDLE=$(readlink -f "${1:?bundle.tgz}"); mkdir -p "${2:?workdir}"; W=$(cd "$2" && pwd); DRY=0; [ "${3:-}" = "--dry-run" ] && DRY=1
MSOLVE_PIN=0.6.5-1build2; MSOLVE_SHA=c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa
IN=$W/in; OUT=$W/out; mkdir -p "$IN" "$OUT/rows" "$OUT/env"
ts() { date -u +%Y-%m-%dT%H:%M:%S.%NZ; }
say() { echo "$(ts) $*" | tee -a "$OUT/job.log"; }
T0=$(date +%s.%N); START_UTC=$(ts)
sha() { sha256sum "$1" 2>/dev/null | cut -d' ' -f1; }
JSELF=$(readlink -f "$0")

# ---- 0. unpack -------------------------------------------------------------
tar xzf "$BUNDLE" -C "$IN" || { say "FATAL: cannot unpack $BUNDLE"; echo '{"error":"unpack failed"}' > "$OUT/done.json"; exit 1; }
PLAN=$IN/plan.json
[ -f "$PLAN" ] || { say "FATAL: no plan.json in bundle"; echo '{"error":"no plan.json"}' > "$OUT/done.json"; exit 1; }

# ---- 1. environment --------------------------------------------------------
INSTALL_RC=0
if [ $DRY = 0 ]; then
  export DEBIAN_FRONTEND=noninteractive
  { apt-get update -qq && apt-get install -y -qq "msolve=$MSOLVE_PIN" time; } > "$OUT/env/apt.log" 2>&1; INSTALL_RC=$?
  if [ $INSTALL_RC != 0 ] && ls "$IN"/debs/*.deb >/dev/null 2>&1; then
    dpkg -i "$IN"/debs/*.deb >> "$OUT/env/apt.log" 2>&1; INSTALL_RC=$?   # optional offline fallback shipped in the bundle
  fi
fi
{
  echo "== sha256sum /usr/bin/msolve"; sha256sum /usr/bin/msolve
  echo "== dpkg -s msolve"; dpkg -s msolve
  echo "== nproc"; nproc
  echo "== free -g"; free -g
  echo "== lscpu"; lscpu
  echo "== cpuinfo model"; grep -m1 'model name' /proc/cpuinfo
  echo "== cpuinfo count"; grep -c '^processor' /proc/cpuinfo
  echo "== allowed cpus"; grep Cpus_allowed_list /proc/self/status; taskset -cp $$
  echo "== cgroup"; cat /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max /sys/fs/cgroup/cpuset.cpus.effective 2>&1
  echo "== ldd --version"; ldd --version | head -1
  echo "== uname"; uname -a
  echo "== time"; /usr/bin/time --version 2>&1 | head -1
  echo "== load"; cat /proc/loadavg
  echo "== tools"; for c in taskset prlimit timeout lscpu python3 sha256sum tar; do printf '%s=%s\n' "$c" "$(command -v $c || echo MISSING)"; done
} > "$OUT/env/env.txt" 2>&1
MSOLVE_ACTUAL=$(sha /usr/bin/msolve)
cp "$PLAN" "$OUT/env/plan.json"
say "msolve sha256 $MSOLVE_ACTUAL (expected $MSOLVE_SHA) install_rc=$INSTALL_RC dry=$DRY"

# ---- 2. helper (JSON only; no analysis) -------------------------------------------
HELPER=$OUT/.helper.py
cat > "$HELPER" <<'PY'
import json, os, re, shlex, sys, hashlib
def sha(p):
    try: return hashlib.sha256(open(p, 'rb').read()).hexdigest()
    except OSError: return None
def allowed_cpus():
    for l in open('/proc/self/status'):
        if l.startswith('Cpus_allowed_list'):
            out = []
            for part in l.split(':')[1].strip().split(','):
                a, _, b = part.partition('-'); out += range(int(a), int(b or a) + 1)
            return out
    return list(range(os.cpu_count()))
cmd = sys.argv[1]
if cmd == 'plan':      # print shell assignments for row i of the plan sorted by order
    plan = json.load(open(sys.argv[2])); rows = plan['rows']
    orders = [r['order'] for r in rows]
    if len(set(orders)) != len(orders): sys.exit('duplicate order values in plan')
    rows = sorted(rows, key=lambda r: r['order'])
    if sys.argv[3] == 'count': print(len(rows)); sys.exit()
    r = rows[int(sys.argv[3])]; d = plan.get('defaults', {})
    g = lambda k, dv=None: r.get(k, d.get(k, dv))
    cell, target, solver = r['cell'], r['target'], r['solver']
    mode = g('mode', 'param' if solver == 'msolve' else 'native')
    v = dict(order=r['order'], cell=cell, target=target, solver=solver, mode=mode, threads=int(g('threads', 1)),
             timeout_s=int(g('timeout_s', 600)), mem_cap_kb=int(g('mem_cap_kb', 0)),
             system=g('system', f'systems/{cell}/{target}.ms'), system_native=g('system_native', f'systems/{cell}/{target}.native.json'),
             cell_dir=g('cell_dir', f'cells/{cell}'), targets_dir=g('targets_dir', f'cells/{cell}'),
             verify=int(bool(g('verify', solver == 'msolve' and mode == 'param'))),
             native_max_degree=int(g('native_max_degree', 12)), native_deadline_s=int(g('native_deadline_s', max(1, int(g('timeout_s', 600)) - 30))),
             flags=' '.join(shlex.quote(str(x)) for x in g('flags', [])), instrument=plan.get('instrument', 'bin/pkm_m8_panel'),
             plan_id=plan.get('plan_id', ''))
    for k, x in v.items(): print(f'R_{k}={shlex.quote(str(x))}')
elif cmd == 'cpus':    # first T allowed cpus
    t = int(sys.argv[2]); a = allowed_cpus(); print(','.join(map(str, a[:t])), len(a[:t]), len(a))
elif cmd == 'row':
    (_, _, order, cell, target, solver, mode, threads, cpus, cpus_n, timeout_s, mem_cap_kb, start, end, wall, rc, system, outf, stdo, stde, timef,
     rowdir, argv_json, verify_json, verify_rc, verify_wall, logtab) = sys.argv
    tm = open(timef, errors='replace').read() if os.path.exists(timef) else ''
    def g(rx):
        m = re.search(rx, tm); return m.group(1) if m else None
    def num(x, f=float): return f(x) if x not in (None, '') else None
    el = g(r'Elapsed \(wall clock\) time \(h:mm:ss or m:ss\): ([\d:.]+)'); elapsed = None
    if el:
        s = 0.0
        for p in el.split(':'): s = s * 60 + float(p)
        elapsed = s
    rc = int(rc); sig = g(r'Command terminated by signal (\d+)')
    sig = int(sig) if sig else (rc - 128 if 128 < rc < 160 else None)
    cap = int(mem_cap_kb)
    censored = None; note = None
    if rc == 124: censored = 'timeout'
    elif sig is not None:
        if sig == 11 and cap: censored, note = 'rlimit_or_crash', 'SIGSEGV with --as cap: msolve reports memory exhaustion (and some tiny-input bugs) as SIGSEGV'
        else: censored = f'signal:{sig}'
    elif rc != 0: censored = 'error'
    out_exists = os.path.exists(outf)
    outcome = None; qdim = None; df = None
    if out_exists:
        head = open(outf, errors='replace').read(4000)
        if head.startswith('[-1]'): outcome = 'no_solution'
        elif head.startswith('[1,'): outcome = 'positive_dimension'
        elif head.startswith('[0,'):
            outcome = 'zero_dimensional_param'
            m = re.match(r'\[0,\s*\[\s*(\d+),\s*(\d+),\s*(\d+),', head)
            if m: qdim = int(m.group(3))
            m = re.search(r'\[1,\s*\[\[\s*(\d+),', head)
            if m: df = int(m.group(1))
        elif head.startswith('#Reduced Groebner basis'): outcome = 'reduced_groebner_basis'
        elif not head.strip(): outcome = 'empty_output'
        else: outcome = 'unparsed'
    row = dict(order=int(order), cell=cell, target=target, solver=solver, mode=mode, argv=json.loads(argv_json), threads=int(threads),
               cpus=cpus, cpus_granted=int(cpus_n), timeout_s=int(timeout_s), mem_cap_kb=cap, start_utc=start, end_utc=end,
               wall_s=float(wall), time_v_elapsed_s=elapsed, user_s=num(g(r'User time \(seconds\): ([\d.]+)')), sys_s=num(g(r'System time \(seconds\): ([\d.]+)')),
               max_rss_kb=num(g(r'Maximum resident set size \(kbytes\): (\d+)'), int), exit_code=rc, signal=sig, censored=censored, censor_note=note,
               input=system, input_sha256=sha(system), output=outf if out_exists else None, output_sha256=sha(outf), output_bytes=os.path.getsize(outf) if out_exists else None,
               stdout=stdo, stdout_sha256=sha(stdo), stdout_bytes=os.path.getsize(stdo) if os.path.exists(stdo) else None, stderr=stde, time_v=timef,
               msolve_outcome=outcome if solver == 'msolve' else None, quotient_dim=qdim, elim_degree=df,
               verify=dict(path=verify_json if verify_json != '-' else None, exit_code=None if verify_rc == '-' else int(verify_rc),
                           wall_s=None if verify_wall == '-' else float(verify_wall), sha256=sha(verify_json) if verify_json != '-' else None))
    if logtab != '-' and os.path.exists(logtab):
        t = json.load(open(logtab))['totals']; row['log_table'] = {k: t.get(k) for k in ('passes', 'rounds', 'max_round_degree', 'sum_rows_x_cols_x_density', 'max_matrix_rows', 'max_matrix_cols', 'completed', 'incomplete_last_row')}
    print(json.dumps(row))
elif cmd == 'done':
    runs = [json.loads(l) for l in open(sys.argv[2]) if l.strip()]
    c = {}
    for r in runs: c[r['censored'] or 'ok'] = c.get(r['censored'] or 'ok', 0) + 1
    d = dict(schema='pkm.m8-panel.done/v1', rows_planned=int(sys.argv[3]), rows_recorded=len(runs), by_status=c,
             total_row_wall_s=sum(r['wall_s'] for r in runs), total_user_s=sum(r['user_s'] or 0 for r in runs), total_sys_s=sum(r['sys_s'] or 0 for r in runs),
             job_wall_s=float(sys.argv[4]), start_utc=sys.argv[5], end_utc=sys.argv[6], results_tgz_sha256=sys.argv[7], results_tgz_bytes=int(sys.argv[8]),
             bundle_sha256=sys.argv[9], plan_sha256=sys.argv[10], job_sh_sha256=sys.argv[11], msolve_sha256=sys.argv[12], msolve_sha_expected=sys.argv[13],
             msolve_install_rc=int(sys.argv[14]), dry_run=bool(int(sys.argv[15])))
    print(json.dumps(d, indent=1))
PY
PY=python3
H() { $PY -I "$HELPER" "$@"; }

N=$(H plan "$PLAN" count) || { say "FATAL: bad plan"; echo '{"error":"bad plan"}' > "$OUT/done.json"; exit 1; }
RUNS=$OUT/runs.jsonl; : > "$RUNS"
say "plan rows: $N; instrument: $(sha "$IN/$(H plan "$PLAN" 0 | sed -n 's/^R_instrument=//p' | tr -d "'")")"

# ---- 3. plan loop ------------------------------------------------------------
for ((i=0; i<N; i++)); do
  eval "$(H plan "$PLAN" $i)"
  ID=$(printf '%03d_%s_%s_%s_%s' "$R_order" "$R_cell" "$R_target" "$R_solver" "$R_mode"); RD=$OUT/rows/$ID; mkdir -p "$RD"
  read -r CPUS CPUS_N CPUS_ALL < <(H cpus "$R_threads")
  SYS=$IN/$R_system; [ "$R_solver" = native ] && SYS=$IN/$R_system_native
  OUTF=$RD/solve.out; STDO=$RD/stdout.log; STDE=$RD/stderr.log; TIMEF=$RD/time_v.txt
  WRAP=(taskset -c "$CPUS" /usr/bin/time -v -o "$TIMEF" timeout -k 30 "$R_timeout_s")
  [ "$R_mem_cap_kb" -gt 0 ] && WRAP+=(prlimit "--as=$((R_mem_cap_kb*1024))")
  if [ "$R_solver" = msolve ]; then
    case "$R_mode" in param) MF=(-P 1);; gb) MF=(-g 2);; *) MF=();; esac   # -g 2 and -P 1 are NOT combinable (MSOLVE_FORMAT.md)
    ARGV=("${WRAP[@]}" /usr/bin/msolve -t "$R_threads" -v 2 "${MF[@]}" $(eval echo $R_flags) -f "$SYS" -o "$OUTF")
  else
    ARGV=("${WRAP[@]}" "$IN/$R_instrument" native-solve --system "$SYS" --max-degree "$R_native_max_degree" --deadline "$R_native_deadline_s" --out "$OUTF")
  fi
  ARGV_JSON=$(printf '%s\0' "${ARGV[@]}" | $PY -I -c "import json,sys;print(json.dumps(sys.stdin.read().split('\0')[:-1]))")
  say "row $ID: start (threads=$R_threads cpus=$CPUS timeout=${R_timeout_s}s cap=${R_mem_cap_kb}kB)"
  S=$(ts); S0=$(date +%s.%N)
  if [ -f "$SYS" ]; then "${ARGV[@]}" > "$STDO" 2> "$STDE"; RC=$?; else echo "input missing: $SYS" > "$STDE"; : > "$STDO"; RC=2; fi
  E0=$(date +%s.%N); E=$(ts); WALL=$($PY -I -c "print($E0-$S0)")
  # verify (msolve param rows only; instrument's `verify`), not part of the solver's timing
  VJ=-; VRC=-; VW=-
  if [ "$R_verify" = 1 ] && [ "$RC" = 0 ] && [ -s "$OUTF" ]; then
    VJ=$RD/verify.json; V0=$(date +%s.%N)
    if [ -x "$IN/$R_instrument" ]; then
      taskset -c "$CPUS" timeout 900 "$IN/$R_instrument" verify --cell "$IN/$R_cell_dir" --targets "$IN/$R_targets_dir" --msolve-out "$OUTF" --target "$R_target" --out "$VJ" > "$RD/verify.stdout" 2> "$RD/verify.stderr"; VRC=$?
    else echo "instrument missing: $IN/$R_instrument" > "$RD/verify.stderr"; VRC=127; fi
    VW=$($PY -I -c "print($(date +%s.%N)-$V0)")
  fi
  LT=-
  if [ "$R_solver" = msolve ] && [ -f "$IN/tools/parse_msolve_log.py" ]; then
    LT=$RD/log_table.json; $PY -I "$IN/tools/parse_msolve_log.py" "$STDO" --json "$LT" > /dev/null 2>&1 || LT=-
  fi
  H row "$R_order" "$R_cell" "$R_target" "$R_solver" "$R_mode" "$R_threads" "$CPUS" "$CPUS_N" "$R_timeout_s" "$R_mem_cap_kb" "$S" "$E" "$WALL" "$RC" "$SYS" "$OUTF" "$STDO" "$STDE" "$TIMEF" "$RD" "$ARGV_JSON" "$VJ" "$VRC" "$VW" "$LT" >> "$RUNS"
  say "row $ID: rc=$RC wall=${WALL}s"
  printf '{"rows_done":%d,"rows_planned":%d,"last":"%s","utc":"%s"}\n' $((i+1)) "$N" "$ID" "$(ts)" > "$OUT/progress.json"
done

# ---- 4. close ----------------------------------------------------------------
T1=$(date +%s.%N); END_UTC=$(ts)
(cd "$W" && tar czf results.tgz.tmp --exclude=out/.helper.py out) && mv "$W/results.tgz.tmp" "$OUT/results.tgz"
H done "$RUNS" "$N" "$($PY -I -c "print($T1-$T0)")" "$START_UTC" "$END_UTC" "$(sha "$OUT/results.tgz")" "$(stat -c %s "$OUT/results.tgz")" \
   "$(sha "$BUNDLE")" "$(sha "$PLAN")" "$(sha "$JSELF")" "$MSOLVE_ACTUAL" "$MSOLVE_SHA" "$INSTALL_RC" "$DRY" > "$OUT/done.json.tmp" && mv "$OUT/done.json.tmp" "$OUT/done.json"
say "done: $(tr -d '\n ' < "$OUT/done.json" | cut -c1-200)"
