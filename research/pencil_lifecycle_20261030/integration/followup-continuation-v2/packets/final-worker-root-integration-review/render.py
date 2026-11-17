from pathlib import Path
from html import escape
import subprocess
p=Path(__file__).parent
rows=["Final worker build plan: accepted as preparation only","Worker build -> executable bytes; root libtest -> executed test evidence","Root path: cryptanalysis::ecbench::three_leads::pencil::tests","Existing 3 guards: fake dispatch / shared meter / unwind / ledger","Full cargo test --release --lib remains unfiltered and separately required","Old 16GiB / 600s timeout occurred during compilation: zero tests","Refresh final source/cache pins; enforce aggregate resources before release","Actual Some/None positive fixture is separate source preparation (T1566)"]
s='<svg xmlns="http://www.w3.org/2000/svg" width="1240" height="430"><rect width="1240" height="430" fill="white"/><g font-family="sans-serif" fill="#172b38">'
for i,t in enumerate(rows):s+=f'<text x="25" y="{40+i*49}" font-size="{24 if i==0 else 18}">{escape(t)}</text>'
s+='</g></svg>';(p/'report.svg').write_text(s);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'report.svg')],check=True,timeout=30)
