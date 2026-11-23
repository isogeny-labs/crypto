#!/usr/bin/env node
'use strict';
// CT222: independent rational-group projector audit for a public F101 cubic cover.
// No secret inputs, production ECDLP solves, or physical timing claims.
const p=101,r=109,h=1060494,order=115593846,e=80597544,g=4;
const zero=[0],one=[1],f=[46,0,0,1,0,0,0,0,0,1];
const mod=x=>((x%p)+p)%p;
function pow(a,n){let q=1;a=mod(a);while(n>0){if(n%2)q=mod(q*a);a=mod(a*a);n=Math.floor(n/2)}return q}
function inv(a){if(mod(a)===0)throw Error('division by 0');return pow(a,p-2)}
function norm(a){a=a.map(mod);while(a.length>1&&a[a.length-1]===0)a.pop();return a}
function eq(a,b){return JSON.stringify(a)===JSON.stringify(b)}
function add(a,b){return norm(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0)+(b[i]||0)))}
function sub(a,b){return norm(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0)-(b[i]||0)))}
function negp(a){return norm(a.map(x=>-x))}
function mul(a,b){let v=Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)v[i+j]=mod(v[i+j]+a[i]*b[j]);return norm(v)}
function scale(a,s){return norm(a.map(x=>x*s))}
function divmod(a,b){if(eq(b,zero))throw Error('zero polynomial');let rem=a.slice(),q=Array(Math.max(a.length-b.length+1,1)).fill(0),ib=inv(b[b.length-1]);while(!eq(rem,zero)&&rem.length>=b.length){const ix=rem.length-b.length,c=mod(rem[rem.length-1]*ib);q[ix]=c;for(let j=0;j<b.length;j++)rem[ix+j]=mod(rem[ix+j]-c*b[j]);rem=norm(rem)}return [norm(q),rem]}
function div(a,b){let [q,rem]=divmod(a,b);if(!eq(rem,zero))throw Error('nonexact div');return q}
function rem(a,b){return divmod(a,b)[1]}
function monic(a){return scale(a,inv(a[a.length-1]))}
function egcd(a,b){let x0=one,y0=zero,x1=zero,y1=one;while(!eq(b,zero)){let [q,t]=divmod(a,b);[a,b]=[b,t];[x0,x1]=[x1,sub(x0,mul(q,x1))];[y0,y1]=[y1,sub(y0,mul(q,y1))]}let z=inv(a[a.length-1]);return [scale(a,z),scale(x0,z),scale(y0,z)]}
const id=[one,zero];
function valid(d){let [u,v]=d;return u[u.length-1]===1&&u.length<=g+1&&(eq(v,zero)||v.length<u.length)&&eq(rem(sub(mul(v,v),f),u),zero)}
let counts={cantor:0,elliptic:0,pullback:0};
function cantor([a,b],[c,d]){
 counts.cantor++;
 let [d1,s1,s2]=egcd(a,c),[dd,t1,t2]=egcd(d1,add(b,d));
 let u=monic(div(mul(a,c),mul(dd,dd)));
 let numer=add(add(mul(mul(mul(t1,s1),a),d),mul(mul(mul(t1,s2),c),b)),mul(t2,add(mul(b,d),f)));
 let v=rem(div(numer,dd),u);
 while(u.length>g+1){let uu=monic(div(sub(f,mul(v,v)),u));u=uu;v=rem(negp(v),u)}
 let out=[u,v];if(!valid(out))throw Error('invalid Cantor result '+JSON.stringify(out));return out;
}
function inverseD([u,v]){return [u,rem(negp(v),u)]}
function scalarD(d,n){let out=id,acc=d;while(n>0){if(n%2)out=cantor(out,acc);n=Math.floor(n/2);if(n)acc=cantor(acc,acc)}return out}
function evalpoly(a,x){let v=0;for(let i=a.length-1;i>=0;i--)v=mod(v*x+a[i]);return v}
function pointD([x,y]){let v=[mod(-x),1],w=[y];if(!valid([v,w]))throw Error('bad rational C point');return [v,w]}
function ecadd(a,b){counts.elliptic++;if(a===null)return b;if(b===null)return a;
 const [x1,y1]=a,[x2,y2]=b;if(x1===x2&&mod(y1+y2)===0)return null;
 const m=x1===x2?mod((3*x1*x1+1)*inv(2*y1)):mod((y2-y1)*inv(x2-x1));
 const x=mod(m*m-x1-x2),y=mod(m*(x1-x)-y1);return [x,y];}
function ecmul(a,n){let out=null,acc=a;while(n>0){if(n%2)out=ecadd(out,acc);n=Math.floor(n/2);if(n)acc=ecadd(acc,acc)}return out}
function alpha(a){counts.pullback++;if(a===null)return id;let [x,y]=a,d=[mod(-x),0,0,1],res=[d,[y]];if(!valid(res))throw Error('bad pullback');return res}
function betaPoint([u,y]){return [pow(u,3),y]}
function sumD(points){return points.reduce((d,pt)=>cantor(d,pointD(pt)),id)}
function sumE(points){return points.reduce((d,pt)=>ecadd(d,betaPoint(pt)),null)}
function check(x,reason){if(!x)throw Error(reason)}
function run(){
 let gE=[2,37];check(mod(gE[1]**2-(gE[0]**3+gE[0]+46))===0,'invalid generator');
 let ge=null,group=[];for(let i=0;i<r;i++){group.push(ge);ge=ecadd(ge,gE)}
 check(ge===null&&group.slice(1).every(x=>x!==null)&&new Set(group.map(JSON.stringify)).size===r,'source group wrong');
 let pts=[];for(let u=0;u<p;u++){let x=pow(u,3),rhs=mod(x*x*x+x+46);for(let y=0;y<p;y++)if(mod(y*y)===rhs){pts.push([u,y]);break}}
 check(pts.length===54,'wrong point roster '+pts.length);
 const samples=[[],...[1,2,3,4].flatMap(n=>Array.from({length:6},(_,k)=>Array.from({length:n},(_,j)=>pts[(k*7+j*11)%pts.length]))),
   [pts[0],pts[1],pts[2],pts[3],pts[4]], [pts[1],pts[10],pts[20],pts[30],pts[40],pts[50]]];
 const stats={tested:samples.length,norm_support_elliptic:0,nonsplit_squarefree_classes:0,nonzero_complements:0,full_order_checks:0,wrong_projector_rejections:0,projector_scalar_cantor:0,norm_projector_cantor:0,norm_projector_elliptic:0,norm_projector_pullbacks:0};
 check(eq(egcd(f,f.map((x,i)=>i*x).slice(1))[0],one),'curve not squarefree');
 check((e%r)===1&&(e%h)===0&&((e*e-e)%order)===0,'CRT fail');
 for(let i=0;i<samples.length;i++){
   let support=samples[i],D=sumD(support);
   const u=D[0],linearRoots=Array.from({length:p},(_,z)=>z).filter(z=>evalpoly(u,z)===0).length;
   if(u.length>2&&linearRoots<u.length-1&&eq(egcd(u,u.map((x,i)=>i*x).slice(1))[0],one))stats.nonsplit_squarefree_classes++;
   let beforeNorm={...counts},normE=sumE(support);stats.norm_support_elliptic+=counts.elliptic-beforeNorm.elliptic;
   let before={...counts};let ed=scalarD(D,e);stats.projector_scalar_cantor+=counts.cantor-before.cantor;
   before={...counts};let en=alpha(ecmul(normE,73));
   stats.norm_projector_cantor+=counts.cantor-before.cantor;
   stats.norm_projector_elliptic+=counts.elliptic-before.elliptic;
   stats.norm_projector_pullbacks+=counts.pullback-before.pullback;
   check(eq(ed,en),'cofactor/norm mismatch at sample '+i);
   let comp=cantor(D,inverseD(ed));
   check(eq(scalarD(ed,r),id),'source 109 not annihilated '+i);
   check(eq(scalarD(comp,h),id),'complement h not annihilated '+i);
   check(eq(scalarD(D,order),id),'Jacobian order not annihilated '+i);
   check(eq(scalarD(ed,e),ed),'projector not idempotent '+i);
   check(eq(cantor(comp,ed),D),'decomposition not exact '+i);
   check(eq(alpha(ecmul(normE,73)),ed),'pullback repeat differs '+i);
   if(!eq(comp,id))stats.nonzero_complements++;
   if(!eq(scalarD(alpha(gE),e+h),alpha(gE)))stats.wrong_projector_rejections++;
   stats.full_order_checks++;
 }
 check(stats.nonzero_complements>0,'no kernel element tested');
 check(stats.nonsplit_squarefree_classes===2,'unexpected nonsplit reduced-divisor coverage');
 check(stats.wrong_projector_rejections===samples.length,'mutation control failed');
 check(eq(scalarD(alpha(gE),r),id),'source 109-order failed');
 check(eq(alpha(ecmul(gE,3)),scalarD(alpha(gE),3)),'linearity failed');
 return {schema:'ct222-f101-independent-rational-projector-v1',pcc:'unverified',field:p,source_order:r,jacobian_order:order,
          projector_scalar:e,samples:stats,standing:'rational finite toy independent JS Cantor vs EC norm checks; not native Rust cost or PCC'};
}
if(require.main===module){let data=run();if(process.argv.includes('--self-test'))console.log('CT222_SELFTEST_PASS',data.samples.tested,data.samples.full_order_checks,data.samples.nonzero_complements);else console.log(JSON.stringify(data,null,2));}
module.exports={run};
