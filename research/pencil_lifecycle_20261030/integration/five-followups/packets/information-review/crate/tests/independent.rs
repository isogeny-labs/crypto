use pencil_target_information::information;
#[test]
fn every_two_row_one_nuisance_system_over_f2_and_f3() {
    for p in [2u64,3] {
        for code in 0..p.pow(6) {
            let mut c=code;
            let mut rows=vec![vec![0;3];2];
            for row in &mut rows { for v in row { *v=c%p; c/=p; } }
            let mut possible=vec![false;p as usize];
            for x in 0..p { for k in 0..p {
                if rows.iter().all(|r| (r[0]*x+r[1]*k+r[2])%p==0) {possible[k as usize]=true;}
            }}
            let n=possible.into_iter().filter(|v| *v).count();
            let d=information(p,1,&rows).unwrap();
            assert_eq!(d.consistent,n>0,"p={p},rows={rows:?}");
            assert_eq!(d.target_determined,n==1,"p={p},rows={rows:?}");
        }
    }
}
#[test]
fn zero_nuisance_and_noncanonical_values() {
    assert!(information(2,0,&[vec![1,1]]).unwrap().target_determined);
    assert!(!information(3,0,&[vec![0,1]]).unwrap().consistent);
    assert!(information(3,0,&[vec![0,0]]).unwrap().consistent);
    let canonical=information(7,1,&[vec![1,2,3],vec![4,0,1]]).unwrap();
    let lifted=information(7,1,&[vec![8,16,24],vec![11,14,22]]).unwrap();
    assert_eq!(canonical,lifted);
}
