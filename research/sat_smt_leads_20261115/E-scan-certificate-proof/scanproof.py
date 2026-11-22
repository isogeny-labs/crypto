#!/usr/bin/env python3
"""Lead E: scan-guided refutations of m = 3 point decomposition, and their checked size.

Instances and the S3-chain system come from sub.py (verbatim copy of
../../sat_smt_leads_20261114/followup-subcube/sub.py).  Added here: the u != 0 clause
(closes the twist false-SAT at u = 0), target filtering, a pure-CNF rendering (dual-rail
gates, XOR rows cut to 3-variable XORs with auxiliaries), the scan-guided clausal proof
(one CDCL call per x1 leaf, proofs concatenated, leaf clauses resolved to the empty
clause), checking/trimming through rup_dep, and the XOR-aware step counter.
"""
import os, subprocess, random
import sub
from pysat.solvers import Solver

HERE = os.path.dirname(os.path.abspath(__file__))
SCR = os.environ.get("E_SCRATCH", "/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/E")
CHECKER = os.path.join(SCR, "rup_dep")


def build_checker():
    os.makedirs(SCR, exist_ok=True)
    if not os.path.exists(CHECKER):
        subprocess.check_call(["gcc", "-O2", "-o", CHECKER, os.path.join(HERE, "rup_dep.c")])


def cell_seed(n, l):
    return 20261115 * 1000003 + 1000 * n + 10 * l


def make_cell(l, n, seed):
    sub.CELLS[l] = n
    return sub.Cell(l, seed)


def u_zero_event(cell, R):
    """True if R -+ P1 is the 2-torsion point for some base point (u = 0), or R is 2-torsion."""
    return R[0] == 0 or any((lambda Q: Q is not None and Q[0] == 0)(cell.E.add(R, P)) for P in cell.signed)


def nondec_targets(cell, k):
    out = []
    while len(out) < k:
        R = cell.random_target()
        if u_zero_event(cell, R) or cell.decomposable(R):
            continue
        out.append(R)
    return out


def planted_targets(cell, k):
    out = []
    while len(out) < k:
        R = cell.planted_target()
        if not u_zero_event(cell, R):
            out.append(R)
    return out


def system(cell, R):
    sysm = sub.System(cell, R)
    sysm.ors.append(list(sysm.u))  # u != 0: u = 0 would admit non-rational x2, x3
    return sysm


def render_cnf(sysm):
    """Dual-rail gates as clauses; every XOR cut to 3-variable XORs (4 clauses each)."""
    nv, clauses, xors = sysm.render("dual")
    clauses = [list(c) for c in clauses]

    def direct(vs, rhs):
        k = len(vs)
        for m in range(1 << k):
            if bin(m).count("1") % 2 != rhs:
                clauses.append([(-v if (m >> i) & 1 else v) for i, v in enumerate(vs)])

    for vs, rhs in xors:
        vs = list(vs)
        while len(vs) > 3:
            nv += 1
            direct([vs[0], vs[1], nv], 0)
            vs = [nv] + vs[2:]
        direct(vs, int(rhs))
    return nv, clauses


def leaf_assumptions(sysm, x1):
    return [(v if (x1 >> i) & 1 else -v) for i, v in enumerate(sysm.c)]


def parse_proof(lines):
    out = []
    for ln in lines:
        if ln.startswith("d"):
            continue
        c = [int(t) for t in ln.split()]
        assert c[-1] == 0
        if len(c) > 1:
            out.append(c[:-1])
    return out


class Proof:
    """Clause list with a role per clause: ('lemma', leaf) / ('leaf', x1) / ('tree', k) / ('empty',)."""

    def __init__(self):
        self.cl, self.role = [], []

    def add(self, c, role):
        self.cl.append(list(c))
        self.role.append(role)

    def close_tree(self, sysm):
        l = sysm.l
        for k in range(l - 1, -1, -1):
            for pre in range(1 << k):
                self.add([(-v if (pre >> i) & 1 else v) for i, v in enumerate(sysm.c[:k])],
                         ("tree", k) if k else ("empty",))

    def sub(self, keep):
        p = Proof()
        for i in keep:
            p.add(self.cl[i], self.role[i])
        return p


def scan_proof(sysm, nv, clauses, solver="cd15", warm=False, order=None, extra=None):
    """One CDCL call per x1 leaf under assumptions.  cold: fresh solver per leaf;
    warm: one incremental solver.  -> (Proof, per-leaf conflicts) or (None, model x1)."""
    l = sysm.l
    pr, confl = Proof(), []
    order = list(order) if order is not None else list(range(1 << l))
    base = clauses + (extra or [])
    s, seen, prev = None, 0, 0
    for x1 in order:
        if s is None or not warm:
            if s is not None:
                s.delete()
            s, seen, prev = Solver(name=solver, bootstrap_with=base, with_proof=True), 0, 0
        ass = leaf_assumptions(sysm, x1)
        if s.solve(assumptions=ass):
            s.delete()
            return None, x1
        lem = parse_proof(s.get_proof())
        for c in lem[seen:] if warm else lem:
            pr.add(c, ("lemma", x1))
        seen = len(lem)
        cf = s.accum_stats()["conflicts"]
        confl.append(cf - prev)
        prev = cf if warm else 0
        pr.add([-a for a in ass], ("leaf", x1))
    s.delete()
    pr.close_tree(sysm)
    return pr, confl


def free_proof(nv, clauses, solver="g4", budget=None):
    """CDCL-found refutation of the same CNF with no guidance. -> (Proof or None, conflicts)."""
    with Solver(name=solver, bootstrap_with=clauses, with_proof=True) as s:
        if budget:
            s.conf_budget(budget)
            r = s.solve_limited()
        else:
            r = s.solve()
        cf = s.accum_stats()["conflicts"]
        if r is not False:
            return None, cf, r
        pr = Proof()
        for c in parse_proof(s.get_proof()):
            pr.add(c, ("lemma", -1))
        pr.add([], ("empty",))
        return pr, cf, r


def write_cnf(path, nv, clauses):
    with open(path, "w") as f:
        f.write("p cnf %d %d\n" % (nv, len(clauses)))
        f.write("".join(" ".join(map(str, c)) + " 0\n" for c in clauses))


def check(cnf_path, pr, tag, deps=False, core_cnf=False):
    """Run rup_dep.  -> dict(status, n_lemmas, core (kept indices), deps {i: [j..]}, ...)."""
    pp = os.path.join(SCR, tag + ".drup")
    with open(pp, "w") as f:
        f.write("".join(" ".join(map(str, c)) + " 0\n" for c in pr.cl))
    ci, cc, dp = pp + ".core", pp + ".corecnf", pp + ".deps"
    p = subprocess.run([CHECKER, cnf_path, pp, ci, cc] + ([dp] if deps else []), capture_output=True, text=True)
    line = p.stdout.strip()
    d = dict(kv.split("=") for kv in line.split()[1:]) if line.startswith("RESULT") else {"status": "ERROR"}
    res = {"status": d.get("status"), "n_clauses": len(pr.cl)}
    if res["status"] != "VERIFIED":
        res["raw"] = line or p.stderr[-200:]
        return res
    keep = [int(x) for x in open(ci)]
    assert keep[-1] == len(pr.cl) - 1 and pr.role[keep[-1]] == ("empty",)
    res.update(core=keep, core_orig=int(d["core_orig"]), core_lits=int(d["core_lits"]),
               n_orig=int(d["n_orig"]), props=int(d["props"]), core_cnf=cc if core_cnf else None)
    if deps:
        dd = {}
        for ln in open(dp):
            a, b = ln.split(":")
            dd[int(a)] = [int(x) for x in b.split()]
        res["deps"] = dd
    for fn in ([pp, ci, dp] if deps else [pp, ci]) + ([] if core_cnf else [cc]):
        if os.path.exists(fn):
            os.remove(fn)
    return res


def trim(cnf_path, pr, tag, rounds=8):
    """T1 (one backward pass) and T2 (iterate to a fixpoint).  -> (t1, t2, final Proof, final check)."""
    r = check(cnf_path, pr, tag, deps=True)
    if r["status"] != "VERIFIED":
        return r, None, None, None
    t1, cur, last = r, pr, r
    for _ in range(rounds):
        nxt = cur.sub(last["core"])
        if len(nxt.cl) == len(cur.cl):
            break
        r2 = check(cnf_path, nxt, tag, deps=True)
        assert r2["status"] == "VERIFIED", "re-check of a core failed"
        cur, last = nxt, r2
    if cur is pr:  # nothing removed: the core is the proof
        cur = pr.sub(last["core"])
        last = check(cnf_path, cur, tag, deps=True)
        assert last["status"] == "VERIFIED"
    return t1, last, cur, last


def size_stats(pr, keep):
    roles = [pr.role[i][0] for i in keep]
    return {"clauses": len(keep), "lemmas": roles.count("lemma"), "leaf": roles.count("leaf"),
            "tree": roles.count("tree") + roles.count("empty"),
            "lits": sum(len(pr.cl[i]) for i in keep)}


def sharing(pr, deps, sysm):
    """Leaves served by each reasoning lemma of a verified proof.

    Closing clause: a proof clause over x1-coordinate variables only (leaf clauses, tree
    clauses, the empty clause, and solver lemmas of that shape); its cube is the set of x1
    leaves that falsify it.  Reasoning lemma: any other proof clause.  A reasoning lemma
    serves the union of the cubes of the closing clauses whose RUP check reaches it through
    reasoning lemmas only (dependencies are not followed through another closing clause).
    -> list of leaf counts, one per reasoning lemma that serves at least one leaf."""
    cpos = {v: i for i, v in enumerate(sysm.c)}
    L = 1 << sysm.l
    full = (1 << L) - 1
    bitmask = []
    for i in range(sysm.l):
        m = 0
        for x in range(L):
            if (x >> i) & 1:
                m |= 1 << x
        bitmask.append(m)

    def cube(c):
        m = full
        for lit in c:
            b = bitmask[cpos[abs(lit)]]
            m &= (full ^ b) if lit > 0 else b
        return m

    closing = [all(abs(x) in cpos for x in c) for c in pr.cl]
    use = [0] * len(pr.cl)
    for i in sorted(deps, reverse=True):
        m = cube(pr.cl[i]) if closing[i] else use[i]
        if m:
            for j in deps[i]:
                assert j < i
                if not closing[j]:
                    use[j] |= m
    return [bin(use[i]).count("1") for i in range(len(pr.cl)) if not closing[i] and use[i]]


# ---------------------------------------------------------------- XOR-aware step counter
def _counting_add(self, r):
    rows = self.rows
    m = r >> 1
    i = 1
    while m:
        low = (m & -m).bit_length()
        v = low + i - 1
        pr = rows.get(v)
        if pr is not None:
            self.wsum += bin((r | pr) >> 1).count("1")
            r ^= pr
            self.xors += 1
        m >>= low
        i += low
    if r == 0:
        return
    if r == 1:
        self.conflict = True
        return
    pv = r.bit_length() - 1
    bit = 1 << pv
    for k, q in rows.items():
        if q & bit:
            self.wsum += bin((q | r) >> 1).count("1")
            rows[k] = q ^ r
            self.xors += 1
    rows[pv] = r


sub.Eng.wsum = 0  # clones start from the class default
sub.Eng.add = _counting_add


def xor_scan(sysm):
    """Own DPLL + Gauss engine, x1 first, then free base variables.  Counts per leaf:
    rowops (XOR row additions), wsum (sum over row additions of the union support size),
    props (gate substitutions = unit-propagation steps), nodes (assignments made).
    -> dict(sat, root{...}, leaves[{...}])."""
    l = sysm.l
    root = sub.Eng(sysm)
    ng = len(sysm.gates)
    rootc = {"rowops": root.xors, "wsum": root.wsum, "props": ng - len(root.pend), "nodes": 0}
    leaves = []
    sat = False

    def take(e, st, parent_pend):
        st["rowops"] += e.xors
        st["wsum"] += e.wsum
        st["props"] += parent_pend - len(e.pend)
        st["nodes"] += 1
        e.xors = e.wsum = e.fix = 0

    def down(e, st):
        if e.conflict:
            return None
        v = e.free_var()
        if v is None:
            return e.model()
        for val in (0, 1):
            ch = e.clone()
            ch.wsum = 0
            ch.assign(v, val)
            take(ch, st, len(e.pend))
            m = down(ch, st)
            if m is not None:
                return m
        return None

    for x1 in range(1 << l):
        st = {"rowops": 0, "wsum": 0, "props": 0, "nodes": 0}
        e = root.clone()
        e.wsum = 0
        for i, v in enumerate(sysm.c):
            if e.conflict:
                break
            pp = len(e.pend)
            e.assign(v, (x1 >> i) & 1)
            take(e, st, pp)
        st["depth_nodes"] = st["nodes"]
        if down(e, st) is not None:
            sat = True
        leaves.append(st)
    return {"sat": sat, "root": rootc, "leaves": leaves}


def xor_translation_clauses(st):
    """Clause count of a standard XOR-to-DRAT translation of the counted steps (formula in
    PROTOCOL.md): 8 per support variable of each row addition + 1, 2 per gate substitution,
    1 per search node."""
    return 8 * st["wsum"] + st["rowops"] + 2 * st["props"] + st["nodes"]


# ---------------------------------------------------------------- independent forward check
def forward_check(clauses, pr):
    """Forward RUP check with MiniSat 2.2 unit propagation (python-sat), lemma by lemma.
    -> True iff every proof clause is RUP at its position and the last one is empty."""
    with Solver(name="m22", bootstrap_with=clauses) as s:
        for c in pr.cl:
            ok, _ = s.propagate(assumptions=[-x for x in c])
            if ok:
                return False
            if c:
                s.add_clause(c)
    return pr.cl[-1] == []


def subsume_filter(clauses, pr):
    """Drop proof clauses subsumed by an original clause or an earlier kept proof clause."""
    occ = {}
    def reg(fs):
        for x in fs:
            occ.setdefault(x, []).append(fs)
    for c in clauses:
        reg(frozenset(c))
    keep = []
    for i, c in enumerate(pr.cl):
        fs = frozenset(c)
        sub_ = False
        if c and pr.role[i][0] == "lemma":
            for x in c:
                for d in occ.get(x, ()):
                    if len(d) <= len(fs) and d <= fs:
                        sub_ = True
                        break
                if sub_:
                    break
        if not sub_:
            keep.append(i)
            if c:
                reg(fs)
    return pr.sub(keep)
