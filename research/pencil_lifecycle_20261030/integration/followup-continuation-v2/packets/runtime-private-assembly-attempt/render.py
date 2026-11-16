from pathlib import Path
import subprocess,json,hashlib,base64
p=Path(__file__).resolve().parent
x=json.loads((p/'result.json').read_text())
svg='<svg xmlns="http://www.w3.org/2000/svg" width="900" height="180"><rect width="900" height="180" fill="white"/><g font-family="sans-serif" fill="#20334d"><text x="20" y="30" font-size="21">One private assembly: retained stage outcomes</text><rect x="20" y="55" width="250" height="60" fill="#dfede6"/><rect x="325" y="55" width="250" height="60" fill="#f4ded7"/><rect x="630" y="55" width="250" height="60" fill="#e1e8ef"/><text x="40" y="80" font-size="15">Worker: image produced</text><text x="40" y="101" font-size="14">private mounts removed</text><text x="280" y="90" font-size="20">→</text><text x="345" y="80" font-size="15">Launcher: parse failure</text><text x="345" y="101" font-size="14">original result unresolved</text><text x="585" y="90" font-size="20">→</text><text x="650" y="80" font-size="15">Read-only reconciliation</text><text x="650" y="101" font-size="14">stopped state observed</text><text x="20" y="152" font-size="14">One attempt:33.701950s launcher wall;9.246s serviceCPU; no scientific targets</text></g></svg>'
(p/'outcomes.svg').write_text(svg);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'outcomes.svg')],check=True,timeout=30)
tex=r'''\documentclass[10pt]{article}
\usepackage[margin=0.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}
\begin{document}\section*{One bounded private runtime assembly}
The worker completed its COPY-only assembly and retained the resulting image and disk. The launcher then failed while parsing its loop-device cleanup query. This report preserves the original unresolved disposition and the separate observations used to investigate it.
\begin{center}\includegraphics[width=\linewidth]{diagram.pdf}\end{center}
\textbf{Exact attempt.} Published revision2b037c3096; owner2d885f49-d570-4a8b-990e-7ea14ee9ea80. One600-second allowance,2CPU quota,2GiB RAM and4GiB disk. The command exited1 after33.701950079seconds. Systemd journal reports9.246CPU-seconds. These are separate clocks, not a speed comparison.

\textbf{Produced artifacts.} The worker receipt identifies image sha256:3a2bb98af4743c88... and disk digest 2a47f831c97c9aab... (full hashes in native-worker-result.json). It records private mounts removed and endpoint not live. Native receipt bytes and completed service-state snapshot remain preserved.

\textbf{Failure and observation.} The original losetup JSON query lacked explicit list mode and returned exit0 with empty output on this host. JSON parsing failed. A separate read-only query with explicit list mode returns a valid empty loopdevices list. The service is now not-found, inactive/dead, with empty ControlGroup. No build was repeated and no qualification or scientific target ran. The independently reviewed read-only checker returned assembled-stopped-observed with two matching stopped/empty observations. Disk custody and native receipt bytes were unchanged; the worker disk digest is inherited, not freshly rehashed. The original receipt remains unchanged.

\textbf{Accounting and custody.} Retain the conservative1200-worker-second debit and USD1 attempt allocation. The separate USD55 shared-cost margin remains recorded. The4GiB backing file stays at its native path under the allocation through November4 at06:00UTC; its hash, inode, device and timestamps are in the native worker receipt. No implicit deletion is authorized.

\textbf{Review artifacts.} publication.json, launch-start.json, launch-completion.json, native receipts, journal.stdout and read-only-observations.json bind the result. result.json is the standardized summary. The diagram is used because there is one attempt and no comparative sample.
\end{document}'''
(p/'report.tex').write_text(tex);r=subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,capture_output=True,timeout=30);(p/'render.stdout').write_bytes(r.stdout);r.check_returncode();raw=(p/'report.log').read_bytes();(p/'report.log.b64').write_bytes(base64.b64encode(raw)+b'\n');(p/'render-log.json').write_text(json.dumps({'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()},indent=2)+'\n')
