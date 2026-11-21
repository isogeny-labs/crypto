"""rho(n') = F_2-dimension of span{ e_p e_q e_r } inside the polynomial ring F_2[nu_1..nu_n'],
e_{ab} = nu_a nu_b^2 + nu_a^2 nu_b  (the 2x2 Moore determinant).
For every field GF(2^n) and every nu in GF(2^n)^{n'}, dim Vbar <= rho(n')  (evaluation is a linear map out of this span)."""
import itertools, json, sys
from math import comb
def pmul(p, q):
    r = set()
    for a in p:
        for b in q:
            k = tuple(i + j for i, j in zip(a, b))
            r ^= {k}
    return r
def rank_sets(polys):
    idx = {}
    rows = []
    for p in polys:
        v = 0
        for mon in p:
            if mon not in idx: idx[mon] = len(idx)
            v |= 1 << idx[mon]
        rows.append(v)
    piv = {}; r = 0
    for v in rows:
        while v:
            b = v.bit_length() - 1
            if b in piv: v ^= piv[b]
            else: piv[b] = v; r += 1; break
    return r
def rho(npr):
    es = []
    for a in range(npr):
        for b in range(a + 1, npr):
            ea = [0] * npr; ea[a] = 1; ea[b] = 2
            eb = [0] * npr; eb[a] = 2; eb[b] = 1
            es.append({tuple(ea), tuple(eb)})
    # E = span of e's: rank
    rE = rank_sets(es)
    prods = [pmul(pmul(es[i], es[j]), es[k]) for i, j, k in itertools.combinations_with_replacement(range(len(es)), 3)]
    return len(es), rE, len(prods), rank_sets(prods)
if __name__ == "__main__":
    out = []
    for npr in range(3, 8):
        ne, rE, npr3, r3 = rho(npr)
        row = dict(nprime=npr, num_e=ne, dim_E_generic=rE, num_cubic_products=npr3, rho=r3, N=3 * npr)
        print(row); out.append(row)
    json.dump(out, open("rho_generic.json", "w"), indent=1)
