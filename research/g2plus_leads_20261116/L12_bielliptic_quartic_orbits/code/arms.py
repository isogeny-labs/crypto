"""Column maps for arms A, B, Bp, C, N (SPEC.md section 6). No counted operations."""
import random


def column_map(arm, points, curve, ss):
    p = curve.p
    n = len(points)
    col = [0] * n
    sign = [1] * n
    reps = []
    if arm == "A":
        for i, q in enumerate(points):
            col[i] = i
            reps.append((q[0], q[2]))
    elif arm in ("B", "Bp"):
        seen = {}
        for i, q in enumerate(points):
            key = (q[0], q[2])
            if key not in seen:
                seen[key] = len(reps)
                reps.append(key)
            col[i] = seen[key]
    elif arm == "C":
        seen = {}
        for i, q in enumerate(points):
            x, w, hx = q[0], q[2], q[3]
            wp = (-w - hx) % p
            key = (x, min(w, wp))
            if key not in seen:
                seen[key] = len(reps)
                reps.append(key)
            col[i] = seen[key]
            sign[i] = 1 if w <= wp else -1
    elif arm == "N":
        perm = list(range(n))
        random.Random(ss + 90000).shuffle(perm)
        t = 0
        k = 0
        while k < n:
            if k + 1 < n:
                col[perm[k]] = col[perm[k + 1]] = t
            else:
                col[perm[k]] = t
            q = points[perm[k]]
            reps.append((q[0], q[2]))
            t += 1
            k += 2
    else:
        raise ValueError(arm)
    return col, sign, reps
