#!/usr/bin/env python3
"""Adapt the checked lexical assembler to three candidate families only."""
from pathlib import Path
h=Path(__file__).resolve().parent
old=(h.parent/'33/ASSEMBLE_QUOTIENT_FUNCTION.py').read_text()
start=old.index('for attempt,marker in ');end=old.index('qlines=',start)
old=old[:start]+'''for attempt,marker in [('prefix34-verify01','PREFIX34_DENOMINATORS_ALL_CHECKS_PASS')]:
 p=h/'runs'/attempt;s=(p/'native.log').read_text();assert (p/'exit.txt').read_text().strip()=='0' and marker in s and not re.search(r'^\\s*\\?',s,re.M)
'''+old[end:]
old=old.replace("runs/local33-quotient01/direct-quotients.sing","source-data/direct-quotients.sing").replace("runs/local33-compact-dump01/rational-family.sing","source-data/rational-family.sing").replace("runs/quotient33-denominators01/quotient-denominators.sing","runs/prefix34-verify01/quotient-denominators.sing")
old=old.replace('range(1,5)','range(1,4)').replace('quotientGenerators333,quotientGenerators433','quotientGenerators333').replace('iRT33<=4','iRT33<=3').replace('QUOTIENT33_FUNCTION_ALL_CHECKS_PASS','PREFIX34_FUNCTION_ALL_CHECKS_PASS')
# Record the complete domain before evaluating any fraction; no field math here.
old=old.replace("for line in body:\n", "for line in body:\n if line==small[0]:\n  func+='int domainBad34=0;int domainIndex34;for(domainIndex34=1;domainIndex34<=size(quotientDenominators33);domainIndex34++){if(number(quotientDenominators33[domainIndex34])==0){domainBad34++;}}if(domainBad34>0){return(list(0,quotientDenominators33));}\\n'\n")
a=old.index(" if line.startswith('quotientDenominators33['):");b=old.index("func+='return(",a)
old=old[:a]+old[b:]
# The old D condition remains checked before any original-source coefficient.
old=old.replace('ERROR("QUOTIENT33_ORIGINAL_COEFFICIENT_DOMAIN")','quotientDenominators33[size(quotientDenominators33)+1]=rationalDenominator33;return(list(0,quotientDenominators33))')
# Execute the text assembler under this module path, not Python algebra.
exec(compile(old,str(h/'generated-lexical-assembler.py'),'exec'),{'__file__':str(h/'generated-lexical-assembler.py'),'__name__':'__main__'})
