import json, random, statistics as st, sys, math
for fn in sys.argv[1:]:
    R = json.load(open(fn))['records']
    key = lambda r: (r['curve'], r['target'])
    by = {}
    for r in R: by.setdefault(r['arm'], {})[key(r)] = r
    ks = sorted(by['ic-single-L1'])
    def g(r, w): return r['adds'] + r['dbls'] + w*(r['second']+r['probe']+r['guard']+r['insert']+r['rowop']+r['lift'])
    def walkadds(r): return r['adds'] - r['pre_adds'] - r.get('restarts',0)*3*r['n'].bit_length()
    rng = random.Random(1)
    def boot(f):
        pt = f(ks); bs = sorted(f([rng.choice(ks) for _ in ks]) for _ in range(2000))
        return pt, bs[50], bs[1949]
    print(fn)
    for L in ('L1', 'L16'):
        s, p = by['ic-single-'+L], by['ic-paired-'+L]
        for w in (0, 1):
            print(f'  {L} total GAE w={w} paired/single: %.3f [%.3f, %.3f]' % boot(lambda K: sum(g(p[k], w) for k in K)/sum(g(s[k], w) for k in K)))
        print(f'  {L} walk adds paired/single: %.3f [%.3f, %.3f]' % boot(lambda K: sum(walkadds(p[k]) for k in K)/sum(walkadds(s[k]) for k in K)))
        print(f'  {L} candidates/row paired / single: %.3f [%.3f, %.3f]' % boot(lambda K: (sum(p[k]['cand'] for k in K)/sum(p[k]['rows'] for k in K))/(sum(s[k]['cand'] for k in K)/sum(s[k]['rows'] for k in K))))
