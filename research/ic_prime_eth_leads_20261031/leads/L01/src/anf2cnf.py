"""ANF (csd.pdp_anf / WDSat format) -> CNF(-XOR) for CDCL solvers (PROTOCOL amendment 14).

Variables 1..nvars keep their ANF numbering; every monomial of degree >= 2 gets
a fresh Tseitin AND gate variable. Each ANF equation is an XOR over variables
and gates equal to its constant. Formats:
  xor  (CryptoMiniSat): native `x` lines, long XORs kept whole.
  cnf  (kissat, cadical): XORs cut into chunks of at most 4 variables joined by
       fresh link variables, 2^(k-1) clauses each.
--sym adds x1 <= x2 <= x3 (block b = variables 1+b*l .. (b+1)*l, bit l-1 most
significant) via "equal so far" variables; every unordered multiset has a
sorted representative, so SAT/UNSAT is preserved.
"""
import argparse
import itertools
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from pdp_anf import parse_anf  # noqa: E402


def lex_leq(a, b, nxt):
    """clauses for A <= B, a/b: lists of vars LSB first; returns (clauses, next free var)."""
    cl = []
    prev = None  # e_{i+1}; None means true
    for i in range(len(a) - 1, -1, -1):
        pre = [] if prev is None else [-prev]
        cl.append(pre + [-a[i], b[i]])
        if i:
            e = nxt
            nxt += 1
            cl.append([e] + pre + [a[i], b[i]])
            cl.append([e] + pre + [-a[i], -b[i]])
            prev = e
    return cl, nxt


def xor_clauses(vs, c):
    """all clauses forbidding assignments of vs whose parity != c."""
    cl = []
    for signs in itertools.product((0, 1), repeat=len(vs)):
        if sum(signs) % 2 != c:  # this assignment is forbidden
            cl.append([-v if s else v for v, s in zip(vs, signs)])
    return cl


def convert(text, l, fmt, sym):
    nv, eqs = parse_anf(text)
    gates = {}
    clauses, xors = [], []
    nxt = nv + 1
    for eq in eqs:
        const = 1 if frozenset() in eq else 0
        vs = []
        for m in sorted((m for m in eq if m), key=lambda m: (len(m), sorted(m))):
            if len(m) == 1:
                vs.append(next(iter(m)))
            else:
                if m not in gates:
                    g = nxt
                    nxt += 1
                    gates[m] = g
                    for v in sorted(m):
                        clauses.append([-g, v])
                    clauses.append([g] + [-v for v in sorted(m)])
                vs.append(gates[m])
        if not vs:
            if const:
                clauses.append([])  # 0 = 1 ... equation sum = 0 with constant 1 is false
            continue
        if fmt == 'xor':
            if len(vs) == 1:
                clauses.append([vs[0] if const else -vs[0]])
            else:
                xors.append((vs, const))
        else:
            vs = list(vs)
            cur_c = const
            while len(vs) > 4:
                t = nxt
                nxt += 1
                chunk = vs[:3] + [t]
                clauses += xor_clauses(chunk, 0)  # t = v1^v2^v3  <=> v1^v2^v3^t = 0
                vs = [t] + vs[3:]
            clauses += xor_clauses(vs, cur_c)
    # equation: XOR(monomials) + const = 0  =>  XOR(vs) = const; unit handled above
    if sym:
        for b in range(2):
            a = [1 + b * l + p for p in range(l)]
            c = [1 + (b + 1) * l + p for p in range(l)]
            cl, nxt = lex_leq(a, c, nxt)
            clauses += cl
    total = nxt - 1
    lines = [f'p cnf {total} {len(clauses) + len(xors)}']
    for cl in clauses:
        lines.append(' '.join(map(str, cl)) + ' 0')
    for vs, c in xors:
        lines.append('x' + ('' if c else '-') + ' '.join(map(str, vs)) + ' 0')
    return '\n'.join(lines) + '\n', nv, total


if __name__ == '__main__':
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('anf', type=Path)
    ap.add_argument('-l', type=int, required=True)
    ap.add_argument('--fmt', choices=['xor', 'cnf'], required=True)
    ap.add_argument('--sym', action='store_true')
    a = ap.parse_args()
    out, nv, total = convert(a.anf.read_text(), a.l, a.fmt, a.sym)
    sys.stdout.write(out)
