# EXP16d-4: msolve calibration at Q = 2 (d = 1): descended system then lives over F_2, which msolve accepts.
# m = 4: build the exp14 system at d = 1 (K = F_16), reduce (t_i = s_i^2), compute the ideal degree in Singular
# (wdegrevlex) and write msolve input files (reduced and original) for comparison.
# usage: sage exp16d_msolve_q2.sage seed ntargets
import sys, time
seed, NTq = int(sys.argv[1]), int(sys.argv[2])
sys.argv = ['x', '1', '1', '1', str(seed), '0', '0', '0']
src = open('exp14_char2_solve.sage').read(); src = src[:src.index('print("d Q curve')]
exec(preparse(src))
set_random_seed(seed)
Wt = TermOrder('wdegrevlex', (2,2,2,1))
Rw2 = PolynomialRing(Fq, 't1,t2,t3,s4', order=Wt)
def reduce(e, RR):
    g = RR.gens(); return sum(c*g[0]^(m.exponents()[0][0]//2)*g[1]^(m.exponents()[0][1]//2)*g[2]^(m.exponents()[0][2]//2)*g[3]^m.exponents()[0][3] for c, m in zip(e.coefficients(), e.monomials()))
import os
os.makedirs('results/exp16d_msolve', exist_ok=True)
done = 0
while done < NTq:
    b = K.random_element()
    if b == 0: continue
    E = EllipticCurve(K, [1,0,0,0,b]); u = K.random_element()
    if u == 0: continue
    Rt = 2*E.random_point()
    if Rt.is_zero(): continue
    eqs = system(u, b, Rt[0]); red = [reduce(e, Rw2) for e in eqs]
    c0 = time.process_time(); gb = Rq.ideal(eqs).groebner_basis('libsingular:std'); t1 = time.process_time()-c0; d1 = len(Rq.ideal(gb).normal_basis())
    c0 = time.process_time(); gbr = Rw2.ideal(red).groebner_basis('libsingular:std'); t2 = time.process_time()-c0; d2 = len(Rw2.ideal(gbr).normal_basis())
    fn = f'results/exp16d_msolve/m4_q2_{done}'
    with open(fn + '_red.ms', 'w') as F:
        F.write('t1,t2,t3,s4\n2\n' + ',\n'.join(str(f).replace(' ', '') for f in red) + '\n')
    with open(fn + '_orig.ms', 'w') as F:
        F.write('s1,s2,s3,s4\n2\n' + ',\n'.join(str(f).replace(' ', '') for f in eqs) + '\n')
    print(f"m=4 Q=2 target {done}: Singular orig deg {d1} ({t1:.2f}s), reduced deg {d2} (wdeg {t2:.2f}s) -> {fn}", flush=True)
    done += 1
