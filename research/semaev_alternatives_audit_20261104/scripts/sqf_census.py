import time, collections
import sympy as sp

def kernel(n):
    f = sp.factorint(n)
    k = 1
    for q, e in f.items():
        if e % 2:
            k *= q
    return k

def census(B):
    t0 = time.time()
    seen = collections.defaultdict(list)
    for x in range(1, B+1):
        seen[kernel(x*x*x + 7)].append(x)
    pairs = [(d, v) for d, v in seen.items() if len(v) > 1]
    npairs = sum(len(v)*(len(v)-1)//2 for _, v in pairs)
    return npairs, pairs[:10], time.time()-t0

for B in (256, 1024, 4096, 16384):
    n, ex, dt = census(B)
    print(f"B={B}: collision pairs={n} (B^0.5={B**0.5:.1f}), {dt:.1f}s, examples={ex[:5]}")
