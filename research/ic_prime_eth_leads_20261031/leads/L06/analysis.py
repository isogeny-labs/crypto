"""Contraction-width analysis of the (modelled) hashed-ECDL relation network.
Variables = bit edges (each edge has exactly two endpoint vertices; shared signals are
expanded to degree-3 equality trees exactly as Encoder::finish does).
Width of an elimination step = log2 of the contracted table = |S_f u S_g| (compute exponent)."""
import heapq, random, sys, time, json
import model

def build_graph(e, pinned_signals=frozenset()):
    """returns scopes: list of sets of edge ids; edge_sig: edge->signal"""
    users = {}   # signal -> list of vertex idx
    nv = len(e.verts)
    scopes_ports = []
    for vi, (lab, ports) in enumerate(e.verts):
        for s in ports: users.setdefault(s, []).append(vi)
    # interface endpoints & padding -> dummy vertices (arity-1 leaves)
    extra = 0
    ifs = set(e.iface)
    scopes = [set() for _ in range(nv)]
    edge_sig = []
    def new_v():
        scopes.append(set()); return len(scopes) - 1
    def conn(a, b, sig):
        eid = len(edge_sig); edge_sig.append(sig)
        scopes[a].add(eid); scopes[b].add(eid)
    for s in range(len(e.uses)):
        u = list(users.get(s, []))
        if s in ifs:
            # interface endpoint count(s) -- one per occurrence in iface
            for _ in range(e.iface.count(s) if False else 1): u.append(new_v())
        while len(u) < 2: u.append(new_v())
        if s in pinned_signals:
            continue   # signal fixed: its edges carry a constant, drop the variables
        if len(u) == 2 and u[0] != u[1]:
            conn(u[0], u[1], s)
        elif len(u) == 2:
            c = new_v(); conn(u[0], c, s); conn(u[1], c, s)
        else:
            c = new_v(); conn(u[0], c, s); conn(u[1], c, s)
            for i in range(2, len(u)):
                if i == len(u) - 1: conn(u[i], c, s)
                else:
                    nx = new_v(); conn(c, nx, s); conn(u[i], nx, s); c = nx
    return scopes, edge_sig

def eliminate(scopes, rule="deg", seed=0, cap=None, tlimit=600):
    rnd = random.Random(seed)
    S = [set(x) for x in scopes]
    ends = {}
    for f, sc in enumerate(S):
        for e_ in sc:
            ends.setdefault(e_, []).append(f)
    ends = {k: v for k, v in ends.items() if len(v) == 2}
    # edges with a single endpoint (pinned other side): treat as free index of its factor (cost counted at merge)
    def score(e_):
        f, g = ends[e_]
        a, b = S[f], S[g]
        if len(a) > len(b): a, b = b, a
        inter = sum(1 for x in a if x in b)
        deg = len(a) + len(b) - inter
        if rule == "deg": return (deg, rnd.random())
        # fill: pairs (x in A\B, y in B\A) not sharing a third factor
        A = [x for x in S[f] if x not in S[g]]; B = [x for x in S[g] if x not in S[f]]
        ca = {}
        for x in A:
            o = ends.get(x)
            if o: 
                t = o[0] if o[1] in (f, g) else o[1]
                ca[t] = ca.get(t, 0) + 1
        share = 0
        for y in B:
            o = ends.get(y)
            if o:
                t = o[0] if o[1] in (f, g) else o[1]
                share += ca.get(t, 0)
        return (len(A) * len(B) - share, deg, rnd.random())
    heap = []
    ver = {}
    for e_ in ends:
        ver[e_] = 0
        heapq.heappush(heap, (score(e_), e_, 0))
    width = 0; steps = 0; t0 = time.time(); profile = []
    # factors with no variables left but nonempty scope of single-ended edges also bound width
    while heap:
        sc, e_, v = heapq.heappop(heap)
        if e_ not in ends or ver[e_] != v: continue
        f, g = ends[e_]
        if f == g:
            del ends[e_]; continue
        cost = len(S[f] | S[g])
        width = max(width, cost); steps += 1
        if steps % 50000 == 0 and time.time() - t0 > tlimit: return dict(width=width, partial=True, steps=steps)
        shared = S[f] & S[g]
        new = (S[f] | S[g]) - shared
        for x in shared: ends.pop(x, None)
        for x in S[g]:
            if x in ends:
                o = ends[x]
                ends[x] = [f if y == g else y for y in o]
        S[f] = new; S[g] = set()
        for x in new:
            if x in ends:
                ver[x] += 1
                heapq.heappush(heap, (score(x), x, ver[x]))
    # leftovers: factors still holding single-ended/free edges
    width = max([width] + [len(s) for s in S])
    return dict(width=width, partial=False, steps=steps)

def primal(scopes):
    adj = {}
    for sc in scopes:
        l = list(sc)
        for x in l:
            adj.setdefault(x, set()).update(l)
    for x in adj: adj[x].discard(x)
    return adj

def degeneracy(adj):
    deg = {v: len(n) for v, n in adj.items()}
    heap = [(d, v) for v, d in deg.items()]; heapq.heapify(heap)
    gone = set(); best = 0
    while heap:
        d, v = heapq.heappop(heap)
        if v in gone or d != deg[v]: continue
        best = max(best, d); gone.add(v)
        for u in adj[v]:
            if u not in gone:
                deg[u] -= 1; heapq.heappush(heap, (deg[u], u))
    return best

def mmd_plus(adj, seed=0):
    """minor-min-width, least-c contraction heuristic (Bodlaender-Koster)."""
    A = {v: set(n) for v, n in adj.items()}
    heap = [(len(n), v) for v, n in A.items()]; heapq.heapify(heap)
    best = 0
    while A:
        while True:
            d, v = heapq.heappop(heap)
            if v in A and len(A[v]) == d: break
        best = max(best, d)
        if d == 0: del A[v]; continue
        u = min(A[v], key=lambda w: len(A[v] & A[w]))
        # contract v into u
        for w in A[v]:
            if w != u:
                A[w].discard(v); A[w].add(u); A[u].add(w)
        A[u].discard(v); del A[v]
        for w in list(A[u]) + [u]:
            heapq.heappush(heap, (len(A[w]), w))
    return best

if __name__ == "__main__":
    mul, b, m = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    rules = sys.argv[4].split(",") if len(sys.argv) > 4 else ["deg"]
    t0 = time.time()
    e = model.build(b, m, mul)
    sc, es = build_graph(e)
    out = dict(mul=mul, b=b, m=m, enum_exp=(m - 1) * b / m, vertices=len(sc), edges=len(es),
               max_arity=max(len(s) for s in sc))
    for r in rules:
        if r in ("deg", "fill"):
            out[r] = eliminate(sc, r)
        elif r == "lb":
            adj = primal(sc); out["degeneracy"] = degeneracy(adj); out["mmd_plus"] = mmd_plus(adj)
    out["secs"] = round(time.time() - t0, 1)
    print(json.dumps(out), flush=True)
