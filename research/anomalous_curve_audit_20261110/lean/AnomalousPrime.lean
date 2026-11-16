import PrattCertificate

namespace AnomalousPrimeAudit

theorem prime_2 : Nat.Prime 2 := by
  norm_num

theorem prime_3 : Nat.Prime 3 := by
  apply lucas_primality 3 (2 : ZMod 3)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 3-1=2^1 := by norm_num
    rw [hfactor] at hdiv
    have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 hdiv
    subst q
    reduce_mod_char; decide +kernel

theorem prime_5 : Nat.Prime 5 := by
  apply lucas_primality 5 (2 : ZMod 5)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 5-1=2^2 := by norm_num
    rw [hfactor] at hdiv
    have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 hdiv
    subst q
    reduce_mod_char; decide +kernel

theorem prime_7 : Nat.Prime 7 := by
  apply lucas_primality 7 (3 : ZMod 7)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 7-1=2^1 * (3^1) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_11 : Nat.Prime 11 := by
  apply lucas_primality 11 (2 : ZMod 11)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 11-1=2^1 * (5^1) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=5 := Nat.prime_eq_prime_of_dvd_pow hq prime_5 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_13 : Nat.Prime 13 := by
  apply lucas_primality 13 (2 : ZMod 13)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 13-1=2^2 * (3^1) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_17 : Nat.Prime 17 := by
  apply lucas_primality 17 (3 : ZMod 17)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 17-1=2^4 := by norm_num
    rw [hfactor] at hdiv
    have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 hdiv
    subst q
    reduce_mod_char; decide +kernel

theorem prime_23 : Nat.Prime 23 := by
  apply lucas_primality 23 (5 : ZMod 23)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 23-1=2^1 * (11^1) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=11 := Nat.prime_eq_prime_of_dvd_pow hq prime_11 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_29 : Nat.Prime 29 := by
  apply lucas_primality 29 (2 : ZMod 29)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 29-1=2^2 * (7^1) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=7 := Nat.prime_eq_prime_of_dvd_pow hq prime_7 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_37 : Nat.Prime 37 := by
  apply lucas_primality 37 (2 : ZMod 37)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 37-1=2^2 * (3^2) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_43 : Nat.Prime 43 := by
  apply lucas_primality 43 (3 : ZMod 43)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 43-1=2^1 * (3^1 * (7^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=7 := Nat.prime_eq_prime_of_dvd_pow hq prime_7 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_47 : Nat.Prime 47 := by
  apply lucas_primality 47 (5 : ZMod 47)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 47-1=2^1 * (23^1) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=23 := Nat.prime_eq_prime_of_dvd_pow hq prime_23 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_59 : Nat.Prime 59 := by
  apply lucas_primality 59 (2 : ZMod 59)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 59-1=2^1 * (29^1) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=29 := Nat.prime_eq_prime_of_dvd_pow hq prime_29 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_109 : Nat.Prime 109 := by
  apply lucas_primality 109 (6 : ZMod 109)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 109-1=2^2 * (3^3) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_113 : Nat.Prime 113 := by
  apply lucas_primality 113 (3 : ZMod 113)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 113-1=2^4 * (7^1) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=7 := Nat.prime_eq_prime_of_dvd_pow hq prime_7 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_131 : Nat.Prime 131 := by
  apply lucas_primality 131 (2 : ZMod 131)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 131-1=2^1 * (5^1 * (13^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=5 := Nat.prime_eq_prime_of_dvd_pow hq prime_5 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=13 := Nat.prime_eq_prime_of_dvd_pow hq prime_13 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_197 : Nat.Prime 197 := by
  apply lucas_primality 197 (2 : ZMod 197)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 197-1=2^2 * (7^2) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=7 := Nat.prime_eq_prime_of_dvd_pow hq prime_7 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_223 : Nat.Prime 223 := by
  apply lucas_primality 223 (3 : ZMod 223)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 223-1=2^1 * (3^1 * (37^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=37 := Nat.prime_eq_prime_of_dvd_pow hq prime_37 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_443 : Nat.Prime 443 := by
  apply lucas_primality 443 (2 : ZMod 443)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 443-1=2^1 * (13^1 * (17^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=13 := Nat.prime_eq_prime_of_dvd_pow hq prime_13 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=17 := Nat.prime_eq_prime_of_dvd_pow hq prime_17 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_2069 : Nat.Prime 2069 := by
  apply lucas_primality 2069 (2 : ZMod 2069)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 2069-1=2^2 * (11^1 * (47^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=11 := Nat.prime_eq_prime_of_dvd_pow hq prime_11 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=47 := Nat.prime_eq_prime_of_dvd_pow hq prime_47 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_3037 : Nat.Prime 3037 := by
  apply lucas_primality 3037 (2 : ZMod 3037)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 3037-1=2^2 * (3^1 * (11^1 * (23^1))) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=11 := Nat.prime_eq_prime_of_dvd_pow hq prime_11 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=23 := Nat.prime_eq_prime_of_dvd_pow hq prime_23 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_5233 : Nat.Prime 5233 := by
  apply lucas_primality 5233 (10 : ZMod 5233)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 5233-1=2^4 * (3^1 * (109^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=109 := Nat.prime_eq_prime_of_dvd_pow hq prime_109 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_16943 : Nat.Prime 16943 := by
  apply lucas_primality 16943 (5 : ZMod 16943)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 16943-1=2^1 * (43^1 * (197^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=43 := Nat.prime_eq_prime_of_dvd_pow hq prime_43 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=197 := Nat.prime_eq_prime_of_dvd_pow hq prime_197 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_104549 : Nat.Prime 104549 := by
  apply lucas_primality 104549 (2 : ZMod 104549)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 104549-1=2^2 * (59^1 * (443^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=59 := Nat.prime_eq_prime_of_dvd_pow hq prime_59 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=443 := Nat.prime_eq_prime_of_dvd_pow hq prime_443 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_237203 : Nat.Prime 237203 := by
  apply lucas_primality 237203 (2 : ZMod 237203)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 237203-1=2^1 * (7^1 * (16943^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=7 := Nat.prime_eq_prime_of_dvd_pow hq prime_7 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=16943 := Nat.prime_eq_prime_of_dvd_pow hq prime_16943 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_777473 : Nat.Prime 777473 := by
  apply lucas_primality 777473 (3 : ZMod 777473)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 777473-1=2^8 * (3037^1) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3037 := Nat.prime_eq_prime_of_dvd_pow hq prime_3037 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_1297871287 : Nat.Prime 1297871287 := by
  apply lucas_primality 1297871287 (5 : ZMod 1297871287)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 1297871287-1=2^1 * (3^1 * (2069^1 * (104549^1))) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=2069 := Nat.prime_eq_prime_of_dvd_pow hq prime_2069 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=104549 := Nat.prime_eq_prime_of_dvd_pow hq prime_104549 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_17012496829997 : Nat.Prime 17012496829997 := by
  apply lucas_primality 17012496829997 (2 : ZMod 17012496829997)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 17012496829997-1=2^2 * (29^1 * (113^1 * (1297871287^1))) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=29 := Nat.prime_eq_prime_of_dvd_pow hq prime_29 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=113 := Nat.prime_eq_prime_of_dvd_pow hq prime_113 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=1297871287 := Nat.prime_eq_prime_of_dvd_pow hq prime_1297871287 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_308605091485069 : Nat.Prime 308605091485069 := by
  apply lucas_primality 308605091485069 (6 : ZMod 308605091485069)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 308605091485069-1=2^2 * (3^2 * (7^2 * (43^1 * (5233^1 * (777473^1))))) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h | h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=7 := Nat.prime_eq_prime_of_dvd_pow hq prime_7 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=43 := Nat.prime_eq_prime_of_dvd_pow hq prime_43 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=5233 := Nat.prime_eq_prime_of_dvd_pow hq prime_5233 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=777473 := Nat.prime_eq_prime_of_dvd_pow hq prime_777473 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_8914548338918429 : Nat.Prime 8914548338918429 := by
  apply lucas_primality 8914548338918429 (2 : ZMod 8914548338918429)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 8914548338918429-1=2^2 * (131^1 * (17012496829997^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=131 := Nat.prime_eq_prime_of_dvd_pow hq prime_131 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=17012496829997 := Nat.prime_eq_prime_of_dvd_pow hq prime_17012496829997 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_17829096677836859 : Nat.Prime 17829096677836859 := by
  apply lucas_primality 17829096677836859 (2 : ZMod 17829096677836859)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 17829096677836859-1=2^1 * (8914548338918429^1) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=8914548338918429 := Nat.prime_eq_prime_of_dvd_pow hq prime_8914548338918429 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_4314641396036519879 : Nat.Prime 4314641396036519879 := by
  apply lucas_primality 4314641396036519879 (7 : ZMod 4314641396036519879)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 4314641396036519879-1=2^1 * (11^2 * (17829096677836859^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=11 := Nat.prime_eq_prime_of_dvd_pow hq prime_11 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=17829096677836859 := Nat.prime_eq_prime_of_dvd_pow hq prime_17829096677836859 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_4093783532256202499433749 : Nat.Prime 4093783532256202499433749 := by
  apply lucas_primality 4093783532256202499433749 (2 : ZMod 4093783532256202499433749)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 4093783532256202499433749-1=2^2 * (237203^1 * (4314641396036519879^1)) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=237203 := Nat.prime_eq_prime_of_dvd_pow hq prime_237203 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=4314641396036519879 := Nat.prime_eq_prime_of_dvd_pow hq prime_4314641396036519879 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_1769742620994356340505209692701 : Nat.Prime 1769742620994356340505209692701 := by
  apply lucas_primality 1769742620994356340505209692701 (2 : ZMod 1769742620994356340505209692701)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 1769742620994356340505209692701-1=2^2 * (3^1 * (5^2 * (11^1 * (131^1 * (4093783532256202499433749^1))))) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h | h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=5 := Nat.prime_eq_prime_of_dvd_pow hq prime_5 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=11 := Nat.prime_eq_prime_of_dvd_pow hq prime_11 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=131 := Nat.prime_eq_prime_of_dvd_pow hq prime_131 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=4093783532256202499433749 := Nat.prime_eq_prime_of_dvd_pow hq prime_4093783532256202499433749 h
      subst q
      reduce_mod_char; decide +kernel

theorem prime_730750818665451459112596905638433048232067471723 : Nat.Prime 730750818665451459112596905638433048232067471723 := by
  apply lucas_primality 730750818665451459112596905638433048232067471723 (2 : ZMod 730750818665451459112596905638433048232067471723)
  · reduce_mod_char
  · intro q hq hdiv
    have hfactor : 730750818665451459112596905638433048232067471723-1=2^1 * (3^1 * (223^1 * (308605091485069^1 * (1769742620994356340505209692701^1)))) := by norm_num
    rw [hfactor] at hdiv
    simp only [Nat.Prime.dvd_mul hq] at hdiv
    rcases hdiv with h | h | h | h | h
    · have heq : q=2 := Nat.prime_eq_prime_of_dvd_pow hq prime_2 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=3 := Nat.prime_eq_prime_of_dvd_pow hq prime_3 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=223 := Nat.prime_eq_prime_of_dvd_pow hq prime_223 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=308605091485069 := Nat.prime_eq_prime_of_dvd_pow hq prime_308605091485069 h
      subst q
      reduce_mod_char; decide +kernel
    · have heq : q=1769742620994356340505209692701 := Nat.prime_eq_prime_of_dvd_pow hq prime_1769742620994356340505209692701 h
      subst q
      reduce_mod_char; decide +kernel

def p : ℕ := 730750818665451459112596905638433048232067471723
theorem p_prime : Nat.Prime p := prime_730750818665451459112596905638433048232067471723
theorem p_bit_bounds : 2^159≤p ∧ p<2^160 := by norm_num [p]

#print axioms p_prime
#print axioms p_bit_bounds
end AnomalousPrimeAudit
