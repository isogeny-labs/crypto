"""Generic-group model of the residual walks: steps to the first table hit.

The group is Z_N (N prime) with negation x -> -x, so the model has exactly
the structure the walks use (an additive group, a sign class, a hash of the
point) and nothing else.  Each arm runs walks that restart from a fresh
random point after every hit, against one persistent table, and records the
steps each walk took until it hit the table.

Arms (the code paths of ic_framework/residual.rs on main):
  base     L <- L + J_j(L); X <- that; L <- X - P_i(X); store L as +-class
  fused    L <- L + C_{j,i}(L), C = J_j - P_i; store L as +-class
  fusedind fused, with i from an independent hash (tests hash correlation)
  base_ns  base, table without sign classes (control: predicts sqrt 2)
  fused_ns fused, table without sign classes

Prediction from merge counting: steps fused / base = sqrt(1.5) = 1.225;
fused_ns / base_ns = sqrt(2) = 1.414; fusedind = fused.
"""
import json, math, random, sys

MASK = (1 << 64) - 1
def mix(v, salt):
    z = (v + salt + 0x9E3779B97F4A7C15) & MASK
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
    return z ^ (z >> 31)

def run(arm, N, F, J, walks, seed):
    rng = random.Random(seed)
    jumps = [rng.randrange(1, N) for _ in range(J)]
    base = [rng.randrange(1, N) for _ in range(F)]
    signed = arm not in ("base_ns", "fused_ns")
    fused = arm.startswith("fused")
    salt_i = 0x1234_5678_9ABC_DEF1
    table = set()
    out = []
    for _ in range(walks):
        L = rng.randrange(1, N)
        steps = 0
        while True:
            if fused:
                h = mix(L, 0)
                j = (h >> 8) % J
                i = (mix(L ^ 0xA5A5, 0) >> 4) % F if arm != "fusedind" else (mix(L, salt_i) >> 4) % F
                L = (L + jumps[j] - base[i]) % N
            else:
                j = (mix(L, 0) >> 8) % J
                X = (L + jumps[j]) % N
                i = (mix(X ^ 0xA5A5, 0) >> 4) % F
                L = (X - base[i]) % N
            steps += 1
            key = min(L, N - L) if signed else L
            if key in table:
                break
            table.add(key)
        out.append(steps)
    return out

if __name__ == "__main__":
    N, F, walks, reps = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
    J = int(sys.argv[5]) if len(sys.argv) > 5 else 16
    seed0 = int(sys.argv[6]) if len(sys.argv) > 6 else 1000
    res = {}
    for arm in ["base", "fused", "fusedind", "base_ns", "fused_ns"]:
        res[arm] = [sum(run(arm, N, F, J, walks, seed0 + r)) / walks for r in range(reps)]
    json.dump({"N": N, "F": F, "J": J, "walks": walks, "reps": reps, "mean_steps_per_walk": res}, sys.stdout)
