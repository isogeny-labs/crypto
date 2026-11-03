#!/usr/bin/env python3
"""btn_census.py -- one Boolean-tensor-network (BTN) cost yardstick for every encoding.

Inputs (auto-detected, or --format):
  dimacs : DIMACS CNF, CryptoMiniSat 'x' XOR lines, and WDSat ANF lines
           ('x' lines containing '.d v1..vd' monomials and/or 'T' = constant 1).
           Optional comment hints: 'c bits B', 'c arity M', 'c iface v1 v2 .. 0'.
  labanf : the lab corpus 'p anf V E' format (one GF(2) polynomial per line, '+', '*').
  crate  : JSON from rust-dump (hashed-ecdl-tensor Network::census_json).
  edges  : plain numbers via --edges/--max-degree/--vertices/--meta (published rows).

Model.  Variables are wires, constraints are vertices.  Constants (DIMACS var 1 when
it is asserted TRUE by the first clause) are absorbed into vertex tensors.
Subdivision rule (Bochum Remark 1 as implemented here; see REPORT.md):
  R1 fan-out: a wire with d>=3 ports -> binary equality tree: d-2 EQ3 vertices,
     2d-3 edges; d=2 -> 1 edge; d=1 -> 1 edge + a unary all-ones 'free' vertex;
     d=0 -> not a wire (counted as unused).  Afterwards every edge has degree 2.
  R2 arity split (cap=3, 'split' mode only): XOR of a terms -> a-2 XOR3 vertices
     (a-3 new wires); OR clause of w literals -> w-2 vertices of arity<=3 (w-3 new
     wires, functional so parsimonious); ANF monomial of degree d -> d-1 AND2
     vertices (d-1 new wires, prefix/monomial sharing across equations).
  'bb' mode: R1 only, every constraint a black-box vertex (D.1's reading).
  Opaque crate vertices are never split (their tensors are not known to be gate-like).
"""
import sys, re, json, math, argparse, gzip
def _open(p): return gzip.open(p, 'rt') if p.endswith('.gz') else open(p)
from collections import Counter, defaultdict

LINEAR = {"xor", "eq", "free", "pin"}

# ---------------------------------------------------------------- parsing
class Model:
    """constraints: list of dict(kind, vars:list[int] (ports, may repeat), meta)"""
    def __init__(self):
        self.cons = []          # raw constraints (black-box vertices)
        self.anf = None         # for ANF inputs: list of (terms, parity) with terms = tuple of tuples
        self.hints = {}
        self.iface = None
        self.nvars_declared = 0
        self.edges_direct = None  # crate: explicit edge list (already degree-2 wires)

def parse_dimacs(path):
    M = Model(); M.kind = "dimacs"
    clauses = []; xors = []; anf = []
    is_anf = False
    with _open(path) as f:
        for line in f:
            line = line.strip()
            if not line: continue
            if line.startswith("c"):
                t = line.split()
                if len(t) >= 3 and t[1] == "bits": M.hints["b"] = int(t[2])
                elif len(t) >= 3 and t[1] == "arity": M.hints["m"] = int(t[2])
                elif len(t) >= 2 and t[1] == "iface":
                    M.iface = [int(x) for x in t[2:] if x != "0"]
                continue
            if line.startswith("p"):
                t = line.split(); M.nvars_declared = int(t[2]); continue
            if line.startswith("x"):
                tok = line[1:].split()
                assert tok[-1] == "0"; tok = tok[:-1]
                terms = []; par = 0; i = 0
                while i < len(tok):
                    t = tok[i]
                    if t == "T": par ^= 1; i += 1
                    elif t.startswith("."):
                        d = int(t[1:]); vs = tuple(sorted(set(abs(int(u)) for u in tok[i+1:i+1+d])))
                        # negative literal inside a monomial is not used by the lab corpora
                        terms.append(vs); i += 1 + d; is_anf = is_anf or d >= 2
                    else:
                        v = int(t)
                        if v < 0: par ^= 1
                        terms.append((abs(v),)); i += 1
                anf.append((terms, par))
            else:
                lits = [int(x) for x in line.split()]
                assert lits[-1] == 0; clauses.append(lits[:-1])
    # constant TRUE absorption (sweeper convention: unit clause on var 1 first)
    const_true = bool(clauses) and clauses[0] == [1]
    const_true = const_true or False
    M.const_true = const_true
    nlit = 0
    for c in clauses:
        if const_true and c == [1]: continue
        if const_true:
            if any(l == 1 for l in c): continue
            c = [l for l in c if l != -1]
        vs = [abs(l) for l in c]
        if len(vs) == 0: continue
        M.cons.append(dict(kind="clause" if len(vs) > 1 else "pin", vars=vs))
    anf2 = []
    for terms, par in anf:
        # cancel repeated identical terms (GF(2)), absorb constant var 1
        cnt = Counter(terms); terms = [t for t, c in cnt.items() if c % 2]
        if const_true:
            nt = []
            for t in terms:
                t2 = tuple(v for v in t if v != 1)
                if len(t2) == 0: par ^= 1
                else: nt.append(t2)
            c2 = Counter(nt); terms = [t for t, c in c2.items() if c % 2]
        anf2.append((sorted(terms), par))
    for terms, par in anf2:
        vs = sorted({v for t in terms for v in t})
        nl = any(len(t) >= 2 for t in terms)
        if not vs: continue
        k = "anfeq" if nl else ("xor" if len(vs) > 1 else "pin")
        M.cons.append(dict(kind=k, vars=vs if nl else [v for t in terms for v in t], terms=terms, parity=par))
    M.has_nl_anf = any(c["kind"] == "anfeq" for c in M.cons)
    return M

def parse_labanf(path):
    M = Model(); M.kind = "labanf"; ids = {}
    def vid(n):
        if n not in ids: ids[n] = len(ids) + 1
        return ids[n]
    with _open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("c"): continue
            if line.startswith("p"): continue
            if line.endswith("=0"): line = line[:-2]
            terms = []; par = 0
            for t in line.split("+"):
                t = t.strip()
                if t == "1": par ^= 1; continue
                vs = tuple(sorted(set(vid(u.strip()) for u in t.split("*"))))
                terms.append(vs)
            cnt = Counter(terms); terms = sorted(t for t, c in cnt.items() if c % 2)
            vs = sorted({v for t in terms for v in t})
            if not vs: continue
            nl = any(len(t) >= 2 for t in terms)
            k = "anfeq" if nl else ("xor" if len(vs) > 1 else "pin")
            M.cons.append(dict(kind=k, vars=vs if nl else [v for t in terms for v in t], terms=terms, parity=par))
    M.has_nl_anf = any(c["kind"] == "anfeq" for c in M.cons)
    M.const_true = False
    return M

def parse_crate(path):
    d = json.load(open(path)); M = Model(); M.kind = "crate"
    M.edges_direct = d["edges"]; M.crate_vertices = d["vertices"]
    M.iface_crate = d["interface"]; M.input_bit_count = d.get("input_bit_count", -1)
    return M

# ---------------------------------------------------------------- BTN construction
class Acc:
    """accumulate vertices (kind, arity) and per-wire port counts"""
    def __init__(self):
        self.verts = []              # list of (kind, arity)
        self.ports = Counter()       # wire id -> number of vertex ports
        self.next_aux = 10**9
    def aux(self):
        self.next_aux += 1; return self.next_aux
    def vertex(self, kind, wires):
        self.verts.append((kind, len(wires)))
        for w in wires: self.ports[w] += 1

def xor_chain(A, wires, kind="xor"):
    a = len(wires)
    if a <= 3:
        A.vertex("xor" if a > 1 else "pin", wires); return
    # a-2 vertices of arity 3 (first and last take two original wires, middle ones one)
    cur = wires[:2]; rest = wires[2:]
    prev = None
    # chain: v1(w0,w1,t1), v2(t1,w2,t2), ..., v_{a-2}(t_{a-3}, w_{a-2}, w_{a-1})
    t = A.aux(); A.vertex("xor", [wires[0], wires[1], t]); prev = t
    for w in wires[2:-2]:
        t = A.aux(); A.vertex("xor", [prev, w, t]); prev = t
    A.vertex("xor", [prev, wires[-2], wires[-1]])

def clause_chain(A, wires):
    w = len(wires)
    if w <= 3:
        A.vertex("clause" if w > 1 else "pin", wires); return
    t = A.aux(); A.vertex("or", [wires[0], wires[1], t]); prev = t
    for x in wires[2:-2]:
        t = A.aux(); A.vertex("or", [prev, x, t]); prev = t
    A.vertex("or", [prev, wires[-2], wires[-1]])

def build(M, mode, share=True):
    A = Acc()
    if M.kind == "crate":
        # wires already degree 2; vertex = crate vertex (opaque, never split)
        for v in M.crate_vertices:
            A.verts.append((v["class"] if v["class"] in LINEAR or v["class"].startswith("xor") else "nl", v["arity"]))
        A.verts = [("xor" if k.startswith("xor") else k, a) for k, a in A.verts]
        A.crate_edges = len(M.edges_direct)
        return A
    mono_cache = {}
    def monomial_wire(t):
        if len(t) == 1: return t[0]
        if share and t in mono_cache: return mono_cache[t]
        prev = monomial_wire(t[:-1]) if len(t) > 2 else t[0]
        if len(t) == 2: prev = t[0]
        out = A.aux()
        A.vertex("and", [prev, t[-1], out])
        if share: mono_cache[t] = out
        return out
    for c in M.cons:
        k = c["kind"]
        if k == "anfeq":
            if mode == "bb":
                A.vertex("nl-anf", sorted({v for t in c["terms"] for v in t}))
            else:
                ws = [monomial_wire(t) for t in c["terms"]]
                if len(ws) == 0: continue
                xor_chain(A, ws)
        elif k == "xor":
            xor_chain(A, c["vars"]) if mode == "split" else A.vertex("xor", c["vars"])
        elif k == "clause":
            clause_chain(A, c["vars"]) if mode == "split" else A.vertex("clause", c["vars"])
        elif k == "pin":
            A.vertex("pin", c["vars"][:1])
    return A

def finish(A, M):
    """apply R1, return dict of structural numbers"""
    if M.kind == "crate":
        s = A.crate_edges
        nverts = len(A.verts)
        maxdeg = max(a for _, a in A.verts)
        nlports = sum(a for k, a in A.verts if k not in LINEAR)
        nlverts = sum(1 for k, a in A.verts if k not in LINEAR)
        return dict(s=s, V=nverts, delta=maxdeg, nl_ports=nlports, nl_verts=nlverts,
                    wires=s, eq_verts=sum(1 for k, a in A.verts if k == "eq"),
                    free_verts=0, unused=0, degree2_check=(sum(a for _, a in A.verts) == 2 * s))
    s = 0; eqv = 0; freev = 0; unused = 0
    for w, d in A.ports.items():
        if d == 1: s += 1; freev += 1
        elif d == 2: s += 1
        else: s += 2 * d - 3; eqv += d - 2
    verts = list(A.verts)
    nverts = len(verts) + eqv + freev
    degs = [a for _, a in verts] + ([3] if eqv else []) + ([1] if freev else [])
    port_total = sum(a for _, a in verts) + 3 * eqv + freev
    nlports = sum(a for k, a in verts if k not in LINEAR)
    nlverts = sum(1 for k, a in verts if k not in LINEAR)
    return dict(s=s, V=nverts, delta=max(degs) if degs else 0, nl_ports=nlports, nl_verts=nlverts,
                wires=len(A.ports), eq_verts=eqv, free_verts=freev,
                unused=0, degree2_check=(port_total == 2 * s))

# ---------------------------------------------------------------- interface
def iface_anf(M):
    """Upper bound on the interface (inputs that determine the rest) for an ANF system.
    Head heuristics (one head per equation; head must be linear in its equation, i.e. a
    singleton term absent from every monomial of that equation).  A candidate is accepted
    only if propagation from the non-heads determines every head (each step: an equation
    whose other variables are known fixes its linear head, valid over GF(2)).  The result
    is the minimum |non-heads| + |undetermined heads| over three orderings, so it is a
    verified upper bound on the minimum interface, not the minimum itself."""
    eqs = []
    for c in M.cons:
        terms = c.get("terms") or [(v,) for v in c["vars"]]
        lin = [t[0] for t in terms if len(t) == 1]
        inmono = {v for t in terms if len(t) > 1 for v in t}
        allv = {v for t in terms for v in t}
        eqs.append((allv, [v for v in lin if v not in inmono]))
    allvars = {v for a, _ in eqs for v in a}
    occ = Counter(v for a, _ in eqs for v in a)
    best = None
    for key in (lambda v: -v, lambda v: v, lambda v: (occ[v], -v), lambda v: (-occ[v], -v)):
        heads = {}
        for i, (allv, lin) in enumerate(eqs):
            for v in sorted(lin, key=key):
                if v not in heads: heads[v] = i; break
        known = set(allvars) - set(heads)
        changed = True
        while changed:
            changed = False
            for v, i in heads.items():
                if v not in known and (eqs[i][0] - {v}) <= known:
                    known.add(v); changed = True
        undet = set(heads) - known
        k = len(allvars) - len(heads) + len(undet)
        if best is None or k < best[0]: best = (k, len(allvars), len(heads) - len(undet))
    return best

# ---------------------------------------------------------------- reporting
def d1(s):
    lam = math.log2(s)
    u0 = lam / 2**17
    thr = lam ** 0.8
    return dict(lambda_=lam, u0=u0, thr=thr, admitted=u0 >= thr, margin_bits=0.2 * math.log2(lam) - 17)

def census(path, fmt="auto", mode="split", share=True, b=None, m=None, label=None, iface_first=None):
    if fmt == "auto":
        if path.endswith(".json"): fmt = "crate"
        else:
            head = _open(path).read(4000)
            fmt = "labanf" if re.search(r"^p anf", head, re.M) else "dimacs"
    M = {"dimacs": parse_dimacs, "labanf": parse_labanf, "crate": parse_crate}[fmt](path)
    if iface_first: M.iface = list(range(1, iface_first + 1))
    A = build(M, mode, share)
    R = finish(A, M)
    R["label"] = label or path.split("/")[-1]
    R["mode"] = mode
    R["format"] = fmt
    R["b"] = b or M.hints.get("b") if hasattr(M, "hints") else b
    R["m"] = m or (M.hints.get("m") if hasattr(M, "hints") else None)
    # interface
    if M.kind == "crate":
        R["k"] = len(M.iface_crate); R["k_src"] = "crate Network::interface()"
    elif M.iface:
        R["k"] = len(M.iface); R["k_src"] = "declared (c iface / --iface-first)"
    elif M.kind in ("labanf",) or (M.kind == "dimacs" and M.has_nl_anf):
        k, nv, nh = iface_anf(M); R["k"] = k; R["k_src"] = f"greedy heads ({nh}/{nv} determined)"
    else:
        R["k"] = None; R["k_src"] = "n/a"
    s = R["s"]
    R["s_over_sqrtlog"] = s / math.sqrt(math.log2(s))
    R["d1"] = d1(s)
    if R.get("b") and R.get("m"):
        R["enum_exp"] = (R["m"] - 1) * R["b"] / R["m"]
        R["ratio_vs_enum"] = R["s_over_sqrtlog"] / R["enum_exp"]
    return R

def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("path"); ap.add_argument("--format", default="auto", choices=["auto", "dimacs", "labanf", "crate"])
    ap.add_argument("--mode", default="split", choices=["split", "bb"])
    ap.add_argument("--no-share", action="store_true"); ap.add_argument("-b", type=int); ap.add_argument("-m", type=int)
    ap.add_argument("--json", action="store_true"); ap.add_argument("--iface-first", type=int, help="declare DIMACS vars 1..N as the interface")
    a = ap.parse_args()
    R = census(a.path, a.format, a.mode, not a.no_share, a.b, a.m, iface_first=a.iface_first)
    print(json.dumps(R, indent=1, default=str))
if __name__ == "__main__": main()
