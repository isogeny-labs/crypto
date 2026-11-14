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
