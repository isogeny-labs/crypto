#!/usr/bin/env python3
"""Literal polynomial recast and source assembly; no arithmetic."""
from pathlib import Path
import hashlib,re
h=Path(__file__).resolve().parent;old=h.parent/'33';p=h/'runs/primitive34-01'
log=(p/'native.log').read_text();assert (p/'exit.txt').read_text().strip()=='0' and 'PRIMITIVE34_ALL_CHECKS_PASS' in log and not re.search(r'^\s*\?',log,re.M)
prim=(p/'primitive-family.sing').read_text().splitlines();s='\n'.join(prim)+'\n'
s+='ring paddingBaseField34=(2,a),(u,T,w),dp;minpoly=a^5+a^2+1;\n'+(h/'source-data/padding-representatives.sing').read_text()+'\n'
s+='list basePointQP34=paddingRepresentatives33[1];number baseUQP34=(number(basePointQP34[1])+number(basePointQP34[2]))/number(basePointQP34[2])^2;number frobUQP34=baseUQP34;poly baseMinimalQP34=1;int iBaseQP34;\n'
s+='for(iBaseQP34=0;iBaseQP34<5;iBaseQP34++){if(iBaseQP34>0 && frobUQP34==baseUQP34){ERROR("QPOLY34_BASE_DEGREE");}baseMinimalQP34=baseMinimalQP34*(u+frobUQP34);frobUQP34=frobUQP34^2;}\n'
s+='if(frobUQP34!=baseUQP34 || deg(baseMinimalQP34)!=5 || subst(baseMinimalQP34,u,baseUQP34)!=0){ERROR("QPOLY34_MINIMAL_POLYNOMIAL");}for(iBaseQP34=1;iBaseQP34<=size(baseMinimalQP34);iBaseQP34++){if(leadcoef(baseMinimalQP34[iBaseQP34])!=1){ERROR("QPOLY34_BASE_COEFFICIENT_NOT_F2");}}short=0;string baseTextQP34=string(baseMinimalQP34);\n'
s+='ring paddingPolynomial34=2,(n,l,v,u),dp;matrix primitiveS34[4][17];ideal localSource33;poly localDenominator33;poly localTateFactor33;\n'
s+='\n'.join(line.replace('rationalSyzygies33[','primitiveS34[') for line in prim if line.startswith('rationalSyzygies33['))+'\n'
name='runs/local33-generate01/local-certificate.sing';manifest={n:d for d,n in (line.split(maxsplit=1) for line in (old/'runs.sha256').read_text().splitlines())};assert hashlib.sha256((old/name).read_bytes()).hexdigest()==manifest[name]
s+='\n'.join(line for line in (old/name).read_text().splitlines() if line.startswith(('localSource33[','localDenominator33=','localTateFactor33=')))+'\n'
s+='execute("poly qBaseQP34="+baseTextQP34+";");print("QPOLY34_BASE_MINIMAL "+string(qBaseQP34));link baseOutQP34="ASCII:w padding-base-minimal.sing";open(baseOutQP34);write(baseOutQP34,"poly paddingBaseMinimal34="+string(qBaseQP34)+";");close(baseOutQP34);ideal qIdealQP34=std(ideal(qBaseQP34));matrix reducedSQP34=reduce(primitiveS34,qIdealQP34);matrix badReductionQP34=reducedSQP34;badReductionQP34[1,1]=badReductionQP34[1,1]+1;matrix zeroReductionQP34[4][17];\n'
s+='if(reduce(primitiveS34-reducedSQP34,qIdealQP34)!=zeroReductionQP34 || reduce(primitiveS34-badReductionQP34,qIdealQP34)==zeroReductionQP34){ERROR("QPOLY34_REDUCTION_CONTROL");}\n'
s+='matrix originalJacQP34[4][3];int rowPrepareQP34;int colPrepareQP34;for(rowPrepareQP34=1;rowPrepareQP34<=4;rowPrepareQP34++){for(colPrepareQP34=1;colPrepareQP34<=3;colPrepareQP34++){originalJacQP34[rowPrepareQP34,colPrepareQP34]=diff(localSource33[rowPrepareQP34],var(colPrepareQP34));}}\n'
s+='matrix reducedJacQP34=reduce(originalJacQP34,qIdealQP34);ideal reducedSourceQP34=reduce(localSource33,qIdealQP34);poly reducedDQP34=reduce(localDenominator33,qIdealQP34);poly reducedHQP34=reduce(localTateFactor33,qIdealQP34);\n'
s+='setring primitiveFamily34;map recastMap34=paddingPolynomial34,n,l,v,u;matrix recastS34=recastMap34(primitiveS34);if(recastS34!=rationalSyzygies33){ERROR("QPOLY34_POLYNOMIAL_RECAST");}\n'
s+=(h/'QPOLY34.sing').read_text();(h/'source-data/qpoly-input-template.sing').write_text(s)
print('Assembled exact padding-base reduction and all68 source-bound products.')
