import json,sys
p=sys.argv[1]
d=json.load(open(p))
r=d['ops']['ratio_b_over_a']
print('AA-GATE',p,'ops.ratio_b_over_a',r,'ops.ci95',d['ops'].get('ci95'),'ops.status',d['ops'].get('status'))
if r is None or not (0.999 <= r <= 1.001):
    print('AA-GATE FAILED')
    sys.exit(1)
print('AA-GATE PASSED')
