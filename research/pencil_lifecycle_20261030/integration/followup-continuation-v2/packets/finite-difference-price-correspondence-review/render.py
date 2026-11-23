from pathlib import Path
import subprocess
p=Path(__file__).parent
rows=['Finite-difference event-policy correspondence','Exact inventory and seven source/policy hashes verified','Baseline: k*p adds, multiplies, reads; k = 2d+1','Candidate: 3p+2 adds; zero evaluation multiplies; k reads','Optional access and scalar moves: consistent finite-policy interpretation','Nonblind review: reviewer authored recurrence; root policy acceptance needed','No numerical price, tests, compilation or scientific admission']
s='<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="370"><rect width="1120" height="370" fill="white"/><g font-family="sans-serif" fill="#172b38">'
for i,t in enumerate(rows):s+=f'<text x="25" y="{40+i*48}" font-size="{24 if i==0 else 18}">{t}</text>'
s+='</g></svg>';(p/'report.svg').write_text(s);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'report.svg')],check=True)
