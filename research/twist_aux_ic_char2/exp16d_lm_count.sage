# EXP16d-6: read an msolve '-g 1' leading-ideal file (grevlex) and count the staircase (quotient dimension = ideal
# degree with multiplicity) of the monomial ideal in Singular. usage: sage exp16d_lm_count.sage file.lm vars(comma-sep)
import sys, re, time
txt = open(sys.argv[1]).read(); vs = sys.argv[2]
R = PolynomialRing(GF(2), vs, order='degrevlex')
body = txt[txt.index('[')+1: txt.rindex(']')]
mons = [R(m.strip()) for m in body.split(',') if m.strip()]
c0 = time.process_time()
I = R.ideal(mons)
print(f"{len(mons)} leading monomials; max deg {max(m.degree() for m in mons)}; dim {I.dimension()}", flush=True)
print(f"quotient dimension (ideal degree) {I.vector_space_dimension()}  ({time.process_time()-c0:.1f}s)", flush=True)
