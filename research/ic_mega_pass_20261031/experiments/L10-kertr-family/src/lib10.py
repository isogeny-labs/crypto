"""L10 generator: treatise families (gen2.py conventions) built with N1 conventions (tensor_closure helpers, to_eqs bit order).
T: V random k-dim subspace of ker Tr, z = curve x-coordinate, draws with Tr(a6/z^2)=1 rejected (gen2.py rule).
R: same but V random (not in ker Tr) -- treatise R family.
A: N1 lib.system arm A (random V, uniform z), unchanged."""
import sys, os, random, json, subprocess
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
EXP = os.path.join(HERE, '..', '..')
sys.path.insert(0, os.path.join(EXP, 'MP-comm_alg-1'))
from tensor_closure import modulus, pmulmod, rand_subspace, f2rank, semaev_parts, to_eqs
BIN = os.path.join(HERE, 'gf2close'); BIN5 = os.path.join(HERE, 'gf2close5')

def fmul(a, b, m, n): return pmulmod(a, b, m, n)
def fpow(a, e, m, n):
    r = 1
    while e:
        if e & 1: r = fmul(r, a, m, n)
        a = fmul(a, a, m, n); e >>= 1
    return r
def finv(a, m, n): return fpow(a, (1 << n) - 2, m, n)
def tr(a, m, n):
    t, x = 0, a
    for _ in range(n): t ^= x; x = fmul(x, x, m, n)
    assert t in (0, 1); return t

def draw(fam, n, s):
    """returns dict(eqs, meta). fam in T,R. Own seed base 10_000_000."""
    k = (n + 1) // 2; m = modulus(n)
    rng = random.Random(10_000_000 + 1000 * n + 100 * k + s + (0 if fam == 'T' else 500_000))
    rejects = 0
    while True:
        a6 = rng.randrange(1, 1 << n); a2 = rng.randrange(2)
        if fam == 'T':
            while True:
                B = []
                while len(B) < k:
                    x = rng.randrange(1, 1 << n)
                    if tr(x, m, n) == 0: B.append(x)
                if f2rank(B) == k: break
        else:
            B = rand_subspace(n, k, rng)
        while True:
            z = rng.randrange(1, 1 << n)
            if tr(z ^ a2 ^ fmul(a6, finv(fmul(z, z, m, n), m, n), m, n), m, n) == 0: break
        if all(tr(v, m, n) == 0 for v in B) and tr(fmul(a6, finv(fmul(z, z, m, n), m, n), m, n), m, n) == 1:
            rejects += 1; continue
        break
    return build(n, m, B, z, a6), dict(fam=fam, n=n, k=k, s=s, modulus=hex(m), a2=a2, a6=hex(a6), z=hex(z),
                                       V=[hex(v) for v in B], rejects=rejects)

def build(n, m, B, z, a6):
    k = len(B); bil, affa, affb, c = semaev_parts(n, m, B, z, a6)
    return to_eqs(n, k, bil, affa, affb, c)

def enc(mode, N, d, eqs):
    return "%s %d %d\n%d\n" % (mode, N, d, len(eqs)) + "".join("%d %s\n" % (len(e), " ".join(map(str, sorted(e)))) for e in eqs)
def call(mode, N, d, eqs, binary=None, env=None, chunk=1024):
    r = subprocess.run([binary or BIN, str(chunk)], input=enc(mode, N, d, eqs), capture_output=True, text=True,
                       env=dict(os.environ, **(env or {})))
    if r.returncode: raise RuntimeError(r.stderr)
    return json.loads(r.stdout.strip().splitlines()[-1])
def span_rank(eqs, N):
    """F_2 rank of span F and dim of its degree<=1 part (as polynomials)."""
    mons = sorted({mm for e in eqs for mm in e}, key=lambda x: (bin(x).count('1'), x), reverse=True)
    idx = {mm: i for i, mm in enumerate(mons)}  # high index bit = high degree? use position: lower i = higher degree
    L = len(mons)
    vecs = [sum(1 << (L - 1 - idx[mm]) for mm in e) for e in eqs]
    piv = {}; r = 0
    for v in vecs:
        while v:
            p = v.bit_length() - 1
            if p in piv: v ^= piv[p]
            else: piv[p] = v; r += 1; break
    # reduced echelon with degree-desc ordering: rows whose leading monomial has degree<=1 span the low part
    low = sum(1 for p in piv if bin(mons[L - 1 - p]).count('1') <= 1)
    return r, low
