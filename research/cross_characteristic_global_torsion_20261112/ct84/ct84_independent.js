'use strict';
// Independent BigInt Hensel, exact symbolic group law and binary F_(2^24).
// Checks CT84's global recovery certificate; LLL producer is Python only.
const p=37n,N=18;
const check=(ok,msg)=>{if(!ok)throw Error(msg)};
const trim=a=>{a=a.slice();while(a.length>1&&a.at(-1)===0n)a.pop();return a};
const mul=(a,b)=>{let c=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)c[i+j]+=a[i]*b[j];return trim(c)};
const sub=(a,b)=>trim(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]??0n)-(b[i]??0n)));
const powPoly=(a,n)=>{let z=[1n];for(let j=0;j<n;j++)z=mul(z,a);return z};
const ev=(a,x,m)=>{let z=0n;for(let i=a.length-1;i>=0;i--){z=z*x+a[i];if(m!==undefined)z=((z%m)+m)%m;}return z};
const D=[-79n,-40n,-4n,4n],P3=[-21n,-237n,-60n,-4n,3n],F4=[-5821n,-1496n,-210n,-790n,-100n,-4n,2n];
const F=sub(mul(sub(mul(powPoly(D,2),F4),powPoly(P3,3)),powPoly(P3,3)),mul(powPoly(D,2),powPoly(F4,3)));
const deriv=F.slice(1).map((z,i)=>z*BigInt(i+1));
check(F.length===25&&F.at(-1)===7n,'psi7');
const inv=(a,m)=>{let t=0n,nt=1n,r=m,nr=((a%m)+m)%m;while(nr){let q=r/nr;[t,nt]=[nt,t-q*nt];[r,nr]=[nr,r-q*nr];}check(r===1n,'nonunit');return ((t%m)+m)%m};
function lift(seed){let x=BigInt(seed),m=p;check(ev(F,x,p)===0n,'seed');const d=inv(ev(deriv,x,p),p);for(let i=1;i<N;i++){let next=m*p;let e=ev(F,x,next)/m;x=((x-e*d*m)%next+next)%next;check(ev(F,x,next)===0n,'Hensel');m=next;}return x;}
const a=lift(9),b=lift(27),q=p**BigInt(N);
const A=[21n,158n,20n,0n,1n],B=[-79n,-40n,-4n,4n];
check(((ev(A,a,q)-b*ev(B,a,q))%q+q)%q===0n,'local rational relation');
const M=[-10n,-2n,3n],pos=sub(mul(M,M),mul([-1n,2n],D));
check(JSON.stringify(pos.map(String))===JSON.stringify(A.map(String)),'exact numerator');
const mod=x=>((x%37n)+37n)%37n;
const slope=mod((3n*9n*9n-2n*9n-10n)*inv(2n*4n+1n,37n));
const sx=mod(slope*slope+1n-18n),sy=mod(slope*(9n-sx)-4n-1n);
check(sx===27n&&sy===9n,'source group law');
check(mod(ev(A,9n)*inv(ev(B,9n),37n))===27n,'target mismatch');
check(mod(ev(A,9n)*inv(ev(B,9n),37n))!==28n,'negative control');
const MOD=(1<<12)|(1<<9)|(1<<5)|(1<<2)|1;
function cmul(u,v){let w=0;while(v){if(v&1)w^=u;v>>=1;u<<=1;if(u&4096)u^=MOD;}return w&4095;}
const add=(u,v)=>[u[0]^v[0],u[1]^v[1]];
const fm=(u,v)=>{let bd=cmul(u[1],v[1]);return [cmul(u[0],v[0])^cmul(bd,12),cmul(u[0],v[1])^cmul(u[1],v[0])^bd]};
const eq=(u,v)=>u[0]===v[0]&&u[1]===v[1];
const one=[1,0],X=[2,0],Y=[0,1];
const lam=fm(X,X),nu=add(Y,fm(lam,X)),xx=add(fm(lam,lam),one),yy=add(add(fm(lam,xx),nu),one);
check(eq(xx,[17,0])&&eq(yy,[77,1]),'binary group law');
check(eq(add(fm(Y,Y),Y),add(fm(fm(X,X),X),fm(X,X)),'binary source curve');
console.log(JSON.stringify({
  basis_dimension:9,binary_image_x:xx[0],binary_image_y:yy,
  exact_global_identity:true,experiment:'ct84-selective-rational-recognition',
  negative_root_rejected:true,precision_digits:N,public_source_point:[27,9],
  recovered_denominator:B.map(Number),recovered_numerator:A.map(Number),
  source_generator:[9,4]
}));
