#!/usr/bin/env python3
"""Text concatenation only; all arithmetic and coefficient clearing is native."""
from pathlib import Path
h=Path(__file__).resolve().parent
s=(h/'source-data/basis-functions.sing').read_text()+'\n'+(h/'DESC33-FUNCTIONS.sing').read_text()
s+='\nring twistLinear33=(2,tau),(w),dp;minpoly=tau^2+tau+1;\n'
s+=(h/'source-data/sigma-matrix.sing').read_text()+'\n'+(h/'source-data/basis-jets389.sing').read_text()+'\n'+(h/'DESC33-LINEAR.sing').read_text()
(h/'source-data/desc-input-template.sing').write_text(s)
print('Assembled explicit function descent and complete product-space input.')
