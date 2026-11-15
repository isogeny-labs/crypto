import json,collections
p=19
R=[json.loads(l) for l in open('../data/census19.jsonl')]
def Q(a,b,t):  # weighted-homogeneous lift of the fitted cubic
    return (a*a*b*t**2+14*a*a*t**5+5*a*b*b*t+8*a*b*t**4+3*a*t**7+6*b**3+4*b*b*t**3+6*b*t**6+11*t**9)%p
c=collections.Counter()
for r in R:
    a,b,t=r['a'],r['b'],r['t']
    c[(r['dim6'],(3*t*t+a)%p==0,Q(a,b,t)==0)]+=1
print('dim, tan0, Q0 : instances'); 
for k,v in sorted(c.items()): print(k,v)
tot=len(R); print('total',tot,'departing',sum(1 for r in R if r['dim6']!=605))
# exactness: tan0 or Q0 <=> dim in {599,600,602,603,607}
S={599,600,602,603,607}
tp=sum(1 for r in R if r['dim6'] in S and ((3*r['t']**2+r['a'])%p==0 or Q(r['a'],r['b'],r['t'])==0))
fn=sum(1 for r in R if r['dim6'] in S)-tp
fp=sum(1 for r in R if r['dim6'] not in S and ((3*r['t']**2+r['a'])%p==0 or Q(r['a'],r['b'],r['t'])==0))
print('tan0 or Q0 vs dim in',S,': TP',tp,'FN',fn,'FP',fp)
print('794 class instances:',sum(1 for r in R if r['dim6']==794),' (tan0 or Q0 among them:',sum(1 for r in R if r['dim6']==794 and ((3*r['t']**2+r['a'])%p==0 or Q(r['a'],r['b'],r['t'])==0)),')')
