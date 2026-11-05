from tensor_closure import *
import ffd_semaev_core as C
ok=True
for n in (7,9,10):
    m=C.MODS[n]
    for s in range(3):
        rng=random.Random(s+10*n); k=(n+1)//2
        V=rand_subspace(n,k,rng); z=rng.randrange(1,1<<n); a6=rng.randrange(1,1<<n)
        mine=to_eqs(n,k,*semaev_parts(n,m,V,z,a6))
        core=C.build_system(n,k,a6,z,V)
        # core drops empty eqs; compare as sets of nonempty polys per coordinate
        ok &= [e for e in mine if e]==core
print("match_core",ok)
