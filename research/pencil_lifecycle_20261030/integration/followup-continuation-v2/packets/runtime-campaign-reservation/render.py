from pathlib import Path
import json,csv,subprocess,base64,hashlib
p=Path(__file__).resolve().parent
x=json.loads((p/'reservation.draft.json').read_text()); total=x['total_authorized_active_worker_seconds']; debit=x['new_reservation']['conservative_worker_seconds_debit']; balance=x['worker_seconds_after_proposed_reservation'];assert total-debit==balance
with (p/'allocation.csv').open('w') as f:
 w=csv.writer(f,lineterminator='\n');w.writerow(['category','worker_seconds','status']);w.writerows([['assembly_conservative_debit',debit,'proposed'],['unreserved_allowance',balance,'derived_proposed_balance']])
svg=f'''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="230"><rect width="900" height="230" fill="white"/><g font-family="sans-serif" fill="#20334d"><text x="25" y="35" font-size="22">Renewed campaign: proposed worker allowance allocation</text><text x="25" y="63" font-size="15">25,200 seconds authorized; deadline November 4, 2026, 06:00 UTC</text><rect x="25" y="90" width="40" height="42" fill="#c47d32"/><rect x="65" y="90" width="800" height="42" fill="#4c819b"/><text x="25" y="160" font-size="15">Orange: 1,200-second conservative debit for one 600-second assembly</text><text x="25" y="185" font-size="15">Blue: 24,000-second derived balance after proposed reservation</text><text x="25" y="213" font-size="13">Planning allocation, not measured use. Dollar allocation remains conditional.</text></g></svg>'''
(p/'allocation.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'allocation.svg')],check=True,timeout=30)
tex=r'''\documentclass[10pt]{article}
\usepackage[margin=0.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}
\begin{document}\section*{Renewed pencil campaign: accounting and reservation}
The explicitly renewed allowance is 25,200 active-worker seconds from November 3 at 17:39:17 UTC through November 4 at 06:00 UTC. Historical records remain unchanged. This packet records a proposed reservation; execution is not admitted.
\begin{center}\includegraphics[width=\linewidth]{diagram.pdf}\end{center}
\textbf{Worker accounting.} Scoped live observations and team launch attestations support zero post-renewal runtime/cohort workers before this reservation. Fourteen pencil state objects retain October 30 modification dates; the current overview contains no pencil Job; the identified C1 controller is complete and its instance query returns no instance. These are scoped observations, not an account-wide usage audit.

\textbf{One proposed attempt.} Maximum 600 seconds, 2 CPU quota (1,200 CPU-seconds), 2 GiB RAM, 4 GiB retained storage, one attempt. The optional stricter policy debits 1,200 worker-seconds, leaving 24,000. Storage remains reserved through qualification or reviewed disposition. Independent review accepts the scoped worker accounting.

\textbf{Separate software overhead.} Known command ceilings and partial elapsed observations are retained in JSON. Rendering, hashing, transport and some wrappers were not globally metered; total software overhead is unknown, not zero. Independent mathematical proof work is not silently reclassified as matched-cohort worker occupancy.

\textbf{Dollar observations.} The larger daily reading is USD 131.13 against USD 200, updated November 3 at 17:40 UTC. Monthly spend is USD 320.369 against USD 1,000, updated at 11:06 UTC. Daily thresholds are nonadditive. These delayed observations leave a shared unreported-spend margin and finite host/storage allocation to reconcile before final build admission. A regional instance inventory is retained with its exact scope.

\textbf{Reproducibility.} reservation.draft.json binds the planning record and assembly inputs. budget-observation.json preserves parsed observations; independent-review.json retains the review. allocation.csv and this renderer reproduce the planning graph. This packet contains no new cohort measurement or runtime execution.
\end{document}'''
(p/'report.tex').write_text(tex)
r=subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,capture_output=True,timeout=30);(p/'render.stdout').write_bytes(r.stdout);r.check_returncode()
raw=(p/'report.log').read_bytes();(p/'report.log.b64').write_bytes(base64.b64encode(raw)+b'\n');(p/'render-log.json').write_text(json.dumps({'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()},indent=2)+'\n')
