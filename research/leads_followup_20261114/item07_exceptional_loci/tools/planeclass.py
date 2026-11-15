import json,sys,collections
sys.path.insert(0,'.')
from nullsp import *
def load(p):
    import os
    if os.path.exists(f'../data/samp{p}.jsonl'):
        return [dict(json.loads(l),unit7=None) for l in open(f'../data/samp{p}.jsonl')]
    R=[]
    for w in (0,1):
        try: R+=[json.loads(l) for l in open(f'../data/plane{p}_t1_w{w}.jsonl')]
        except FileNotFoundError: pass
    return R
if __name__=='__main__':
    p=int(sys.argv[1]); R=load(p)
    c=collections.Counter((r.get('dim6'),r['unit7'],(3+r['alpha'])%p==0) for r in R)
    for k,v in sorted(c.items(),key=str): print(k,v)
