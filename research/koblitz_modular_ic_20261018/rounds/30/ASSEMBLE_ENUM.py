#!/usr/bin/env python3
"""Assemble separate native completeness checks, without replaying isogeny norms."""
from pathlib import Path
here=Path(__file__).resolve().parent
chunks=[]
for degree,expected in [(12,2),(436,1)]:
 source=(here/f'source-data/color-{degree}-input-template.sing').read_text()
 prefix=source[:source.index('list sourceTriple30=triple30(firstX,firstY);')]
 old=' return(roots30);'
 new=' print("ENUM30_ROOTS degree="+string(deg(ff))+" rational="+string(size(roots30)));\n if(deg(ff)!=size(roots30)){ERROR("ENUM30_NONRATIONAL_OR_NONSEPARABLE_ROOT_GAP");}\n return(roots30);'
 assert old in prefix;prefix=prefix.replace(old,new,1)
 main='''
list sourceTriple30=triple30(firstX,firstY);list targetTriple30=triple30(secondX,secondY);
list common030=common30(sourceTriple30[1],targetTriple30[1]);
list common730=common30(sourceTriple30[2],targetTriple30[2]);
list common4930=common30(sourceTriple30[3],targetTriple30[3]);
int iEnum30;int jEnum30;int kEnum30;int tEnum30;int uEnum30;int enumCount30;int squareEnum30;
for(iEnum30=1;iEnum30<=size(common030);iEnum30++){
 for(jEnum30=1;jEnum30<=size(common730);jEnum30++){
  list t0Enum30=parameters30(common030[iEnum30],common730[jEnum30]);
  for(kEnum30=1;kEnum30<=size(common4930);kEnum30++){
   list t7Enum30=parameters30(common730[jEnum30],common4930[kEnum30]);
   for(tEnum30=1;tEnum30<=size(t0Enum30);tEnum30++){for(uEnum30=1;uEnum30<=size(t7Enum30);uEnum30++){
    number middleXEnum30=t0Enum30[tEnum30]*t7Enum30[uEnum30];
    for(squareEnum30=0;squareEnum30<sourceDegree30-2;squareEnum30++){middleXEnum30=middleXEnum30^2;}
    if(middleXEnum30^2+middleXEnum30+1==0){ERROR("ENUM30_EXCEPTIONAL_PARAMETER_BRANCH");}
    number middleYEnum30=(t7Enum30[uEnum30]+middleXEnum30+1)/(middleXEnum30^2+middleXEnum30+1);
    list tripleEnum30=triple30(middleXEnum30,middleYEnum30);
    if(tripleEnum30[1]!=common030[iEnum30] || tripleEnum30[2]!=common730[jEnum30] || tripleEnum30[3]!=common4930[kEnum30]){ERROR("ENUM30_ACTUAL_TRIPLE_MISMATCH");}
    enumCount30++;
   }}
  }
 }
}
'''
 chunks.append(prefix+main+f'if(enumCount30!={expected}){{ERROR("ENUM30_PRIOR_CANDIDATE_COUNT_MISMATCH");}}\nprint("ENUM30_COMPLETE_COMPATIBLE_MIDDLES degree={degree} count="+string(enumCount30));\n')
(here/'source-data/enum-input-template.sing').write_text('\n'.join(chunks)+'\nprint("ENUM30_ALL_CHECKS_PASS");quit;\n')
print('Assembled strict native degree/rational-root exhaustion checks for both retained pairs.')
