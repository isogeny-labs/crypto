"""Merge worker verdict files into leads.json; report schema problems (no judgement)."""
import json, glob, os, sys, collections
S=os.path.dirname(os.path.abspath(__file__))
V=set("refuted engineering open blocked unexamined out-of-scope".split())
ST=set("proof measured toy desk none".split()); COLS=set("FB ORACLE LIFT TRANSFER BARRIER ENG".split()); SK=set("atlas idea-pr crypto-study".split())
req=["id","label","column","source_kind","verdict","evidence_strength","flagship","exponent_delta","distance_to_rho","mechanism","why_this_verdict","cheapest_next_test","sources_checked"]
leads=[]; problems=[]; seen={}
for f in sorted(glob.glob(S+'/verdicts/W*.json')):
    w=os.path.basename(f)[:-5]
    try: arr=json.load(open(f))
    except Exception as e: problems.append(f"{w}: unparsable {e}"); continue
    if isinstance(arr,dict): arr=arr.get('leads') or arr.get('records') or list(arr.values())[0]
    for r in arr:
        r['worker']=w
        for k in req:
            if k not in r: problems.append(f"{w} {r.get('id')}: missing {k}"); r.setdefault(k, None)
        if r['verdict'] not in V: problems.append(f"{w} {r['id']}: bad verdict {r['verdict']}"); r['verdict']='unexamined'
        if r['evidence_strength'] not in ST: problems.append(f"{w} {r['id']}: bad strength {r['evidence_strength']}"); r['evidence_strength']='none'
        if r['column'] not in COLS: problems.append(f"{w} {r['id']}: bad column {r['column']}"); r['column']='BARRIER'
        if r['source_kind'] not in SK: problems.append(f"{w} {r['id']}: bad source_kind {r['source_kind']}")
        if r['id'] in seen: problems.append(f"{w} {r['id']}: duplicate of {seen[r['id']]}")
        seen[r['id']]=w
        r['label']=str(r['label'])[:38]
        r['flagship']=bool(r.get('flagship'))
        leads.append(r)
out={'updated':'2026-11-14','generated_by':'scratchpad/merge.py over W1..W9 worker files','standing':'[pcc:unverified] throughout','leads':leads}
json.dump(out,open(sys.argv[1] if len(sys.argv)>1 else S+'/leads.json','w'),indent=1,ensure_ascii=False)
print(len(leads),'leads;',len(problems),'problems')
for p in problems[:60]: print(' -',p)
c=collections.Counter((r['column'],r['verdict']) for r in leads)
for col in sorted(COLS): print(col, {v:c[(col,v)] for v in sorted(V) if c[(col,v)]})
print('flagship:',[r['id'] for r in leads if r['flagship']])
print('by source:',collections.Counter(r['source_kind'] for r in leads), 'by verdict:',collections.Counter(r['verdict'] for r in leads))
