#[path="../feature-native-multiseed-factor/residual.rs"]mod original_residual;
#[test]fn projected_residual_matches_full_ratio(){
 let(mut valid,mut rejected,mut batches)=(0,0,0);let mut state=723u64;
 for p in [5u64,7,13,29,509]{for r in [7u64,13,19,31,479]{
  let part=partition(r).unwrap();let[w,k,s,_]=part;
  for _ in 0..100{
   let mut draw=||{state=state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);state%p};
   let q=[draw(),draw()];let grid:Vec<_>=(0..1+w+k+u64::from(s!=0)).map(|_|[draw(),draw()]).collect();
   let want=original_residual::geometry_ratio(p,part,q,&grid);residual::reset_projection_count();let got=residual::geometry_ratio(p,part,q,&grid);assert_eq!(residual::projection_count(),usize::from(got.is_ok()));batches+=residual::batch_count();
   assert_eq!(got.is_ok(),want.is_ok(),"p={p} r={r}");
   if let(Ok(a),Ok(b))=(got,want){let odd=(p-1)/(1u64<<(p-1).trailing_zeros());assert_eq!(mul(a[0],pow(b[1],odd,p),p),mul(a[1],pow(b[0],odd,p),p));valid+=1;}else{rejected+=1;}
  }
 }}assert!(valid>0&&rejected>0&&batches>0);println!("synthetic_geometry_cases={} valid={valid} rejected={rejected} one_projection_per_valid=true recursive_batches={batches}",valid+rejected);
 let part=partition(7).unwrap();assert!(residual::geometry_ratio(5,part,[0,1],&[]).is_err());assert!(residual::geometry_ratio(17,part,[0,1],&[[1,1];3]).is_err());
}
