#!/usr/bin/env python3
from pathlib import Path
import hashlib
here=Path(__file__).resolve().parent
files=['runs/action27-01/basis-jets389.sing','runs/mapbuild27-02/map-basis-coefficients.sing','runs/mapbuild27-02/map-equations.sing']
with (here/'source-data/mapsolve-input-template.sing').open('w')as f:
 f.write('int mcChecks=0;int mcFailures=0;int mcAllowed=0;\n')
 f.write('ring mapValidation27=(2,beta32),(w),lp;minpoly=beta32^5+beta32^2+1;\n')
 for name in files[:2]:f.write((here/name).read_text())
 f.write((here/'MAPCHECK27-prefix.sing').read_text())
 f.write((here/files[2]).read_text())
 sample1=['0']*102;sample1[0]='1'
 sample2=['0']*102;sample2[1]='beta32';sample2[36]='1';sample2[68]='1';sample2[69]='1/beta32'
 f.write('map mcSample1=mapTarget27,'+','.join(sample1)+';\n')
 f.write('map mcSample2=mapTarget27,'+','.join(sample2)+';\n')
 f.write('ideal mcValues1=mcSample1(mapEquations);ideal mcValues2=mcSample2(mapEquations);\n')
 f.write('setring mapValidation27;ideal mcEvaluated1=imap(mapTarget27,mcValues1);ideal mcEvaluated2=imap(mapTarget27,mcValues2);\n')
 f.write((here/'MAPCHECK27-suffix.sing').read_text())
 f.write((here/'MAPSOLVE27.sing').read_text())
(here/'mapsolve-inputs.sha256').write_text(''.join(hashlib.sha256((here/name).read_bytes()).hexdigest()+'  '+name+'\n' for name in files))
