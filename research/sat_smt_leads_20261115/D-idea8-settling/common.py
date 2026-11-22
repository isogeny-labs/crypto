"""Frozen design constants for the settling run (see PROTOCOL.md)."""
MASTER = 2026111508          # new master seed (powered run: 2026111408)
NSEEDS = 3
# l: n, non-decomposable targets, planted decomposable targets (per sequence)
CELLS = {3: dict(n=11, nu=60, nd=6), 4: dict(n=13, nu=985, nd=15), 5: dict(n=17, nu=940, nd=60)}


def cell_seed(l, s):
    """setup (V) seed for dimension l, seed index s; target seed = +1, shuffle seed = +2."""
    return MASTER + 1000 * l + 10 * s


def schedule(T, l):
    """which arm solves which position. T = [(R, decomposable)]. Returns dict arm -> set(idx).
    Last third = idx >= (2N)//3. INC solves every target. At l <= 4 the fresh arms solve every
    non-decomposable target (AA: last third); at l = 5 they solve a fixed position subsample."""
    N = len(T); L0 = (2 * N) // 3
    U = [i for i, (_, d) in enumerate(T) if not d]
    early = [i for i in U if i < L0]; late = [i for i in U if i >= L0]
    if l <= 4:
        return {'INC': set(range(N)), 'SPEC': set(U), 'FP': set(U), 'AA': set(late)}
    return {'INC': set(range(N)),
            'SPEC': set(late) | set(early[::2]),
            'FP': set(late[::2]) | set(early[::4]),
            'AA': set(late[::2])}
