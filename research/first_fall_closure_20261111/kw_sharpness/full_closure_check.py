"""Full (iterated) operational closure W_c, c=1..7, for a few instances, to compare with the stage-one lower bound."""
import random, time, json
import ffd
from run_table import indep_nus
out = []
for (n, npr) in [(9, 3), (10, 3), (12, 4)]:
    rnd = random.Random(10_000 * n + 100 * npr + 0)   # same instance as run_table sample idx 0
    F, a2, a6, c = ffd.random_instance(n, rnd); nus = indep_nus(F, npr, rnd)
    N = 3 * npr; s = ffd.descend(F, nus, c, a6); M = ffd.masks(N)
    seq = []
    t = time.time()
    for cc in range(1, 8):
        dimW, low = ffd.closure(s, N, cc, M)
        seq.append((cc, dimW, low)); 
    row = dict(n=n, npr=npr, N=N, dim_B_le7=sum(int(x) for x in [__import__('math').comb(N, i) for i in range(8)]), seq=seq, seconds=round(time.time() - t, 1))
    print(row, flush=True); out.append(row)
json.dump(out, open("full_closure_check.json", "w"), indent=1)
