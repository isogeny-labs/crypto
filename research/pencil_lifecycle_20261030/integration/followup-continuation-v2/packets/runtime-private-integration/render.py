from pathlib import Path
import subprocess
p=Path(__file__).resolve().parent
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="390" viewBox="0 0 1000 390"><rect width="1000" height="390" fill="#fafbfc"/><g font-family="sans-serif" fill="#172b4d"><text x="30" y="38" font-size="24">Private runtime: reviewed source, execution pending</text><g fill="#e7f0fa" stroke="#35659b"><rect x="30" y="70" width="285" height="180" rx="10"/><rect x="357" y="70" width="285" height="180" rx="10"/><rect x="685" y="70" width="285" height="180" rx="10"/></g><text x="50" y="104" font-size="19">1. Private assembly</text><text x="50" y="136" font-size="16">600 s proposed phase ceiling</text><text x="50" y="164" font-size="16">Pinned base + COPY recipe</text><text x="50" y="192" font-size="16">21 software controls pass</text><text x="50" y="220" font-size="16">No daemon/build executed</text><text x="377" y="104" font-size="19">2. Stopped storage custody</text><text x="377" y="136" font-size="16">Owned disk + exact digest</text><text x="377" y="164" font-size="16">Empty predecessor cgroup</text><text x="377" y="192" font-size="16">Loop/mount reconciliation</text><text x="377" y="220" font-size="16">Actual readback pending</text><text x="705" y="104" font-size="19">3. Private qualification</text><text x="705" y="136" font-size="16">300 s proposed phase ceiling</text><text x="705" y="164" font-size="16">Held gate + cgroup ancestry</text><text x="705" y="192" font-size="16">14 software controls pass</text><text x="705" y="220" font-size="16">Native gate checks pending</text><path d="M320 158h30m-8 -7l8 7 -8 7 M647 158h30m-8 -7l8 7 -8 7" stroke="#35659b" fill="none" stroke-width="3"/><text x="30" y="290" font-size="18">Proposed per-service envelope: 2 CPUs | 2 GiB RAM | 256 tasks | 4 GiB storage</text><text x="30" y="325" font-size="17">35 non-author replayed mock/static/temporary-file checks. Not runtime qualification.</text><text x="30" y="357" font-size="16">Inputs, source hashes, corrected findings and remaining obligations: independent-review.json</text></g></svg>'
(p/'flow.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'diagram.pdf'),str(p/'flow.svg')],check=True)
tex=r"""\documentclass[10pt]{article}
\usepackage[margin=0.65in]{geometry}
\usepackage{graphicx}
\usepackage[T1]{fontenc}
\begin{document}
\section*{Private runtime source integration}
The separate private assembly and qualification sources now have matching interfaces and an independent non-author source review. All 21 supervisor and 14 qualifier software controls passed. The reviewer participated in advisory design but authored neither implementation.
\begin{center}\includegraphics[width=\linewidth]{diagram.pdf}\end{center}
\textbf{Corrections checked.} Exact context and token modes despite umask 0077; narrowly permitted predecessor-disk access; complete four-file callback snapshot; actual phase expiry; daemon readiness; managed child discovery; retained terminal unit state and explicit stop reconciliation. The original qualifier packet remains preserved alongside its corrected version.

\textbf{Gate check.} GCC 11.5.0 accepted the exact C source with -Wall -Wextra -Werror -fsyntax-only. No executable was produced by this check. Source requires a complete exact nonce and host-side container cgroup checks before payload release.

\textbf{Evidence boundary.} These are mock/static and temporary-file tests, not actual systemd, mount, Docker or native PID1-gate observations. Both phase drafts remain unadmitted. Native gate controls, concrete runtime admission and actual resource/cleanup readback remain required. Unknown surviving processes or mounts prohibit automatic retry.

\textbf{Review artifacts.} independent-review.json binds eight exact sources and records all 35 checks and limitations. PLAN.md, syntax-check-plan.json and raw syntax exit/output records preserve this integration check. Full matched-cohort and mathematical obligations remain unchanged.
\end{document}
"""
(p/'report.tex').write_text(tex)
subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,check=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
