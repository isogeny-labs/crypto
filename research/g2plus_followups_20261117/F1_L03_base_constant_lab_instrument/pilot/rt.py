import resource, subprocess, sys
r = subprocess.run(sys.argv[2:])
u = resource.getrusage(resource.RUSAGE_CHILDREN)
print(f"{sys.argv[1]} exit={r.returncode} cpu_s={u.ru_utime+u.ru_stime:.1f} maxrss_kb={u.ru_maxrss}", flush=True)
