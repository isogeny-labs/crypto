// Appended to the unchanged census source by check.sh, inside the same crate.
// This calls the actual low_dim and low_dim_iter functions on a known fixture.
#[cfg(test)]
mod degree_control_tests {
    use super::*;

    fn fixture(eqs: Vec<Poly>) -> Instance {
        Instance {
            n: 1,
            nprime: 1,
            big_n: 2,
            a: 0,
            b: 0,
            basis: vec![],
            x_r: 0,
            eqs,
            quad_rows: vec![],
        }
    }

    #[test]
    fn known_degree_three_control() {
        let f = vec![mono_encode(&[0, 1]), mono_encode(&[0]), mono_encode(&[1])];
        let instance = fixture(vec![f]);
        let a = low_dim(&instance, 2);
        let b = low_dim(&instance, 3);
        let c = low_dim_iter(&instance, 2);
        let d = low_dim_iter(&instance, 3);
        assert_eq!((a.0, a.1), (0, 3));
        assert_eq!((b.0, b.1), (2, 9));
        assert_eq!((c.0, c.1), (0, 3));
        assert_eq!((d.0, d.1), (2, 9));
    }

    #[test]
    fn field_equations_alone_do_not_invent_linear_consequences() {
        let instance = fixture(vec![]);
        assert_eq!(low_dim(&instance, 2).0, 0);
        assert_eq!(low_dim_iter(&instance, 3).0, 0);
    }

    #[test]
    fn linear_positive_control_is_not_rejected() {
        let instance = fixture(vec![vec![mono_encode(&[0])], vec![mono_encode(&[1])]]);
        assert_eq!(low_dim(&instance, 2).0, 2);
        assert_eq!(low_dim_iter(&instance, 2).0, 2);
    }
}
