"""Run solver arms on generated instances. Each run is a fresh process; CPU time via RUSAGE_CHILDREN delta.
arms: wd_b (WDSat -b, GE off)  wd_bx (WDSat -b -x)  cms_std (CMS on std xor-CNF)  cms_gad (CMS on gadget xor-CNF)
usage: run_arms.py INSTDIR OUT.jsonl ARMS(comma) TIMEOUT NAMEFILTER(regex) [jobs]"""
import sys, json, re, subprocess, resource, time, os
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
HERE = Path(__file__).resolve().parent.parent
def run(cmd, timeout):
    import tempfile, signal
    t0 = time.time()
    with tempfile.TemporaryFile() as tf:
        pr = subprocess.Popen(cmd, stdout=tf, stderr=subprocess.DEVNULL)
        to = False
        while True:
            pid, status, ru = os.wait4(pr.pid, os.WNOHANG)
            if pid:
                break
            if time.time() - t0 > timeout:
                pr.kill(); to = True
                pid, status, ru = os.wait4(pr.pid, 0)
                break
            time.sleep(0.05)
        tf.seek(0); out = tf.read().decode(errors='replace')
    return out, (None if to else os.waitstatus_to_exitcode(status)), to, ru.ru_utime + ru.ru_stime, time.time() - t0
def one(job):
    name, arm, inst, n, l, timeout = job
    if arm.startswith('wd'):
        exe = HERE / 'work' / f'wd_l{l}' / 'wdsat_solver'
        cmd = [str(exe), '-i', str(inst / f'{name}.anf'), '-n', str(n), '-l', str(l), '-m', '3', '-b'] + (['-x'] if arm == 'wd_bx' else [])
    else:
        f = inst / f'{name}.{"std" if arm.startswith("cms_std") else "gad"}.cnf'
        gj = ['--maxmatrixcols', '100000', '--maxmatrixrows', '100000', '--autodisablegauss', '0'] if arm.endswith('_gj') else []
        cmd = ['cryptominisat5', '--verb', '1', '--maxtime', str(timeout)] + gj + [str(f)]
    # timeout via subprocess wall-clock is unreliable under load; give 3x wall slack, CMS has its own cpu-time limit
    out, rc, to, cpu, wall = run(cmd, timeout * 1.25 + 30)
    rec = dict(name=name, arm=arm, n=n, l=l, cpu_s=round(cpu, 3), wall_s=round(wall, 2))
    if arm.startswith('wd'):
        st = 'SAT' if re.search(r'^SAT$|^[01]{10,}$', out, re.M) else 'UNSAT' if re.search(r'^UNSAT$', out, re.M) else 'ERR'
        try:
            rec['conflicts'] = int(out.strip().splitlines()[-1])
        except Exception:
            rec['conflicts'] = None
        if st == 'ERR' and to:
            st = 'TIMEOUT'
    else:
        m = re.search(r'^s (SATISFIABLE|UNSATISFIABLE|INDETERMINATE)', out, re.M)
        st = {'SATISFIABLE': 'SAT', 'UNSATISFIABLE': 'UNSAT', 'INDETERMINATE': 'TIMEOUT'}.get(m.group(1), 'ERR') if m else ('TIMEOUT' if to else 'ERR')
        c = re.search(r'^c conflicts\s*:\s*(\d+)', out, re.M)
        rec['conflicts'] = int(c.group(1)) if c else None
        d = re.search(r'^c decisions\s*:\s*(\d+)', out, re.M)
        rec['decisions'] = int(d.group(1)) if d else None
        pr = re.search(r'^c propagations\s*:\s*([\d.]+)([KMG]?)', out, re.M)
        if pr:
            rec['propagations_approx'] = float(pr.group(1)) * {'': 1, 'K': 1e3, 'M': 1e6, 'G': 1e9}[pr.group(2)]
        ct = re.search(r'^c Total time \(this thread\)\s*:\s*([\d.]+)', out, re.M)
        rec['cms_time_s'] = float(ct.group(1)) if ct else None
    rec['status'] = st
    return rec
def main():
    inst = Path(sys.argv[1]); outp = Path(sys.argv[2]); arms = sys.argv[3].split(','); timeout = int(sys.argv[4])
    flt = re.compile(sys.argv[5]); jobs = int(sys.argv[6]) if len(sys.argv) > 6 else 3
    man = json.loads((inst / 'manifest.json').read_text())
    J = [(m['name'], a, inst, m['n'], m['l'], timeout) for m in man if flt.search(m['name']) for a in arms]
    done = set()
    if outp.exists():
        for ln in outp.read_text().splitlines():
            r = json.loads(ln); done.add((r['name'], r['arm']))
    J = [j for j in J if (j[0], j[1]) not in done]
    with ThreadPoolExecutor(jobs) as ex, outp.open('a') as f:
        for rec in ex.map(one, J):
            f.write(json.dumps(rec) + '\n'); f.flush()
            print(rec['name'], rec['arm'], rec['status'], rec.get('conflicts'), rec['cpu_s'], flush=True)
main()
