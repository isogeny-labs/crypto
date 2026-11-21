import json,collections,statistics as st,sys
here=__file__.rsplit('/',1)[0]
R=[json.loads(l) for l in open(here+'/raw_output.jsonl')]
A=json.load(open(here+'/../../analysis/partA_v3_summary.json'))
pa={(c['fb'],c['m']):c['arms'] for c in A['cells']}
print("records",len(R),"incorrect",sum(1 for r in R if not r['correct']),"aborted",sum(r['aborted'] for r in R),
      "lib_mismatch",sum(1 for r in R if r['lib_matches_reimpl'] is False),"lib_incorrect",sum(1 for r in R if not r['lib_correct']))
g=collections.defaultdict(lambda:collections.defaultdict(list))
for r in R:
    for k in ('core_dim','fill','pivot_nnz','arith','book','order_ops','solve_row_ops','mixedpeel_equiv_row_ops','nd_top_sep'):
        g[(r['fb'],r['m'],r['order'])][k].append(r[k])
    g[(r['fb'],r['m'],r['order'])]['rows'].append(r['rows_read'])
mean=lambda v:sum(v)/len(v)
print("\nfb m | order | core_dim fill pivot_nnz | arith book(order part) | solve_row_ops | mixed-peel-equiv row_ops | vs markowitz | wins/16 vs markowitz (equiv)")
for fb in (128,256,512,1024):
  for m in (2,3,4):
    base=[r for r in R if r['fb']==fb and r['m']==m and r['order']=='markowitz']
    bk={r['seed']:r['mixedpeel_equiv_row_ops'] for r in base}
    bs={r['seed']:r['solve_row_ops'] for r in base}
    for o in ('markowitz','amd','nd'):
        d=g[(fb,m,o)]
        rs=[r for r in R if r['fb']==fb and r['m']==m and r['order']==o]
        w=sum(1 for r in rs if r['mixedpeel_equiv_row_ops']<bk[r['seed']])
        wf=sum(1 for r in rs if r['fill']<[b['fill'] for b in base if b['seed']==r['seed']][0])
        ratio=mean(d['mixedpeel_equiv_row_ops'])/mean([bk[s] for s in bk])
        print(f"{fb:5d} {m} | {o:9s} | {mean(d['core_dim']):7.1f} {mean(d['fill']):8.1f} {mean(d['pivot_nnz']):8.1f} | {mean(d['arith']):9.1f} {mean(d['book']):9.1f}({mean(d['order_ops']):8.1f}) | {mean(d['solve_row_ops']):9.1f} | {mean(d['mixedpeel_equiv_row_ops']):9.1f} | x{ratio:.3f} | ops<{w}/16 fill<{wf}/16")
print("\nPart A reference (row_ops means over 16 seeds): fb m | rows | mixed-peel | filter+lanczos | lanczos/markowitz-equiv-here | best-order-equiv")
for fb in (128,256,512):
  for m in (2,3):
    a=pa[(fb,m)]
    mk=mean(g[(fb,m,'markowitz')]['mixedpeel_equiv_row_ops'])
    best=min(('markowitz','amd','nd'),key=lambda o:mean(g[(fb,m,o)]['mixedpeel_equiv_row_ops']))
    print(fb,m,a['mixed-peel']['rows'],a['mixed-peel']['row_ops'],a['filter+lanczos']['row_ops'],"here markowitz-equiv %.1f"%mk,"best",best,"%.1f"%mean(g[(fb,m,best)]['mixedpeel_equiv_row_ops']),"lanczos/best=%.3f"%(a['filter+lanczos']['row_ops']/mean(g[(fb,m,best)]['mixedpeel_equiv_row_ops'])))
print("\nMEDIANS over 16 seeds (Part A json holds medians): mixed-peel-equivalent row_ops by order vs Part A")
for fb in (128,256,512):
  for m in (2,3):
    a=pa[(fb,m)]
    med={o:st.median([r['mixedpeel_equiv_row_ops'] for r in R if r['fb']==fb and r['m']==m and r['order']==o]) for o in ('markowitz','amd','nd')}
    print(f"{fb} {m} PartA mixed-peel median {a['mixed-peel']['row_ops']} | filter+lanczos median {a['filter+lanczos']['row_ops']} | here markowitz {med['markowitz']} amd {med['amd']} nd {med['nd']} | best/lanczos={min(med.values())/a['filter+lanczos']['row_ops']:.2f}")
