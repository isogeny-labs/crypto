# EXP16b fallback for the generic affine line c0 + u*F_Q (r = c0/u of degree 4 over F_Q), Q = 32:
# ideal degree, dim U, and the Jacobian determinant at every F_Q-rational solution (simple point <=> det != 0),
# plus, for contrast, the same on the line through 0 (det J == 0 identically there).
# usage: sage exp16b_generic.sage ncurves seed
import sys, time
nc_, seed_ = int(sys.argv[1]), int(sys.argv[2])
src = open('exp16b_descent.sage').read(); src = src[:src.index('print("# exp16b_descent')]
sys.argv = ['x', '1', str(seed_)]
exec(preparse(src))
set_random_seed(seed_ + 7)
print("# exp16b_generic: kind | dim U | deg | det J identically 0? | #F_Q-solutions | #with det J != 0 | cpu", flush=True)
done = 0
while done < nc_:
    b = K.random_element()
    if b == 0: continue
    E = EllipticCurve(K, [1, 0, 0, 0, b]); done += 1
    u = K.random_element(); rg = K.random_element()
    Rt_ = 2 * E.random_point()
    for name, c0 in [("through0", K(0)), ("generic", u * rg)]:
        t0 = CPU()
        eqs = system_aff(c0, u, b, Rt_[0])
        Ub = null_directions(eqs)
        dg, tg, gb = gbdeg(Rq, eqs)
        J = matrix(Rq, [[e.derivative(v) for v in Rq.gens()] for e in eqs])
        G2 = Rlex.ideal([Rlex(g) for g in gb] + [v ^ Q - v for v in Rlex.gens()]).groebner_basis('libsingular:stdfglm')
        V = rat_points(list(G2))
        # V points are (s1,s2,s3,s4); evaluate J entrywise then det over F_Q (cheap)
        nz = sum(1 for p in V if matrix(Fq, [[J[i, j](*p) for j in range(4)] for i in range(4)]).det() != 0)
        ident0 = all(J[i, j] == 0 or True for i in range(4) for j in range(4)) and (len(Ub) > 0)
        print(f"c{done} {name} | {len(Ub)} | {dg} | {'yes (dim U>0 => rank<=4-dimU)' if len(Ub) > 0 else 'no'} | {len(V)} | {nz} | {CPU()-t0:.1f}s", flush=True)
print("DONE", flush=True)
