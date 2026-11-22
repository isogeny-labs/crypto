'use strict';
// CT61 independent verifier: direct (x,y) enumeration and
// polynomial-factor irreducibility, not the Python trace/Artin-Schreier engine.
const primes = [3,5,7,11,19,43,163,257,65537,2147483647,4294967291];
const fixtures = [
[3,2,-1,6,7,2,2,[1,3],[3,3],[3,3],1],
[5,3,-1,10,11,1,2,[1,5],[3,5],[1,4],2],
[7,3,-5,14,11,1,1,[2,5],[3,3],null,0],
[11,4,-5,22,19,8,9,[1,1],[8,9],[15,11],7],
[19,5,-5,38,37,1,3,[5,11],[28,30],[30,22],7],
[43,8,-1,258,283,32,6,[2,182],[149,174],[100,91],7],
[163,10,47,978,1033,128,41,[3,627],[761,872],[409,677],7]
];
function need(ok,msg){if(!ok) throw Error(msg)}
function target(p,k){const l=BigInt(p),q=1n<<BigInt(k);let m=(q+1n+l)/(2n*l);if(m===0n)m=1n;const N=2n*l*m,t=q+1n-N;return {q,N,t,ok:t*t<=4n*q};}
function bitdegree(x){return x===0?-1:31-Math.clz32(x)}
function rem(a,b){let d=bitdegree(b);while(a && bitdegree(a)>=d) a^=b<<(bitdegree(a)-d);return a}
function irred(poly,k){for(let d=1;d<=Math.floor(k/2);d++)for(let f=(1<<d)|1;f<(1<<(d+1));f+=2)if(rem(poly,f)===0)return false;return true}
function gfMul(a,b,poly,k){let c=0;const Q=1<<k;while(b){if(b&1)c^=a;b>>=1;a<<=1;if(a&Q)a^=poly}return c}
function verifyToy(w){const [ell,k,t,N,poly,a2,a6,P,R,Q,d]=w,q=1<<k;
 need(irred(poly,k),'polynomial reducible');
 const mul=(a,b)=>gfMul(a,b,poly,k);
 const sq=Array.from({length:q},(_,x)=>mul(x,x));
 const cube=Array.from({length:q},(_,x)=>mul(x,sq[x]));
 const RHS=Array.from({length:q},(_,x)=>cube[x]^mul(a2,sq[x])^a6);
 let count=1;
 for(let x=0;x<q;x++)for(let y=0;y<q;y++)if((sq[y]^mul(x,y))===RHS[x])count++;
 need(count===N,'point count '+ell+' got '+count+' != '+N);
 const on=(p)=>p===null || ((sq[p[1]]^mul(p[0],p[1]))===RHS[p[0]]);
 function inv(a){need(a!==0,'inverse zero');let u=a,v=poly,A=1,B=0;while(u!==1){need(u!==0,'noninvertible');let shift=bitdegree(u)-bitdegree(v);if(shift<0){[u,v]=[v,u];[A,B]=[B,A];shift=-shift;}u^=v<<shift;A^=B<<shift;}return rem(A,poly)}
 function add(p,r){if(p===null)return r;if(r===null)return p;
  const [x,y]=p,[u,v]=r;let slope,X,Y;
  if(x===u){if((y^v)===x)return null;need(y===v,'bad same-x input');if(x===0)return null;
    slope=x^mul(y,inv(x));X=sq[slope]^slope^a2;Y=sq[x]^mul(slope^1,X);
  }else{slope=mul(y^v,inv(x^u));X=sq[slope]^slope^x^u^a2;Y=mul(slope,x^X)^X^y;}
  return [X,Y];
 }
 function times(n,p){let r=null;while(n){if(n&1)r=add(r,p);p=add(p,p);n=Math.floor(n/2);}return r}
 function eq(a,b){return JSON.stringify(a)===JSON.stringify(b)}
 need(on(P)&&on(R)&&on(Q),'point not on curve');
 need(eq(times(N/ell,P),R),'cofactor multiplication mismatch');
 need(times(ell,R)===null,'ell-torsion failed');
 need(eq(times(d,R),Q),'planted log failed');
 let cur=null;for(let j=0;j<ell;j++){if(eq(cur,Q))need(j===d,'log uniqueness failed');cur=add(cur,R)}
 need(cur===null,'subgroup closure failed');
 return {ell,k,q,N,t,poly,a2,a6,P,R,Q,d};
}
const rows=[];
for(const p of primes){const bound=2*(Math.floor(Math.log2(p))+1)-2;
 need(target(p,bound).ok,'uniform upper bound failed '+p);
 let chosen=-1;
 for(let k=1;k<=bound;k++)if(target(p,k).ok){chosen=k;break}
 need(chosen>0,'no ordinary trace');
 const {q,N,t}=target(p,chosen);
 need((t&1n)!==0n && N%BigInt(p)===0n,'ordinary trace malformed');
 rows.push([p,chosen,Number(t),Number(N),bound]);
}
const toys=fixtures.map(verifyToy);
console.log(JSON.stringify({rows,toys}));
