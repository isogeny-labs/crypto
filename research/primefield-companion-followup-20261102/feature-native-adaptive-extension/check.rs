//! Replay the frozen pre-cursor source against the saved recovery trace.
//! Does not import this component's cursor, geometry, or source implementation.
#[path="../feature-native-adaptive-fibers/lib.rs"]mod baseline;
fn main(){
 let path=std::env::args().nth(1).expect("recovery.tsv");let text=std::fs::read_to_string(path).unwrap();let mut lines=text.lines();
 assert_eq!(lines.next(),Some("scalar\trecovered\textra_signs\tmethod\tlane_updates\tconverted_points\tbatches"));let rows:Vec<_>=lines.collect();
 let fixture:Vec<_>=include_str!("../feature-native-recovery/targets.tsv").lines().collect();assert_eq!(rows.len(),fixture.len());assert_eq!(rows.len(),8);
 let program=baseline::AdaptiveProgram::construct(479,15,&[(25,173),(292,440)],479,576).unwrap();let source=baseline::PublicSource::new(509,2,10,479,[3,68]).unwrap();let mut extras=0;
 for(line,f)in rows.iter().zip(fixture){let f:Vec<i64>=f.split('\t').map(|v|v.parse().unwrap()).collect();let point=if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])};let expected=source.recover(&program,point).unwrap();
  let row:Vec<_>=line.split('\t').collect();assert_eq!(row.len(),7);assert_eq!(row[0].parse::<u64>().unwrap(),f[0]as u64);assert_eq!(row[1].parse::<u64>().unwrap(),expected.value);assert_eq!(row[2].parse::<usize>().unwrap(),expected.extra_signs);extras+=expected.extra_signs;
  // These are consistency checks on reported metadata, not execution proofs.
  if expected.extra_signs==0{assert_eq!(&row[3..],&["none","0","0","0"]);}else{assert_eq!(&row[3..],&["recurrence","23","24","1"]);}
 }
 assert_eq!(extras,4);println!("{{\"reference_recoveries\":8,\"extra_signs\":4,\"new_cursor_imports\":0,\"metadata_execution_proof\":false}}");
}
