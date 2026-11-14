import json, os, re, shlex, signal, sys, time, hashlib
KINDS = {'aa', 'planted', 'natural', 'gb_check', 'native', 'canary', 'timer_floor', 'probe'}
KIND_DEFAULTS = {'canary': dict(threads=8, timeout_s=120, msolve_mode='gb', verify=False),
                 'timer_floor': dict(threads=1, timeout_s=60, msolve_mode='gb', verify=False)}
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
def rd(p):
    try: return open(p).read().strip()
    except OSError: return None
def meminfo_kb():
    for l in open('/proc/meminfo'):
        if l.startswith('MemTotal'): return int(l.split()[1])
cmd = sys.argv[1]
if cmd == 'mem':   # write env/mem.json, print the usable memory limit in kB
    v2 = rd('/sys/fs/cgroup/memory.max'); v1 = rd('/sys/fs/cgroup/memory/memory.limit_in_bytes'); tot = meminfo_kb()
    cands = []
    for raw in (v2, v1):
        if raw and raw != 'max' and raw.isdigit() and int(raw) // 1024 < tot * 4: cands.append(int(raw) // 1024)
    lim = min(cands + [tot]); src = 'cgroup' if cands and min(cands) <= tot else 'meminfo (no finite cgroup limit)'
    json.dump(dict(cgroup_v2_memory_max=v2, cgroup_v1_limit_in_bytes=v1, meminfo_memtotal_kb=tot, limit_kb=lim, source=src), open(sys.argv[2], 'w'), indent=1)
    print(lim)
elif cmd == 'plan':      # shell assignments for row i of the plan sorted by order
    plan = json.load(open(sys.argv[2])); rows = plan['rows']
    if os.environ.get('REPLACEMENT') != '1': rows = [r for r in rows if not r.get('on_replacement')]
    orders = [r['order'] for r in rows]
    if len(set(orders)) != len(orders): sys.exit('duplicate order values in plan')
    for r in rows:
        if r.get('kind') not in KINDS: sys.exit(f"row {r.get('order')}: kind must be one of {sorted(KINDS)}")
        if r['solver'] not in ('msolve', 'native'): sys.exit(f"row {r['order']}: bad solver")
        mm = r.get('msolve_mode', r.get('mode', KIND_DEFAULTS.get(r['kind'], {}).get('msolve_mode', plan.get('defaults', {}).get('msolve_mode', 'param'))))
        if r['solver'] == 'msolve' and mm not in ('param', 'gb'): sys.exit(f"row {r['order']}: msolve_mode must be param or gb (never combined)")
    rows = sorted(rows, key=lambda r: r['order'])
    if sys.argv[3] == 'count': print(len(rows)); sys.exit()
    r = rows[int(sys.argv[3])]; d = plan.get('defaults', {}); kd = KIND_DEFAULTS.get(r['kind'], {})
    g = lambda k, dv=None: r.get(k, kd.get(k, d.get(k, dv)))
    cell, target, solver = r['cell'], r['target'], r['solver']
    mode = r.get('msolve_mode', r.get('mode', kd.get('msolve_mode', d.get('msolve_mode', 'param')))) if solver == 'msolve' else 'native'
    cap = g('rss_cap_kb', g('mem_cap_kb', 'cgroup0.9'))
    if isinstance(cap, str):
        m = re.fullmatch(r'cgroup([0-9.]+)', cap)
        if not m: sys.exit('bad rss_cap_kb')
        cap = int(float(m.group(1)) * int(os.environ['CGROUP_LIMIT_KB']))
    tmo = int(g('timeout_s', g('wall_timeout_s', 600)))
    v = dict(order=r['order'], kind=r['kind'], cell=cell, target=target, solver=solver, mode=mode, threads=int(g('threads', 1)),
             timeout_s=tmo, rss_cap_kb=int(cap), system=g('system', f'systems/{cell}/{target}.ms'),
             system_native=g('system_native', f'systems/{cell}/{target}.native.json'),
             cell_dir=g('cell_dir', f'cells/{cell}'), targets_dir=g('targets_dir', f'cells/{cell}'),
             verify=int(bool(g('verify', solver == 'msolve' and mode == 'param'))),
             native_max_degree=int(g('native_max_degree', 12)), native_deadline_s=int(g('native_deadline_s', max(1, tmo - 30))),
             flags=' '.join(shlex.quote(str(x)) for x in g('flags', [])), instrument=plan.get('instrument', 'bin/pkm_m8_panel'),
             plan_id=plan.get('plan_id', ''))
    for k, x in v.items(): print(f'R_{k}={shlex.quote(str(x))}')
elif cmd == 'cpus':
    t = int(sys.argv[2]); a = allowed_cpus(); print(','.join(map(str, a[:t])), len(a[:t]), len(a))
elif cmd == 'watch':   # watch PID CAP_KB OUTJSON : poll /proc every 2 s over the process tree, stop it at the resident cap
    pid, cap, outp = int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
    def tree(root):
        kids = {}
        for e in os.listdir('/proc'):
            if e.isdigit():
                try:
                    s = open(f'/proc/{e}/stat').read(); rp = s.rindex(')'); f = s[rp + 2:].split(); kids.setdefault(int(f[1]), []).append((int(e), f[0]))
                except (OSError, ValueError, IndexError): pass
        out, st = [], [root]
        while st:
            p = st.pop(); out.append(p); st += [c for c, _ in kids.get(p, [])]
        return out
    def alive(p):
        try: return open(f'/proc/{p}/stat').read().split(')')[-1].split()[0] != 'Z'
        except OSError: return False
    def vm(p):
        d = {}
        try:
            for l in open(f'/proc/{p}/status'):
                for k in ('VmRSS', 'VmHWM', 'VmPeak'):
                    if l.startswith(k + ':'): d[k] = int(l.split()[1])
        except OSError: pass
        return d
    res = dict(polls=0, max_vmhwm_kb=0, max_vmpeak_kb=0, max_rss_sum_kb=0, cap_kb=cap, rss_cap_hit=False, rss_at_hit_kb=None, hit_utc=None, sigkill_sent=False)
    while alive(pid):
        res['polls'] += 1; procs = tree(pid); s = 0
        for p in procs:
            m = vm(p); s += m.get('VmRSS', 0)
            res['max_vmhwm_kb'] = max(res['max_vmhwm_kb'], m.get('VmHWM', 0)); res['max_vmpeak_kb'] = max(res['max_vmpeak_kb'], m.get('VmPeak', 0))
        res['max_rss_sum_kb'] = max(res['max_rss_sum_kb'], s)
        if cap > 0 and s > cap and not res['rss_cap_hit']:
            res.update(rss_cap_hit=True, rss_at_hit_kb=s, hit_utc=time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()))
            for p in procs:
                try: os.kill(p, signal.SIGTERM)
                except OSError: pass
            t_end = time.time() + 10
            while time.time() < t_end and alive(pid): time.sleep(0.5)
            if alive(pid):
                res['sigkill_sent'] = True
                for p in tree(pid):
                    try: os.kill(p, signal.SIGKILL)
                    except OSError: pass
            break
        time.sleep(2)
    json.dump(res, open(outp, 'w'))
elif cmd == 'row':
    a = sys.argv[2:]
    (order, cell, target, solver, mode, threads, cpus, cpus_n, timeout_s, cap, start, end, wall, rc, system, outf, stdo, stde, timef,
     rowdir, argv_json, verify_json, verify_rc, verify_wall, logtab, wdf, pre) = a
    E = os.environ
    tm = open(timef, errors='replace').read() if os.path.exists(timef) else ''
    def g(rx):
        m = re.search(rx, tm); return m.group(1) if m else None
    def num(x, f=float): return f(x) if x not in (None, '') else None
    el = g(r'Elapsed \(wall clock\) time \(h:mm:ss or m:ss\): ([\d:.]+)'); elapsed = None
    if el:
        s = 0.0
        for p in el.split(':'): s = s * 60 + float(p)
        elapsed = s
    rc = int(rc); cap = int(cap); wall = float(wall)
    sig = g(r'Command terminated by signal (\d+)'); sig = int(sig) if sig else (rc - 128 if 128 < rc < 160 else None)
    wd = json.load(open(wdf)) if os.path.exists(wdf) else {}
    max_rss = num(g(r'Maximum resident set size \(kbytes\): (\d+)'), int)
    as_limit = int(cap * 1.25) if cap else 0
    peak = wd.get('max_vmpeak_kb') or 0
    note = None
    if wd.get('rss_cap_hit'): censored = 'rss_cap'; note = 'watchdog stopped the process at the resident cap'
    elif rc == 124: censored = 'timeout'
    elif sig == 11 or rc == 139:
        small = (max_rss is not None and cap and max_rss < 0.5 * cap) or wall < 60
        if small: censored, note = 'crash', 'rc 139 with max RSS < 50% of cap or within 60 s (protocol crash rule)'
        else: censored, note = 'as_rlimit', 'rc 139 beyond the crash rule: address-space backstop (1.25 x cap) presumed'
    elif sig is not None: censored = f'signal:{sig}'
    elif rc != 0: censored = 'error'
    else: censored = 'ok'
    out_exists = os.path.exists(outf); outcome = qdim = df = None
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
    row = dict(order=int(order), kind=E.get('R_kind'), pod_id=E.get('POD_ID'), replacement=E.get('REPLACEMENT') == '1', cell=cell, target=target, solver=solver,
               msolve_mode=mode if solver == 'msolve' else None, argv=json.loads(argv_json), threads=int(threads), cpus=cpus, cpus_granted=int(cpus_n),
               timeout_s=int(timeout_s), rss_cap_kb=cap, as_backstop_kb=as_limit, start_utc=start, end_utc=end, wall_s=wall, time_v_elapsed_s=elapsed,
               user_s=num(g(r'User time \(seconds\): ([\d.]+)')), sys_s=num(g(r'System time \(seconds\): ([\d.]+)')), max_rss_kb=max_rss,
               vmpeak_kb=wd.get('max_vmpeak_kb'), vmhwm_kb=wd.get('max_vmhwm_kb'), watchdog=wd or None, exit_code=rc, signal=sig,
               time_v_exit_status=num(g(r'Exit status: (\d+)'), int), censored=censored, censor_note=note,
               concurrent_process_before_start=pre if pre != '-' else None,
               input=system, input_sha256=sha(system), output=outf if out_exists else None, output_sha256=sha(outf), output_bytes=os.path.getsize(outf) if out_exists else None,
               stdout=stdo, stdout_sha256=sha(stdo), stdout_bytes=os.path.getsize(stdo) if os.path.exists(stdo) else None, stderr=stde, time_v=timef,
               msolve_outcome=outcome if solver == 'msolve' else None, quotient_dim=qdim, elim_degree=df,
               verify=dict(path=verify_json if verify_json != '-' else None, exit_code=None if verify_rc == '-' else int(verify_rc),
                           wall_s=None if verify_wall == '-' else float(verify_wall), sha256=sha(verify_json) if verify_json != '-' else None))
    if logtab != '-' and os.path.exists(logtab):
        t = json.load(open(logtab))['totals']
        row['log_table'] = {k: t.get(k) for k in ('passes', 'rounds', 'max_round_degree', 'sum_rows_x_cols_x_density', 'max_matrix_rows', 'max_matrix_cols', 'completed', 'incomplete_last_row', 'up_to_degree_8')}
    print(json.dumps(row))
elif cmd == 'done':
    runs = [json.loads(l) for l in open(sys.argv[2]) if l.strip()]
    c = {}
    for r in runs: c[r['censored']] = c.get(r['censored'], 0) + 1
    d = dict(schema='pkm.m8-panel.done/v2', rows_planned=int(sys.argv[3]), rows_recorded=len(runs), by_status=c,
             total_row_wall_s=sum(r['wall_s'] for r in runs), total_user_s=sum(r['user_s'] or 0 for r in runs), total_sys_s=sum(r['sys_s'] or 0 for r in runs),
             job_wall_s=float(sys.argv[4]), start_utc=sys.argv[5], end_utc=sys.argv[6], results_tgz_sha256=sys.argv[7], results_tgz_bytes=int(sys.argv[8]),
             bundle_sha256=sys.argv[9], plan_sha256=sys.argv[10], job_sh_sha256=sys.argv[11], msolve_sha256=sys.argv[12], msolve_sha_expected=sys.argv[13],
             msolve_install_rc=int(sys.argv[14]), dry_run=bool(int(sys.argv[15])), pod_id=os.environ.get('POD_ID'), replacement=os.environ.get('REPLACEMENT') == '1')
    print(json.dumps(d, indent=1))
