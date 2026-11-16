"""(a) edge/degree census of the char-2 Schonhage gadget (one split, XOR layers + 2*N2 black-box pointwise products)
against Bochum Theorem 3: edges < 1100(d+1), degree < 136 (d+1)^{3/4}, taking d+1 = n (bit width).
Edge model: every nonempty wire between layers counted once (inputs 2n, mid/out layers of both forward transforms,
pointwise outputs, inverse mid/out, recombined product bits, reduced output n).  Vertex = one row/column of a
four-step stage, one pointwise product (black box), one N1-bit block of recombination, one N1-bit block of reduction.
Degree = |distinct input wires| + number of output wires of the vertex."""
import sys, json
sys.path.insert(0, '.')
import gadget as G
from pdp_anf import Field

def mod_for(n):
    # any modulus works for counting; use a trinomial/pentanomial-like shape x^n + x^k + 1 (weight 3, worst case for reduction width k)
    return (1 << n) | (1 << max(1, n // 2)) | 1

def census(n):
    N, nu = G.ceil_pow3(n)
    rows = G.reduction_rows(n, mod_for(n))
    C = G.Circ(2 * n)
    A = [frozenset([i + 1]) for i in range(n)]
    B = [frozenset([n + i + 1]) for i in range(n)]
    G.mul_circuit(C, A, B, n, N, rows)
    edges = 2 * n + sum(v for k, v in C.stat.items() if not k.endswith('_and'))
    deg = {}
    for tag, ins, outs in C.vertices:
        key = tag
        deg[key] = max(deg.get(key, 0), len(ins) + outs)
    N1, N2 = G.split(nu)
    return dict(n=n, N=N, N1=N1, N2=N2, L=3 * N2, P_Q=list(G.factor_L(3 * N2)), edges=edges,
                edges_over_n=edges / n, thm3_edges=1100 * n, edge_ratio_to_thm3=edges / (1100 * n),
                max_degree=max(deg.values()), thm3_degree=136 * n ** 0.75, deg_ratio=max(deg.values()) / (136 * n ** 0.75),
                and_gates=C.stat.get('m_and', 0), wires_by_layer={k: v for k, v in sorted(C.stat.items())},
                max_degree_by_vertex={k: v for k, v in sorted(deg.items())},
                schoolbook_ands=n * n)

if __name__ == '__main__':
    ns = [13, 15, 17, 19, 23, 27, 28, 31, 40, 54, 64, 81, 82, 100, 121, 131]
    out = []
    for n in ns:
        r = census(n); out.append(r)
        print(f"n={n:4d} N={r['N']:4d} N1xN2={r['N1']}x{r['N2']} LxPQ={r['L']}={r['P_Q']} edges={r['edges']:6d} ({r['edges_over_n']:.1f}/bit; {r['edge_ratio_to_thm3']:.3f} of 1100n) maxdeg={r['max_degree']:5d} ({r['deg_ratio']:.3f} of 136n^.75) pointwise_ANDs={r['and_gates']} (schoolbook {r['schoolbook_ands']})", flush=True)
    json.dump(out, open('../raw/census.json', 'w'), indent=1)
