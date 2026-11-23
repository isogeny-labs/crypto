fn second(f:&Poly,x:u64,y:u64,ax:usize,bx:usize)->u64 {
 let mut z=0;
 for(i,j)in monos(4){let mut e=[i,j];let mut c=f[7*i+j];for k in[ax,bx]{if e[k]==0{c=0;break;}c=mul(c,e[k]as u64);e[k]-=1;}z=add(z,mul(c,mul(pow(x,e[0]as u64),pow(y,e[1]as u64))));}z
}
fn base_divisor_control(v:&[Poly],h:&[Vec<u64>],ts:&[[usize;3]]){
 let c=crate::config::get();let f=curve();let r=c.residual;
 if sub(mul(r[1],r[1]),mul(4,r[0]))!=0 {regular_base_control(v,h,ts);return;}

 assert_eq!(sub(mul(r[1],r[1]),mul(4,r[0])),0,"registered_double_base");
 let qx=mul(sub(0,r[1]),inv(2));let qy=add(mul(c.tangent[0],qx),c.tangent[1]);
 assert_eq!(val(&f,qx,qy),0);let fy=partial(&f,qx,qy,1);assert_ne!(fy,0);
 let yp=mul(sub(0,partial(&f,qx,qy,0)),inv(fy));
 let ypp=mul(sub(0,add(add(second(&f,qx,qy,0,0),mul(mul(2,second(&f,qx,qy,0,1)),yp)),mul(second(&f,qx,qy,1,1),mul(yp,yp)))),inv(fy));
 let ps=[(qx,qy),(c.controls[0][0],c.controls[0][1]),(c.controls[1][0],c.controls[1][1])];
 let(u,y)=graph(&ps);let(a,g)=corrected(v,u,y);assert_eq!(g,r,"full_double_base_cancelled");
 let vals:Vec<[u64;3]>=v.iter().map(|n|std::array::from_fn(|k|{
  if k!=0{return mul(val(n,ps[k].0,ps[k].1),inv(peval(&r,ps[k].0)));}
  assert_eq!(val(n,qx,qy),0);assert_eq!(add(partial(n,qx,qy,0),mul(partial(n,qx,qy,1),yp)),0,"first_order_zero");
  mul(add(add(add(second(n,qx,qy,0,0),mul(mul(2,second(n,qx,qy,0,1)),yp)),mul(second(n,qx,qy,1,1),mul(yp,yp))),mul(partial(n,qx,qy,1),ypp)),inv(2))
 })).collect();
 let mut ev=vec![];
 for t in ts{let direct=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]].iter().fold(0,|z,s|add(z,mul(mul(vals[t[s[0]]][0],vals[t[s[1]]][1]),vals[t[s[2]]][2])));let z=polar(a[t[0]],a[t[1]],a[t[2]],u);assert_eq!(z,direct,"independent_second_derivative_tensor");ev.push(z);}
 let coords:Vec<_>=h.iter().map(|row|row.iter().zip(&ev).fold(0,|z,(&a,&b)|add(z,mul(a,b)))).collect();assert!(coords.iter().any(|&a|a!=0));
 // Cancelling only one copy leaves all sections zero at Q, hence every triple tensor zero.
 assert!(v.iter().all(|n|add(partial(n,qx,qy,0),mul(partial(n,qx,qy,1),yp))==0));
 println!("DOUBLE_BASE_CONTROL Q={qx},{qy} multiplicity=2 squarefree_effective_u=true tensors={} independent_second_derivatives=true projective_nonzero=true old_first_order_vector_zero=true coords={coords:?}",ts.len());
}
