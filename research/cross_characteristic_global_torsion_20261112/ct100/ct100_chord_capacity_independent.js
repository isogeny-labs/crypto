#!/usr/bin/env node
"use strict";
// CT100 independent integer finite-field verifier; no imported CT99 code.
// Recomputes group orders by enumerating both coordinates and dlogs by exhaustive search.
const samples = [[5,7,10,[5,1]],[7,37,35,[9,4]],[11,59,55,[46,28]],
                 [13,139,130,[81,134]],[17,811,850,[27,85]],
                 [19,193,190,[84,73]]];
const check=(v,s)=>{if(!v)throw Error(s)};
const mod=(x,p)=>((x%p)+p)%p;
const rhs=(x,p)=>mod(x*x*x-x*x-10*x-20,p);
function inverse(x,p) {
 let [a,b]=[mod(x,p),p], [s,t]=[1,0];
 while(b!==0) {
   const q=Math.floor(a/b);
   [a,b]=[b,a-q*b];[s,t]=[t,s-q*t];
 }
 check(a===1,'no inverse');return mod(s,p);
}
function opposite(P,p){return P===null?null:[P[0],mod(-1-P[1],p)]}
function sum(P,Q,p) {
 if(P===null)return Q;
 if(Q===null)return P;
 const [x,y]=P,[u,v]=Q;
 if(x===u&&mod(y+v+1,p)===0)return null;
 let num,den;
 if(x===u&&y===v){num=3*x*x-2*x-10;den=2*y+1}
 else {num=v-y;den=u-x}
 const slope=mod(num*inverse(den,p),p);
 const x3=mod(slope*slope+1-x-u,p);
 const y3=mod(-slope*(x3-x)-y-1,p);
 return [x3,y3];
}
function eq(P,Q){return JSON.stringify(P)===JSON.stringify(Q)}
function slowTimes(P,k,p){let V=null;for(let i=0;i<k;i++)V=sum(V,P,p);return V}
function bruteLog(P,Q,ell,p){let W=null;for(let k=0;k<ell;k++){if(eq(W,Q))return k;W=sum(W,P,p)}throw Error('log not found')}
function countPoints(p){let N=1;for(let x=0;x<p;x++)for(let y=0;y<p;y++)if(mod(y*y+y-rhs(x,p),p)===0)N++;return N}
function run([ell,p,n,P]){
 check(p>3&&p!==11&&p!==ell&&countPoints(p)===n,'wrong independent point count');
 check(mod(P[1]*P[1]+P[1]-rhs(P[0],p),p)===0,'seed not on curve');
 check(slowTimes(P,ell,p)===null,'seed does not have order ell');
 const multiples=[];let Q=null;
 for(let k=1;k<ell;k++){
   Q=sum(Q,P,p);check(Q!==null,'premature zero');multiples.push(Q);
   check(eq(opposite(Q,p),slowTimes(P,ell-k,p)),'inverse check');
   check(bruteLog(P,Q,ell,p)===k,'independent public dlog');
 }
 check(sum(Q,P,p)===null,'final element is not inverse');
 const distinctXs=new Set(multiples.map(u=>u[0]));
 const m=(ell-1)/2;check(distinctXs.size===m,'wrong x class count');
 const P2=multiples[1];
 check(P2[0]!==P[0],'vertical chord');
 const lambda=mod((P2[1]-P[1])*inverse(P2[0]-P[0],p),p);
 const nu=mod(P[1]-lambda*P[0],p);
 const hits=[];
 for(let k=1;k<ell;k++){
   const [x,y]=multiples[k-1];
   if(mod(y-lambda*x-nu,p)===0)hits.push(k);
 }
 const expected=[...new Set([1,2,ell-3])].sort((a,b)=>a-b);
 check(JSON.stringify(hits)===JSON.stringify(expected),'line orientation mismatch');
 const covered=new Set(hits.map(k=>multiples[k-1][0])).size;
 check(covered===Math.min(m,3),'one-line capacity exceeded');
 check(mod(nu+1-lambda*P[0]-P[1],p)!==0,'changed intercept control failed');
 return {bsgs_cases:ell-1,chord_intercept:nu,chord_oriented_indices:hits,
         chord_slope:lambda,chord_x_classes:covered,ell,
         fixed_basepoint_line_lower_bound:Math.floor(m/2),
         general_line_lower_bound:Math.ceil(m/3),group_order:n,
         missing_x_classes:m-covered,prime:p,x_classes:m};
}
const rows=samples.map(run);
check(rows[1].chord_x_classes===rows[1].x_classes&&rows.slice(2).every(x=>x.missing_x_classes>0),
      'degree 3 must saturate only ell<=7');
const answer={experiment:'CT100-chord-incidence-capacity',fixtures:rows,
 global_bound:'single affine line meets a nonsingular plane cubic in <=3 distinct x-values',
 pcc:'unverified',source_bsgs_verified:true};
const canonical=obj=>Array.isArray(obj)?'['+obj.map(canonical).join(',')+']':
  obj!==null&&typeof obj==='object'?'{'+Object.keys(obj).sort().map(k=>JSON.stringify(k)+':'+canonical(obj[k])).join(',')+'}':JSON.stringify(obj);
process.stdout.write(canonical(answer)+'\n');
