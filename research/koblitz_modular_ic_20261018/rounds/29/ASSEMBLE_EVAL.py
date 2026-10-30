#!/usr/bin/env python3
from pathlib import Path
import hashlib,json
here=Path(__file__).resolve().parent
source=(here/'../28/runs/eval28-01/input.sing').read_text()
inputs=[];functions=[]
for weight in range(3):
 p=here/f'runs/jet29-01/jet29-weight-{weight}.json';inputs.append(p);functions.append(json.loads(p.read_text()))
rows=[]
for i,r in enumerate(functions):rows.append(f'number secondCoefficient{i}=hexField("{r["A_bits"]}")+firstY*hexField("{r["B_bits"]}");\n')
body=''.join(rows)+'''
print("EVAL29_DEGREE12_SECOND_ADJUGATE");
if(derivativeWeighted!=0){ERROR("degree12 firstweighted coefficient changed");}
if(secondCoefficient0==0){
 print("EVAL29_SECOND_DENOMINATOR_ZERO_UNRESOLVED");
}else{
 number pairProduct=secondCoefficient1/secondCoefficient0;
 number pairSum=secondCoefficient2/secondCoefficient0+1+pairProduct;
 poly pairPolynomial=X^2+pairSum*X+pairProduct;
 print("EVAL29_SELECTED_IMAGE_QUADRATIC");pairPolynomial;
 int selectedWeight=0;int selectedNumber=0;int pointWeight;number candidateX;
 poly pairRebuilt=1;
 for(factorIndex=1;factorIndex<=size(imageFactors[1]);factorIndex++){
  if(deg(imageFactors[1][factorIndex])==1){
   candidateX=leadcoef(subst(imageFactors[1][factorIndex],X,0))/leadcoef(imageFactors[1][factorIndex]);
   if(subst(pairPolynomial,X,candidateX)==0){
    pointWeight=1;if(pairSum==0){pointWeight=2;}
    pairRebuilt=pairRebuilt*(X+candidateX)^pointWeight;
    selectedWeight=selectedWeight+pointWeight;selectedNumber=selectedNumber+1;
    if(secondY^2+candidateX*secondY+candidateX^3+candidateX^2+1!=0){ERROR("second-adjugate point off actual curve");}
    print("EVAL29_SELECTED_SECOND_X_AND_WEIGHT");candidateX;pointWeight;
    int orbitDegree29=0;number orbitX29=candidateX;number orbitY29=secondY;int orbitStep29;
    for(orbitStep29=1;orbitStep29<=12;orbitStep29++){
     orbitX29=orbitX29^2;orbitY29=orbitY29^2;
     if(orbitX29==candidateX && orbitY29==secondY){orbitDegree29=orbitStep29;break;}
    }
    if(orbitDegree29==0 || 12%orbitDegree29!=0){ERROR("degree12 second-image orbit mismatch");}
    print("EVAL29_SELECTED_SECOND_DEGREE");orbitDegree29;
    link pairOut29="ASCII:w paired-degree12-"+string(selectedNumber)+".sing";open(pairOut29);
    write(pairOut29,"ring degree12Image29=(2,beta),(X),lp;");write(pairOut29,"minpoly="+string(minpoly)+";");
    write(pairOut29,"number firstX="+string(firstX)+";");write(pairOut29,"number firstY="+string(firstY)+";");
    write(pairOut29,"number secondX="+string(candidateX)+";");write(pairOut29,"number secondY="+string(secondY)+";");
    write(pairOut29,"int sourceImageMultiplicity="+string(pointWeight)+";");write(pairOut29,"int secondImageDegree="+string(orbitDegree29)+";");close(pairOut29);
   }
  }
 }
 if(selectedWeight!=2 || pairRebuilt!=pairPolynomial){ERROR("second-adjugate pair multiset not reconstructed by actual cubic roots");}
 print("EVAL29_TOTAL_SELECTED_MULTIPLICITY");selectedNumber;selectedWeight;
}
'''
start=source.index('EVAL28_ADJUGATE_CASE 1 function=1')
needle=' print("EVAL28_ZERO_DENOMINATOR_UNRESOLVED");derivativeWeighted;'
idx=source.index(needle,start)
source=source[:idx]+body+source[idx+len(needle):]
source=source.replace('print("EVAL28_ALL_CHECKS_PASS");','print("EVAL29_ALL_CHECKS_PASS");')
(here/'source-data').mkdir(exist_ok=True)
(here/'source-data/eval-input-template.sing').write_text(source)
(here/'eval-inputs.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(here))+'\n'for p in inputs)+hashlib.sha256((here/'../28/runs/eval28-01/input.sing').read_bytes()).hexdigest()+'  ../28/runs/eval28-01/input.sing\n')
