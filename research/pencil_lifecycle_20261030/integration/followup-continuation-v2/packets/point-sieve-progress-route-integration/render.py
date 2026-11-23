from pathlib import Path
from html import escape
import subprocess
p=Path(__file__).parent
rows=[('Diagnostic progress route now uses finite differences',25),('Some(sink) -> complete progress module -> exact reviewed recurrence',18),('Canonical source 87f3581c... | all other bytes and diagnostic sites unchanged',18),('6398 ordered-result pairs + 20207 sequences + 32 boundary transitions',18),('Actual counters: delta A=3p+2-kp; delta M=-kp; delta visits=-k(p-1)',18),('Real helper: 3 exact events, 8 milestones, payload, disabled sink and Noop',18),('Attempt1: wrapper ENOENT, 20.784s; preserved, no tests, group reaped',18),('Reviewed successor: passed 33.860s; exit0, unchanged inputs, empty group',18),('Actual module integration; complete root adapter and final binary binds remain',18)]
s='<svg xmlns="http://www.w3.org/2000/svg" width="1240" height="480"><rect width="1240" height="480" fill="white"/><g font-family="sans-serif" fill="#172b38">'
for i,(t,size) in enumerate(rows):s+=f'<text x="25" y="{42+i*49}" font-size="{size}">{escape(t)}</text>'
s+='</g></svg>';(p/'report.svg').write_text(s);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'report.svg')],check=True,timeout=30)
