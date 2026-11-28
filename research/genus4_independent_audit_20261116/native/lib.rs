#![allow(dead_code)]
pub mod cryptanalysis {
    pub mod ic_boundary {
        include!("generated/boundary.rs");
    }
    pub mod residual_walk {
        include!("generated/residual.rs");
    }
    pub mod chained_transfer {
        pub mod ic_control {
            include!("generated/control.rs");
        }
    }
    pub mod genus_ladder {
        include!("generated/ladder.rs");
    }
}
