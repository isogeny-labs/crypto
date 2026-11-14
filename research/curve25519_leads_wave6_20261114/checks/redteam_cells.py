import glob, json, os, sys
R='/home/user/crypto/research/'
def lpf(n):
    f,b=2,1
    while n>1:
        while n%f==0: b,n=f,n//f
        f+=1
    return b
fs=sorted(glob.glob(R+'curve25519_exponent_study_20261108/cells/*.json'))+sorted(glob.glob(R+'curve25519_exponent_study_ext_20261110/cells/*.json'))
for f in fs:
    j=json.load(open(f))
    pt=j.get('per_target') or []
    d=j['d_or_t']
    sdm=[r.get('solving_degree_max') for r in pt]
    sd=[r.get('solving_degree') for r in pt]
    dr=[r.get('degree_reached') for r in pt]
    inc=sum(1 for r in pt if r.get('inconsistent'))
    mc=max([r.get('max_cols',0) for r in pt] or [0])
    print(os.path.basename(f), 'cap' if j['capped'] else '   ', 'L=%d nv=%d indeg=%d'%(j.get('levels',-1),j.get('n_vars',-1),j.get('input_degree',-1)), 'lpf',lpf(d),'SDmax',sorted(set(sdm)),'SD1',sorted(set(sd)),'DR',sorted(set(dr)),'inc %d/%d'%(inc,len(pt)),'maxcols',mc,'T %.3g'%j['mean_field_ops'] if j.get('mean_field_ops') else '')
