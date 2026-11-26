"""Generate prime-order curves and the twist-study label at any prime (re-implements the
selection of ../g3_affine_twist_20261121/run.py main_arm; label s = chi(-1) chi(f(t)) chi(S_L))."""
import importlib.util, os, random, sys
HERE = os.path.dirname(os.path.abspath(__file__))
TW = os.path.join(HERE, "..", "g3_affine_twist_20261121")
sys.path.insert(0, HERE)
import build as BD  # noqa
FE = BD.FE
_spec = importlib.util.spec_from_file_location("census", os.path.join(HERE, "..", "g3_transfer_gates_20261120", "census.py"))
CEN = importlib.util.module_from_spec(_spec); _spec.loader.exec_module(CEN)
FM, FPAIRS, FREP = FE.load_formula(os.path.join(TW, "formulas", "f_001000.json"))


def leg(x, p):
    x %= p
    return 0 if x == 0 else (1 if pow(x, (p - 1) // 2, p) == 1 else -1)


def stable_keys():
    perm = [1, 2, 3, 4, 5, 6, 0, 7]
    out = [k for k in BD.SIGTAB if FE.lag_key([[perm[i] for i in s] for s in k]) == k]
    return sorted(out, key=lambda k: sorted(sorted(s) for s in k))


def curves(p, n, seed):
    fields, reps = CEN.setup_fields(p)
    rng = random.Random(seed)
    got = 0
    while got < n:
        f = CEN.random_irreducible_septic(rng, p)
        N = CEN.count_points(fields, reps, f)
        P, integral, _ = CEN.charpoly_from_counts(p, N)
        if not CEN.is_prime(sum(P)):
            continue
        got += 1
        yield f, P


def label(p, f, key, t, rng):
    T = FE.Tower(p, f, rng)
    K = T.K
    z = (0, 1, 0, 0, 0, 0, 0)
    roots = [z]
    for _ in range(6):
        roots.append(K.pow(roots[-1], p))
    us = [K.inv(K.sub(e, K.c(t))) for e in roots] + [K.zero]
    sig = BD.SIGTAB[key][0]
    v = FE.eval_formula(T, FM, FPAIRS, [us[sig[i]] for i in range(8)], rng)
    x = FE.in_Fp(T, v)
    c = sum(cf * pow(t, i, p) for i, cf in enumerate(f)) % p
    if x is None:
        return None, None
    return (0 if x == 0 else leg(-1, p) * leg(c, p) * leg(x, p)), x
