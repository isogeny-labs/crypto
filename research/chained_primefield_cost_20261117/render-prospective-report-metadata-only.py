"""Lay out the prospective protocol only; no scientific computation."""
from pathlib import Path
import hashlib,json,subprocess
import typst
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_primefield_cost_20261117';d=s/'prospective-report-01';d.mkdir(exist_ok=False)
graph='''digraph g {rankdir=LR;bgcolor="transparent";node[shape=box,style="rounded,filled",fillcolor="#eef3f8",fontname="DejaVu Sans",fontsize=10];edge[fontname="DejaVu Sans",fontsize=9];a[label="12 hand calibration oracles\n+1 corrupt-counter negative"];b[label="All64 cubic +8 general inverses\n17 forward +17 dual map calls"];c[label="Full source-operation vectors\n+inversion-input histograms\nEvery gain/loss retained"];a->b->c;}'''
(d/'flow.dot').write_text(graph);subprocess.run(['dot','-Tsvg',str(d/'flow.dot'),'-o',str(d/'flow.svg')],check=True)
paras=[
 'Unit16 asks what lies below the qualified inverse15 wrapper savings. The inner Euclidean input changes from a² to a, so its underlying arithmetic must be counted. This is a prospective bounded correctness/accounting unit, without a timing experiment or a required improvement. ACTIVE-only01438723 precedes research. [pcc:unverified]',
 'The ordered model is [M,A,S,R,I,R64]: explicit BigUint multiplication/addition/subtraction/remainder outside modpow, atomic prime-inverse modpow calls, and CubicField.c’s u64 remainder. Preserve every original expression. Record the complete inversion-input histogram, including unit arguments. The actual0.4.8 library chooses Montgomery arithmetic at odd modulus; do not replace it with a guessed exponentiation cost. [pcc:unverified]',
 'Keep simultaneous whole-backend and inverse-only budgets; a scope around Field.inv captures all arithmetic in the inverse component. Whole-map backend counts exclude direct Fp2/u64 work elsewhere. Allocation, comparisons, cloning, limb/CPU work and empirical prices are outside this model. The table below is fixed before code. For its first11 rows p=7,x=2+3X,y=4+X; the last uses the existing101 cubic. [pcc:unverified]',
 'All64/8 primitive and34 map rows must match the full frozen15B scientific snapshot. Keep2 zero/8 sign/3 constructor controls,17 witnesses,both closed orbits,9/8 forward routes,1054 shortcuts/0general and all5/2 wrapper relations. Record both arms and expanded inversion-input vectors; require no cost win and discard no unfavorable observation. [pcc:unverified]',
 'Before calls publish the complete source/caller closure. Rust1.99.1,exact3 prebuilt rlibs,one worker,120s phases,strict Clippy with both sentinels,format and unchanged8/3MiB preflight floors. Shell handles thin build/run; Python is limited to metadata/hash/file I/O. A separately copied555 qualifier has distinct immutable build/pre/post manifests. No cold dependency build,paid job or replacement for original EKS workloads. [pcc:unverified]',
 'Retain full source,deps,bothELFs,all raw streams/exits and every failure in a whole byte/mode-checked packet. Current artifact policy remains unchanged; whole raw containers stay private while S3 migration is unresolved. Source/readable reports/allowed exact text and custody metadata are checkpointed. KB/Conductor127 fallback and exact source receipts are recorded, without novelty inference. Existing conditional Field proof69b890 is reused. [pcc:unverified]'
]
table=[['constant9','[0,0,0,1,0,0]'],['coeffs8,0,9','[0,0,0,3,0,0]'],['x+y','[0,2,0,2,0,0]'],['x−y','[0,2,2,2,0,0]'],['−x','[0,0,2,2,0,0]'],['x*y','[4,4,0,8,0,0]'],['scalar x,2','[2,0,0,3,0,0]'],['eval x,2','[2,2,0,2,0,0]'],['inv2','[0,0,1,1,1,0]'],['inv0','[0,0,0,1,0,0]'],['divrem x,1','[4,5,4,10,1,0]'],['cubic101.c203','[0,0,0,1,0,1]']]
body='#set page(paper:"a4",margin:15mm)\n#set text(font:"DejaVu Sans",size:9.0pt)\n#set par(justify:true)\n#align(center)[#text(size:17pt,weight:"bold")[Prime-field work below the cubic inverse]\n#linebreak()Frozen prospective unit16 · 17 November 2026]\n'
for i,p in enumerate(paras):
 body+='#text('+json.dumps(p,ensure_ascii=False)+')\n\n'
 if i==0:body+='#align(center)[#image("flow.svg",width:97%)]\n'
 if i==2:
  items=['Case','[M,A,S,R,I,R64]']+[x for row in table for x in row]
  body+='#align(center)[#table(columns:(1.3fr,1.7fr),inset:3pt,'+','.join('['+x+']' for x in items)+')]\n'
(d/'REPORT.md').write_text('# Prime-field work below the cubic inverse\n\n'+'\n\n'.join(paras)+'\n');(d/'report.typ').write_text(body.rstrip()+'\n');typst.compile(str(d/'report.typ'),output=str(d/'REPORT.pdf'),root=str(d))
info=subprocess.run(['pdfinfo',str(d/'REPORT.pdf')],stdout=subprocess.PIPE,check=True).stdout.decode();assert 'Pages:           1' in info,info
preview=Path('/dev/shm/ct798-cost16-prospective');preview.mkdir(exist_ok=False);subprocess.run(['pdftoppm','-singlefile','-scale-to','1400','-png',str(d/'REPORT.pdf'),str(preview/'page')],check=True)
record={'PDF_bytes':(d/'REPORT.pdf').stat().st_size,'PDF_sha256':hashlib.sha256((d/'REPORT.pdf').read_bytes()).hexdigest(),'pages':1,'preview':str(preview/'page.png'),'inspection':'pending'};(s/'PROSPECTIVE_REPORT_BUILD_16.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record,indent=2))
