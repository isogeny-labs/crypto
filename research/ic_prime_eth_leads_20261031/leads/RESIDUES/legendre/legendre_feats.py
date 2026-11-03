"""Char-2 analogue of 'Legendre per bit' features for P2 arm-A data, plus paired baseline-vs-feature evaluation.
Usage: legendre_feats.py n   (n in 17,19,23)  -> writes result_{n}.json next to this file.
Read-only on the P2 worktree and its data dir. Prereg: prereg.md (written before this was run)."""
import glob, json, os, sys, time
sys.dont_write_bytecode = True
import numpy as np
P2 = "/Volumes/SSD990-2/ml-cryptanalysis-worktrees/p2-sumset-audit"
sys.path.insert(0, P2)
from csd.sumset_audit import LogField          # noqa
import scripts.sumset_stage2 as S2              # noqa
DATA = "/Volumes/SSD990-2/ml-ic-20261031-data/p2"
HERE = os.path.dirname(os.path.abspath(__file__))
S2.torch.set_num_threads(1)
NB = 200


def char_feats(L, xs):
    """2n columns: Tr(z^i x^-1), Tr(z^i x^3), i<n.  x=0 -> inverse 0.  Returns float32 0/1, zero-padded to 2*NPAD."""
    n = L.n
    xs = np.asarray(xs, dtype=np.int64)
    inv = np.where(xs == 0, 0, L.exp[(L.N - np.maximum(L.log[xs], 0)) % L.N])
    cube = L.mul(xs, L.sq(xs))
    cols = []
    for y in (inv, cube):
        out = np.zeros((len(xs), S2.NPAD), dtype=np.float32)
        for i in range(n):
            out[:, i] = L.trace(L.mul(y, np.int64(1 << i)))
        cols.append(out)
    return np.concatenate(cols, axis=1)


def design(model, n, x, L, extra):
    B = S2.bits(x, n)
    F = S2.feats(model, B)
    if extra:
        F = np.concatenate([F, char_feats(L, x) - 0.5], axis=1)
    return F.astype(np.float32)


def run_arm(model, n, L, xtr, ytr, xte, yte, extra, nperm, rng_perm):
    Ftr = design(model, n, xtr, L, extra); Fte = design(model, n, xte, L, extra)
    s = S2.fit(model, Ftr, ytr, 1)(Fte)
    nulls = []
    for k in range(nperm):
        yp = rng_perm.permutation(ytr)
        nulls.append(float(S2.auc_score(yte, S2.fit(model, Ftr, yp, 100 + k)(Fte))))
    return s, nulls


def q99(nulls):
    a = np.array(nulls); return float(max(a.max(), a.mean() + 2.33 * a.std(ddof=1)))


def paired_ci(y, s0, s1, seed):
    r = np.random.default_rng(seed); out = []
    for _ in range(NB):
        i = r.integers(0, len(y), len(y))
        if y[i].min() == y[i].max(): continue
        a0, a1 = S2.auc_score(y[i], s0[i]), S2.auc_score(y[i], s1[i]); out.append(a1 / a0)
    return [float(np.percentile(out, 2.5)), float(np.percentile(out, 97.5))]


def main(n):
    L = LogField(n)
    NP = {"LR": 100, "LR2": 20, "MLP": 20}
    res = {"n": n, "configs": [], "controls": []}
    files = sorted(glob.glob(f"{DATA}/armA_{n}_*.npz"))
    for fn in files:
        t0 = time.time(); f = np.load(fn)
        xtr, ytr, xte, yte = f["xtr"], f["ytr"], f["xte"], f["yte"]
        cfg = {"file": os.path.basename(fn), "n_test": int(len(yte)), "models": {}}
        for m in ("LR", "LR2", "MLP"):
            s0, _ = run_arm(m, n, L, xtr, ytr, xte, yte, False, 0, None)
            s1, nl = run_arm(m, n, L, xtr, ytr, xte, yte, True, NP[m], np.random.default_rng(777))
            a0, a1 = S2.auc_score(yte, s0), S2.auc_score(yte, s1)
            cfg["models"][m] = {"auc_base": float(a0), "auc_feat": float(a1), "lift": float(a1 / a0),
                                "lift_ci": paired_ci(yte, s0, s1, 5), "q99_feat": q99(nl), "feat_exceeds_q99": bool(a1 > q99(nl))}
        cfg["sec"] = time.time() - t0
        res["configs"].append(cfg); print(cfg["file"], {m: round(v["lift"], 4) for m, v in cfg["models"].items()}, round(cfg["sec"]), flush=True)
        json.dump(res, open(f"{HERE}/result_{n}.json", "w"))
    # planted character leak control on the first file
    fn = files[0]; f = np.load(fn); rng = np.random.default_rng(31)
    xs = np.concatenate([f["xtr"], f["xte"]]); ys = np.concatenate([f["ytr"], f["yte"]])
    ntr = len(f["ytr"]); ch = char_feats(L, xs)[:, S2.NPAD + 3]          # Tr(z^3 x^-1)
    ctrl = {}
    pool = {c: np.nonzero(ch == c)[0] for c in (0, 1)}
    x2 = xs.copy(); sel = rng.random(len(xs)) < 0.10
    for i in np.nonzero(sel)[0]: x2[i] = xs[rng.choice(pool[int(ys[i])])]
    for m in ("LR", "LR2", "MLP"):
        s0, _ = run_arm(m, n, L, x2[:ntr], ys[:ntr], x2[ntr:], ys[ntr:], False, 0, None)
        s1, nl = run_arm(m, n, L, x2[:ntr], ys[:ntr], x2[ntr:], ys[ntr:], True, NP[m], np.random.default_rng(777))
        ctrl[m] = {"auc_base": float(S2.auc_score(ys[ntr:], s0)), "auc_feat": float(S2.auc_score(ys[ntr:], s1)), "q99": q99(nl),
                   "detected": bool(S2.auc_score(ys[ntr:], s1) > q99(nl))}
    res["controls"].append({"file": os.path.basename(fn), "kind": "Tr(z^3 x^-1)=label on 10%", "models": ctrl})
    json.dump(res, open(f"{HERE}/result_{n}.json", "w")); print("done", ctrl, flush=True)


if __name__ == "__main__":
    main(int(sys.argv[1]))
