#!/usr/bin/env node
'use strict';
// Independently digit-search local torsion; no division polynomial or Newton step.
const p=29n;
function chk(condition,why){if(!condition)throw new Error(why)}
function mod(n,m){let r=n%m;return r<0n?r+m:r}
function inverse(a,m){let [oldr,r]=[m,mod(a,m)], [oldt,t]=[0n,1n];while(r!==0n){const q=oldr/r;[oldr,r]=[r,oldr-q*r];[oldt,t]=[t,oldt-q*t]}chk(oldr===1n,'nonunit');return mod(oldt,m)}
function f(x,y){return y*y+y-(x*x*x-x*x-10n*x-20n)}
function valid(x,y,m){return mod(f(x,y),m)===0n}
function twice(x,y,m){const lam=mod((3n*x*x-2n*x-10n)*inverse(2n*y+1n,m),m);
  const xx=mod(lam*lam+1n-2n*x,m);
  return [xx,mod(-lam*xx-y+lam*x-1n,m)];}
function isThree(x,y,m){if(!valid(x,y,m))return false;
  const [xx,yy]=twice(x,y,m);return xx===x && yy===mod(-y-1n,m);}
function yDigit(x,y,place){const numerator=f(x,y);chk(numerator%place===0n,'bad y seed');
  const v=mod(-(numerator/place)*inverse(2n*y+1n,p),p);return y+place*v;}
function main(){let x=18n,y=5n,wrong=5n,place=1n;
  chk(isThree(x,y,p),'source generator wrong');
  for(let n=1;n<=6;n++){
    if(n>1){const m=place*p;const candidates=[];
      for(let a=0n;a<p;a++){
        const candidate=x+place*a;
        const ordinate=yDigit(candidate,y,place);
        if(isThree(candidate,ordinate,m))candidates.push([candidate,ordinate]);
      }
      chk(candidates.length===1,'source lift not uniquely 3 torsion');
      [x,y]=candidates[0];
      wrong=yDigit(18n,wrong,place);
      place=m;
    }else{place=p;}
    const neg=mod(-y-1n,place);
    chk(isThree(x,y,place) && isThree(x,neg,place),'order3');
    chk(mod(x,p)===18n && mod(y,p)===5n && mod(neg,p)===23n,'reduction');
    chk(valid(18n,wrong,place),'wrong point is off curve');
    const badOrder=isThree(18n,wrong,place);
    chk(badOrder===(n===1),'wrong control');
    const result={N:n,modulus:place.toString(),negative_curve_ok:true,negative_order3:badOrder,
      point_3:[x.toString(),y.toString()],point_3_neg:[x.toString(),neg.toString()],
      point_lift_3:true,reduction_ok:true};
    process.stdout.write(JSON.stringify(result)+'\n');
  }
}
main();
