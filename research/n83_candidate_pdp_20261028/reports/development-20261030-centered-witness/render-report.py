from pathlib import Path
import json,shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst,fitz
s=Path('/home/ec2-user/crypto-n83-degree-sweep-20261027/research/n83_candidate_pdp_20261028');e=s/'validation-centered-witness';j=json.loads((e/'summary.json').read_text());o=s/'reports/development-20261030-centered-witness';o.mkdir(exist_ok=True)
groups=[]
for n in [3,5]:
 for centered in [True,False]:
  rows=[r for r in j['rows'] if r['ell']==n and r['centered_target']==centered];degrees=sorted(set(r['remainder_degree'] for r in rows));assert len(degrees)==1
  groups.append({'ell':n,'centered':centered,'count':len(rows),'degree':degrees[0]})
fig,ax=plt.subplots(figsize=(8,2.4));ax.bar(range(4),[g['degree'] for g in groups],color=['#2a9d8f','#447aa5']*2);ax.set(xticks=range(4),xticklabels=[f"width {g['ell']} / {'centered' if g['centered'] else 'ordinary'}\n{g['count']} retained cases" for g in groups],yticks=range(6),ylim=(0,5.7),ylabel='Remainder total degree',title='Exact small development fixtures: centered target lowers remainder degree')
fig.tight_layout();fig.savefig(o/'degrees.svg');plt.close(fig)
f=o/'degrees.svg';f.write_text('\n'.join(x.rstrip() for x in f.read_text().splitlines())+'\n')
(o/'plot-data.json').write_text(json.dumps({'rows':j['rows'],'plotted_groups':groups,'aggregation':'All cases within each group have identical recorded degree. No uncertainty interval for exact arithmetic.'},indent=2)+'\n')
c=json.loads((e/'custody.json').read_text())
(o/'report.typ').write_text(f'''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set heading(numbering: none)
= Native centered terminal witness
Development validation · T-1002 · 30 October 2026

The Rust instrument implements centered selector normal forms, the alternating projection, a canonical polynomial codec and exact byte replay. It retains all three coefficient polynomials of the terminal remainder in unrestricted U. The accepted executable passed eight development tests and replayed all 24 retained small fixtures, including after S3 restoration.

== Construction and checks
For odd width ell=2s+1, center y_j=z_j−w_j/2, so repeated variables reduce through y_j²=(w_j/2)². Starting from P, the product of paired differences, compute P, NF(XP), NF(X²P) and the complete terminal remainder. The alternating projection selects monomials with exactly one selector from each pair and retains both P and y_last P coordinates. Both resulting U-polynomials are checked against the symbolic formula.

The test inventory checks 168 complete squarefree basis monomials by literal antisymmetrization; 108 bounded low-degree rows for annihilation; 32 basis products against independent general quotient multiplication; and 1,440 direct selector-point evaluations of F=P·S. The latter cover primes 101/103, widths 3/5, two nonzero weight choices and targets c,c+1,0, where c=sum(w)/2. These are exact fixed-loop coverage counts. The n83 modulus separately passes an exact Lucas certificate with witness 3.

#image("degrees.svg", width: 100%)
#text(size: 8pt)[Recorded degrees from 24 retained development fixtures. Within each plotted group every degree agrees. These are remainder degrees, not regularity measurements; no statistical interval applies.]

== Preserved artifacts and failure controls
Each fixture retains input.json, A0/A1/A2, F0/F1/F2, a certificate and a payload manifest. The codec uses N83CEN1 followed by newline, then width, degree, term count, modulus and sorted mask/coefficient pairs. It is distinct from the historical idempotent codec. The generator requires a fresh output directory. Tests reject invalid parameters, malformed bytes and a changed coefficient; command-line replay of the retained mutant exits 101 as expected.

Builds had a 180-second, 2 GiB address-space cap; final serial tests had a 120-second, 512 MiB cap. Final tests: 8 passed, 0 failed. Test-log elapsed time is diagnostic only. Initial build logs, earlier fixtures and the later complete fixture set remain archived. The test command requires a fresh persistent N83_CENTERED_TEST_OUTPUT directory. Source and lockfile, final generator and test executable, all raw coefficients and logs are retained.

== Scope and next production step
This validates the native construction on exhaustive small development fixtures. The 83-bit width-27 production panel has not run. Next bind the terminal generator to the retained full per-m sources, freeze a production protocol and EKS image, then include centered and adjacent targets alongside the saved ordinary cases. The reviewed mathematical filtered-space argument remains a separate dependency of degree-of-fall conclusions. Arithmetic records keep exact LFD and regularity null. Existing fixed factor bases are unchanged.

== Custody
S3 raw archive: {c['archive_bytes']} bytes; 276 restored payload hashes verified; all 24 cases replayed from the restored native executable and restored inputs. SHA256:
#text(size: 7pt)[{c['archive_sha256']}]
See validation-centered-witness/summary.json, custody.json, FORMAT.json, CENTERED_WITNESS_DESIGN.md and CENTERED_WITNESS_FORMAT.json. Archive prefix: runs/n83-centered-witness-development-20261030 under setec-astronomy-artifacts.
''')
typst.compile(str(o/'report.typ'),output=str(o/'report.pdf'));d=fitz.open(o/'report.pdf');print('pages',len(d))
for i,page in enumerate(d):page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save('/home/ec2-user/n83-candidate-recovery/centered-development-report-'+str(i+1)+'.png')
shutil.copy2(__file__,o/'render-report.py')
