#!/usr/bin/env python3
"""Text assembly only. No finite-field or other mathematical computation."""
from pathlib import Path
import hashlib, json
h=Path(__file__).resolve().parent
original=(h.parent/'27/LAMBDA14.sing').read_text().splitlines(True)[:254]
procs=(h/'COLOR31.sing').read_text().split('list sourceTriple30=')[0]
for d in (83,284):
    lines=(h/f'runs/color{d}-01/native.log').read_text().splitlines()
    marker=lines.index('COLOR30_MIDDLE_CANDIDATE 1')
    assert sum(x.startswith('COLOR30_MIDDLE_CANDIDATE ') for x in lines)==1
    kernels=[i for i,x in enumerate(lines) if x=='COLOR30_KERNEL_POLYNOMIAL']
    assert len(kernels)==2 and 'COLOR31_ALL_CHECKS_PASS' in lines
    paired=(h/f'source-data/paired-{d}.sing').read_text().replace('(X),lp;','(X,Z,T),lp;')
    # Retain the literal phi polynomial procedure from the accepted input.
    accepted=(h/f'source-data/color-{d}-input-template.sing').read_text()
    phi=accepted[accepted.index('proc phi30('):accepted.index('// Actual selected pair')]
    out=''.join(original)+'\n'+paired+f'\nint sourceDegree30={d};\n'+phi+procs
    out+=f'\nnumber midX31={lines[marker+1]};\nnumber midY31={lines[marker+2]};\n'
    out+=f'poly secondKernel31={lines[kernels[1]+1]};\n'
    out+=(h/'REVERSE31.sing').read_text()
    (h/f'source-data/reverse-{d}-input-template.sing').write_text(out)
print('Assembled literal accepted COLOR31 point/kernel coefficients and original source circuit.')
