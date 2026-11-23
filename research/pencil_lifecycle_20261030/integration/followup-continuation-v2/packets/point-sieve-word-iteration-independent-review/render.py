from pathlib import Path
import subprocess
p=Path(__file__).parent
rows=['WordIteration: fresh composition review passed','Canonical word state -> decoded ring state -> ordered observations','21 named readbacks | 4 own controls | false update rejected','28797 dependency/toolchain entries unchanged | 59.989s ordinary check','Reviewer authored inherited Difference, not new WordIteration','Pure dd recomputation gives values, not source counter equivalence','Rust slicing, iterator/compiler and concrete branch proofs remain separate']
s='<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="370"><rect width="1120" height="370" fill="white"/><g font-family="sans-serif" fill="#172b38">'
for i,t in enumerate(rows):s+=f'<text x="25" y="{40+i*48}" font-size="{24 if i==0 else 18}">{t}</text>'
s+='</g></svg>';(p/'report.svg').write_text(s);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'report.svg')],check=True)
