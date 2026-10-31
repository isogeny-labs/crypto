"""Render stored native results only; no cryptographic computation or fitting."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import A4
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.lib.utils import ImageReader
import textwrap
p=Path(__file__).resolve().parent
s=json.loads((p/'summary.json').read_text())
rows=s['rows']
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':10})
fig,ax=plt.subplots(1,2,figsize=(11,4.4),gridspec_kw={'width_ratios':[1.3,1]})
x=list(range(4));w=.25
ax[0].bar([i-w for i in x],[r['first_proxy'] for r in rows],w,label='Fixed first',color='#8896a5')
ax[0].bar(x,[r['min_weight_proxy'] for r in rows],w,label='Fixed min-weight',color='#34758c')
ax[0].bar([i+w for i in x],[r['selected_including_overhead_proxy'] for r in rows],w,label='Selector + overhead',color='#df9a34')
ax[0].set_yscale('log');ax[0].set_ylabel('Counted proxy units (log scale)')
ax[0].set_xticks(x,['n5\ntrain','n9\ntrain','n15\nheldout fresh','n17\nheldout archive'])
ax[0].legend(fontsize=8);ax[0].set_title('Constant selector chooses min-weight')
ax[1].bar(x,[r['loss_against_fixed_min_weight_proxy'] for r in rows],color='#bd644e')
for i,r in enumerate(rows):ax[1].text(i,r['loss_against_fixed_min_weight_proxy'],f"{r['loss_against_fixed_min_weight_proxy']:,}",ha='center',va='bottom',fontsize=9)
ax[1].set_xticks(x,['n5','n9','n15','n17']);ax[1].set_ylabel('Additional proxy units');ax[1].set_ylim(0,8000);ax[1].set_title('Strict loss versus fixed min-weight')
fig.suptitle('Retrospective whole-pass selector; original item 9 remains partial',fontsize=12)
fig.tight_layout();fig.savefig(p/'reports/policy-counts.svg');fig.savefig(p/'reports/policy-counts.png',dpi=150);plt.close(fig)
font='/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf';pdfmetrics.registerFont(TTFont('DV',font))
c=canvas.Canvas(str(p/'reports/report.pdf'),pagesize=A4);W,H=A4
c.setTitle('Restricted whole-pass learned pivot selector: null result')
def para(t,y,size=10,width=91):
 c.setFont('DV',size)
 for line in textwrap.wrap(t,width=width):c.drawString(38,y,line);y-=size+4
 return y-8
y=H-42;y=para('A constant selector is a null result against fixed min-weight',y,14,65)
y=para('Native fitting on n5/n9 returns the constant min-weight policy. All four small independent dense RREF controls pass. Both heldout n15 policy replays complete; chosen-policy counts match the archived vector exactly. n17 is archival evaluation only. Original item 9 remains partial.',y)
y=para('Retrospective split: old heldout summaries were already visible. Input-only features: N, D and the sum/maximum of term counts in the first 64 rows. Candidate family: two constants plus one-level stumps over training thresholds. Two train shapes support no calibrated uncertainty estimate.',y)
c.setFont('DV',9)
for line in ['Shape / replay             First proxy       Min-weight       Selected + overhead',*[(f"{r['shape'].split('-')[0]:4} / {'archive' if r['evaluation']=='archived-only' else 'fresh':7}  {r['first_proxy']:>15,}  {r['min_weight_proxy']:>15,}  {r['selected_including_overhead_proxy']:>15,}") for r in rows]]:
 c.drawString(38,y,line);y-=17
y-=12
y=para('Training: 91,734 trace records; 8,984,687 uncompressed bytes; 28 candidate/sample evaluations; 24 threshold comparisons. Acquisition of both policies on train shapes costs 9,765,340 proxy units. Declared counted training components total 11,076,480.',y)
y=para('Proxy = sparse term visits + journal bytes / 8, with components preserved separately. Selection adds input terms, words read, scalar feature updates and model output. Equal word weights are conventional. Verification, trace writing, parsing, hashing, decompression, allocation and bookkeeping are not fully instruction-priced. This is not complete machine cost.',y)
y=para('The selector adds 5,486 proxy units versus fixed min-weight at n15. Charging training once gives 178,884,333 units; treating historical training as sunk gives 167,807,853. The historical min-weight versus first difference is not an AI gain.',y)
y=para('Large n15 row-space correctness remains independently uncertified. Same-count replay proves reproducibility only. This is one fixed D=4 Macaulay pass, not F4, closure, a solving-degree finding or an ECDLP speedup. Fit/inference/reducers: one CPU and hard 512 MiB caps. Disk use was 198,237,682 bytes. Aggregate disk/time budgets were operational targets, not global script limits. Shared-host timings are descriptive.',y)
y=para('Evidence: protocol.json; summary.json; src/selector.cpp; src/summarize.cpp; runs/frozen-v1/model.json, attempts.tsv, sources.sha256, binaries.sha256 and heldout-n15-comparison.json. Full source-linked report: reports/REPORT.md.',y,8,110)
c.showPage();c.setFont('DV',14);c.drawString(38,H-45,'Counted policy comparison and its boundary')
c.drawImage(ImageReader(str(p/'reports/policy-counts.png')),25,H-320,width=W-50,height=235,preserveAspectRatio=True,anchor='c')
y=H-350;y=para('Left: selected and fixed min-weight bars nearly coincide because the rule is constant. Right: the explicit positive selection overhead makes the result a strict loss against simply fixing min-weight. No new learned-policy benefit was observed.',y)
y=para('Remaining work: independent large-row-space verification; diverse shapes with differing winning policies; prospective train/calibration/test split; pivot-level and column-level learned F4 policies; complete feature, training, inference and trace costs. No external fastest-engine comparison or end-to-end gain is claimed.',y)
y=para('Immutable previous inputs/traces remain under ../bounded_memory/. This additive run leaves all previous source and records unchanged. New raw matrices, journals and traces are losslessly compressed with original hashes and stream-verification receipts.',y)
c.save()
