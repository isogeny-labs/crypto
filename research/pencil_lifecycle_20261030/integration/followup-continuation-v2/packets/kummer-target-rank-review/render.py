from pathlib import Path
import subprocess,hashlib,json
p=Path(__file__).parent
(p/'diagram.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="950" height="250"><rect width="950" height="250" fill="white"/><g font-family="sans-serif" fill="#182433"><text x="20" y="35" font-size="22">Signed duplicate review: fixed algebra controls</text><rect x="20" y="60" width="910" height="55" fill="#e8eff5"/><text x="35" y="94" font-size="19">Same canonical divisor + both normalized labels → Δa + Δb k = 0</text><text x="35" y="151" font-size="18">Δb ≠ 0: candidate −Δa/Δb; actual group verification required</text><text x="35" y="188" font-size="18">Δb = 0: valid labels force Δa = 0; redundant equation</text><text x="35" y="230" font-size="17">12 fixed controls reproduced. Source insertion remains a proposal.</text></g></svg>''')
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'figure.pdf'),str(p/'diagram.svg')],check=True)
(p/'report.tex').write_text(r'''\documentclass[10pt]{article}\usepackage[margin=.7in]{geometry}\usepackage{graphicx}\begin{document}\section*{Independent target-rank review}\includegraphics[width=\linewidth]{figure.pdf}
No blocking discrepancy was found in the stated algebra or proposed source placement. All producer hashes matched. One fixed-checker replay returned exit zero and reproduced the producer result byte for byte.

\paragraph{Rank controls} Forest, balanced target-bearing triangle, balanced redundant triangle and unbalanced triangle have left-nullities 0, 1, 1, 0 over F11. The balanced dependence is $(-1,-1,1)$; its target coefficient distinguishes useful and redundant equations. The graph model still requires a justified field-coordinate interpretation.

\paragraph{Source placement} The existing signed-divisor set rejects duplicates before graph insertion at pruned.rs:521. A proposed map can compare the first stored pair with the new sign-normalized pair before rejection. Both labels must receive the sign; first-occurrence graph signs and labels must remain mutually consistent. This is not implemented collector evidence.

\paragraph{Coverage} Four duplicate cases, four invalid-assumption fixtures and four rank cases were checked. The fixed F11 checker is not a generic prime-order or group validator. No naturally generated duplicate, Jacobian verification, source trajectory or historical 114-edge forest was rerun. A planted relabeling would test an interface only.

\paragraph{Artifacts} REVIEW.md provides derivations and precise limitations. status.json and receipt.json bind the replay; the manifest supplies full artifact hashes. No scientific execution or repository edit occurred.
\end{document}''')
r=subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,capture_output=True);(p/'render.stdout').write_bytes(r.stdout);(p/'render.stderr').write_bytes(r.stderr);assert r.returncode==0
subprocess.run(['pdftoppm','-f','1','-singlefile','-scale-to','1000','-png',str(p/'report.pdf'),str(p/'preview')],check=True)
(p/'manifest.json').write_text(json.dumps({'files':{str(q.relative_to(p)):hashlib.sha256(q.read_bytes()).hexdigest() for q in sorted(p.rglob('*')) if q.is_file() and q!=p/'manifest.json'}},indent=2)+'\n');print(hashlib.sha256((p/'manifest.json').read_bytes()).hexdigest())
