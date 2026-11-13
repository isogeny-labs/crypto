'use strict';
// Independent BigInt modular Newton, non-enumerative point lifting from encoded coordinates.
const p=37n, n=5, mod=p**BigInt(n), trunc=(x,m)=>((x%m)+m)%m;
const sum=(a,b)=>Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0n)+(b[i]||0n));
const diff=(a,b)=>Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0n)-(b[i]||0n));
function prod(a,b){let z=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)z[i+j]+=a[i]*b[j];return z;}
function exp(a,k){let b=[1n];while(k-->0)b=prod(b,a);return b;}
let D=[-79n,-40n,-4n,4n], P3=[-21n,-237n,-60n,-4n,3n], F4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
let f=diff(prod(diff(prod(exp(D,2),F4),exp(P3,3)),exp(P3,3)),prod(exp(D,2),exp(F4,3)));
if(f.length!==25||f[24]!==7n)throw Error('bad psi7');
let df=f.slice(1).map((v,i)=>v*BigInt(i+1));
function horner(a,x,m){let q=0n;for(let i=a.length-1;i>=0;i--)q=trunc(q*x+a[i],m);return q;}
function inv(a,m){a=trunc(a,m);let [b,x,y,z,w]=[m,1n,0n,0n,1n];while(b!==0n){let q=a/b;[a,b]=[b,a-q*b];[x,z]=[z,x-q*z];[y,w]=[w,y-q*w];}if(a!==1n)throw Error('nonunit');return trunc(x,m);}
function lift(x0){let x=BigInt(x0),m=p;if(horner(f,x,p)!==0n)throw Error('bad root');for(let j=2;j<=n;j++){m*=p;x=trunc(x-horner(f,x,m)*inv(horner(df,x,m),m),m);}if(horner(f,x,m)!==0n)throw Error('not root');return x;}
let roots=[9n,27n,28n].map(lift);
function yLift(x,y){let m=p;for(let j=2;j<=n;j++){m*=p;const defect=trunc(y*y+y-x*x*x+x*x+10n*x+20n,m);y=trunc(y-defect*inv(2n*y+1n,m),m);}if(trunc(y*y+y-x*x*x+x*x+10n*x+20n,m)!==0n)throw Error('curve lift');return y;}
const small=(q)=>{let r=[];for(let y=0n;y<p;y++)if(trunc(y*y+y-q*q*q+q*q+10n*q+20n,p)===0n)r.push(y);return r;};
let lifted=[];for(let i=0;i<3;i++)for(let y of small([9n,27n,28n][i]))lifted.push([roots[i],yLift(roots[i],y)]);
if(lifted.length!==6)throw Error('source orientations');
function on(P){if(P===null)return true;let [x,y]=P;return trunc(y*y+y-x*x*x+x*x+10n*x+20n,mod)===0n;}
function add(A,B){if(A===null)return B;if(B===null)return A;let [x,y]=A,[u,v]=B;
 if(x===u&&trunc(y+v+1n,mod)===0n)return null;
 let slope= A[0]===B[0]&&A[1]===B[1] ? trunc((3n*x*x-2n*x-10n)*inv(2n*y+1n,mod),mod) : trunc((v-y)*inv(u-x,mod),mod);
 let nu=trunc(y-slope*x,mod),X=trunc(slope*slope+1n-x-u,mod),Y=trunc(-slope*X-nu-1n,mod);let z=[X,Y];if(!on(z))throw Error('add');return z;}
let g=lifted.find(z=>z[0]%p===9n&&z[1]%p===4n),cur=null,mults=[];for(let j=0;j<7;j++){mults.push(cur);cur=add(cur,g);}if(cur!==null)throw Error('point order');
let sorted=a=>a.map(v=>v.map(x=>String(x)).join(':')).sort().join(';');
if(sorted(mults.slice(1))!==sorted(lifted))throw Error('Hensel vs group');
let curveCount=1;
for(let x=0n;x<p;x++)for(let y=0n;y<p;y++)if(trunc(y*y+y-x*x*x+x*x+10n*x+20n,p)===0n)curveCount++;
if(curveCount!==35||curveCount%7!==0||curveCount%49===0)throw Error('source curve point count');
let result={ell:7,experiment:'ct82-fixed-place-prime-to-p-reduction',fully_split_roots_found:3,group_law_verified:true,modulus:Number(mod),oriented_points:6,precision:5,rational_full_rank_excluded:true,source:37,torsion_group_size:7,x_roots:roots.map(Number)};
console.log(JSON.stringify(result));
