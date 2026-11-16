IC=/Volumes/SSD990-2/cargo-target-ic-norm/release/ic
DEG=$1; shift
for fb in "$@"; do
 timeout 280 $IC bench --char2-degree $DEG --seed 5 --factor-base "$fb" --oracle mitm:negation_folded=1,m=3 --targets walk --repeats 4 2>&1 | python3 -c "
import sys
rows=[l for l in sys.stdin if l.startswith('| ') and l.count('|')>=14 and ('binary-' in l)]
S=[];T=[];ok=True;lr=None
for l in rows:
    c=[x.strip() for x in l.strip().strip('|').split('|')]
    if len(c)<14: continue
    S.append(float(c[11])); T.append(int(c[6])); ok&=(c[13]=='yes'); b=c[2]; cols=c[3]
if S: print('$fb','pts',b,'cols',cols,'meanS %.1f minS %.1f'%(sum(S)/len(S),min(S)),'trials/rel',round(sum(T)/len(T)),'ok',ok)
else: print('$fb','no rows')
"
done
