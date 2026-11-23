from pathlib import Path
import csv,json,hashlib,subprocess,tempfile
P=Path(__file__).resolve().parent
source=P.parent/'reconciliation.json'
j=json.loads(source.read_text())
with (P/'source-comparison.csv').open('w',newline='') as f:
 w=csv.writer(f,lineterminator='\n');w.writerow(['path','prior_sha256','current_sha256','matches','prior_snapshot_hash_valid'])
 for x in j['source_comparison']:w.writerow([x[k] for k in ['path','prior_sha256','current_sha256','matches','prior_snapshot_hash_valid']])
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="380" viewBox="0 0 900 380"><rect width="900" height="380" fill="white"/><g font-family="sans-serif" fill="#172638"><text x="35" y="44" font-size="25">Source bindings compared with historical review</text><text x="35" y="76" font-size="15">Complete inventory: 17 files | count of files, not a performance measure</text><text x="35" y="143" font-size="18">Unchanged</text><text x="35" y="213" font-size="18">Changed</text><rect x="200" y="112" width="500" height="45" fill="#256c89"/><rect x="200" y="182" width="350" height="45" fill="#bd783d"/><text x="714" y="141" font-size="22">10</text><text x="564" y="211" font-size="22">7</text><line x1="200" y1="250" x2="750" y2="250" stroke="#172638"/><text x="195" y="276" font-size="15">0</text><text x="445" y="276" font-size="15">5</text><text x="690" y="276" font-size="15">10</text><text x="410" y="305" font-size="16">Number of source files</text><text x="35" y="345" font-size="14">Source: reconciliation.json; T-1503; historical nine-category review preserved.</text><text x="35" y="367" font-size="14">Hash equality is source identity; changed files require scoped delta review.</text></g></svg>'''
assert j['identical_files']==10 and j['changed_files']==7
(P/'source-bindings.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(P/'source-bindings.pdf'),str(P/'source-bindings.svg')],check=True)
subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','-output-directory',str(P),str(P/'report.tex')],cwd=P,check=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=60)
for suffix in ['aux','log','out']:
 f=P/('report.'+suffix)
 if f.exists():f.unlink()
(P/'render-input.json').write_text(json.dumps({'source':'../reconciliation.json','sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'scope':'render existing fixed source inventory; no pricing or tests'},indent=2)+'\n')
