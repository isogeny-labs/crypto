#!/bin/bash
cd /Volumes/SSD990-2/s1-build/s1b-20261101
for l in $(seq 3 17); do for b in bins/first/l$l/wdsat_id*; do
 /usr/bin/time -l -o su.txt $b -i t2.anf -x -b -n 17 -l $l -m 3 >/dev/null 2>&1
 echo "$l $(basename $b) $(grep 'instructions retired' su.txt | awk '{print $1}')"
done; done > startup.txt
python3 - <<'PY'
import statistics as st, json, collections
d=collections.defaultdict(list)
for ln in open('/Volumes/SSD990-2/s1-build/s1b-20261101/startup.txt'):
    p=ln.split()
    if len(p)==3: d[p[0]].append(int(p[2]))
json.dump({k:st.median(v) for k,v in d.items()}, open('/Volumes/SSD990-2/s1-build/s1b-20261101/start.json','w'), indent=1)
print({k:(st.median(v),min(v),max(v)) for k,v in d.items()})
PY
