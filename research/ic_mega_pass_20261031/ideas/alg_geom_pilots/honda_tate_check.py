"""Pilot for MP-alg_geom-6: for prime-order E/F_p, which elliptic curves E'/F_p have r | #E'(F_p)?
Only #E' = r itself (same isogeny class) when r > 4 sqrt(p)+... ; so any abelian surface A ~ E x E'
with a rational r-torsion plane has E' isogenous to E, and the Weil pairing of the two rational
r-lines (for ANY principal polarisation) takes values in mu_r cap F_p^* = {1} when r !| p-1.
Also checks the embedding degree k = ord_r(p) for sample curves (k=1 would be the MOV exception)."""
import cypari2, json, random
P = cypari2.Pari()
out = []
rng = random.Random(3)
for p in [1009, 4099, 16411, 65537, 262147]:
    found = 0
    tries = 0
    while found < 3 and tries < 5000:
        tries += 1
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b**2) % p == 0: continue
        E = P.ellinit([a, b], p)
        N = int(P.ellcard(E))
        if not P.isprime(N): continue
        found += 1
        r = N
        lo, hi = p + 1 - int(2*p**0.5) - 1, p + 1 + int(2*p**0.5) + 1
        multiples = [k*r for k in range(1, 6) if lo <= k*r <= hi]
        k_emb = int(P.znorder(P.Mod(p, r)))
        out.append({"p": p, "a": a, "b": b, "r": r, "hasse_interval": [lo, hi],
                    "multiples_of_r_in_hasse": multiples, "embedding_degree": k_emb,
                    "r_divides_p_minus_1": (p - 1) % r == 0})
print(json.dumps(out, indent=1))
json.dump(out, open("RESULTS_honda_tate.json", "w"), indent=1)
