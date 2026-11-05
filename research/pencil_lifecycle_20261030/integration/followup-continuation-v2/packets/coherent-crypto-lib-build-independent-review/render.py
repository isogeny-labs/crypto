from pathlib import Path
import subprocess
p=Path(__file__).parent
rows=["Prospective coherent crypto library build review", "Verified custody: 1103 source + 506 dependency + 145 compiler/runtime files", "Exact freeze -> real inherited pipe -> one owned rustc process group", "300s outer envelope | 8GiB AS | one jobserver-managed implicit slot", "4GiB sampled output threshold: overshoot possible; retained evidence", "Success requires exit0 + passed receipt + unchanged inputs + confirmed reap", "Source review only: no compiler, exporter test or scientific run by reviewer"]
s='<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="370"><rect width="1100" height="370" fill="white"/><g font-family="sans-serif" fill="#172b38">'
for i,t in enumerate(rows):s+=f'<text x="25" y="{40+i*48}" font-size="{24 if i==0 else 18}">{t.replace("&","&amp;").replace("<","&lt;")}</text>'
s+='</g></svg>';(p/'graph.svg').write_text(s)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'graph.svg')],check=True)
