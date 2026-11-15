# Presentation only: render values already produced by frozen native analysis.
from pathlib import Path
import re,json,sys,textwrap
import typst
root=Path(__file__).resolve().parent
study=Path(sys.argv[2]).resolve() if len(sys.argv)>2 else Path(__file__).resolve().parents[2]
raw=study/'results-v1';report=study/'reports'/sys.argv[1];report.mkdir(parents=True,exist_ok=False)
summary=(raw/'summary.txt').read_text();p=json.loads((study/'protocol.json').read_text())
values={};seed=None
for line in summary.splitlines():
 if line.startswith('seed='):
  seed=line.split()[0].split('=')[1];values[seed]={'totals':line}
 elif line.startswith('  '):
  name=line.split(':')[0].strip();values[seed][name]=dict(re.findall(r'(\w+)=([0-9.]+)',line))
rows=[]
for name,label in [('load_convert','Cache load + input conversion'),('subtract','Affine subtraction'),('normalize','Batch normalization (inverse tree)'),('affine_out','Affine output + conversion'),('lookup','Negative filter + index lookup'),('empty','Empty barrier / clock control')]:
 cells=[label]+[f"{float(values[s][name]['share_including_empty'])*100:.2f}%" for s in ['101','202']]
 rows.append(', '.join('['+c+']' for c in cells)+',')
source='''#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false)
#text(size: 20pt, weight: "bold")[GPU relation-query cost breakdown]

#text(fill: rgb("555555"))[RTX 4090 · fixed 63-bit inputs · 31 October 2026]

This diagnostic replays the first eight actual queries from each of the two
large affine inputs, preserving the original bases, full right cache, filter,
and index. Five interleaved rounds yield 30 replay cells and 240 queries.
Every response matched the uninstrumented capture byte for byte.

#image("stages.svg", width: 100%)

#table(columns: (2fr, 1fr, 1fr), inset: 5pt,
  [*Instrumented block-cycle share*], [*Seed 101*], [*Seed 202*],
'''+ '\n'.join(rows)+'''
)

*Interpretation.* These are sums of thread-zero block-cycle intervals, including
barriers, scheduling, and compiler fences. They are not GPU wall-time shares.
The empty interval is shown without subtraction. Input conversion includes the
cache load; output conversion includes affine reconstruction; lookup includes
the negative filter. No fraction is applied to the historical oracle seconds.

*Overhead and controls.* CUDA events bracket the original or staged search
kernel only. Setup, host launch gaps, and winner resolution are excluded.
Raw per-round A/A spreads and the staged/stock event ratio are reproduced on
the following page. Shared-host clocks remain L0 diagnostics.

#pagebreak()
#text(size: 17pt, weight: "bold")[Verification and provenance]

'''
source += '#raw('+json.dumps("\n".join(textwrap.fill(line,width=94,subsequent_indent="    ") for line in summary.splitlines()))+', block: true)\n'
source += '''
*Validation.* All 30 cells / 240 queries passed exact reply, table identity,
exit and coverage checks. All 37,584 arithmetic cases passed on host and GPU;
315 differential query cases included 261 hits and 54 exhausted misses.
Memcheck, racecheck and synccheck reported zero errors; 29 Rust tests and
Clippy passed. Native analysis recomputed byte-identically; canonical export
and coverage/file checks reported zero findings. The owned pod was deleted.

*Profiler failure.* All five optional external profiler invocations exited 2
before collection because the image lacked gawk. No Nsight counters or timeline
values were obtained in v1. The separate version-2 protocol repairs prerequisites
and attempts only these external profiles; it does not replace this panel.

*Perturbation.* Staged/stock kernel-event ratios were 0.9334 and 0.9366.
The diagnostic kernel specializes the full-cache path and changes code generation
as well as adding instrumentation: 58 registers / 56-byte stack versus stock
48 registers / 120-byte stack; neither reports spills. These ratios do not isolate
clock overhead or establish a production improvement. The next comparison needs
an uninstrumented full-cache specialization on matched complete workloads.

*Scope.* This is a fixed query-prefix diagnostic, not another complete recovery
panel. Instrumentation changes synchronization and register use. The preserved
ptxas logs and any supported Nsight counters help assess that perturbation.
The source and all data remain reproducible from the linked study artifacts.

'''
source += '#text(size: 8pt)[Source: '+p['freeze']['commit']+'\nProtocol: '+p['freeze']['protocol_sha256']+'\nStudy: research/explicit_relations_profile_20261031\nRaw data: results-v1/export.json and archive-manifest.json]\n'
(report/'report.typ').write_text(source)
(report/'stages.svg').write_bytes((raw/'stages.svg').read_bytes())
(report/'render-report.py').write_bytes(Path(__file__).read_bytes())
typst.compile(str(report/'report.typ'),output=str(report/'report.pdf'))
print(report)
