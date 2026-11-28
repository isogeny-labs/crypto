import json,sys,re
VOL=re.compile(r'(isolation|wall|time|stamp|host|binary|elapsed|seconds|_ns|_ms|cpu|session_id|record_id|run_id|started|finished|duration|pid|sha256_bin|build)',re.I)
def clean(o):
    if isinstance(o,dict): return {k:clean(v) for k,v in o.items() if not VOL.search(k)}
    if isinstance(o,list): return [clean(v) for v in o]
    return o
for l in open(sys.argv[1]):
    print(json.dumps(clean(json.loads(l)),sort_keys=True))
