import json, sys, numpy as np
sys.path.insert(0, '.')
import cnum as C
seed=int(sys.argv[1]); Sb=[int(c) for c in sys.argv[2]]
d=json.load(open(f'dev/tau_{seed}.json')); T=np.array(d['T_re'])+1j*np.array(d['T_im'])
S=np.zeros((3,3)); k=0
for i in range(3):
    for j in range(i,3):
        S[i,j]=S[j,i]=Sb[k]; k+=1
TB=(T+S)/2
val,grad=C.theta_all(TB)
mx=max(abs(val[c]) for c in C.EVEN)
print('min |even|/max', min(abs(val[c]) for c in C.EVEN)/mx, 'max |odd val|', max(abs(val[c]) for c in C.ODD)/mx)
ar=C.find_aronhold()
print('aronhold',ar)
L7=[grad[m] for m in ar]
H,A=C.weber_frame(L7)
Xi,kk,res=C.riemann_xi(A); print('xi residual',res)
F=C.quartic_eval_coeffs(Xi)
Hi=np.linalg.inv(H)
defs=[]
for m in C.ODD:
    l=grad[m]@Hi
    defs.append(C.bitangent_defect(F,l))
print('28 bitangent defects max',max(defs))
rnd=np.random.default_rng(0).normal(size=(5,3))
print('random line defects', [round(C.bitangent_defect(F,l),3) for l in rnd])
