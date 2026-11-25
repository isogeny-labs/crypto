// Pure integer and finite-matrix combinatorics; no elliptic arithmetic.
function assert(x,m){if(!x)throw Error(m)}
const gcd=(a,b)=>b?gcd(b,a%b):a;
const rows=[];
for(let B=1;B<=6;B++){
 let labels=Array.from({length:2*B},(_,i)=>{let a=Array(B).fill(0);a[i>>1]=i%2?-1:1;return a});
 let vecs=new Map();
 for(let i=0;i<labels.length;i++)for(let j=i;j<labels.length;j++)for(let k=j;k<labels.length;k++)for(let l=k;l<labels.length;l++){
  let v=labels[i].map((x,t)=>x+labels[j][t]+labels[k][t]+labels[l][t]);vecs.set(v.join(","),v)
 }
 let counts={0:0,2:0,4:0},classes=new Map();
 for(let v of vecs.values()){
  let norm=v.reduce((s,x)=>s+Math.abs(x),0);counts[norm]++;
  if(!norm)continue;
  let g=v.reduce((s,x)=>gcd(s,Math.abs(x)),0);let sign=v.find(x=>x)>0?1:-1;
  let key=v.map(x=>x/g*sign).join(",");classes.set(key,(classes.get(key)||0)+1);
 }
 let c2=[...classes.values()].filter(x=>x===2).length,c4=[...classes.values()].filter(x=>x===4).length;
 let K2=2*B*B,K4=(2*B**4+4*B*B)/3;
 let gf=[1,0,0,0,0];for(let i=0;i<B;i++){let prev=gf;gf=prev.map((_,k)=>prev[k]+2*prev.slice(0,k).reduce((s,x)=>s+x,0))}
 assert(counts[0]===1&&counts[2]===K2&&counts[4]===K4&&gf[2]===K2&&gf[4]===K4&&c2===(K4-K2)/2&&c4===K2/2&&classes.size===c2+c4,"count "+B);
 rows.push({B,counts,c2,c4});
}
console.log(JSON.stringify({small_signed_multiset_checks:rows}));
const mod=x=>((x%101)+101)%101;
function rank(M){if(!M.length)return 0;M=M.map(a=>a.map(mod));let z=0;for(let j=0;j<M[0].length&&z<M.length;j++){let i=M.findIndex((row,i)=>i>=z&&row[j]);if(i<0)continue;[M[z],M[i]]=[M[i],M[z]];let v=1;while(mod(v*M[z][j])!==1)v++;M[z]=M[z].map(x=>mod(x*v));for(let k=0;k<M.length;k++)if(k!==z){let a=M[k][j];M[k]=M[k].map((x,t)=>mod(x-a*M[z][t]))}z++}return z}
let seed=27831;const rnd=()=>{seed=(Math.imul(1664525,seed)+1013904223)>>>0;return seed%101};
let cases=0,fullTotal=0,starTotal=0;
for(let d=1;d<=7;d++)for(let a=0;a<=8;a++)for(let b=0;b<=8;b++)for(let m=0;m<3;m++){
 let u=Array.from({length:a},()=>Array.from({length:d},()=>rnd()%7-3)),v=Array.from({length:b},()=>Array.from({length:d},()=>rnd()%7-3));
 if(m===1){if(a>1)u[a-1]=u[0];if(b>1)v[b-1]=v[0]}
 const tail=m===0?[0,0]:[1,5];
 const row=(x,y)=>x.map((e,i)=>e+y[i]).concat(tail);
 let full=u.flatMap(x=>v.map(y=>row(x,y))),star=a&&b?u.map(x=>row(x,v[0])).concat(v.slice(1).map(y=>row(u[0],y))):[];
 assert(rank(full)===rank(star),"star rank");cases++;fullTotal+=full.length;starTotal+=star.length;
}
const N=2147446830n,K=618884n,A=307520n,B=961n;
const ch2=t=>t*(t-1n)/2n,ch3=t=>t*(t-1n)*(t-2n)/6n;
function bounds(t){const linear=(A*2n+B*4n)*t*N*N,quadratic=(A*4n+B*16n)*ch2(t)*N,cubic=(A*8n+B*64n)*ch3(t);return [linear-quadratic,linear-quadratic+cubic]}
let [l0,u0]=bounds(96813n),[l1,u1]=bounds(96814n);assert(u0*10n<279n*N**3n&&l1*10n>279n*N**3n,"occupancy bracket");
let lin=(9n*31n*N+10n*K-1n)/(10n*K);assert(lin===96810n,"linear threshold");
console.log(JSON.stringify({abstract_rank_cases:cases,cartesian_rows:fullTotal,star_rows:starTotal,exact_occupancy:{threshold:96814,first_moment:lin.toString(),denominator:(N**3n).toString(),upper_96813:u0.toString(),lower_96814:l1.toString()}}));
