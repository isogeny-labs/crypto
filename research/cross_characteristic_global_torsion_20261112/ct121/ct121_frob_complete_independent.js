#!/usr/bin/env node
// CT121 independent spectral projector versus full matrix-product verifier.
'use strict';
const mod=(x,p)=>((x%p)+p)%p;
function primes(n){let a=[];for(let p=3;p<=n;p+=2){let ok=true;for(let j=3;j*j<=p;j+=2)if(p%j===0){ok=false;break;}if(ok)a.push(p);}return a;}
function power(a,n,p){let z=1;while(n){if(n%2)z=mod(z*a,p);a=mod(a*a,p);n=Math.floor(n/2);}return z;}
function inv(a,p){if(!mod(a,p))throw Error('division by zero');return power(mod(a,p),p-2,p)}
function ord(a,p){let n=1,z=mod(a,p);if(!z)throw Error('not a unit');while(z!==1){z=mod(z*a,p);n++;if(n>p)throw Error('bad order');}return n;}
function gcd(a,b){while(b){[a,b]=[b,a%b]}return a;}
function spectral(v,p){if(v[0]===0&&v[1]===0)throw Error('zero');if(p%4===3){const h=ord(-4,p);return [4*h,h%2===0?2*h:4*h,'inert'];}
  let i;for(i=1;i<p;i++)if(mod(i*i,p)===p-1)break;
  if(i===p)throw Error('missing square root');
  const la=mod(-1+i,p),lb=mod(-1-i,p);
  // P_a = (A-lb I)/(la-lb), A=[[0,-2],[1,-2]].
  const xa=mod((-lb*v[0]-2*v[1])*inv(la-lb,p),p);
  const ya=mod((v[0]+(-2-lb)*v[1])*inv(la-lb,p),p);
  const xb=mod(v[0]-xa,p),yb=mod(v[1]-ya,p);
  const hasA=(xa!==0||ya!==0),hasB=(xb!==0||yb!==0);
  if(!hasA&&!hasB)throw Error('zero projection');
  const a=ord(la,p),b=ord(lb,p);
  const d=hasA&&hasB?a/gcd(a,b)*b:hasA?a:b;
  const minus=d%2===0&&(!hasA||power(la,d/2,p)===p-1)&&(!hasB||power(lb,d/2,p)===p-1);
  return [d,minus?d/2:d,hasA&&hasB?'mixed':'eigen'];
}
function mm(A,B,p){return [mod(A[0]*B[0]+A[1]*B[2],p),mod(A[0]*B[1]+A[1]*B[3],p),mod(A[2]*B[0]+A[3]*B[2],p),mod(A[2]*B[1]+A[3]*B[3],p)];}
function observed(v,p){const A=[0,p-2,1,p-2];let M=[1,0,0,1];let xdeg=0;
  for(let n=1;n<p*p;n++){M=mm(A,M,p);const w=[mod(M[0]*v[0]+M[1]*v[1],p),mod(M[2]*v[0]+M[3]*v[1],p)];
    const pos=w[0]===v[0]&&w[1]===v[1];const neg=w[0]===mod(-v[0],p)&&w[1]===mod(-v[1],p);
    if(!xdeg&&(pos||neg))xdeg=n;
    if(pos)return [n,xdeg];
  }
  throw Error('no period');
}
function eigenVectors(p){if(p%4===3)return [];let i=1;while(mod(i*i,p)!==p-1)i++;
 const l=[mod(-1+i,p),mod(-1-i,p)];return l.map(x=>[mod(-2*inv(x,p),p),1]);}
function record(p){const vectors=[[1,0],[0,1],[1,1],...eigenVectors(p)];let cases=[];
 for(const v of vectors){const got=spectral(v,p),ref=observed(v,p);if(got[0]!==ref[0]||got[1]!==ref[1])throw Error('mismatch '+p+':'+v);cases.push({vector:v,point_degree:got[0],x_degree:got[1],class:got[2]});}
 let counts={};if([5,13,17,29].includes(p)){
 for(let x=0;x<p;x++)for(let y=0;y<p;y++)if(x||y){const got=spectral([x,y],p);const ref=observed([x,y],p);if(got[0]!==ref[0]||got[1]!==ref[1])throw Error('exhaustive');const k=got[2]+':'+got[0]+':'+got[1];counts[k]=(counts[k]||0)+1;}
 const sum=Object.values(counts).reduce((a,b)=>a+b,0);const ce=Object.entries(counts).filter(([k])=>k.startsWith('eigen:')).reduce((a,[_,v])=>a+v,0);
 if(sum!==p*p-1||ce!==2*(p-1))throw Error('count');
 }
 counts=Object.fromEntries(Object.entries(counts).sort(([a],[b])=>a.localeCompare(b,'en')));
 return {ell:p,split:p%4===1,cases,full_counts:counts};
}
const rows=primes(211).map(record);
if(rows.length!==46)throw Error('prime coverage');
const samples=[[3,8,4],[5,4,4],[11,40,20],[13,12,12],[17,16,8],[31,40,20],[47,184,92],[211,840,420]];
const actual=rows.filter(r=>samples.some(s=>s[0]===r.ell)).map(r=>[r.ell,r.cases[0].point_degree,r.cases[0].x_degree]);
if(JSON.stringify(actual)!==JSON.stringify(samples))throw Error('fixture mismatch '+JSON.stringify(actual));
if(JSON.stringify(spectral(eigenVectors(5)[0],5).slice(0,2))!==JSON.stringify([1,1]))throw Error('split mutation');
try{spectral([0,0],5);throw Error('zero accepted')}catch(e){if(!String(e).includes('zero'))throw e;}
process.stdout.write(JSON.stringify({curve:'y^2+y=x^3+x^2 over F2',tested_primes:rows.length,rows})+'\n');
