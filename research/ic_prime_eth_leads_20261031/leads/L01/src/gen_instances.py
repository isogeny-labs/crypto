"""Generate standard-ANF, standard-CNF(xor) and gadget-CNF(xor) files for (n,l) cells.
usage: gen_instances.py OUTDIR K   -> K uniform + K planted targets per cell"""
import sys, random, json, hashlib
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent))
from pdp_anf import Field, system, to_anf, sample_target, linking_equations, layout, _form, _mul, _scale, _sum
from anf2cnf import convert
import gadget as G

CELLS = {(11, 3): (1 << 11) | (1 << 2) | 1, (13, 4): (1 << 13) | (1 << 4) | (1 << 3) | 3, (15, 5): (1 << 15) | 3, (17, 6): (1 << 17) | (1 << 3) | 1, (19, 7): (1 << 19) | (1 << 5) | (1 << 2) | (1 << 1) | 1}

def lin_elem(F, vars_, t):
    """(sum_j c_j z^j)^(2^t): n-bit list of exprs."""
    out = [frozenset() for _ in range(F.n)]
    for j, v in enumerate(vars_):
        img = F.frob(1 << j, t)
        for i in range(F.n):
            if (img >> i) & 1:
                out[i] = out[i] ^ frozenset([v])
    return out

def scale(F, el, c):
    out = [frozenset() for _ in range(F.n)]
    for j in range(F.n):
        if not el[j]:
            continue
        img = F.mul(1 << j, c)
        for i in range(F.n):
            if (img >> i) & 1:
                out[i] = out[i] ^ el[j]
    return out

def add(*els):
    out = [frozenset() for _ in els[0]]
    for el in els:
        out = [a ^ b for a, b in zip(out, el)]
    return out

def gadget_s4(F, l, xR, C, N):
    """returns (list of (expr,const)) n XOR constraints: XOR(expr) = const ; products by Schonhage gadget."""
    n = F.n
    L = layout(l)
    e1, e2, e3 = L['e']
    rows = G.reduction_rows(n, F.mod)
    r = xR; r2 = F.mul(r, r); r3 = F.mul(r2, r); r4 = F.mul(r2, r2)
    E3 = lin_elem(F, e3, 0); a = lin_elem(F, e1, 1); b = lin_elem(F, e2, 1); c = lin_elem(F, e3, 1)
    a2 = lin_elem(F, e1, 2); c2 = lin_elem(F, e3, 2); b2 = lin_elem(F, e2, 2)
    def prod(X, Y, tag):
        # materialise operands as single-var wires first (they are linear forms of e-bits)
        Xm = C.mat_word(X, tag + '_opx'); Ym = C.mat_word(Y, tag + '_opy')
        return G.mul_circuit(C, Xm, Ym, n, N, rows, tag)
    cE = prod(c, E3, 'p1'); Eb = prod(E3, b, 'p2'); Ea = prod(E3, a, 'p3'); ac = prod(a, c, 'p4')
    tot = add(a2, c2, scale(F, b2, r4), scale(F, cE, r), scale(F, Eb, r3), scale(F, Ea, r),
              scale(F, E3, r3), scale(F, ac, r2), scale(F, c, r4), c, scale(F, b, r2))
    return [(tot[i], (r4 >> i) & 1) for i in range(n)]   # XOR(tot_i) + r4_i = 0  => XOR = r4_i

def gadget_cnf(n, mod, l, xR, sym=True):
    F = Field(n, mod)
    nv = layout(l)['nvars']
    text, _, total = convert(to_anf(nv, linking_equations(l)), l, 'xor', sym)   # linking + sym, same as std arm
    lines = text.splitlines()[1:]
    C = G.Circ(total)
    N, nu = G.ceil_pow3(n)
    cons = gadget_s4(F, l, xR, C, N)
    extra = []
    for g in C.gates:
        if g[0] == 'x':
            vs = g[2] + [g[1]]
            extra.append('x' + ' '.join(map(str, vs)) + ' 0'.replace('x', ''))  # XOR(vs)=0 -> 'x-' form below
        else:
            o, u, v = g[1:]
            extra += [f'-{o} {u} 0', f'-{o} {v} 0', f'{o} -{u} -{v} 0']
    # fix xor lines: XOR(vs)=0 needs odd number of negations -> use 'x-' prefix
    extra2 = []
    for g in C.gates:
        if g[0] == 'x':
            extra2.append('x-' + ' '.join(map(str, g[2] + [g[1]])) + ' 0')
        else:
            o, u, v = g[1:]
            extra2 += [f'-{o} {u} 0', f'-{o} {v} 0', f'{o} -{u} -{v} 0']
    for e_, c in cons:
        vs = sorted(e_)
        if not vs:
            if c:
                extra2.append('0')   # empty clause
            continue
        extra2.append('x' + ('' if c else '-') + ' '.join(map(str, vs)) + ' 0')
    allv = C.nxt - 1
    out = [f'p cnf {allv} {len(lines) + len(extra2)}'] + lines + extra2
    return '\n'.join(out) + '\n', allv, dict(C.stat)

def main():
    out = Path(sys.argv[1]); K = int(sys.argv[2]); out.mkdir(parents=True, exist_ok=True)
    cells = [tuple(map(int, c.split(','))) for c in sys.argv[3].split(';')] if len(sys.argv) > 3 else list(CELLS)
    man = []
    for (n, l) in cells:
        mod = CELLS[(n, l)]; F = Field(n, mod)
        for mode in ('uniform', 'planted'):
            for seed in range(1, K + 1):
                rng = random.Random(f'L01-{n}-{l}-{mode}-{seed}')
                t = sample_target(F, l, rng, mode)
                name = f'n{n}l{l}-{mode[:1]}{seed}'
                nv, eqs = system(n, mod, l, t['xR'])
                (out / f'{name}.anf').write_text(to_anf(nv, eqs))
                txt, _, tot = convert(to_anf(nv, eqs), l, 'xor', True)
                (out / f'{name}.std.cnf').write_text(txt)
                gtxt, gv, stat = gadget_cnf(n, mod, l, t['xR'])
                (out / f'{name}.gad.cnf').write_text(gtxt)
                man.append(dict(name=name, n=n, l=l, mode=mode, seed=seed, xR=t['xR'], mod=mod, anf_vars=nv, std_cnf_vars=tot, gad_cnf_vars=gv, gad_stat=stat))
                print(name, nv, tot, gv, flush=True)
    (out / 'manifest.json').write_text(json.dumps(man, indent=1))
main()
