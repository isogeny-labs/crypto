# Search a Curve25519-like Montgomery curve: p = 5 mod 8, #E = 8q, #twist = 4q'
set_random_seed(25519)
p = next_prime(2^20)
while p % 8 != 5: p = next_prime(p+1)
F = GF(p)
for A in range(486662 % p, p, 4):
    if (A*A-4) % p == 0: continue
    E = EllipticCurve(F, [0, A, 0, 1, 0])
    N = E.order(); Nt = 2*(p+1) - N
    if N % 8 == 0 and is_prime(N//8) and Nt % 4 == 0 and is_prime(Nt//4):
        t = p+1-N
        print("p",p,"A",A,"N",N,"q",N//8,"twist",Nt,"t",t,"disc",(t^2-4*p).factor())
        print("rational ell-isogenies for ell<=200:", [l for l in primes(3,200) if kronecker(t^2-4*p, l) >= 0])
        break
