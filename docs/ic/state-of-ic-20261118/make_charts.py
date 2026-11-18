import json, matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
out='/home/user/crypto/docs/ic/state-of-ic-20261118/'
d=json.load(open('docs/ic/leaderboard.json'))
plt.rcParams.update({'font.size':9,'axes.spines.top':False,'axes.spines.right':False})
cols={'prime':'#2a6fdb','char2':'#d9822b','koblitz':'#1f9e5a','koblitz_batch':'#8e44ad'}
names={'prime':'B: prime field F_p (matched neg. rho)','char2':'B: random binary GF(2^m)','koblitz':'A: Koblitz, one target','koblitz_batch':'C: Koblitz, 32-target batch'}
fig,ax=plt.subplots(figsize=(6.4,3.6))
rows=[]
for reg in cols:
  pts=[]
  for e in d['board']:
    if e['regime']!=reg: continue
    b=min((r['ratio_rho'] for r in e['recipes'] if r.get('ratio_rho')),default=None)
    if b: pts.append((e['log2_r'],b)); rows.append((reg,e['slug'],e['log2_r'],b,e['validation']['status']))
  pts.sort(); ax.plot([p[0] for p in pts],[p[1] for p in pts],'o-',color=cols[reg],label=names[reg],ms=4,lw=1.3)
ax.axhline(1,color='k',lw=0.8,ls='--'); ax.text(9.5,1.08,'parity with matched rho (1.0x)',fontsize=7)
ax.set_yscale('log'); ax.set_xlabel('log2 r (subgroup order)'); ax.set_ylabel('best IC cost / matched rho (log)')
ax.set_ylim(0.8,60); ax.legend(fontsize=7,frameon=False,loc='upper left'); ax.set_title('Leaderboard tables A-C: distance to rho by size',fontsize=10)
fig.tight_layout(); fig.savefig(out+'fig1_board.png',dpi=200); fig.savefig(out+'fig1_board.svg')
json.dump(rows,open(out+'fig1_board_data.json','w'),indent=1)
# timeline
pts=[p for p in d['progress']['points'] if p.get('value') and p['value']<200]
fig,ax=plt.subplots(figsize=(6.4,3.2))
import datetime
for p in pts:
  dt=datetime.date.fromisoformat(p['date']); c='#888' if p['reference']!='matched' else '#2a6fdb'
  ax.plot(dt,p['value'],'o',color=c,ms=5)
ax.axhline(1,color='k',lw=0.8,ls='--'); ax.set_yscale('log'); ax.set_ylabel('IC / rho (log; <1 = IC cheaper)')
ax.set_title('Cold one-target headline points by landing date (mixed units; grey = unmatched rho)',fontsize=9)
fig.autofmt_xdate(); fig.tight_layout(); fig.savefig(out+'fig2_timeline.png',dpi=200)
print(len(rows),len(pts))
