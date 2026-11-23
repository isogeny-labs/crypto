import json,cypari2; pari=cypari2.Pari()
S=json.load(open('../analysis/sym_rref.json')); out=[]
for r in S['new_raw']:
    vals={l:pari('Mod(1,2)*('+v+')') for l,v in r.items()}
    den=pari.lcm([pari.lcm(pari.denominator(pari.lift(v)),pari.denominator(pari.content(pari.lift(v)))) for v in vals.values()])
    piv=[l for l,v in r.items() if v=='1'][0]
    out.append(dict(pivot=piv,multiplier=str(den),poly={l:str(pari.factor(pari.lift(v*den))) for l,v in vals.items()},
                    expanded={l:str(pari.lift(v*den)) for l,v in vals.items()}))
out.sort(key=lambda d:d['pivot'])
json.dump(out,open('../analysis/relations_cleared.json','w'),indent=1)
for d in out:
    print('R[%s] * %s ='%(d['pivot'],d['multiplier']))
    print('   '+' + '.join('(%s)*%s'%(v,l) for l,v in d['expanded'].items()))
