import json, sys, time, numpy as np
sys.path.insert(0, '..'); sys.path.insert(0, '.')
import cnum as C
t0=time.time()
seed=int(sys.argv[1])
es,T,asym=C.period_matrix_seed(seed)
print('tau time',time.time()-t0,asym)
json.dump({'seed':seed,'es':es,'T_re':T.real.tolist(),'T_im':T.imag.tolist()},open(f'dev/tau_{seed}.json','w'))
