use hec_native_ic::{instance::{factor_base,thread_curve}, jac::Div, poly::Poly, fp::Fp};
use std::collections::BTreeMap;
fn rank(mut a:Vec<Vec<u64>>,f:&Fp)->usize {
 if a.is_empty(){return 0} let mut r=0;
 for col in 0..a[0].len(){let Some(p)=(r..a.len()).find(|&i|a[i][col]!=0) else{continue}; a.swap(r,p);let inv=f.inv(a[r][col]);for j in col..a[r].len(){a[r][j]=f.mul(a[r][j],inv)}for i in 0..a.len(){if i==r{continue}let z=a[i][col];for j in col..a[i].len(){a[i][j]=f.sub(a[i][j],f.mul(z,a[r][j]))}}r+=1;if r==a.len(){break}}
 r
}
fn root(p:&mut [usize],i:usize)->usize {if p[i]!=i{p[i]=root(p,p[i])}p[i]}
fn main(){
 println!("g\tp\tn\tstatus\tscanned_points\texceptional_t\tfull_fibres\ttransitions\toriented_rank\tsigned_rank\tclosed_components\tgraph_rank");
 let lin=Fp::new(1000003);
 for (g,p,c0) in [(2,101,1),(3,61,1),(4,31,1)] {
 let Some(c)=thread_curve(p,c0,g) else{println!("{g}\t{p}\t0\tsingular_curve");continue};let f=&c.fp;let fb=factor_base(&c);let &(x,y)=fb.iter().find(|z|z.1!=0).unwrap();let base=c.place(x,y);
 let mut points=vec![];for (j,&(x,y)) in fb.iter().enumerate(){points.push((x,y,j,1u64));if y!=0{points.push((x,f.neg(y),j,lin.p-1))}}
 for n in 17..=40 {
 let d=c.mul(&base,n);assert!(c.is_valid(&d));
 if d.u.deg()!=g as isize || Poly::gcd(&d.u,&d.u.deriv(f),f).deg()!=0 || Poly::gcd(&d.u,&d.v,f).deg()!=0 {println!("{g}\t{p}\t{n}\tinput_excluded");continue}
 let mut buckets=vec![vec![];p as usize];let mut scanned=0;
 for (i,&(x,y,_,_)) in points.iter().enumerate(){let u=d.u.eval(x,f);if u==0{continue}let t=f.mul(f.sub(y,d.v.eval(x,f)),f.inv(u));buckets[t as usize].push(i);scanned+=1;}
 let mut fibres=vec![];let mut exceptions=0;
 for t in 0..p {let h=d.v.add(&d.u.scale(t,f),f);let q=c.f.sub(&h.mul(&h,f),f).div_exact(&d.u,f);assert_eq!(q.deg(),(g+1) as isize);assert_eq!(q.lc(),1);
 if Poly::gcd(&q,&q.deriv(f),f).deg()!=0 || Poly::gcd(&q,&d.u,f).deg()!=0 {exceptions+=1;continue}
 let direct:Vec<_>=points.iter().enumerate().filter(|(_,z)| q.eval(z.0,f)==0 && h.eval(z.0,f)==z.1).map(|(i,_)|i).collect();assert_eq!(direct,buckets[t as usize],"all regular rational roots");
 if direct.len()!=g+1{continue}
 let sum=direct.iter().fold(c.zero(),|a,&i|c.add_place(&a,points[i].0,points[i].1));assert!(c.is_zero(&c.add(&d,&sum)),"original fibre identity");
 for &i in &direct {let mut up=Poly::one();for &j in &direct{if j!=i{up=up.mul(&Poly::x_minus(points[j].0,f),f)}}let e=Div{u:up,v:h.rem(&up,f)};assert!(c.is_valid(&e));let got=c.add_place(&d,points[i].0,points[i].1);assert_eq!(got,c.neg(&e),"point-add residual");let descent=c.add(&got,&c.neg(&c.place(points[i].0,points[i].1)));assert_eq!(descent,d,"same decomposition");}
 fibres.push(direct);
 }
 let count=fibres.len();let oriented:Vec<Vec<u64>>=fibres.iter().map(|is|{let mut row=vec![0;points.len()];for &i in is{row[i]=1}row}).collect();
 let signed:Vec<Vec<u64>>=fibres.iter().map(|is|{let mut row=vec![0;fb.len()];for &i in is{let (_,y,j,s)=points[i];if y!=0{row[j]=lin.add(row[j],s)}}row}).collect();
 let differences=|rows:&Vec<Vec<u64>>|->Vec<Vec<u64>>{if rows.is_empty(){return vec![]}rows[1..].iter().map(|r|r.iter().zip(&rows[0]).map(|(&a,&b)|lin.sub(a,b)).collect()).collect()};
 let ro=rank(differences(&oriented),&lin);let rs=rank(differences(&signed),&lin);assert_eq!(ro,count.saturating_sub(1));
 let mut parent:Vec<_>=(0..count).collect();let mut ground=vec![false;count];
 for j in 0..fb.len(){let hits:Vec<_>=(0..count).filter(|&i|signed[i][j]!=0).collect();assert!(hits.len()<=2);match hits.as_slice(){[a]=>ground[*a]=true,[a,b]=>{assert_eq!(lin.add(signed[*a][j],signed[*b][j]),0);let aa=root(&mut parent,*a);let bb=root(&mut parent,*b);parent[aa]=bb},_=>{}}}
 let mut comps=BTreeMap::new();for i in 0..count{let rr=root(&mut parent,i);let z=comps.entry(rr).or_insert((0usize,false));z.0+=1;z.1|=ground[i];}
 let closed:Vec<_>=comps.values().filter(|z|!z.1).collect();let delta=usize::from(closed.iter().any(|z|z.0 as u64 % lin.p!=0));let predicted=if count==0{0}else{count-1-closed.len()+delta};assert_eq!(rs,predicted,"signed grounded graph rank");assert!((g+1)*count<=scanned);
 println!("{g}\t{p}\t{n}\tchecked\t{scanned}\t{exceptions}\t{count}\t{}\t{ro}\t{rs}\t{}\t{predicted}",(g+1)*count,closed.len());
 }
 }
}
