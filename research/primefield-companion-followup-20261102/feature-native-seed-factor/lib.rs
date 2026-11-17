#[path="routed/recovery.rs"]pub mod recovery;
#[cfg(test)]#[path="../feature-native-pair-seeds/lib.rs"]mod reference;
#[cfg(test)]mod pair_recovery_tests{
 use super::*;
 #[test]fn complete_recovery_matches_reference(){let candidate=recovery::Decoder::repaired_479().unwrap();let candidate=candidate.prepare().unwrap();let baseline=reference::recovery::Decoder::repaired_479().unwrap();let baseline=baseline.prepare().unwrap();for line in include_str!("../feature-native-recovery/targets.tsv").lines(){let f:Vec<i64>=line.split('\t').map(|s|s.parse().unwrap()).collect();let q=if f[1]<0{None}else{Some([f[1]as u64,f[2]as u64])};let got=candidate.recover(q).unwrap();let expected=baseline.recover(q).unwrap();assert_eq!(format!("{:?}",got),format!("{:?}",expected));assert_eq!(got.scalar,f[0]as u64);}assert_eq!(candidate.recover(Some([0,0])),Err(recovery::Error::Source));}
}
