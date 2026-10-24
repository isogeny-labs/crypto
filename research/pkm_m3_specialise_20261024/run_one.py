#!/usr/bin/env python3
"""run_one.py NAME  -- run msolve on systems/NAME.ms under a 16 GB RSS guard and a wall cap.
Env: THREADS (4), CAP_KB (default 12 GiB), CPU_CAP (21600 CPU-s), WALL (43200 s backstop).
Writes runs/NAME.{log,out,mem}. Needs `ps` (blocked inside the Claude sandbox)."""
import os, sys, time, subprocess, resource
name = sys.argv[1]
here = os.path.dirname(os.path.abspath(__file__)); os.chdir(here); os.makedirs('runs', exist_ok=True)
thr = os.environ.get('THREADS', '4'); cap = int(os.environ.get('CAP_KB', 12 * 1024 * 1024)); wall = float(os.environ.get('WALL', 43200)); cpucap = float(os.environ.get('CPU_CAP', 21600))
env = dict(os.environ, OMP_NUM_THREADS=thr)
log = open(f'runs/{name}.log', 'wb')
t0 = time.time()
pr = subprocess.Popen(['/opt/homebrew/bin/msolve', '-t', thr, '-v', '2', '-f', f'systems/{name}.ms', '-o', f'runs/{name}.out'],
                      stdout=log, stderr=subprocess.STDOUT, env=env)
peak = 0; maxcpu = 0.0; why = 'exit'
while pr.poll() is None:
    time.sleep(0.5)
    q = subprocess.run(['ps', '-o', 'rss=,cputime=', '-p', str(pr.pid)], capture_output=True, text=True)
    if pr.poll() is not None: break
    if q.returncode != 0 or not q.stdout.strip():  # guard unavailable (e.g. sandbox denies ps): never run unguarded
        why = 'guard_unavailable'; pr.kill(); break
    f = q.stdout.split(); r = int(f[0]); cpu = 0.0
    for x in f[1].replace('-', ':').split(':'): cpu = cpu * 60 + float(x)  # [dd-]hh:mm:ss.ss (day field approximated)
    maxcpu = max(maxcpu, cpu)
    if cpu > cpucap: why = 'cpu_cap'; pr.kill(); break
    peak = max(peak, r)
    if r > cap: why = 'mem_guard'; pr.kill(); break
    if time.time() - t0 > wall: why = 'wall_backstop'; pr.kill(); break
pr.wait(); w = time.time() - t0
ru = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss  # bytes on macOS
open(f'runs/{name}.mem', 'w').write(f'name={name} threads={thr} stop={why} exit={pr.returncode} wall_s={w:.1f} cpu_s={maxcpu:.0f} '
    f'ps_peak_rss_kB={peak} ru_maxrss_kB={ru//1024}\n')
