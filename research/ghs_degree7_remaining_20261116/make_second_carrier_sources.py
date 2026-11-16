#!/usr/bin/env python3
"""Copy the five frozen search arms to the second trace +22 carrier."""
from pathlib import Path
import re

HERE=Path(__file__).resolve().parent
OLD='x^11+x^9+x^8+2*x^7+2*x^6+2*x^4+x^3+x^2+2*x'
NEW='x^11+x^9+2*x^8+x^7+x^5+x^4+x^3+2*x'
SOURCES={
 'sparse_trace22.sage':'sparse_35349.sage',
 'linear_square_trace22.sage':'linear_square_35349.sage',
 'cubic_square.sage':'cubic_square_35349.sage',
 'two_poles_linear.sage':'two_poles_linear_35349.sage',
 'three_poles.sage':'three_poles_35349.sage',
}
for original,copy in SOURCES.items():
 text=(HERE/original).read_text()
 lines=text.splitlines(keepends=True)
 matches=[i for i,line in enumerate(lines) if re.match(r'^\s*f\s*=',line)]
 assert len(matches)==1,original
 i=matches[0]
 lhs,rhs=lines[i].split('=',1)
 assert rhs.strip().replace(' ','')==OLD,original
 lead=rhs[:len(rhs)-len(rhs.lstrip())]
 lines[i]=lhs+'='+lead+NEW+'\n'
 output=''.join(lines)
 path=HERE/copy
 if path.exists():
  assert path.read_text()==output,copy
 else:
  path.write_text(output)
 print(copy)
