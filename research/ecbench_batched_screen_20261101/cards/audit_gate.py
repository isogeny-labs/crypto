import json,sys
p=sys.argv[1]
d=json.load(open(p))
reps=d.get('replays') or []
bad=[]
if d.get('ok') is not True: bad.append('ok is not true')
if d.get('problems'): bad.append('problems: '+str(d.get('problems')[:5]))
if d.get('session_status')!='complete': bad.append('session_status '+str(d.get('session_status')))
if d.get('records')!=d.get('verified_records'): bad.append('records '+str(d.get('records'))+' verified_records '+str(d.get('verified_records')))
if not all(r.get('reproduced') for r in reps): bad.append('a replay did not reproduce')
print('AUDIT-GATE',p,'ok',d.get('ok'),'status',d.get('session_status'),'records',d.get('records'),'verified_records',d.get('verified_records'),'replays',len(reps),'reproduced',sum(1 for r in reps if r.get('reproduced')),'problems',len(d.get('problems') or []))
if bad:
    print('AUDIT-GATE FAILED',bad)
    sys.exit(1)
print('AUDIT-GATE PASSED')
