import json,sys,collections
for name,p in [('C-10-14','/dev/shm/restore/snapshot-1730/sessions/C-10-14/records.jsonl'),('C-16','/dev/shm/restore/v2/sessions/C-16/records.jsonl'),('B','/dev/shm/restore/snapshot-1730/sessions/B/records.jsonl')]:
    rs=[json.loads(l) for l in open(p)]
    c=collections.Counter((r['workload']['curve']['slug'],r['arm'],r['outcome']['status'],r['warmup'],r['round']) for r in rs)
    print('==',name,len(rs))
    arms=collections.defaultdict(set)
    pin=collections.Counter()
    for r in rs:
        arms[r['arm']].add(tuple(r['cost']['unpriced']))
        pin[(r['arm'],bool(r['detail'].get('calibration_pinned_units')))]+=1
    for a,u in arms.items(): print(a,[list(x) for x in u])
    print('pinned',dict(pin))
    for k,v in sorted(c.items()): print(k,v)
