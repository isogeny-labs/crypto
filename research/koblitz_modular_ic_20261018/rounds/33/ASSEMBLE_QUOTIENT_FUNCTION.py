#!/usr/bin/env python3
"""Lexical specialization with native formal-argument roundtrip."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent
for attempt,marker in [('local33-quotient01','LOCAL33_QUOTIENT_ALL_CHECKS_PASS'),('quotient33-denominators01','QUOTIENT33_DENOMINATORS_ALL_CHECKS_PASS')]:
 p=h/'runs'/attempt;s=(p/'native.log').read_text();assert (p/'exit.txt').read_text().strip()=='0' and marker in s and not re.search(r'^\s*\?',s,re.M)
qlines=(h/'runs/local33-quotient01/direct-quotients.sing').read_text().splitlines()
small=(h/'runs/local33-compact-dump01/rational-family.sing').read_text().splitlines()
small=[s for s in small if s.startswith(('rationalSource33[','rationalJacobian33[','rationalH33=','rationalDenominator33='))]
qselected=[s for s in qlines if re.match(r'quotientGenerators[1-4]33\[',s)]
dlines=(h/'runs/quotient33-denominators01/quotient-denominators.sing').read_text().splitlines()
dselected=[s for s in dlines if s.startswith('quotientDenominators33[')]
header='ideal rationalSource33;matrix rationalJacobian33[4][3];poly rationalH33;poly rationalDenominator33;list quotientDenominators33;\n'
header+=''.join(f'ideal quotientGenerators{i}33;\n' for i in range(1,5))
rename={'u':'uQ33','n':'nQ33','l':'lQ33','v':'vQ33'}
body=dselected+small+qselected
exps=re.findall(r'\bu\^([0-9]+)\b','\n'.join(body));maximum=max(exps,key=lambda x:(len(x),x)) if exps else '1'
func='proc quotientFamily33(number uQ33,poly nQ33,poly lQ33,poly vQ33){\n'+header
func+='list uCacheQ33;number uRunningQ33=1;int uIndexQ33;for(uIndexQ33=0;uIndexQ33<='+maximum+';uIndexQ33++){uCacheQ33[uIndexQ33+1]=uRunningQ33;uRunningQ33=uRunningQ33*uQ33;}\n'
for line in body:
 cached=re.sub(r'\bu\^([0-9]+)\b',lambda m:'number(uCacheQ33[('+m[1]+')+1])',line)
 func+=re.sub(r'\b(u|n|l|v)\b',lambda m:rename[m[0]],cached)+'\n'
 if line.startswith('rationalDenominator33='):
  func+='if(rationalDenominator33==0){ERROR("QUOTIENT33_ORIGINAL_COEFFICIENT_DOMAIN");}\n'
 if line.startswith('quotientDenominators33['):
  label=line.split('=')[0];func+=f'if(number({label})==0){{ERROR("QUOTIENT33_SPECIALIZATION_DENOMINATOR");}}\n'
func+='return(list(rationalSource33,rationalJacobian33,rationalH33,rationalDenominator33,list(quotientGenerators133,quotientGenerators233,quotientGenerators333,quotientGenerators433),quotientDenominators33));}\n'
(h/'source-data/quotient-family-function.sing').write_text(func)
s=qlines[0]+'\n'+header+'\n'.join(body)+'\n'+func+'\nlist qRT33=quotientFamily33(u,n,l,v);\n'
s+='ideal jRT33=qRT33[1];matrix jacRT33=qRT33[2];poly hRT33=qRT33[3];poly dRT33=qRT33[4];list gsRT33=qRT33[5];list dsRT33=qRT33[6];ideal oneGRT33;ideal origGRT33;int iRT33;\n'
s+='if(matrix(jRT33)!=matrix(rationalSource33) || jacRT33!=rationalJacobian33 || hRT33!=rationalH33 || dRT33!=rationalDenominator33){ERROR("QUOTIENT33_SMALL_ROUNDTRIP");}\n'
s+='for(iRT33=1;iRT33<=4;iRT33++){oneGRT33=gsRT33[iRT33];execute("origGRT33=quotientGenerators"+string(iRT33)+"33;");if(matrix(oneGRT33)!=matrix(origGRT33)){ERROR("QUOTIENT33_GENERATOR_ROUNDTRIP");}}\n'
s+='if(size(dsRT33)!=size(quotientDenominators33)){ERROR("QUOTIENT33_DENOMINATOR_COUNT");}for(iRT33=1;iRT33<=size(dsRT33);iRT33++){if(dsRT33[iRT33]!=quotientDenominators33[iRT33]){ERROR("QUOTIENT33_DENOMINATOR_ROUNDTRIP");}}\n'
s+='print("QUOTIENT33_FUNCTION_ALL_CHECKS_PASS");quit;\n'
(h/'source-data/quotient-function-input-template.sing').write_text(s)
print('Assembled all quotient coefficients with checked coefficient domains and native roundtrip.')
