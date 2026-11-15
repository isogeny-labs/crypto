#!/usr/bin/env python3
"""Lead E, arm `xproof`: the scan certificate written out as a checked clausal proof.

The scan (x1 first, Gaussian elimination at each leaf, one more branch when a free bit is left)
is replayed and every step is emitted as clauses:
  * definitions d = a XOR b of fresh variables (4 clauses each, extended resolution); they are
    appended to the CNF and verified syntactically by `check_defs` (d is larger than every
    variable used before, so each is a conservative extension; in DRAT they would be RAT lines);
  * lemmas, each of which must be RUP for the checker (rup_dep) against CNF + definitions.
Rows are kept as sorted variable lists with prefix-parity variables P[j] = v_0 + .. + v_j.
Substituting the gates of an original row under a leaf, and adding two rows, are both linear
sweeps with a constant number of lemmas per position.
CNF of this arm: same constraints as scanproof.render_cnf (dual-rail gates), but each stage row
is written as a full prefix chain with its variables ordered by the non-selector operand of
their gate (so that substitution leaves equal variables adjacent), closed by a unit clause.
"""
import os, sys, subprocess
import scanproof as sp


def xor_cl(vs, rhs):
    out = []
    k = len(vs)
    for m in range(1 << k):
        if bin(m).count("1") % 2 != rhs:
            out.append([(-v if (m >> i) & 1 else v) for i, v in enumerate(vs)])
    return out


class Row:
    __slots__ = ("vs", "P", "rhs")

    def __init__(self, vs, P, rhs):
        self.vs, self.P, self.rhs = vs, P, rhs


class Mirror:
    """Plain Gauss-Jordan over bitmask rows (bit v = variable v, bit 0 = constant); no proof."""

    def __init__(self):
        self.rows, self.conflict = {}, False

    def copy(self):
        m = Mirror()
        m.rows, m.conflict = dict(self.rows), self.conflict
        return m

    def add(self, r):
        rows = self.rows
        for v, pr in rows.items():
            if (r >> v) & 1:
                r ^= pr
        if r == 0:
            return
        if r == 1:
            self.conflict = True
            return
        pv = r.bit_length() - 1
        for k in list(rows):
            if (rows[k] >> pv) & 1:
                rows[k] ^= r
        rows[pv] = r

    def const(self, v):
        """0/1 if v is constant, else None."""
        r = self.rows.get(v)
        if r is None:
            return None
        r ^= 1 << v
        return r if r <= 1 else None


class XProof:
    def __init__(self, sysm, out_path):
        self.sys = sysm
        self.gate = {t: (a, b) for t, a, b in sysm.gates}
        nv, clauses, xors = sysm.render("dual")
        self.clauses = [list(c) for c in clauses]
        self.nv = nv
        nrows = len(sysm.rows)
        self.raw = []
        for idx, (vs, rhs) in enumerate(xors):
            if idx >= nrows:  # rail XOR t + r + b = 0: direct clauses, not used by the proof
                self.clauses += xor_cl(vs, int(rhs))
                continue
            vs = sorted(vs, key=lambda v: (self.gate[v][1] if v in self.gate else v, v))
            P = [vs[0]]
            for v in vs[1:]:
                self.nv += 1
                self.clauses += xor_cl([self.nv, P[-1], v], 0)
                P.append(self.nv)
            self.clauses.append([P[-1] if rhs else -P[-1]])
            self.raw.append(Row(vs, P, int(rhs)))
        self.nv0 = self.nv
        self.defs = []          # (d, a, b)
        self.out = open(out_path, "w")
        self.n_lem = 0
        self.stat = {"subst_pos": 0, "merges": 0, "merge_pos": 0, "closings": 0, "leaves": 0, "nodes": 0}
        self.base_order = list(range(1, sysm.nbase + 1))

    # ---- emission
    def lem(self, c):
        self.out.write(" ".join(map(str, c)) + " 0\n")
        self.n_lem += 1

    def define(self, a, b):
        self.nv += 1
        self.defs.append((self.nv, a, b))
        return self.nv

    def split_lem(self, K, w, direct=False):
        if direct or w in K or -w in K:
            self.lem(K)
        else:
            self.lem(K + [w])
            self.lem(K + [-w])
            self.lem(K)

    # ---- substitution of constants and gates in an original row, under context
    def substitute(self, A, mir, neg):
        """-> Row over non-constant variables, or 'conflict', or None (row vanished)."""
        cur, grp, par, D, pol = None, None, 0, None, 0
        nvs, nP = [], []
        for j, v in enumerate(A.vs):
            self.stat["subst_pos"] += 1
            if v in self.gate:
                a, b = self.gate[v]
                ca = mir.const(a)
                if ca == 0:
                    e, ep = None, 0
                else:
                    cb = mir.const(b)
                    e, ep = (b, 0) if cb is None else (None, cb)
            else:
                cv = mir.const(v)
                e, ep = (v, 0) if cv is None else (None, cv)
            pol ^= ep
            if e is not None:
                if e != grp:
                    if par:
                        cur = D
                        nvs.append(grp)
                        nP.append(D)
                    grp, par, D = e, 0, None
                par ^= 1
                if par and D is None:
                    D = e if cur is None else self.define(cur, e)
            X = D if par else cur
            Pj = A.P[j]
            if X is None:
                Ks = [[Pj if pol else -Pj]]
            else:
                Ks = xor_cl([Pj, X], pol)
            for K in Ks:
                self.split_lem(K + neg, v, direct=(j == 0))
        if par:
            cur = D
            nvs.append(grp)
            nP.append(D)
        rhs = A.rhs ^ pol
        if cur is None:
            if rhs:
                return "conflict"
            return None
        self.lem([cur if rhs else -cur] + neg)
        return Row(nvs, nP, rhs)

    # ---- row addition
    def merge(self, A, B, neg):
        self.stat["merges"] += 1
        i = j = 0
        a = b = c = None
        cvs, cP = [], []
        na, nb = len(A.vs), len(B.vs)
        while i < na or j < nb:
            self.stat["merge_pos"] += 1
            va = A.vs[i] if i < na else None
            vb = B.vs[j] if j < nb else None
            if vb is None or (va is not None and va < vb):
                w, a, i, both = va, A.P[i], i + 1, False
            elif va is None or vb < va:
                w, b, j, both = vb, B.P[j], j + 1, False
            else:
                w, a, b, i, j, both = va, A.P[i], B.P[j], i + 1, j + 1, True
            if not both:
                c = w if c is None else self.define(c, w)
                cvs.append(w)
                cP.append(c)
            S = []
            for x in (c, a, b):
                if x is None:
                    continue
                if x in S:
                    S.remove(x)
                else:
                    S.append(x)
            if S:
                for K in xor_cl(S, 0):
                    self.split_lem(K, w)
        rhs = A.rhs ^ B.rhs
        if c is None:
            return "conflict" if rhs else None
        self.lem([c if rhs else -c] + neg)
        return Row(cvs, cP, rhs)

    def insert(self, r, rows, neg):
        """Forward elimination: reduce r by pivot rows (largest pivot first), then store it."""
        while True:
            pv = None
            for v in reversed(r.vs):
                if v in rows:
                    pv = v
                    break
            if pv is None:
                break
            r = self.merge(r, rows[pv], neg)
            if r is None or r == "conflict":
                return r
        rows[r.vs[-1]] = r
        return True

    # ---- the scan
    def refute(self, ctx, rows, mir, pend):
        self.stat["nodes"] += 1
        neg = [-x for x in ctx]
        if len(ctx) == self.sys.l:
            self.stat["leaves"] += 1
        while True:
            if mir.conflict:
                break
            # constants are taken from a snapshot, and only when every variable of the rows
            # derived so far is constant: then unit propagation alone reproduces them
            # (the proof keeps rows in echelon form, without back-substitution)
            if any(mir.const(v) is None for r in rows.values() for v in r.vs):
                break
            mir0 = mir.copy()
            ready = [A for A in pend if all(mir0.const(self.gate[v][0]) is not None for v in A.vs if v in self.gate)]
            if not ready:
                break
            ids = set(map(id, ready))
            pend = [A for A in pend if id(A) not in ids]
            for A in ready:
                r = self.substitute(A, mir0, neg)
                if r is None:
                    continue
                if r == "conflict":
                    mir.conflict = True
                    break
                m = r.rhs
                for v in r.vs:
                    m |= 1 << v
                mir.add(m)
                if self.insert(r, rows, neg) == "conflict":
                    mir.conflict = True
                    break
        if not mir.conflict:
            for o in self.sys.ors:
                if all(mir.const(v) == 0 for v in o):
                    mir.conflict = True
        if not mir.conflict:
            fv = next((v for v in self.base_order if mir.const(v) is None), None)
            if fv is None:
                return False  # satisfying assignment
            for lit in (-fv, fv):
                m2 = mir.copy()
                m2.add((1 << fv) | (1 if lit > 0 else 0))
                if not self.refute(ctx + [lit], dict(rows), m2, list(pend)):
                    return False
        self.lem(neg)  # closing clause: RUP from the conflict (or from the two children)
        self.stat["closings"] += 1
        return True

    def run(self):
        ok = self.refute([], {}, Mirror(), list(self.raw))
        self.out.close()
        return ok

    def formula(self):
        cl = list(self.clauses)
        for d, a, b in self.defs:
            cl += xor_cl([d, a, b], 0)
        return self.nv, cl


def check_defs(nv0, n_cnf_clauses, clauses):
    """Independent syntactic check: clauses after the CNF are groups of 4 defining a fresh
    variable d = nv0 + k + 1 as the XOR of two different smaller variables."""
    ext = clauses[n_cnf_clauses:]
    if len(ext) % 4:
        return False
    for k in range(len(ext) // 4):
        d = nv0 + k + 1
        grp = ext[4 * k:4 * k + 4]
        vs = sorted({abs(x) for c in grp for x in c})
        if len(vs) != 3 or vs[2] != d or any(len(c) != 3 for c in grp):
            return False
        if sorted(map(sorted, grp)) != sorted(map(sorted, xor_cl([d, vs[0], vs[1]], 0))):
            return False
    return True


def run_checker(cnf, proof, core=None, corecnf=None):
    p = subprocess.run([sp.CHECKER, cnf, proof] + ([core, corecnf] if core else []), capture_output=True, text=True)
    line = p.stdout.strip()
    return dict(kv.split("=") for kv in line.split()[1:]) if line.startswith("RESULT") else {"status": "ERROR", "raw": line}


def measure(sysm, tag):
    """Build, check, trim once, re-check the trimmed proof.  -> dict."""
    pp = os.path.join(sp.SCR, tag + ".xp")
    xp = XProof(sysm, pp)
    ok = xp.run()
    res = {"engine_verdict": "UNSAT" if ok else "SAT", "stat": xp.stat, "defs": len(xp.defs),
           "raw_lemmas": xp.n_lem, "raw_clauses": xp.n_lem + 4 * len(xp.defs), "cnf_vars": xp.nv0,
           "cnf_clauses": len(xp.clauses)}
    if not ok:
        res.update(status="NO_PROOF", checked=False)
        return res
    nv, cl = xp.formula()
    res["defs_syntax_ok"] = check_defs(xp.nv0, len(xp.clauses), cl)
    cp = pp + ".cnf"
    sp.write_cnf(cp, nv, cl)
    d = run_checker(cp, pp, pp + ".core", pp + ".corecnf")
    res["status"] = d["status"]
    if d["status"] != "VERIFIED" or not res["defs_syntax_ok"]:
        res.update(checked=False, raw=d)
        return res
    keep = set(int(x) for x in open(pp + ".core"))
    tp = pp + ".trim"
    n = 0
    with open(pp) as f, open(tp, "w") as g:
        for i, ln in enumerate(f):
            if i in keep:
                g.write(ln)
                n += 1
    d2 = run_checker(cp, tp, tp + ".core", tp + ".corecnf")
    assert d2["status"] == "VERIFIED", d2
    core_defs = 0
    with open(tp + ".corecnf") as f:
        for ln in f:
            if ln[0] != "p" and max(abs(int(x)) for x in ln.split()) > xp.nv0:
                core_defs += 1
    res.update(checked=True, T1_lemmas=int(d["core_lemmas"]), final_lemmas=int(d2["core_lemmas"]) + 1,
               final_lits=int(d2["core_lits"]), final_defs=core_defs,
               final_core_cnf=int(d2["core_orig"]) - core_defs)
    res["final"] = {"clauses": res["final_lemmas"] + core_defs}
    for fn in (pp, cp, tp, pp + ".core", pp + ".corecnf", tp + ".core", tp + ".corecnf"):
        os.remove(fn)
    return res


if __name__ == "__main__":
    import time
    n, l = int(sys.argv[1]), int(sys.argv[2])
    sp.build_checker()
    cell = sp.make_cell(l, n, 999)
    R = sp.nondec_targets(cell, 1)[0]
    t = time.time()
    print(measure(sp.system(cell, R), "xp_probe"), round(time.time() - t, 1))
