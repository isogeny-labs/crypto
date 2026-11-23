import random, sys, time
sys.path.insert(0, '/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/chase/cm-orbit')
from cmorbit import *
rng = random.Random(1)
E = find_curve(16, rng); print("p", E.p, "b", E.b, "n", E.n, "w", E.w)
cs = setup(E, rng); print("lam", cs[3])
for arm in ("fold", "cmorb", "foldcm"):
    for orc in ("table", "semaev"):
        t = time.time(); r = run_ic(E, arm, orc, 60, random.Random(5), cs)
        print(arm, orc, {k: r[k] for k in r if k != 'counts'}, round(time.time()-t, 2))
for cls in ("plain", "neg", "aut6"):
    r = run_rho(E, cls, random.Random(3), cs); print(cls, r)
