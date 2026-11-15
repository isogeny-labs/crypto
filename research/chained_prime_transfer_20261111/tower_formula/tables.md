### p = 2^64  (rho on E = 31.83 log2 group ops)

| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |
|---|---|---|---|---|---|---|
| Gaudry basic g!p+p^2 | 2 | 128.0 | 31.8 | +96.2 | +100.2 | g in 2..64 |
| p^(2-2/g) bare | 2 | 64.0 | 31.8 | +32.2 | +36.2 | constants dropped, lower bound on the family |
| g!p+g^2 p^(2-2/g) | 2 | 66.6 | 31.8 | +34.8 | +38.8 | CostModel form |
| L[1/2,sqrt2], g >= ln p | 45 | 251.3 | 31.8 | +219.5 | +233.9 | in-regime, inside g<=64 |
| L[1/2,sqrt2], unconstrained | 1 | 26.5 | 31.8 | -5.4 | -5.4 | INVALID outside g >= ln p |
| best valid overall | 2 | 64.0 | 31.8 | +32.2 | +36.2 | min of the rows above except unconstrained |

- theta=1: regime g0=45; L at g0 = 251.3 (c=1: 177.7); min over g0..32768 = 251.3; gap to rho = +219.5 bits (+145.9 with c=1)
- theta=0.5: regime g0=23; L at g0 = 171.5 (c=1: 121.3); min over g0..32768 = 171.5; gap to rho = +139.7 bits (+89.5 with c=1)
- theta=0.25: regime g0=12; L at g0 = 117.9 (c=1: 83.4); min over g0..32768 = 117.9; gap to rho = +86.1 bits (+51.6 with c=1)

### p = 2^128  (rho on E = 63.83 log2 group ops)

| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |
|---|---|---|---|---|---|---|
| Gaudry basic g!p+p^2 | 2 | 256.0 | 63.8 | +192.2 | +196.2 | g in 2..64 |
| p^(2-2/g) bare | 2 | 128.0 | 63.8 | +64.2 | +68.2 | constants dropped, lower bound on the family |
| g!p+g^2 p^(2-2/g) | 2 | 130.6 | 63.8 | +66.8 | +70.8 | CostModel form |
| L[1/2,sqrt2], g >= ln p | none | n/a | 63.8 | n/a | n/a | in-regime, inside g<=64 |
| L[1/2,sqrt2], unconstrained | 1 | 40.7 | 63.8 | -23.1 | -23.1 | INVALID outside g >= ln p |
| best valid overall | 2 | 128.0 | 63.8 | +64.2 | +68.2 | min of the rows above except unconstrained |

- theta=1: regime g0=89; L at g0 = 543.1 (c=1: 384.0); min over g0..32768 = 543.1; gap to rho = +479.3 bits (+320.2 with c=1)
- theta=0.5: regime g0=45; L at g0 = 371.2 (c=1: 262.5); min over g0..32768 = 371.2; gap to rho = +307.4 bits (+198.7 with c=1)
- theta=0.25: regime g0=23; L at g0 = 254.4 (c=1: 179.9); min over g0..32768 = 254.4; gap to rho = +190.6 bits (+116.1 with c=1)

### p = 2^192  (rho on E = 95.83 log2 group ops)

| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |
|---|---|---|---|---|---|---|
| Gaudry basic g!p+p^2 | 2 | 384.0 | 95.8 | +288.2 | +292.2 | g in 2..64 |
| p^(2-2/g) bare | 2 | 192.0 | 95.8 | +96.2 | +100.2 | constants dropped, lower bound on the family |
| g!p+g^2 p^(2-2/g) | 2 | 194.6 | 95.8 | +98.8 | +102.8 | CostModel form |
| L[1/2,sqrt2], g >= ln p | none | n/a | 95.8 | n/a | n/a | in-regime, inside g<=64 |
| L[1/2,sqrt2], unconstrained | 1 | 52.1 | 95.8 | -43.8 | -43.8 | INVALID outside g >= ln p |
| best valid overall | 2 | 192.0 | 95.8 | +96.2 | +100.2 | min of the rows above except unconstrained |

- theta=1: regime g0=134; L at g0 = 852.5 (c=1: 602.8); min over g0..32768 = 852.5; gap to rho = +756.6 bits (+506.9 with c=1)
- theta=0.5: regime g0=67; L at g0 = 581.0 (c=1: 410.9); min over g0..32768 = 581.0; gap to rho = +485.2 bits (+315.0 with c=1)
- theta=0.25: regime g0=34; L at g0 = 398.2 (c=1: 281.6); min over g0..32768 = 398.2; gap to rho = +302.4 bits (+185.7 with c=1)

### p = 2^256  (rho on E = 127.83 log2 group ops)

| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |
|---|---|---|---|---|---|---|
| Gaudry basic g!p+p^2 | 2 | 512.0 | 127.8 | +384.2 | +388.2 | g in 2..64 |
| p^(2-2/g) bare | 2 | 256.0 | 127.8 | +128.2 | +132.2 | constants dropped, lower bound on the family |
| g!p+g^2 p^(2-2/g) | 2 | 258.6 | 127.8 | +130.8 | +134.8 | CostModel form |
| L[1/2,sqrt2], g >= ln p | none | n/a | 127.8 | n/a | n/a | in-regime, inside g<=64 |
| L[1/2,sqrt2], unconstrained | 1 | 61.8 | 127.8 | -66.0 | -66.0 | INVALID outside g >= ln p |
| best valid overall | 2 | 256.0 | 127.8 | +128.2 | +132.2 | min of the rows above except unconstrained |

- theta=1: regime g0=178; L at g0 = 1167.1 (c=1: 825.3); min over g0..32768 = 1167.1; gap to rho = +1039.3 bits (+697.5 with c=1)
- theta=0.5: regime g0=89; L at g0 = 797.2 (c=1: 563.7); min over g0..32768 = 797.2; gap to rho = +669.4 bits (+435.9 with c=1)
- theta=0.25: regime g0=45; L at g0 = 546.5 (c=1: 386.4); min over g0..32768 = 546.5; gap to rho = +418.7 bits (+258.6 with c=1)

### p = 2^521  (rho on E = 260.33 log2 group ops)

| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |
|---|---|---|---|---|---|---|
| Gaudry basic g!p+p^2 | 2 | 1042.0 | 260.3 | +781.7 | +785.7 | g in 2..64 |
| p^(2-2/g) bare | 2 | 521.0 | 260.3 | +260.7 | +264.7 | constants dropped, lower bound on the family |
| g!p+g^2 p^(2-2/g) | 2 | 523.6 | 260.3 | +263.3 | +267.3 | CostModel form |
| L[1/2,sqrt2], g >= ln p | none | n/a | 260.3 | n/a | n/a | in-regime, inside g<=64 |
| L[1/2,sqrt2], unconstrained | 1 | 94.1 | 260.3 | -166.2 | -166.2 | INVALID outside g >= ln p |
| best valid overall | 2 | 521.0 | 260.3 | +260.7 | +264.7 | min of the rows above except unconstrained |

- theta=1: regime g0=362; L at g0 = 2532.0 (c=1: 1790.4); min over g0..32768 = 2532.0; gap to rho = +2271.7 bits (+1530.1 with c=1)
- theta=0.5: regime g0=181; L at g0 = 1736.9 (c=1: 1228.2); min over g0..32768 = 1736.9; gap to rho = +1476.6 bits (+967.9 with c=1)
- theta=0.25: regime g0=91; L at g0 = 1192.8 (c=1: 843.4); min over g0..32768 = 1192.8; gap to rho = +932.5 bits (+583.1 with c=1)

