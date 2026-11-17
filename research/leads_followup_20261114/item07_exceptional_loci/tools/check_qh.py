import json,collections
p=19
R=[json.loads(l) for l in open('../data/census19.jsonl')]
def Qh(a,b,t): return (5377*a**3*t**3+10384*a*a*b*t**2+6548*a*b*b*t+1352*b**3+27709*a*a*t**5+36028*a*b*t**4+11364*b*b*t**3+47435*a*t**7+31524*b*t**6+26615*t**9)%p
S={599,600,602,603,607}
c=collections.Counter(((r['dim6'] in S),((3*r['t']**2+r['a'])%p==0 or Qh(r['a'],r['b'],r['t'])==0)) for r in R); print(c)
