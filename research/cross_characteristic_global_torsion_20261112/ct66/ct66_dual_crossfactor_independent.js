'use strict';
const Q=1024,RED=0x409,L=31;
const check=(c,s)=>{if(!c)throw Error(s)};
function m(a,b){let z=0;while(b){if(b&1)z^=a;b>>=1;a<<=1;if(a&Q)a^=RED;}return z&(Q-1)}
function pow(a,n){let z=1;for(;n;n>>=1){if(n&1)z=m(z,a);a=m(a,a)}return z}
function div(a,b){check(b!==0,'zero denominator');return m(a,pow(b,Q-2))}
const sq=a=>m(a,a),cube=a=>m(sq(a),a),fifth=a=>m(sq(a),cube(a));
const omega=Array.from({length:Q},(_,i)=>i).find(i=>i>1&&(sq(i)^i^1)===0);
check(omega===236,'omega');
const roots=new Map();for(let y=0;y<Q;y++){let h=sq(y)^y;if(!roots.has(h))roots.set(h,[]);roots.get(h).push(y)}
const Rs=[20,534,619,809,832];
function mk(r,cShift){const r2=sq(r),r3=m(r2,r),r4=sq(r2),a=pow(r,Q/2),b=r4^m(omega,r2),v=m(r4,r)^m(omega,r3);
 check((sq(b)^a)===m(omega,r),'translation polynomial');const cs=roots.get(v);check(cs?.length===2,'translation no lift');let c=cs[0]^cShift;
 check((sq(c)^c)===v,'bad c');check((m(a,r2)^m(b,r))===0,'order four');const A=div(a,r), D=div(c^m(A,r3),r),coef=[div(1,r),r,div(D,r)];return {r,a,b,c,A,D,coef};}
let maps=Rs.flatMap(r=>[mk(r,0),mk(r,1)]);
function mapP(P,f){let [x,y]=P,u=sq(x)^m(f.r,x),v=y^m(f.A,cube(x))^m(f.D,x);return [u,v];}
function onE(P,co){if(P===null)return true;const [x,y]=P,[a,b,g]=co;return (sq(y)^y)===(m(a,cube(x))^m(b,sq(x))^m(g,x));}
function negE(P){return P===null?null:[P[0],P[1]^1]}
function addE(P,R,co){if(P===null)return R;if(R===null)return P;let [x,y]=P,[z,w]=R,[a,b,g]=co;
 let slope=x===z?m(a,sq(x))^g:div(y^w,x^z);if(x===z&&y!==w)return null;
 let xx=x^z^div(b^sq(slope),a), yy=m(slope,x^xx)^y^1;check(onE([xx,yy],co),'bad elliptic addition');return [xx,yy];}
function key(P){return P===null?'O':P[0]+':'+P[1]}
function coordTable(co){let pts=[];for(let x=0;x<Q;x++){let rhs=m(co[0],cube(x))^m(co[1],sq(x))^m(co[2],x);for(let y of roots.get(rhs)||[])pts.push([x,y])}
 check(pts.length===960,'E point count');const A=pts[0],seen=new Set();let t=null;for(let i=0;i<L;i++){seen.add(key(t));t=addE(t,A,co)}check(t===null&&seen.size===31,'first generator order');const B=pts.find(P=>!seen.has(key(P)));check(B,'second generator');let table=new Map(),row=null;for(let b=0;b<L;b++){let col=row;for(let a=0;a<L;a++){let k=key(col);check(!table.has(k),'duplicates');table.set(k,[a,b]);col=addE(col,A,co)}row=addE(row,B,co)}check(table.size===961,'table size');return table;}
function sigma(P,f){let [x,y]=P;return [x^f.r,y^m(f.a,sq(x))^m(f.b,x)^f.c]}
const points=[];for(let x=0;x<Q;x++){let h=fifth(x)^m(omega,cube(x));for(let y of roots.get(h)||[])points.push([x,y])}check(points.length===896,'C points');
for(let f of maps){f.table=coordTable(f.coef);for(let P of points)check(onE(mapP(P,f),f.coef),'bad quotient')}
const inv31=a=>{for(let x=1;x<L;x++)if(a*x%L===1)return x;throw Error('not invertible')};
const mod31=x=>((x%L)+L)%L;
function matrixFor(source,dest){let seen=new Map(),samples=[],B=null;for(let P of points){let inCoords=source.table.get(key(mapP(P,source))),out=addE(mapP(P,dest),mapP(sigma(P,source),dest),dest.coef),outCoords=dest.table.get(key(out));check(inCoords&&outCoords,'missing coordinates');let k=inCoords.join(':');if(seen.has(k))check(seen.get(k).join(':')===outCoords.join(':'),'not fibre independent');else seen.set(k,outCoords);
 if(samples.length===0&&inCoords.some(x=>x!==0))samples.push([inCoords,outCoords]);
 else if(samples.length===1){let z=samples[0][0];if(mod31(z[0]*inCoords[1]-z[1]*inCoords[0])!==0){samples.push([inCoords,outCoords]);break;}}
 }
 check(samples.length===2,'input sample rank');let [[u0,v0],[u1,v1]]=samples,det=mod31(u0[0]*u1[1]-u0[1]*u1[0]),inv=inv31(det);
 let get=(j)=>[mod31((v0[j]*u1[1]-v1[j]*u0[1])*inv),mod31((u0[0]*v1[j]-u1[0]*v0[j])*inv)];
 let [row0,row1]=[get(0),get(1)],rank=mod31(row0[0]*row1[1]-row0[1]*row1[0])!==0?2:(row0.some(x=>x!==0)||row1.some(x=>x!==0)?1:0);
 // All 896 C points independently validate the single induced E[31] matrix.
 for(let P of points){let x=source.table.get(key(mapP(P,source))),Y=addE(mapP(P,dest),mapP(sigma(P,source),dest),dest.coef),y=dest.table.get(key(Y));check(y[0]===mod31(row0[0]*x[0]+row0[1]*x[1]) && y[1]===mod31(row1[0]*x[0]+row1[1]*x[1]),'not a linear induced map')}
 return {rank,matrix:[row0,row1]};}
let ranks=[],nonzero=[],maxOffDiag=0;for(let i=0;i<10;i++){let row=[];for(let j=0;j<10;j++){let r=matrixFor(maps[i],maps[j]);row.push(r.rank);if(i!==j&&r.rank)nonzero.push([i,j,r.matrix]);if(i!==j&&r.rank>maxOffDiag)maxOffDiag=r.rank;
 if(i===j)check(r.rank===2 && r.matrix[0].join(',')==='2,0'&&r.matrix[1].join(',')==='0,2','self norm not two');
 if(Math.floor(i/2)===Math.floor(j/2)&&i!==j)check(r.rank===0,'complementary cross map nonzero');}
 ranks.push(row)}
let complementPhi=[];for(let i=0;i<5;i++){let left=maps[2*i],right=maps[2*i+1],images=[];
 for(let P of points){images.push([...left.table.get(key(mapP(P,left))),...right.table.get(key(mapP(P,right)))]);}
 const rank=linearRank4(images);check(rank===4,'complement product rank');complementPhi.push([left.r,rank]);}
function linearRank4(rows){let piv=[],b=[];for(let row of rows){let v=row.slice();for(let z of b){let c=v[z.p];if(c)for(let j=z.p;j<4;j++)v[j]=mod31(v[j]-c*z.v[j]);}let p=v.findIndex(x=>x!==0);if(p===-1)continue;let iv=inv31(v[p]);for(let j=p;j<4;j++)v[j]=mod31(v[j]*iv);b.push({p,v});b.sort((a,b)=>a.p-b.p);if(b.length===4)return 4;}return b.length;}
console.log(JSON.stringify({omega,rs:Rs,quotients:maps.map(f=>[f.r,f.c,...f.coef]),rank_matrix:ranks,non_complementary_full_rank:nonzero.filter(t=>Math.floor(t[0]/2)!==Math.floor(t[1]/2)&&t[2]).length,complementary_phi_ranks:complementPhi,example_nonzero_cross:nonzero.find(t=>t[0]===0&&t[1]===2),base_points:points.length,source_generators_verified:10},null,2));
