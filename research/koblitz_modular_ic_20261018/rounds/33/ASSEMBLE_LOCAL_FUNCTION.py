#!/usr/bin/env python3
"""Lexical renaming only; a native formal-argument roundtrip checks the result."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent;p=h/'runs/local33-compact-dump01'
s=(p/'native.log').read_text();assert (p/'exit.txt').read_text().strip()=='0' and 'LOCAL33_COMPACT_DUMP_ALL_CHECKS_PASS' in s and not re.search(r'^\s*\?',s,re.M)
lines=(p/'rational-family.sing').read_text().splitlines();assert lines[0]=='ring rationalFamily33=(2,u),(n,l,v),dp;'
rename={'u':'uValue33','n':'nValue33','l':'lValue33','v':'vValue33'}
exponent_tokens=re.findall(r'\bu\^([0-9]+)\b','\n'.join(lines))
maximum_token=max(exponent_tokens,key=lambda x:(len(x),x)) if exponent_tokens else '1'
body=[]
for line in lines[1:]:
 cached=re.sub(r'\bu\^([0-9]+)\b',lambda m:'number(uPowerCache33[('+m[1]+')+1])',line)
 body.append(re.sub(r'\b(u|n|l|v)\b',lambda m:rename[m[0]],cached))
 if line.startswith('rationalDenominator33='):
  body.append('if(rationalDenominator33==0){ERROR("LOCAL33_COEFFICIENT_DENOMINATOR_SPECIALIZATION_ZERO");}')
cache='list uPowerCache33;number uPowerRunning33=1;int uPowerIndex33;\n'
cache+='for(uPowerIndex33=0;uPowerIndex33<='+maximum_token+';uPowerIndex33++){uPowerCache33[uPowerIndex33+1]=uPowerRunning33;uPowerRunning33=uPowerRunning33*uValue33;}\n'
func='proc specialiseLocalFamily33(number uValue33,poly nValue33,poly lValue33,poly vValue33){\n'+cache+'\n'.join(body)+'\n'
func+='return(list(rationalSource33,rationalSyzygies33,rationalH33,rationalDenominator33,rationalJacobian33,rationalTate33,rationalBezout33,rationalBase33,rationalOriginalQR33));\n}\n'
(h/'source-data/local-family-function.sing').write_text(func)
rt='\nlist roundtripFamily33=specialiseLocalFamily33(u,n,l,v);\n'
rt+='if(size(roundtripFamily33)!=9){ERROR("LOCAL33_FUNCTION_ARITY");}\n'
for i,name,kind in [(1,'rationalSource33','ideal'),(2,'rationalSyzygies33','matrix'),(3,'rationalH33','poly'),(4,'rationalDenominator33','poly'),(5,'rationalJacobian33','matrix'),(6,'rationalTate33','ideal'),(7,'rationalBezout33','matrix'),(8,'rationalBase33','poly'),(9,'rationalOriginalQR33','ideal')]:
 rt+=f'{kind} roundtripObject{i}33=roundtripFamily33[{i}];\n'
 a=f'roundtripObject{i}33';b=name
 if kind=='ideal':a='matrix('+a+')';b='matrix('+b+')'
 rt+=f'if({a}!={b}){{ERROR("LOCAL33_FUNCTION_ROUNDTRIP_{i}");}}\n'
rt+='print("LOCAL33_FUNCTION_ALL_CHECKS_PASS");quit;\n'
(h/'source-data/local-function-input-template.sing').write_text('\n'.join(lines)+'\n'+func+rt)
print('Assembled lexical function and native roundtrip at every formal argument.')
