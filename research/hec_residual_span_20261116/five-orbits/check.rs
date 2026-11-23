fn orbit_checks(ls:&[&&Vec<Vec<u64>>],sigma:&Mat,id:i128){
 let perm:Vec<usize>=ls.iter().map(|w|ls.iter().position(|v|w.iter().all(|x|contains(v,image(sigma,x,5)))).unwrap()).collect();
 let mut seen=[false;6];let mut sizes=vec![];for i in 0..6{if seen[i]{continue}let mut k=i;let mut size=0;while !seen[k]{seen[k]=true;size+=1;k=perm[k];}assert_eq!(k,i);sizes.push(size);}sizes.sort();assert_eq!(sizes,vec![1,5]);
 let rho=matrix(&[1,2,0,0,2]);let sq=mm(&rho,&rho);for i in 0..4{for j in 0..4{assert_eq!(sq[i][j],if i==j{5}else{0});}}assert_eq!(rank(&rho,5),2);
 let ops:Vec<_>=(0..625).map(|mut code|{let mut a=vec![0i128;5];for x in a.iter_mut().take(4){*x=code%5;code/=5;}matrix(&a)}).collect();
 for(i,w)in ls.iter().enumerate(){let fixed=perm[i]==i;let isrho=w.iter().all(|x|image(&rho,x,5).iter().all(|&x|x==0));assert_eq!(fixed,isrho);
 let ring:std::collections::BTreeSet<_>=ops.iter().filter(|m|w.iter().all(|x|contains(w,image(m,x,5)))).map(|m|(0..4).map(|j|red(m[j][0],5)).collect::<Vec<_>>()).collect();
 assert_eq!(ring.len(),if fixed{625}else{125});let conductor=ops.iter().filter(|m|(0..4).all(|j|ring.contains(&(0..4).map(|i|red(m[i][j],5)).collect::<Vec<_>>()))).count();assert_eq!(conductor,if fixed{625}else{25});
 println!("{id}\t{i}\t{fixed}\t{isrho}\t{}\t{conductor}\t1,5",ring.len());}
}
