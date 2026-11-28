"""Report layout only. Experimental phase means and raw totals come from Rust audit223."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
s=Path(__file__).resolve().parents[1]
a=json.loads((s/'PARENT_TRACE_ANALYSIS_223.json').read_text())
keys=[('capture_input_and_deadline_gate','Capture/input','#cddce9'),('spawn_call','Spawn','#6c91b3'),('wait_for_child','Wait (includes worker)','#315e84'),('process_receipt_write','Receipt write','#b7c9b5'),('output_parse_and_child_gates','Output parse/gates','#87aa85'),('artifact_readback_and_input_binding','Readback/input binding','#c3ad82'),('controller_whole_proof_replay','Controller full replay','#c87937'),('basis_stat_cross_arm_binding_and_digests','Basis/stats/digests','#9e869d'),('residual_bookkeeping','Residual','#d4ccd2')]
plt.rcParams['svg.hashsalt']='relation-parent-trace-216'
fig,ax=plt.subplots(figsize=(10,4.5),layout='constrained')
labels=[]
for i,c in enumerate(a['cases']):
 arm=next(r for r in c['arms'] if r['arm']=='packed-dag-487')
 left=0
 for key,label,color in keys:
  value=arm['mean_exclusive_phase_ns'][key]/1e6
  ax.barh(i,value,left=left,height=.55,color=color,label=label if i==0 else None)
  left+=value
 ax.plot([v/1e6 for v in arm['sorted_parent_ns']],[i]*3,'k.',ms=6,label='Three recorded full-call totals' if i==0 else None)
 labels.append(f"{'p31' if c['prime']==2147483647 else 'p53'}, n{c['n']}")
ax.set_yticks(range(6),labels);ax.invert_yaxis();ax.set_xlim(0,7)
ax.set_xlabel('Parent time (ms); stack = mean exclusive phases; dots = observed totals')
ax.set_title('Trace216: the wait interval includes computation and process lifecycle',loc='left',fontsize=11,fontweight='bold')
ax.spines[['top','right']].set_visible(False)
ax.legend(loc='upper left',bbox_to_anchor=(0,-.22),ncol=3,frameon=False,fontsize=8)
fig.savefig(s/'visual-242/parent-phases.svg',metadata={'Date':None})
fig.savefig(s/'visual-242/parent-phases.png',dpi=150)
p=s/'visual-242/parent-phases.svg';p.write_text('\n'.join(line.rstrip() for line in p.read_text().splitlines())+'\n')
# An explanatory diagram; all timing labels refer to existing recorded data or future work.
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="235" viewBox="0 0 1000 235">
<rect width="1000" height="235" fill="white"/><g font-family="DejaVu Sans,sans-serif" font-size="16" fill="#193147">
<text x="20" y="25" font-weight="bold">Qualified transport paths; no speed comparison yet</text>
<rect x="20" y="45" width="455" height="65" rx="7" fill="#e7edf3"/><text x="35" y="69">Wrapped: taskset → timeout → prlimit → worker</text><text x="35" y="94" font-size="13">Three utility execs; existing limits and full proof checks</text>
<rect x="505" y="45" width="475" height="65" rx="7" fill="#e1efe9"/><text x="520" y="69">Native: pre_exec limits → one worker exec</text><text x="520" y="94" font-size="13">Private group · pidfd/poll deadline · parent-death guard</text>
<path d="M 247 111 V 145 H 500 M 742 111 V 145 H 500 V 160" fill="none" stroke="#47637c" stroke-width="2"/>
<rect x="180" y="160" width="640" height="55" rx="7" fill="#f5eee3"/><text x="200" y="184">Complete worker proof replay + parent full replay / exact bindings</text><text x="200" y="204" font-size="13">Same source-bound frozen workers and inputs; generic changes also reach controls</text>
</g></svg>'''
(s/'visual-242/supervisor-path.svg').write_text(svg+'\n')
