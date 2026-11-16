include!("../main.rs");
#[cfg(test)]
mod reviewer_boundaries {
    use super::*;
    #[test]
    fn euclid_nontrivial_and_negative_difference() {
        assert_eq!(inverse(3,11),Ok(4));
        let inv=inverse(123,739).unwrap();
        assert_eq!(((369i128-246).rem_euclid(739) as u128*inv as u128)%739,1);
        assert_eq!(((123i128-246).rem_euclid(739) as u128*inv as u128)%739,738);
    }
    #[test]
    fn euclid_largest_32bit_prime() {
        let p=4_294_967_291;
        assert_eq!(inverse(p-1,p),Ok(p-1));
        assert_eq!((inverse(2,p).unwrap() as u128*2)%p as u128,1);
    }
    #[test]
    fn invalid_euclid_domains_reject() {
        assert!(inverse(0,7).is_err());
        assert!(inverse(6,15).is_err());
    }
    #[test]
    fn high_residue_matrix_square() {
        let p=4_294_967_291;let a=p-1;
        square(&json!([[a,0,0,0],[0,a,0,0],[0,0,a,0],[0,0,0,a]]),p,1).unwrap();
    }
    #[test]
    fn widened_noncancelled_dot_sum() {
        let p=4_294_967_291;let a=p-1;
        // Hadamard rows with canonical representatives of minus one.
        square(&json!([[1,1,1,1],[1,a,1,a],[1,1,a,a],[1,a,a,1]]),p,4).unwrap();
    }
    #[test]
    fn strict_scalar_and_coefficient_boundaries() {
        let id=json!([[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]);
        assert!(square(&id,7,7).is_err());
        let negative=json!([[-1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]);
        assert!(square(&negative,7,1).is_err());
        let boolean=json!([[true,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]);
        assert!(square(&boolean,7,1).is_err());
    }
    #[test]
    fn near_bound_composite_and_oversize_reject() {
        let id=json!([[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]);
        assert!(square(&id,4_294_967_296,1).is_err());
        assert!(square(&id,4_294_967_311,1).is_err());
        assert!(square(&id,341,1).is_err());
    }
    #[test]
    fn zero_and_off_diagonal_reject() {
        let z=json!([[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]);
        assert!(square(&z,7,1).is_err());
        let shear=json!([[1,1,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]);
        assert!(square(&shear,7,1).is_err());
    }
}
