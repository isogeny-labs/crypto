IC=/Volumes/SSD990-2/cargo-target-ic-norm/release/ic
BITS=$1; FAM=$2; shift 2
for fb in "$@"; do
 timeout 280 $IC bench --bits $BITS --family $FAM --seed 5 --factor-base "$fb" --oracle mitm:negation_folded=1,m=3 --targets walk --repeats 4 2>&1 | python3 -c "
import sys,re
rows=[l for l in sys.stdin if l.startswith('| ') and l.count('|')>=14 and ('cm-mult' in l or 'glv-orbit' in l or 'prime-abs' in l)]
S=[];T=[];ok=True
for l in rows:
    c=[x.strip() for x in l.strip().strip('|').split('|')]
    if len(c)<14: continue
    S.append(float(c[11])); T.append(int(c[6])); ok&=(c[13]=='yes'); b=c[2]; cols=c[3]
if S: print('$fb','pts',b,'cols',cols,'meanS %.1f minS %.1f'%(sum(S)/len(S),min(S)),'trials/rel',round(sum(T)/len(T)),'ok',ok)
else: print('$fb','no rows')
"
done
