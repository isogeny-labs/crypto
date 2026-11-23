from pathlib import Path
import json,hashlib,csv,importlib.util,subprocess,base64
p=Path(__file__).resolve().parent
src=p.parent/'runtime-private-supervisor-v2/supervise.py'
assert hashlib.sha256(src.read_bytes()).hexdigest()=='f2c638ed4ec07509a3254632fb7831e009559667b6a82eacb9fb5ea31ac9cb66'
spec=importlib.util.spec_from_file_location('reviewed_source',src);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
b=json.loads((p/'assembly-binding.draft.json').read_text());paths=m.validate_socket_paths(b['inputs']['owner']);rows=[(k,len(v.encode()),107) for k,v in paths.items() if k.startswith('assembly.')]
with (p/'socket-lengths.csv').open('w') as f:
 w=csv.writer(f,lineterminator="\n");w.writerow(['path_kind','derived_bytes','maximum_path_bytes_with_nul_reserved']);w.writerows(rows)
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="360" viewBox="0 0 900 360"><rect width="900" height="360" fill="white"/><g font-family="sans-serif" fill="#20334d"><text x="25" y="32" font-size="22">Corrected runtime socket path lengths</text><text x="25" y="56" font-size="13">Derived from pinned source; assembly and qualification lengths agree</text>']
for i,(name,n,maximum) in enumerate(rows):
 y=90+i*43;svg.append(f'<text x="25" y="{y+17}" font-size="14">{name.split(".")[1]}</text><rect x="200" y="{y}" width="{n*5}" height="26" fill="#3d789a"/><text x="{210+n*5}" y="{y+18}" font-size="14">{n} bytes</text>')
svg.append('<line x1="735" y1="78" x2="735" y2="307" stroke="#b44b37" stroke-dasharray="5 4"/><text x="743" y="95" font-size="13">107-byte maximum</text><text x="743" y="113" font-size="13">plus NUL = 108</text><text x="25" y="337" font-size="13">Source f2c638ed...; private-shim mount behavior remains runtime-unexecuted.</text></g></svg>');(p/'flow.svg').write_text(''.join(svg))
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
tex=r'''\documentclass[10pt]{article}
\usepackage[margin=0.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}
\begin{document}
\section*{Private runtime: exact inputs and prospective admission}
The corrected supervisor has an independent 29-test software replay. The gate dependency now has 45 parser checks, 154 held-release checks and the pristine candidate's expected non-PID1 rejection (exit 126). These are separate software scopes; no successful container or runtime service was executed.
\begin{center}\includegraphics[width=\linewidth]{diagram.pdf}\end{center}
\textbf{Fresh input checks.} Eleven executable hashes, the 549,662,208-byte base archive and all 19 context members match their bindings. The exact new owner root is absent and no matching private units were listed. Generated directives passed the local systemd static unit verifier (exit 0); its unrelated installed acpid.socket warning is retained.

\textbf{Prospective review.} The independent reviewer accepted the technical scope of one 600-second assembly attempt: 2 CPU quota, 2 GiB RAM, zero swap, 256 tasks and 4 GiB retained storage. The decision is conditional, bound to canonical input digest ba15f0b8...; execution allowance remains false. Qualification is a distinct 300-second phase requiring its own image, predecessor and review joins.

\textbf{Remaining operational evidence.} The new planning record provides 25,200 active-worker seconds through 2026-11-04 06:00UTC. Retained spent, remaining and outstanding-credit fields are null; prior C1 worker closure remains unverified. A read through the existing scoped job-submitter context failed before Kubernetes because the host role cannot assume its configured launcher role. No alternate identity or permission change was attempted. Zero active jobs or zero historical consumption is not inferred.

\textbf{Review material.} preflight.json, conditional-review.json, accounting-reconciliation.json, unit-check-receipt.json and socket-lengths.csv preserve inputs, provenance and unresolved fields. Full 1,260-target enrollment and matched-cost obligations remain unchanged.
\end{document}
''';(p/'report.tex').write_text(tex);subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,check=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
raw=(p/'report.log').read_bytes();(p/'report.log.b64').write_bytes(base64.b64encode(raw)+b'\n');(p/'render-log.json').write_text(json.dumps({'source':'report.log','encoding':'base64','bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()},indent=2)+'\n')
