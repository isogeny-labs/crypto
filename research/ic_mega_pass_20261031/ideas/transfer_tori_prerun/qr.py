# Frame Hamming balls: F_{2^n} = F_2[Y]/(Y^(2n+1)-1) quotient by the QR-code ideal; check min distance and injectivity at n=11 (Golay) and n=23
import itertools
def factor_cyclo_over_f2(N, n):
    # find degree-n irreducible factor of (Y^N-1)/(Y-1) over F_2 by brute force: roots are gamma^(2^i), use Frobenius coset of 1: {2^i mod N}
    coset = sorted({pow(2,i,N) for i in range(n)})
    return coset
def code_min_distance(N, n):
    # cyclic code of length N whose check "syndrome" is evaluation at gamma: kernel of F_2^N -> F_{2^n}, e -> sum_{j in supp} gamma^j.
    # Build field F_2[Y]/(Q1(Y)) via a generator polynomial: compute Q1 = prod_{i in coset}(Y - gamma^i) by working in F_2[Y]/(Y^N-1) ... easier: use the known relation: syndrome map matrix via companion of Q1.
    # Derive Q1 by polynomial gcd: Q1 = gcd(Y^N-1, (Y^(2^n) - Y)) restricted... simpler: factor Y^N-1 over F_2 by Berlekamp via sympy
    from sympy import Poly, symbols, GF, factor_list
    Y = symbols('Y')
    fl = factor_list(Y**N - 1, modulus=2)[1]
    Q1 = [f for f,_ in fl if Poly(f,Y).degree()==n][0]
    q1 = Poly(Q1, Y, modulus=2).all_coeffs()[::-1]  # low to high
    # syndrome: reduce Y^j mod Q1 -> vector length n
    def red(j):
        v=[0]*N; v[j]=1
        # polynomial division mod 2
        p=v[:]
        for d in range(N-1, n-1, -1):
            if p[d]:
                for k in range(n+1):
                    p[d-n+k] ^= q1[k]
        return tuple(p[:n])
    H=[red(j) for j in range(N)]
    # min weight nonzero codeword: smallest w with two distinct supports of total size w having equal syndrome; check injectivity of balls
    from collections import defaultdict
    seen={}
    inj_w=0
    for w in range(0, WMAX+1):
        coll=0
        for S in itertools.combinations(range(N), w):
            s=0
            for j in S: s ^= int(''.join(map(str,H[j])),2)
            if s in seen: coll+=1
            else: seen[s]=S
        if coll==0: inj_w=w
        print(f"N={N} n={n}: weight<= {w}: distinct syndromes so far {len(seen)} of 2^{n}={2**n}, collisions at this weight {coll}")
        if coll: break
    return inj_w
WMAX=3; code_min_distance(23, 11)
WMAX=4; code_min_distance(47, 23)
