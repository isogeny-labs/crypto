#!/usr/bin/env python3
"""Bind original source supports and exact adjugate functions, no Python math."""
from pathlib import Path
import json,hashlib
here=Path(__file__).resolve().parent
original=here/'../27/runs/direct27-02/input.sing'
chunks=original.read_text().split('kill hexField;\n')
assert len(chunks)==5
out=here/'source-data';out.mkdir(exist_ok=True)
inputs=[original]
with (out/'eval-input-template.sing').open('w')as f:
 for case,kind in enumerate([1,1,2,2]):
  one=here/f'runs/dual28-02/dual28-function-{kind}-one.json'
  weighted=here/f'runs/dual28-02/dual28-function-{kind}-xinf.json'
  inputs.extend([one,weighted]);a=json.loads(one.read_text());b=json.loads(weighted.read_text())
  f.write(chunks[case])
  f.write(f'''number derivativeOne=hexField("{a['A_bits']}")+firstY*hexField("{a['B_bits']}");
number derivativeWeighted=hexField("{b['A_bits']}")+firstY*hexField("{b['B_bits']}");
print("EVAL28_ADJUGATE_CASE {case} function={kind}");
if(derivativeOne==0){{
 print("EVAL28_ZERO_DENOMINATOR_UNRESOLVED");derivativeWeighted;
}}else{{
 number selectedX=derivativeWeighted/derivativeOne;
 if(selectedX^3+selectedX^2+secondY*selectedX+1+secondY^2!=0){{ERROR("adjugate ratio not on actual target cubic");}}
 if(secondY^2+selectedX*secondY+selectedX^3+selectedX^2+1!=0){{ERROR("adjugate ratio not on E49");}}
 int rootMatches=0;
 for(factorIndex=1;factorIndex<=size(imageFactors[1]);factorIndex++){{
  if(deg(imageFactors[1][factorIndex])==1 && subst(imageFactors[1][factorIndex],X,selectedX)==0){{rootMatches=rootMatches+1;}}
 }}
 if(rootMatches!=1){{ERROR("adjugate ratio did not select exactly one rational root");}}
 print("EVAL28_ACTUAL_SELECTED_SECOND_X");selectedX;
 print("EVAL28_ACTUAL_SECOND_Y");secondY;
 int originalDegree={ [436,12,284,83][case] };int orbitDegree=0;number orbitX=selectedX;number orbitY=secondY;
 int orbitStep;
 for(orbitStep=1;orbitStep<=originalDegree;orbitStep++){{
  orbitX=orbitX^2;orbitY=orbitY^2;
  if(orbitX==selectedX && orbitY==secondY){{orbitDegree=orbitStep;break;}}
 }}
 if(orbitDegree==0 || originalDegree%orbitDegree!=0){{ERROR("selected image Frobenius orbit mismatch");}}
 print("EVAL28_SOURCE_RESIDUE_AND_SECOND_IMAGE_DEGREES");originalDegree;orbitDegree;originalDegree div orbitDegree;
 link pairOut="ASCII:w paired-image-{case}.sing";open(pairOut);
 write(pairOut,"ring pairedImage{case}=(2,beta),(X),lp;");
 write(pairOut,"minpoly="+string(minpoly)+";");
 write(pairOut,"number firstX="+string(firstX)+";");write(pairOut,"number firstY="+string(firstY)+";");
 write(pairOut,"number secondX="+string(selectedX)+";");write(pairOut,"number secondY="+string(secondY)+";");
 write(pairOut,"int sourceResidueDegree="+string(originalDegree)+";");
 write(pairOut,"int secondImageDegree="+string(orbitDegree)+";");close(pairOut);
}}
kill hexField;
''')
 f.write('print("EVAL28_SCOPE paired Hecke images only; scalarC5 source lifts and full reverse signed divisors OPEN");\nprint("EVAL28_ALL_CHECKS_PASS");quit;\n')
(here/'eval-inputs.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(here))+'\n'for p in inputs if p.is_relative_to(here)))
# Original input is outside here; retain its stable relative label separately.
with (here/'eval-inputs.sha256').open('a')as f:f.write(hashlib.sha256(original.read_bytes()).hexdigest()+'  ../27/runs/direct27-02/input.sing\n')
