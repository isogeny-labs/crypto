# EXP16d-5: Singular std + FGLM cost vs ideal degree D over F_8 (char 2), on random dense zero-dim systems with
# Bezout degree D (3 variables, degrees a,b,c, D = abc). Used only to fit an exponent for extrapolating to D = 2^16
# (estimate; random systems are not the summation systems). Also times std on the m = 3 reduced-like scale.
import sys, time
set_random_seed(1607)
F = GF(2^3, 'z')
for degs in [(4,4,4), (8,4,4), (8,8,4), (8,8,8), (16,8,8), (16,16,8)]:
    R = PolynomialRing(F, 'x,y,w', order='degrevlex'); Rl = PolynomialRing(F, 'x,y,w', order='lex')
    eqs = [R.random_element(degree=dd, terms=Infinity) for dd in degs]
    c0 = time.process_time(); gb = R.ideal(eqs).groebner_basis('libsingular:std'); tg = time.process_time()-c0
    D = len(R.ideal(gb).normal_basis())
    c0 = time.process_time(); lx = Rl.ideal([Rl(f) for f in eqs]).groebner_basis('libsingular:stdfglm'); tf = time.process_time()-c0
    print(f"degs {degs} D {D}: std cpu {tg:.2f}s ; stdfglm cpu {tf:.2f}s (fglm ~ {tf-tg:.2f}s)", flush=True)
