use std::collections::BTreeSet;
fn permutations(prefix:Vec<usize>)->Vec<[usize;4]> {
    if prefix.len()==4 {return vec![prefix.try_into().unwrap()];}
    (0..4).filter(|i|!prefix.contains(i)).flat_map(|i|{let mut p=prefix.clone();p.push(i);permutations(p)}).collect()
}
fn cycles(p:&[usize])->Vec<usize> {
    let mut unseen:BTreeSet<usize>=(0..p.len()).collect();let mut out=Vec::new();
    while let Some(&start)=unseen.first() {
        let mut i=start;let mut size=0;
        while unseen.remove(&i) {size+=1;i=p[i];}
        assert_eq!(i,start);out.push(size);
    }out.sort_unstable();out
}
fn orbits(actions:&[Vec<usize>],degree:usize)->Vec<Vec<usize>> {
    let mut unseen:BTreeSet<usize>=(0..degree).collect();let mut out=Vec::new();
    while let Some(&start)=unseen.first() {
        let mut orbit=BTreeSet::from([start]);let mut queue=vec![start];
        while let Some(i)=queue.pop() {for action in actions {if orbit.insert(action[i]) {queue.push(action[i]);}}}
        for i in &orbit {assert!(unseen.remove(i));}out.push(orbit.into_iter().collect());
    }out
}
pub fn qualify() {
    let pairs:Vec<[usize;2]>=(0..4).flat_map(|i|(i+1..4).map(move |j|[i,j])).collect();
    let complement:Vec<usize>=pairs.iter().map(|p|pairs.iter().position(|q|q.iter().all(|x|!p.contains(x))).unwrap()).collect();
    let representatives:Vec<usize>=(0..6).filter(|&i|i<complement[i]).collect();assert_eq!(representatives.len(),3);
    let partition:Vec<usize>=(0..6).map(|i|representatives.iter().position(|&j|i==j||i==complement[j]).unwrap()).collect();
    let m:Vec<Vec<i128>>=pairs.iter().map(|p|(0..4).map(|i|i128::from(p.contains(&i))).collect()).collect();
    let n:Vec<Vec<i128>>=(0..3).map(|j|partition.iter().map(|&p|i128::from(p==j)).collect()).collect();
    let gram=|m:&[Vec<i128>]|->Vec<Vec<i128>> {(0..4).map(|i|(0..4).map(|j|m.iter().map(|r|r[i]*r[j]).sum()).collect()).collect()};
    let mtm=gram(&m);let expected:Vec<Vec<i128>>=(0..4).map(|i|(0..4).map(|j|1+2*i128::from(i==j)).collect()).collect();assert_eq!(mtm,expected);
    let nm:Vec<Vec<i128>>=n.iter().map(|r|(0..4).map(|j|r.iter().zip(&m).map(|(&a,b)|a*b[j]).sum()).collect()).collect();assert!(nm.iter().flatten().all(|&x|x==1));
    println!("FULL_PAIR_INCIDENCE\tpairs={pairs:?}\tcomplement={complement:?}\tpartition={partition:?}\tM={m:?}\tN={n:?}\tMtransposeM={mtm:?}\tNM={nm:?}");
    let all=permutations(Vec::new());assert_eq!(all.len(),24);
    let mut sign_h=Vec::new();let mut three_h=Vec::new();let mut six_h=Vec::new();
    for p in &all {
        let six:Vec<usize>=pairs.iter().map(|q|{let mut image=q.map(|i|p[i]);image.sort_unstable();pairs.iter().position(|q|q==&image).unwrap()}).collect();
        let three:Vec<usize>=representatives.iter().map(|&r|partition[six[r]]).collect();
        let parity=(0..4).flat_map(|i|(i+1..4).map(move |j|(i,j))).filter(|&(i,j)|p[i]>p[j]).count()%2;
        for (i,row) in m.iter().enumerate() {for (j,&c) in row.iter().enumerate() {assert_eq!(c,m[six[i]][p[j]]);}}
        for (i,&j) in partition.iter().enumerate() {assert_eq!(partition[six[i]],three[j]);assert_eq!(six[complement[i]],complement[six[i]]);}
        if p[0]==0 {sign_h.push(vec![parity,1^parity]);three_h.push(three.clone());six_h.push(six.clone());}
        println!("ALL_S4_EQUIVARIANCE\tfour={p:?}\tsix={six:?}\tthree={three:?}\tparity={parity}\tM-N-complement-preserved=true");
        if *p==[1,0,2,3] {
            let c6=cycles(&six);let c3=cycles(&three);assert_eq!(c6,vec![1,1,2,2]);assert_eq!(c3,vec![1,2]);
            let def6=4*(6-c6.len());let def3=4*(3-c3.len());let g6=1+def6/2;let g3=1+def3/2;
            assert_eq!((g6,g3),(5,3));assert_eq!(2*g6-2,2*(2*g3-2));
            println!("ACTUAL_BRANCH_INERTIA_APPLICATION\tlocked-four-distinct-simple-branches\tsix-cycles={c6:?}\tthree-cycles={c3:?}\tgenus6={g6}\tgenus3={g3}\tcomplement-cover-different-degree=0\tPrym-dimension={}",g6-g3);
        }
    }
    assert_eq!(six_h.len(),6);
    let (os,o3,o6)=(orbits(&sign_h,2),orbits(&three_h,3),orbits(&six_h,6));
    assert_eq!((os.len(),o3.len(),o6.len()),(1,1,2));assert!(o6.iter().all(|o|o.len()==3));
    println!("FULL_POINT_STABILIZER_DOUBLE_COSETS\tsign={os:?}\tcubic={o3:?}\tsix={o6:?}\tcommon-base-factor-for-sign-cubic-only");
    let mut bad=m.clone();bad[0][0]+=1;let bad_gram=gram(&bad);assert_ne!(bad_gram,expected);assert_eq!(gram(&m),expected);
    println!("REAL_INCIDENCE_MUTANT\tM00+1\tfull-bad-gram={bad_gram:?}\tfull-identity-rejects\tpristine-restored");
}
