fn power(mut a:u64,mut n:u64,p:u64)->u64{let mut v=1;while n>0{if n&1!=0{v=v*a%p;}a=a*a%p;n>>=1;}v}
fn canonical(mut a:Vec<u64>)->Vec<u64>{while a.len()>1&&a.last()==Some(&0){a.pop();}a}
fn slow_div(mut a:Vec<u64>,b:&[u64],p:u64)->Vec<u64>{let inv=power(*b.last().unwrap(),p-2,p);while a!=[0]&&a.len()>=b.len(){let k=a.len()-b.len();let c=*a.last().unwrap()*inv%p;for(i,&v)in b.iter().enumerate(){a[k+i]=(a[k+i]+p-c*v%p)%p;}a=canonical(a);}a}
#[test]fn lifted_batches_match_exact_trace(){let(mut cases,mut recursive,mut corrected)=(0,0,0);let mut state=713u64;
 for p in [5u64,7,13,29,509]{for d in [2usize,3,7,8,9,15,16,17,31,32,33,64,65,127,128,129]{for n in [d-1,(d+1)/2,d/3]{for _ in 0..4{
  let mut draw=||{state=state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);state%p};let mut a:Vec<_>=(0..=d).map(|_|draw()).collect();let mut b:Vec<_>=(0..=n).map(|_|draw()).collect();a[d]=1+draw()%(p-1);b[n]=1+draw()%(p-1);
  let(mut x,mut y)=(a.clone(),b.clone());let mut want=vec![];while y!=[0]&&y.len()-1>=(d+1)/2{let rem=slow_div(x.clone(),&y,p);want.push(Transition{m:x.len()-1,n:y.len()-1,k:degree(&rem),leading:*y.last().unwrap()});x=y;y=rem;}
  let mut e=Poly::new(p).unwrap();reset_batches();let got=e.half_gcd(&a,&b).unwrap();recursive+=batches();assert_eq!(got.steps,want,"p={p} d={d} n={n}");assert_eq!((&got.a,&got.b),(&x,&y));let applied=e.matrix_apply(&got.matrix,&a,&b);assert_eq!(applied,(x,y));
  let v=e.mul(&got.matrix[0][0],&got.matrix[1][1]);let w=e.mul(&got.matrix[0][1],&got.matrix[1][0]);assert_eq!(e.sub(&v,&w),vec![if want.len()%2==0{1}else{p-1}]);
  let actual=e.resultant_class(&a,&b).unwrap().project();let mut reference=reference_euclid::Poly::new(p).unwrap();let expected=reference.projected_resultant(&a,&b).unwrap();assert_eq!(actual,expected);
  if d>8&&n>=(d+1)/2{let h=(d+1)/2;let small=e.half_gcd(&a[h..],&b[h..]).unwrap();let old=small.steps.last().and_then(|t|t.k.map(|k|k+h));let full=e.lifted(small,&a,&b,h).unwrap();if !full.steps.is_empty()&&old!=degree(&full.b){corrected+=1;}}
  cases+=1;
 }}}}
 assert!(recursive>0&&corrected>0);println!("trace_cases={cases} recursive_calls={recursive} changed_final_degree_cases={corrected}");
}
