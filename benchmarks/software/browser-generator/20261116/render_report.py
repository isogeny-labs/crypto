from pathlib import Path
import hashlib,io,json,statistics
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from reportlab.pdfgen import canvas
from reportlab.lib.utils import ImageReader

root=Path.cwd();base=root/'benchmarks/software/browser-generator/20261116';order=['baseline-a1','candidate-b1','candidate-b2','baseline-a2'];data=[json.loads((base/'raw'/f'{n}.json').read_bytes()) for n in order]
assert all(r['correctness'] for r in data) and len({r['semantic_sha256'] for r in data})==1
values=[r['wall_s'] for r in data];a=statistics.mean([values[0],values[3]]);b=statistics.mean(values[1:3]);ratio=a/b;reduction=100*(1-b/a)
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':10,'svg.hashsalt':'catalog-controls-20261116'})
fig,ax=plt.subplots(figsize=(7.5,3.8));colors=['#235b85','#cd671d','#cd671d','#235b85'];bars=ax.bar(['A1 baseline','B1 candidate','B2 candidate','A2 baseline'],values,color=colors,width=.62)
ax.set_ylabel('Generator main wall time (seconds)');ax.set_ylim(0,210);ax.set_axisbelow(True);ax.grid(axis='y',alpha=.2);ax.spines[['top','right']].set_visible(False)
for bar,value in zip(bars,values):ax.text(bar.get_x()+bar.get_width()/2,value+4,f'{value:.2f}',ha='center',fontsize=10)
fig.suptitle('Catalog generator: registered local controls',fontsize=14,fontweight='bold',y=.98)
fig.text(.5,.91,'Unisolated host | fixed workload | two observations per variant',ha='center',fontsize=9)
fig.text(.5,.025,'Includes build, render and document check; excludes module import and independent digest.',ha='center',fontsize=8)
fig.tight_layout(rect=(0,.06,1,.88));svg=base/'report.svg';fig.savefig(svg,metadata={'Date':None,'Creator':'render_catalog_control_report.py'})
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines()) + '\n')
image=io.BytesIO();fig.savefig(image,format='png',dpi=240);plt.close(fig);image.seek(0)
pdf=base/'report.pdf';c=canvas.Canvas(str(pdf),pagesize=(612,792),invariant=1);c.setTitle('Registered local catalog generator controls');c.setAuthor('Repository software audit')
c.setFillColorRGB(.10,.16,.22);c.setFont('Helvetica-Bold',20);c.drawString(36,750,'Catalog generator controls')
c.setFont('Helvetica',10);c.drawString(36,731,'Registered A-B-B-A replay | 2026-11-16 | local software component')
c.setFont('Helvetica',9);c.drawString(36,708,'All four arms pass the complete document check and the same independent semantic digest.')
c.drawString(36,694,'Workload: 22,300 curves, 257 sessions and 16,998 yield rows. Host execution is unisolated.')
c.drawImage(ImageReader(image),36,410,width=540,height=274,mask='auto')
c.setFillColorRGB(.91,.94,.96);c.rect(36,376,540,23,fill=1,stroke=0);c.setFillColorRGB(.10,.16,.22);c.setFont('Helvetica-Bold',10)
for x,text in [(44,'Registered arm'),(216,'Variant'),(360,'Wall time (seconds)'),(494,'Check')]:c.drawString(x,384,text)
c.setFont('Helvetica',10)
for i,(name,row) in enumerate(zip(order,data)):
 y=357-i*21;c.drawString(44,y,name);c.drawString(216,y,row['variant']);c.drawRightString(458,y,f"{row['wall_s']:.6f}");c.drawString(494,y,'pass')
c.setStrokeColorRGB(.75,.8,.83);c.line(36,280,576,280);c.setFont('Helvetica-Bold',10);c.drawString(36,260,'Derived local comparison')
c.setFont('Helvetica',10);c.drawString(36,244,f'Baseline mean: {a:.3f} s. Candidate mean: {b:.3f} s. Ratio of means: {ratio:.2f}.')
c.drawString(36,228,f'Observed mean time reduction: {reduction:.2f}%. Two observations per variant; no uncertainty estimate.')
c.setFont('Helvetica',8);c.drawString(36,205,'Timing scope: generator main, including build/render/document check; module import and independent digest excluded.')
c.drawString(36,191,'Scope: this local catalog utility and fixed workload. CI archive verification is required before merge.')
c.setFont('Helvetica-Bold',9);c.drawString(36,168,'Immutable source and data identity')
c.setFont('Courier',7.3);c.drawString(36,152,'Workload/code: '+data[0]['workload_commit'])
protocol=json.loads((base/'protocol.json').read_bytes());c.drawString(36,139,'Baseline code: '+protocol['baseline_commit']);c.drawString(36,126,'Semantic SHA256: '+data[0]['semantic_sha256'])
c.setFont('Helvetica',8);c.drawString(36,106,'Protocol and raw receipts: benchmarks/software/browser-generator/20261116/')
c.drawString(36,93,'protocol.json; raw/baseline-a1.json; raw/candidate-b1.json; raw/candidate-b2.json; raw/baseline-a2.json')
c.setFont('Helvetica',7.5);c.drawString(36,58,'Raw values are retained at full precision in separate bench.record/v1 entries and their JSON receipts.')
c.drawRightString(576,36,'1 / 1');c.showPage();c.save()
summary={'schema':'catalog-software-control-comparison/v1','trial_order':order,'wall_s':values,'baseline_mean_s':a,'candidate_mean_s':b,'derived_ratio_of_means':ratio,'derived_mean_reduction_percent':reduction,'n_per_variant':2,'isolation':'unisolated local software','semantic_sha256':data[0]['semantic_sha256'],'workload_commit':data[0]['workload_commit'],'report_pdf_sha256':hashlib.sha256(pdf.read_bytes()).hexdigest(),'report_svg_sha256':hashlib.sha256(svg.read_bytes()).hexdigest(),'claim_scope':'generator main on the fixed local software workload; no cryptanalysis price or rho claim'}
(base/'comparison.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary))
