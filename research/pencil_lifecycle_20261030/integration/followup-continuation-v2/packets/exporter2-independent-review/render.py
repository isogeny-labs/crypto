from pathlib import Path
import subprocess
p=Path(__file__).parent
rows=['Exporter2 prospective source review','5 source + 888 external entries independently hash-verified','Original validators -> synthetic fixture -> exact 12 named controls','Corrected private fixture calls; all test bodies preserved','120s envelope | 8GiB AS | compiler jobserver + one-thread tests','Passed receipt AND exit0 AND binary/input joins AND resolved cleanup','No reviewer compiler/test invocation; root release remains separate']
s='<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="370"><rect width="1100" height="370" fill="white"/><g font-family="sans-serif" fill="#172b38">'
for i,t in enumerate(rows):s+=f'<text x="25" y="{40+i*48}" font-size="{24 if i==0 else 18}">{t}</text>'
s+='</g></svg>';(p/'report.svg').write_text(s);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'report.svg')],check=True)
