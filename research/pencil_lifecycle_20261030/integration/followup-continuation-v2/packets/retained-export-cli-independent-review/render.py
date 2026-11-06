from pathlib import Path
import subprocess
p=Path(__file__).parent
rows=['Retained exporter CLI: prospective recipe accepted','7 source + 888 external inputs + 464 dependency alias joins verified','Strict typed request -> exact request/executable hashes -> unchanged exporter','Eight real CLI controls; prior twelve exporter controls retained','Fresh absolute output | exclusive receipts | errors preserve partial files','120s envelope, 8GiB AS; passed receipt plus exit0 and resolved cleanup','Synthetic software validation; real expected pins require external authentication']
s='<svg xmlns="http://www.w3.org/2000/svg" width="1180" height="370"><rect width="1180" height="370" fill="white"/><g font-family="sans-serif" fill="#172b38">'
for i,t in enumerate(rows):s+=f'<text x="25" y="{40+i*48}" font-size="{24 if i==0 else 18}">{t}</text>'
s+='</g></svg>';(p/'report.svg').write_text(s);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'report.svg')],check=True)
