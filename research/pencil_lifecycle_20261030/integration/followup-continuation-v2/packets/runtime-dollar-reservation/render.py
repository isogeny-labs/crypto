from pathlib import Path
import json, subprocess, base64, hashlib
from decimal import Decimal
p=Path(__file__).resolve().parent
x=json.loads((p/'allocation.json').read_text())
assert Decimal(x['observed_daily_headroom_usd'])-Decimal(x['total_reserved_usd'])==Decimal(x['estimated_unreserved_headroom_usd'])
svg='<svg xmlns="http://www.w3.org/2000/svg" width="900" height="225"><rect width="900" height="225" fill="white"/><g font-family="sans-serif" fill="#20334d"><text x="25" y="35" font-size="22">Finite allocation from observed dollar headroom</text><text x="25" y="62" font-size="14">USD68.87 delayed observed daily headroom; thresholds nonadditive</text><rect x="25" y="87" width="660" height="40" fill="#547c9a"/><rect x="685" y="87" width="12" height="40" fill="#c47d32"/><rect x="697" y="87" width="154" height="40" fill="#d1d9dd"/><text x="25" y="156" font-size="15">Blue: USD55 shared footprint/delay reserve; orange: USD1 attempt</text><text x="25" y="181" font-size="15">Grey: USD12.87 estimated remainder; no guarantee of real-time spending</text><text x="25" y="208" font-size="13">Reserve through 2026-11-04 06:00UTC. Exact observations and assumptions retained.</text></g></svg>'
(p/'allocation.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'allocation.svg')],check=True,timeout=30)
tex=r'''\documentclass[10pt]{article}
\usepackage[margin=0.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}
\begin{document}\section*{Finite private assembly and shared-cost reservation}
The coordinator records USD1 for one bounded existing-host assembly and its storage share, plus USD55 for observed standing resources and delayed spending. The daily reported headroom is USD68.87; reserving USD56 leaves an estimated USD12.87. These are conservative accounting estimates.
\begin{center}\includegraphics[width=\linewidth]{diagram.pdf}\end{center}
\textbf{Attempt allocation.} The existing m8i.8xlarge Spot request cap is USD1.69344/hour. Charging its entire host for 600 seconds gives USD0.28224. Allocating 4GiB of existing gp3 storage through the deadline adds approximately USD0.005478. No new EC2 or EBS resource is created. The allocation is USD1.

\textbf{Shared footprint.} The retained observations identify three instances, 49gp3 volumes (4666GiB), 60standard snapshots (6794GiB full source-size upper estimate), one EKS cluster and one NAT gateway. Conservative hourly charges total USD3.411890. Over the interval from the delayed budget observation through 06:00UTC, this is USD42.050365; the USD55 reserve includes USD12.949635 extra margin.

\textbf{Assumptions and conditions.} The margin estimates variable or omitted S3, transfer, NAT traffic, IPv4, other-region and delayed charges; these are not assumed absent. No additional paid launch is included. Reconcile any known conflicting commitment before launch. Retention beyond the deadline needs a continued allocation or reviewed disposition, preserving failure evidence.

\textbf{Independent admission.} The final non-author review approves one 600-second private assembly at 2CPU, 2GiB RAM and 4GiB disk. It binds the exact source, allocation and worker ledger. Qualification and scientific execution remain separate. Inputs, command and review must be published before launch; timeout or unresolved cleanup does not permit an automatic retry.

\textbf{Evidence.} cost-review.json and observations.tar.gz preserve exact read-only query receipts, pricing and arithmetic. allocation.json records coordinator reservations; assembly-review.json and assembly-binding.json supply the exact admission. This is a cost reservation, not a scientific comparison.
\end{document}'''
(p/'report.tex').write_text(tex)
r=subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,capture_output=True,timeout=30);(p/'render.stdout').write_bytes(r.stdout);r.check_returncode()
raw=(p/'report.log').read_bytes();(p/'report.log.b64').write_bytes(base64.b64encode(raw)+b'\n');(p/'render-log.json').write_text(json.dumps({'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()},indent=2)+'\n')
