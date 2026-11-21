"""Lay out already recorded Rust observations; no research computation."""
from pathlib import Path
import hashlib,json,subprocess
import typst
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117'
preview=Path('/dev/shm/ct753-map14-report-inspection');preview.mkdir(exist_ok=False)
def render(folder,title,subtitle,paragraphs,dot):
 folder.mkdir(parents=True,exist_ok=False)
 (folder/'flow.dot').write_text(dot)
 subprocess.run(['dot','-Tsvg',str(folder/'flow.dot'),'-o',str(folder/'flow.svg')],check=True)
 md='# '+title+'\n\n'+subtitle+'\n\n'+'\n\n'.join(paragraphs)+'\n'
 (folder/'REPORT.md').write_text(md)
 body='''#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9.3pt)
#set par(justify: true)
#set heading(numbering: none)
'''
 body+='#align(center)[#text(size:17pt,weight:"bold")['+title+']#linebreak()'+subtitle+']\n'
 for i,p in enumerate(paragraphs):
  if i==1:body+='#align(center)[#image("flow.svg",width:96%)]\n'
  body+=p.replace('_','\\_')+'\n\n'
 (folder/'report.typ').write_text(body)
 typst.compile(str(folder/'report.typ'),output=str(folder/'REPORT.pdf'),root=str(folder))
 info=subprocess.run(['pdfinfo',str(folder/'REPORT.pdf')],check=True,stdout=subprocess.PIPE).stdout.decode();assert 'Pages:           1' in info,info
 name=folder.name
 subprocess.run(['pdftoppm','-singlefile','-scale-to','1400','-png',str(folder/'REPORT.pdf'),str(preview/name)],check=True)
 return {'folder':str(folder.relative_to(r)),'pdf_bytes':(folder/'REPORT.pdf').stat().st_size,'pdf_sha256':hashlib.sha256((folder/'REPORT.pdf').read_bytes()).hexdigest(),'pages':1,'preview':str(preview/(name+'.png')),'inspection':'pending visual inspection'}
graph='''digraph g { rankdir=TB;bgcolor="transparent";node[shape=box,style="rounded,filled",fillcolor="#eef3f8",fontname="DejaVu Sans",fontsize=11,color="#34556a"];edge[fontname="DejaVu Sans",fontsize=9,color="#34556a"];
 {rank=same;F[label="J(F) over F101\\n[97,29,96,18,87,99,1]"];G[label="J(G) over F101\\n[16,53,24,80,73,57,1]"];}
 F->G[label="Richelot φ: fixed (2,2) correspondence\\n17 witnesses +34 sums +10 scalar cases"];
 G->F[label="fixed positive dual ψ\\nψφ=[2] on declared observations"];
 old[label="Old coordinate models\\nchart pullback → old φ → target chart"];
 old->G[label="κ=100 fixed by correspondence\\n17 canonical conjugacy equalities"];
 }'''
paragraphs=[
 'The first frozen divisor-map unit passes:17 witnesses,34 diagonal/cyclic homomorphism pairs and17 old/new chart conjugacies. Both declared closed quadratic orbits use the Quadratic route. Nonzero generators of orders109 and113 and all10 declared scalar cases pass; three malformed constructors and two wrong-dual-sign controls are detected. Data claim403870 is unsigned. [pcc:unverified]',
 'The actual positive dual returns [2]D; its closed route agrees. The forward-after-dual orientation is checked on the17 image classes, and inverse6159 recovers the declared inputs. The composed old-chart return chain agrees with the new dual on those17 observations. These are canonical divisor computations, including poles, infinities, repeated supports and closed quadratic orbits. [pcc:unverified]',
 'The pre-map written derivation11b4a9 predicted old scale30, square root16, normalized target ordinate76, new scale13 and κ100. The first output line records exactly these values. The minus sign was derived from x−z=−(w−v)/(wv) in the defining correspondence before source code and map execution; the earlier plus-sign draft remains in full. PROOF_14.md is a written argument, not a new general Lean theorem or signed certificate. [pcc:unverified]',
 'Public source f35867831245560527428e3562ab55d74eefb5a0 precedes compilation. Rust1.99.1, exact prebuilt dependencies, one worker,120s per call and a separately copied555 qualifier were used. One new test passes;13 inherited tests were compiled and filtered. All24 phase expectations, strict Clippy, format and source/dependency/qualifier guards pass. Full42,435B stdout and the74-member1,730,144B archive027021bb retain complete source, all3rlibs and both ELF copies; every full byte and mode was compared. [pcc:unverified]',
 'Earlier storage, Clippy text-matcher, missing embedded CSV and item-layout failures remain in separate complete packets and run reports. E prepublication format/sparse/whitespace records are packed unchanged; F preserves its scientific bytes. A metadata counter correction distinguishes the two required closed orbits from eight total Quadratic routes; no arithmetic rerun occurred. [pcc:unverified]',
 'Scope and next step: same-engine finite correctness controls, with no timing ratio, independent all-class replay or DLP solve. The original12,317-class and fullLP workloads remain EKS-only; paid STOPe42949 and locked vault8edec422 persist. Next is cubic-subfield closed-fibre inversion, a written branch-equivalence argument, frozen native tests, then an admitted cost comparison. [pcc:unverified]',
 'Sources: ConsumerE079 nonsplit.rs, nonsplit_map.rs and CLOSED_FIBRES.md; retained unit12 balanced/shifted engines; Kunzweiler Propositions7–8 and degree-five assumptions; corrected Galbraith8.3.8/8.3.10/10.4.2. Source snapshots and full commands are in evidence/first-arithmetic-F-09. Shared arithmetic means route agreement is not independent full replay. This report updates only study-local coverage; canonical cost/leaderboard/curve graphs gain no new measured value or edge. [pcc:unverified]'
]
records=[render(s/'report-actual-map-10','Operational Richelot maps in joint charts','First frozen arithmetic unit · 17 November 2026',paragraphs,graph)]
negatives=[
 ('run-20261117T0628-storage-A','First storage preflight: no compiler call','RAM preflight446464B below fixed8MiB; no compiler or arithmetic call. Complete13-member1416B raw archive retained. Claim9eccf0.','infrastructure-first-storage-preflight-03','storage guard → stopped'),
 ('run-20261117T0640-frontend-B','First frontend text guard: no main compilation','Direct Clippy positive0/negative1 works, but the caller expects clippy::eq_op instead of the actual canonical clippy::eq-op diagnostic. All16 phase expectations match; no main scientific compilation or arithmetic. Complete50-member1305644B archive b3062a8e, publicac01a90b. Claim7af5b2.','infrastructure-first-lint-frontend-B-04','frontend controls → text guard stopped'),
 ('run-20261117T0714-compiler-C','First main compiler: missing retained include','Main rustc exits1 because unchanged shifted12.rs embeds fixtures-12.csv absent from copied closure. Two unused retained imports remain in diagnostics.17 phases; no strict-main Clippy or arithmetic. Complete58-member1307576B archive2b3913f1, publicab9297b2. Claim6cfcef.','infrastructure-first-main-compiler-C-05','source/dependencies → compiler stopped'),
 ('run-20261117T0756-storage-D','D storage preflight: unchanged scratch floor','Scratch3039232B is below fixed3145728B. Publicc0a D setup stops before compiler/arithmetic. Complete10-member1272B raw archive retained. Claim0852fe. Nine exact unchanged ledger caches were later normally packed; no ledger row/trust/policy/source change.','infrastructure-D-storage-and-ledger-cache-06','storage guard → stopped'),
 ('run-20261117T0814-strict-Clippy-D2','D2 compiler passes; strict Clippy stops','Main rustc exits0, strict Clippy exits1 for items_after_test_module: new visibility bridge follows the retained old test block.18 phases, source/dependency after guards pass; no arithmetic call. Complete62-member1723108B archive1e51cee6 includes full3rlibs and compiled ELF. Claim309c13, publicf3586783.','infrastructure-first-strict-Clippy-D2-07','compiler passed → strict lint stopped')]
for name,title,outcome,packet,flow in negatives:
 dot='digraph g { rankdir=LR;bgcolor="transparent";node[shape=box,style="rounded,filled",fillcolor="#eef3f8",fontname="DejaVu Sans",fontsize=11]; a[label="Frozen inputs\\nfull source custody"]; b[label="'+flow.replace(' → ','\\n')+'"]; c[label="No divisor arithmetic\\nfull negative packet retained"]; a->b->c; }'
 paras=[outcome+' [pcc:unverified]',
  'The run uses the fixed prospective17-witness/34-pair protocol; no population, sign, timeout or resource acceptance was weakened. This infrastructure outcome is not a mathematical rejection. Full source, exact commands/environment, every recorded output and exit, and all declared dependency bytes are preserved in '+packet+'. Every full archived member byte and mode was compared. [pcc:unverified]',
  'Subsequent versioned repairs preserve each failed source and its outcome before any new execution: canonical diagnostic spelling, exact original embedded CSV, compile-time retained-import uses, and relocation of the complete unchanged adapter. F is the first successful arithmetic source; its result and full packet are reported separately in report-actual-map-10. [pcc:unverified]',
  'Flow: one bounded setup/build attempt was requested and attempted; its recorded gate stopped it before mathematical execution. There are fewer than two comparable scientific measurements, so the figure diagrams the executed phases. No throughput, speedup or mathematical nonexistence conclusion follows. Original full EKS workloads, paidSTOP, vault lock, signed independent verification and complete costs remain open. [pcc:unverified]',
  'Sources checked: PROTOCOL_14.md, PROOF_14.md, exact ConsumerE079 inputs and the retained unit12 source. KB/Conductor unavailable127; documented fallback used without novelty/absence inference. No canonical cost, curve or leaderboard graph changes from this setup result. [pcc:unverified]']
 records.append(render(s/'reports'/name,title,'Preserved setup/build outcome · 17 November 2026',paras,dot))
(s/'REPORT_BUILD_INVENTORY_14.json').write_text(json.dumps(records,indent=2)+'\n')
print(json.dumps(records,indent=2))
