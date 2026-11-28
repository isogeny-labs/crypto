import json,sys
def strip(d):
    if isinstance(d,dict): return {k:strip(v) for k,v in d.items() if not k.endswith('_ms') and k!='wall_ms'}
    if isinstance(d,list): return [strip(x) for x in d]
    return d
base=json.load(open(sys.argv[1]))
b=strip(base)
for f in sys.argv[2:]:
    o=json.load(open(f)); s=strip(o)
    fm=[t['f4_muls'] for t in o[0]['per_test']]
    print(f, 'identical(non-timing)=',s==b, 'f4_muls sum',sum(fm), 'lin', sum(t['lin_muls'] for t in o[0]['per_test']), 'sols', sum(t['fp_solutions'] for t in o[0]['per_test']))
