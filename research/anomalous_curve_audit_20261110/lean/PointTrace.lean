import AffineCertificates

/-! Untrusted candidate coordinates and slopes are bound to actual Mathlib
points. Private theorems package the finite certificate; the final theorem
depends transitively on every referenced proof and is axiom-audited. -/
namespace AnomalousPointTraceAudit
noncomputable section
open AnomalousPrimeAudit AnomalousCurveDataAudit AnomalousAffineCertificateAudit

private def node_0 : formulaCurve.Point :=
  .some (1 : F) (310536468939899693718962354338996655381367569020 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

def G : formulaCurve.Point := node_0
private theorem nsmul_0 : (1 : ℕ) • G=node_0 := by simp [G]

private def node_1 : formulaCurve.Point :=
  .some (136592105715295425798875952213491891668610014425 : F) (613416110811409526078331535659819403857636083969 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_1 : node_0+node_0=node_1 := by
  dsimp only [node_0,node_1]
  apply tangent_double (ℓ:=(465804703409849540578443531508494983072051353530 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_1 : (2 : ℕ) • G=node_1 := by
  rw [show (2 : ℕ)=1+1 by norm_num,
    add_nsmul,nsmul_0]
  exact add_1

private def node_2 : formulaCurve.Point :=
  .some (452207791974347080237101374908943307739827916530 : F) (411388273048196451476937472855048028560443729375 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_2 : node_1+node_1=node_2 := by
  dsimp only [node_1,node_2]
  apply tangent_double (ℓ:=(502773687785924327274022973522445818007685727861 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_2 : (4 : ℕ) • G=node_2 := by
  rw [show (4 : ℕ)=2+2 by norm_num,
    add_nsmul,nsmul_1]
  exact add_2

private def node_3 : formulaCurve.Point :=
  .some (543589241351397075762843955982486467553037168365 : F) (247330925960109983414264716878517109856188271466 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_3 : node_2+node_2=node_3 := by
  dsimp only [node_2,node_3]
  apply tangent_double (ℓ:=(106574556803807753913422679767332910780576356839 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_3 : (8 : ℕ) • G=node_3 := by
  rw [show (8 : ℕ)=4+4 by norm_num,
    add_nsmul,nsmul_2]
  exact add_3

private def node_4 : formulaCurve.Point :=
  .some (681274936587814018291733959367827046611440660410 : F) (178020685725606249804445856907392618511670040935 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_4 : node_3+node_3=node_4 := by
  dsimp only [node_3,node_4]
  apply tangent_double (ℓ:=(113772220443151256620326685526278075313146089707 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_4 : (16 : ℕ) • G=node_4 := by
  rw [show (16 : ℕ)=8+8 by norm_num,
    add_nsmul,nsmul_3]
  exact add_4

private def node_5 : formulaCurve.Point :=
  .some (276722308501312962985777797007896791320575689355 : F) (174538318711333136069624980462716615904098796883 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_5 : node_4+node_4=node_5 := by
  dsimp only [node_4,node_5]
  apply tangent_double (ℓ:=(597263888472287124347714233714199218951384429490 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_5 : (32 : ℕ) • G=node_5 := by
  rw [show (32 : ℕ)=16+16 by norm_num,
    add_nsmul,nsmul_4]
  exact add_5

private def node_6 : formulaCurve.Point :=
  .some (594249545727405419369066390599897509902967550663 : F) (99553983048173504713971262607161466276667193768 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_6 : node_5+node_5=node_6 := by
  dsimp only [node_5,node_6]
  apply tangent_double (ℓ:=(171555002639905814448489255973836831990758102593 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_6 : (64 : ℕ) • G=node_6 := by
  rw [show (64 : ℕ)=32+32 by norm_num,
    add_nsmul,nsmul_5]
  exact add_6

private def node_7 : formulaCurve.Point :=
  .some (557765020419096237252971393701209638271803257494 : F) (457990429056615155189553827181427925965358065557 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_7 : node_6+node_6=node_7 := by
  dsimp only [node_6,node_7]
  apply tangent_double (ℓ:=(523245099900193778735028351438981736313499425829 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_7 : (128 : ℕ) • G=node_7 := by
  rw [show (128 : ℕ)=64+64 by norm_num,
    add_nsmul,nsmul_6]
  exact add_7

private def node_8 : formulaCurve.Point :=
  .some (380535440619682249656142890251511395488200971607 : F) (72549666912806014545129339718165294093997601192 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_8 : node_7+node_7=node_8 := by
  dsimp only [node_7,node_8]
  apply tangent_double (ℓ:=(101320589825781288719990915924365583042810186182 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_8 : (256 : ℕ) • G=node_8 := by
  rw [show (256 : ℕ)=128+128 by norm_num,
    add_nsmul,nsmul_7]
  exact add_8

private def node_9 : formulaCurve.Point :=
  .some (386173725263371317323053020724242439197166515487 : F) (587302890332516210717878901896928384475828123871 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_9 : node_8+node_8=node_9 := by
  dsimp only [node_8,node_9]
  apply tangent_double (ℓ:=(98735112071887151195503947329137252590331169217 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_9 : (512 : ℕ) • G=node_9 := by
  rw [show (512 : ℕ)=256+256 by norm_num,
    add_nsmul,nsmul_8]
  exact add_9

private def node_10 : formulaCurve.Point :=
  .some (485970046379240648859449777848041250962441368776 : F) (14957561558348957182332855134002997923000505365 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_10 : node_9+node_9=node_10 := by
  dsimp only [node_9,node_10]
  apply tangent_double (ℓ:=(7321732000994003621341625913403539362943658608 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_10 : (1024 : ℕ) • G=node_10 := by
  rw [show (1024 : ℕ)=512+512 by norm_num,
    add_nsmul,nsmul_9]
  exact add_10

private def node_11 : formulaCurve.Point :=
  .some (675125574978354513583692068049233885222885262082 : F) (402073120288194974728632223866206372194041878789 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_11 : node_10+node_10=node_11 := by
  dsimp only [node_10,node_11]
  apply tangent_double (ℓ:=(689194473891070795118228065642076450301797315251 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_11 : (2048 : ℕ) • G=node_11 := by
  rw [show (2048 : ℕ)=1024+1024 by norm_num,
    add_nsmul,nsmul_10]
  exact add_11

private def node_12 : formulaCurve.Point :=
  .some (680089531699988047749933825980541986998825096823 : F) (164665820971555877291090930622593568399309491512 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_12 : node_11+node_11=node_12 := by
  dsimp only [node_11,node_12]
  apply tangent_double (ℓ:=(162056313280280670409469237601583876245285200827 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_12 : (4096 : ℕ) • G=node_12 := by
  rw [show (4096 : ℕ)=2048+2048 by norm_num,
    add_nsmul,nsmul_11]
  exact add_12

private def node_13 : formulaCurve.Point :=
  .some (705362197690406960994252247299405796795710910876 : F) (638447381104983847546145728903642856557097618725 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_13 : node_12+node_12=node_13 := by
  dsimp only [node_12,node_13]
  apply tangent_double (ℓ:=(629129269174706525713781261309086457499307300728 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_13 : (8192 : ℕ) • G=node_13 := by
  rw [show (8192 : ℕ)=4096+4096 by norm_num,
    add_nsmul,nsmul_12]
  exact add_13

private def node_14 : formulaCurve.Point :=
  .some (331855765946609193827330953520424849577469099560 : F) (701066999548885893677331479345794080147925390501 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_14 : node_13+node_13=node_14 := by
  dsimp only [node_13,node_14]
  apply tangent_double (ℓ:=(372437625187601300518238214594272212221533797895 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_14 : (16384 : ℕ) • G=node_14 := by
  rw [show (16384 : ℕ)=8192+8192 by norm_num,
    add_nsmul,nsmul_13]
  exact add_14

private def node_15 : formulaCurve.Point :=
  .some (454122774586516088936822236107676954001032141549 : F) (317785086985042871056674421067356545532230597243 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_15 : node_14+node_14=node_15 := by
  dsimp only [node_14,node_15]
  apply tangent_double (ℓ:=(99580601093300265512884198890456169118223068543 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_15 : (32768 : ℕ) • G=node_15 := by
  rw [show (32768 : ℕ)=16384+16384 by norm_num,
    add_nsmul,nsmul_14]
  exact add_15

private def node_16 : formulaCurve.Point :=
  .some (90919669984236070490275173116683364050471061779 : F) (545881102326511752595005282150384005027044396694 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_16 : node_15+node_15=node_16 := by
  dsimp only [node_15,node_16]
  apply tangent_double (ℓ:=(414005311716344093968146801457989558108144443547 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_16 : (65536 : ℕ) • G=node_16 := by
  rw [show (65536 : ℕ)=32768+32768 by norm_num,
    add_nsmul,nsmul_15]
  exact add_16

private def node_17 : formulaCurve.Point :=
  .some (94792483996573362541698081542573486059624779018 : F) (581821305782752744340509415560251645288017732606 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_17 : node_16+node_16=node_17 := by
  dsimp only [node_16,node_17]
  apply tangent_double (ℓ:=(604843229455976147696351327588615475695223911984 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_17 : (131072 : ℕ) • G=node_17 := by
  rw [show (131072 : ℕ)=65536+65536 by norm_num,
    add_nsmul,nsmul_16]
  exact add_17

private def node_18 : formulaCurve.Point :=
  .some (205995808209516467255854732871450465648781350342 : F) (476157434650910852687605808802548842171332174249 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_18 : node_17+node_17=node_18 := by
  dsimp only [node_17,node_18]
  apply tangent_double (ℓ:=(402111059143678453971704612435723675777490022415 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_18 : (262144 : ℕ) • G=node_18 := by
  rw [show (262144 : ℕ)=131072+131072 by norm_num,
    add_nsmul,nsmul_17]
  exact add_18

private def node_19 : formulaCurve.Point :=
  .some (573492912465852932810860523757519464666569321233 : F) (19496466100972269465141586001745556814241320232 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_19 : node_18+node_18=node_19 := by
  dsimp only [node_18,node_19]
  apply tangent_double (ℓ:=(166646991927347550174943706310761115567976324962 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_19 : (524288 : ℕ) • G=node_19 := by
  rw [show (524288 : ℕ)=262144+262144 by norm_num,
    add_nsmul,nsmul_18]
  exact add_19

private def node_20 : formulaCurve.Point :=
  .some (55032585272683879027978704062991722544842885387 : F) (9953952663399574711925671064851768148561929900 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_20 : node_19+node_19=node_20 := by
  dsimp only [node_19,node_20]
  apply tangent_double (ℓ:=(195275586895387160207021248533290305219972258247 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_20 : (1048576 : ℕ) • G=node_20 := by
  rw [show (1048576 : ℕ)=524288+524288 by norm_num,
    add_nsmul,nsmul_19]
  exact add_20

private def node_21 : formulaCurve.Point :=
  .some (360011677558984522114611026052693077161777207228 : F) (43926698751103774318752670852381706899474017838 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_21 : node_20+node_20=node_21 := by
  dsimp only [node_20,node_21]
  apply tangent_double (ℓ:=(78119904115752542563879746379305260120162219580 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_21 : (2097152 : ℕ) • G=node_21 := by
  rw [show (2097152 : ℕ)=1048576+1048576 by norm_num,
    add_nsmul,nsmul_20]
  exact add_21

private def node_22 : formulaCurve.Point :=
  .some (120612212196292683638450929978586871098231297891 : F) (339138684547045247163341890224410777671688086171 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_22 : node_21+node_21=node_22 := by
  dsimp only [node_21,node_22]
  apply tangent_double (ℓ:=(298505154774804635092151808603966735592732844948 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_22 : (4194304 : ℕ) • G=node_22 := by
  rw [show (4194304 : ℕ)=2097152+2097152 by norm_num,
    add_nsmul,nsmul_21]
  exact add_22

private def node_23 : formulaCurve.Point :=
  .some (694834669539316278803836445044203570987008086716 : F) (541746428909931194002118163034492201268512767182 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_23 : node_22+node_22=node_23 := by
  dsimp only [node_22,node_23]
  apply tangent_double (ℓ:=(211544294798919117712290518139631009338818406461 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_23 : (8388608 : ℕ) • G=node_23 := by
  rw [show (8388608 : ℕ)=4194304+4194304 by norm_num,
    add_nsmul,nsmul_22]
  exact add_23

private def node_24 : formulaCurve.Point :=
  .some (694633235813591298657229417930659261468144365015 : F) (392438844226652087984059542330122032646181600835 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_24 : node_23+node_23=node_24 := by
  dsimp only [node_23,node_24]
  apply tangent_double (ℓ:=(11944717485778093081586808720631914145735746425 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_24 : (16777216 : ℕ) • G=node_24 := by
  rw [show (16777216 : ℕ)=8388608+8388608 by norm_num,
    add_nsmul,nsmul_23]
  exact add_24

private def node_25 : formulaCurve.Point :=
  .some (450650070090750652699428122104707768402222731104 : F) (384631500089961227620091496775849625016306277612 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_25 : node_24+node_24=node_25 := by
  dsimp only [node_24,node_25]
  apply tangent_double (ℓ:=(198842217241693347659792266580203758842605238677 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_25 : (33554432 : ℕ) • G=node_25 := by
  rw [show (33554432 : ℕ)=16777216+16777216 by norm_num,
    add_nsmul,nsmul_24]
  exact add_25

private def node_26 : formulaCurve.Point :=
  .some (9880930882016793062334492102126350743968834722 : F) (213361278780478511896619448438140464353904752741 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_26 : node_25+node_25=node_26 := by
  dsimp only [node_25,node_26]
  apply tangent_double (ℓ:=(459327295579332078018813373024287179346324028469 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_26 : (67108864 : ℕ) • G=node_26 := by
  rw [show (67108864 : ℕ)=33554432+33554432 by norm_num,
    add_nsmul,nsmul_25]
  exact add_26

private def node_27 : formulaCurve.Point :=
  .some (236111788698329491869316169011860515428558504542 : F) (156483650984087347706718773896726265629191144166 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_27 : node_26+node_26=node_27 := by
  dsimp only [node_26,node_27]
  apply tangent_double (ℓ:=(568519770772117282637396728021942440597882003709 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_27 : (134217728 : ℕ) • G=node_27 := by
  rw [show (134217728 : ℕ)=67108864+67108864 by norm_num,
    add_nsmul,nsmul_26]
  exact add_27

private def node_28 : formulaCurve.Point :=
  .some (175565864856913687912092488955468770373988113841 : F) (348409431588210206201929842455074908354775300044 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_28 : node_27+node_27=node_28 := by
  dsimp only [node_27,node_28]
  apply tangent_double (ℓ:=(144432867533930583819805595928338140693108707482 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_28 : (268435456 : ℕ) • G=node_28 := by
  rw [show (268435456 : ℕ)=134217728+134217728 by norm_num,
    add_nsmul,nsmul_27]
  exact add_28

private def node_29 : formulaCurve.Point :=
  .some (255547061184611341728519949099601395398641733559 : F) (193630904320180503020786057816950724935221573286 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_29 : node_28+node_28=node_29 := by
  dsimp only [node_28,node_29]
  apply tangent_double (ℓ:=(179093741385069333374853562306380033417009561571 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_29 : (536870912 : ℕ) • G=node_29 := by
  rw [show (536870912 : ℕ)=268435456+268435456 by norm_num,
    add_nsmul,nsmul_28]
  exact add_29

private def node_30 : formulaCurve.Point :=
  .some (503653493212502369447642286180529681414725035289 : F) (485999688284684560691162819232596832983422910457 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_30 : node_29+node_29=node_30 := by
  dsimp only [node_29,node_30]
  apply tangent_double (ℓ:=(299682080702846538870361560189689084605887126859 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_30 : (1073741824 : ℕ) • G=node_30 := by
  rw [show (1073741824 : ℕ)=536870912+536870912 by norm_num,
    add_nsmul,nsmul_29]
  exact add_30

private def node_31 : formulaCurve.Point :=
  .some (247510303158151209694379210338694929459117134280 : F) (463196843369247499627664364366380135395681795789 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_31 : node_30+node_30=node_31 := by
  dsimp only [node_30,node_31]
  apply tangent_double (ℓ:=(585654975409117160220412057827593442172366511599 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_31 : (2147483648 : ℕ) • G=node_31 := by
  rw [show (2147483648 : ℕ)=1073741824+1073741824 by norm_num,
    add_nsmul,nsmul_30]
  exact add_31

private def node_32 : formulaCurve.Point :=
  .some (50785031238881418025262981807317665308994496158 : F) (183747794393117721681762684204754540028658210424 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_32 : node_31+node_31=node_32 := by
  dsimp only [node_31,node_32]
  apply tangent_double (ℓ:=(113230821080930071565400122717901324468813051447 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_32 : (4294967296 : ℕ) • G=node_32 := by
  rw [show (4294967296 : ℕ)=2147483648+2147483648 by norm_num,
    add_nsmul,nsmul_31]
  exact add_32

private def node_33 : formulaCurve.Point :=
  .some (80866757326167731283244958829426927850791188726 : F) (607806842637053062439032332357958750026556456909 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_33 : node_32+node_32=node_33 := by
  dsimp only [node_32,node_33]
  apply tangent_double (ℓ:=(98790534340026129826629329360935601758628514583 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_33 : (8589934592 : ℕ) • G=node_33 := by
  rw [show (8589934592 : ℕ)=4294967296+4294967296 by norm_num,
    add_nsmul,nsmul_32]
  exact add_33

private def node_34 : formulaCurve.Point :=
  .some (190351986700799153831130987139144296028929147404 : F) (307367206629638719566677272434200174004063861238 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_34 : node_33+node_33=node_34 := by
  dsimp only [node_33,node_34]
  apply tangent_double (ℓ:=(591107444393273880161472680577270182184852871258 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_34 : (17179869184 : ℕ) • G=node_34 := by
  rw [show (17179869184 : ℕ)=8589934592+8589934592 by norm_num,
    add_nsmul,nsmul_33]
  exact add_34

private def node_35 : formulaCurve.Point :=
  .some (296550235753865599134646475685791163140042023252 : F) (393896657222824920055295476225927609771970456204 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_35 : node_34+node_34=node_35 := by
  dsimp only [node_34,node_35]
  apply tangent_double (ℓ:=(488452707437827625653593993941019989713541775003 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_35 : (34359738368 : ℕ) • G=node_35 := by
  rw [show (34359738368 : ℕ)=17179869184+17179869184 by norm_num,
    add_nsmul,nsmul_34]
  exact add_35

private def node_36 : formulaCurve.Point :=
  .some (93011890179258174878313667728692106696526941521 : F) (257475442845814749248019722275584808998032942045 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_36 : node_35+node_35=node_36 := by
  dsimp only [node_35,node_36]
  apply tangent_double (ℓ:=(549072836152167585217845555062493577370816108305 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_36 : (68719476736 : ℕ) • G=node_36 := by
  rw [show (68719476736 : ℕ)=34359738368+34359738368 by norm_num,
    add_nsmul,nsmul_35]
  exact add_36

private def node_37 : formulaCurve.Point :=
  .some (650392429866924413866449550432130879930464738903 : F) (41306955280377224610022516484250241954483940232 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_37 : node_36+node_36=node_37 := by
  dsimp only [node_36,node_37]
  apply tangent_double (ℓ:=(632777826926634114099472457517463420860913678445 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_37 : (137438953472 : ℕ) • G=node_37 := by
  rw [show (137438953472 : ℕ)=68719476736+68719476736 by norm_num,
    add_nsmul,nsmul_36]
  exact add_37

private def node_38 : formulaCurve.Point :=
  .some (195289913197090058667076029731998551703544287834 : F) (18218377534601983148175432668288275330203903532 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_38 : node_37+node_37=node_38 := by
  dsimp only [node_37,node_38]
  apply tangent_double (ℓ:=(505401853988478573121976585864708217545296507227 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_38 : (274877906944 : ℕ) • G=node_38 := by
  rw [show (274877906944 : ℕ)=137438953472+137438953472 by norm_num,
    add_nsmul,nsmul_37]
  exact add_38

private def node_39 : formulaCurve.Point :=
  .some (445653174020367561777776506754965769857558007631 : F) (546049760905973627763640589743811044576409771665 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_39 : node_38+node_38=node_39 := by
  dsimp only [node_38,node_39]
  apply tangent_double (ℓ:=(408639105102138744551230298225982359072681879639 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_39 : (549755813888 : ℕ) • G=node_39 := by
  rw [show (549755813888 : ℕ)=274877906944+274877906944 by norm_num,
    add_nsmul,nsmul_38]
  exact add_39

private def node_40 : formulaCurve.Point :=
  .some (421444716235476302368451141051261915549253676565 : F) (120067579941360310896476161421913388116530737851 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_40 : node_39+node_39=node_40 := by
  dsimp only [node_39,node_40]
  apply tangent_double (ℓ:=(501347808024485081216823608350456265537355856885 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_40 : (1099511627776 : ℕ) • G=node_40 := by
  rw [show (1099511627776 : ℕ)=549755813888+549755813888 by norm_num,
    add_nsmul,nsmul_39]
  exact add_40

private def node_41 : formulaCurve.Point :=
  .some (367202296209898714397559068626787495963168795326 : F) (207752100483476754662277365782600967464193165954 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_41 : node_40+node_40=node_41 := by
  dsimp only [node_40,node_41]
  apply tangent_double (ℓ:=(11166515881313262867466650144150759909730442737 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_41 : (2199023255552 : ℕ) • G=node_41 := by
  rw [show (2199023255552 : ℕ)=1099511627776+1099511627776 by norm_num,
    add_nsmul,nsmul_40]
  exact add_41

private def node_42 : formulaCurve.Point :=
  .some (66773372386617751482441577283561221228056050292 : F) (164328931201338900409357103591359228269932184237 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_42 : node_41+node_41=node_42 := by
  dsimp only [node_41,node_42]
  apply tangent_double (ℓ:=(628656155441884859220517290065777184941953960067 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_42 : (4398046511104 : ℕ) • G=node_42 := by
  rw [show (4398046511104 : ℕ)=2199023255552+2199023255552 by norm_num,
    add_nsmul,nsmul_41]
  exact add_42

private def node_43 : formulaCurve.Point :=
  .some (1779998868414651251823863924550266377280667285 : F) (213027076150824325115786349265711125666340828261 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_43 : node_42+node_42=node_43 := by
  dsimp only [node_42,node_43]
  apply tangent_double (ℓ:=(424443252718842132538749363215193659734840103790 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_43 : (8796093022208 : ℕ) • G=node_43 := by
  rw [show (8796093022208 : ℕ)=4398046511104+4398046511104 by norm_num,
    add_nsmul,nsmul_42]
  exact add_43

private def node_44 : formulaCurve.Point :=
  .some (678669799804923628893433767544593390679490184687 : F) (372294122269943026436859536390303162402863136275 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_44 : node_43+node_43=node_44 := by
  dsimp only [node_43,node_44]
  apply tangent_double (ℓ:=(449304064356807929764733351643625535836113577281 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_44 : (17592186044416 : ℕ) • G=node_44 := by
  rw [show (17592186044416 : ℕ)=8796093022208+8796093022208 by norm_num,
    add_nsmul,nsmul_43]
  exact add_44

private def node_45 : formulaCurve.Point :=
  .some (268725053927581742475377204202243764265469515175 : F) (42193358093786233203484428326498963825573655766 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_45 : node_44+node_44=node_45 := by
  dsimp only [node_44,node_45]
  apply tangent_double (ℓ:=(165930886102809017245422044774868433502551980808 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_45 : (35184372088832 : ℕ) • G=node_45 := by
  rw [show (35184372088832 : ℕ)=17592186044416+17592186044416 by norm_num,
    add_nsmul,nsmul_44]
  exact add_45

private def node_46 : formulaCurve.Point :=
  .some (7148520289367243784818695203965396582201824226 : F) (559967088400014154160126893929242100511115506722 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_46 : node_45+node_45=node_46 := by
  dsimp only [node_45,node_46]
  apply tangent_double (ℓ:=(110982209211606041505986614613636066121378340961 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_46 : (70368744177664 : ℕ) • G=node_46 := by
  rw [show (70368744177664 : ℕ)=35184372088832+35184372088832 by norm_num,
    add_nsmul,nsmul_45]
  exact add_46

private def node_47 : formulaCurve.Point :=
  .some (713743896186037852674628365837007235759349814011 : F) (442842110406962208505121513678123106037446559851 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_47 : node_46+node_46=node_47 := by
  dsimp only [node_46,node_47]
  apply tangent_double (ℓ:=(70020003629503989490697991507626140055395879431 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_47 : (140737488355328 : ℕ) • G=node_47 := by
  rw [show (140737488355328 : ℕ)=70368744177664+70368744177664 by norm_num,
    add_nsmul,nsmul_46]
  exact add_47

private def node_48 : formulaCurve.Point :=
  .some (438808805159966807297903328854531351729453010873 : F) (308944913140445432888749237398570797118169983925 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_48 : node_47+node_47=node_48 := by
  dsimp only [node_47,node_48]
  apply tangent_double (ℓ:=(716327914897820996238339652540151022361326793295 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_48 : (281474976710656 : ℕ) • G=node_48 := by
  rw [show (281474976710656 : ℕ)=140737488355328+140737488355328 by norm_num,
    add_nsmul,nsmul_47]
  exact add_48

private def node_49 : formulaCurve.Point :=
  .some (468055355377317381022408048270105531278735244495 : F) (707156498543761118935951050235684761361973812346 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_49 : node_48+node_48=node_49 := by
  dsimp only [node_48,node_49]
  apply tangent_double (ℓ:=(115816726725144030132036217676073509293576547854 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_49 : (562949953421312 : ℕ) • G=node_49 := by
  rw [show (562949953421312 : ℕ)=281474976710656+281474976710656 by norm_num,
    add_nsmul,nsmul_48]
  exact add_49

private def node_50 : formulaCurve.Point :=
  .some (712027955618452105134684804048487839521216183552 : F) (715172635855951742648485131750678704646526153659 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_50 : node_49+node_49=node_50 := by
  dsimp only [node_49,node_50]
  apply tangent_double (ℓ:=(203662970420505519298261972753500705075181546862 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_50 : (1125899906842624 : ℕ) • G=node_50 := by
  rw [show (1125899906842624 : ℕ)=562949953421312+562949953421312 by norm_num,
    add_nsmul,nsmul_49]
  exact add_50

private def node_51 : formulaCurve.Point :=
  .some (233590246085989186691785056435899011966870321078 : F) (277552179832556034388178239188889056621945267095 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_51 : node_50+node_50=node_51 := by
  dsimp only [node_50,node_51]
  apply tangent_double (ℓ:=(368797233512241164196506995623128914587398544968 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_51 : (2251799813685248 : ℕ) • G=node_51 := by
  rw [show (2251799813685248 : ℕ)=1125899906842624+1125899906842624 by norm_num,
    add_nsmul,nsmul_50]
  exact add_51

private def node_52 : formulaCurve.Point :=
  .some (580631821689474686580351899143210614592715683606 : F) (723317791346518663766733493332052686262766318550 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_52 : node_51+node_51=node_52 := by
  dsimp only [node_51,node_52]
  apply tangent_double (ℓ:=(225923140921908995178283654025909448479476172406 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_52 : (4503599627370496 : ℕ) • G=node_52 := by
  rw [show (4503599627370496 : ℕ)=2251799813685248+2251799813685248 by norm_num,
    add_nsmul,nsmul_51]
  exact add_52

private def node_53 : formulaCurve.Point :=
  .some (78982476165223983635200969331178712756341982838 : F) (195860592604170332614783841729980781125912857736 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_53 : node_52+node_52=node_53 := by
  dsimp only [node_52,node_53]
  apply tangent_double (ℓ:=(603932629770011843759203167127082170085923602098 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_53 : (9007199254740992 : ℕ) • G=node_53 := by
  rw [show (9007199254740992 : ℕ)=4503599627370496+4503599627370496 by norm_num,
    add_nsmul,nsmul_52]
  exact add_53

private def node_54 : formulaCurve.Point :=
  .some (160013057015042022122398992233393200773875406259 : F) (522093498229315015157754566440952547302710567978 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_54 : node_53+node_53=node_54 := by
  dsimp only [node_53,node_54]
  apply tangent_double (ℓ:=(332892535481216676226753174091749659475183916193 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_54 : (18014398509481984 : ℕ) • G=node_54 := by
  rw [show (18014398509481984 : ℕ)=9007199254740992+9007199254740992 by norm_num,
    add_nsmul,nsmul_53]
  exact add_54

private def node_55 : formulaCurve.Point :=
  .some (702375479271016110218054378568307114496990319585 : F) (113363118495212659298176422930985191109431786952 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_55 : node_54+node_54=node_55 := by
  dsimp only [node_54,node_55]
  apply tangent_double (ℓ:=(376652427762715536558259700300156301954870076492 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_55 : (36028797018963968 : ℕ) • G=node_55 := by
  rw [show (36028797018963968 : ℕ)=18014398509481984+18014398509481984 by norm_num,
    add_nsmul,nsmul_54]
  exact add_55

private def node_56 : formulaCurve.Point :=
  .some (115440312571719474940553195285521850555419777524 : F) (639992821224694547184051132884794011394683438908 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_56 : node_55+node_55=node_56 := by
  dsimp only [node_55,node_56]
  apply tangent_double (ℓ:=(541639012404124557390516761616376341238971669211 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_56 : (72057594037927936 : ℕ) • G=node_56 := by
  rw [show (72057594037927936 : ℕ)=36028797018963968+36028797018963968 by norm_num,
    add_nsmul,nsmul_55]
  exact add_56

private def node_57 : formulaCurve.Point :=
  .some (278988060663920001714523412523068691880648456924 : F) (131666927522555221960294067436715671782791613989 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_57 : node_56+node_56=node_57 := by
  dsimp only [node_56,node_57]
  apply tangent_double (ℓ:=(51560724758166395250272739850533391889667433292 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_57 : (144115188075855872 : ℕ) • G=node_57 := by
  rw [show (144115188075855872 : ℕ)=72057594037927936+72057594037927936 by norm_num,
    add_nsmul,nsmul_56]
  exact add_57

private def node_58 : formulaCurve.Point :=
  .some (657629101907483037858413490962644086357706438263 : F) (289975879716612838586769972130407479150807585295 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_58 : node_57+node_57=node_58 := by
  dsimp only [node_57,node_58]
  apply tangent_double (ℓ:=(331776751368269031605825228883846985124661325415 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_58 : (288230376151711744 : ℕ) • G=node_58 := by
  rw [show (288230376151711744 : ℕ)=144115188075855872+144115188075855872 by norm_num,
    add_nsmul,nsmul_57]
  exact add_58

private def node_59 : formulaCurve.Point :=
  .some (210329319844702291528089756435954664330417715459 : F) (90293271016592399490498858695553206802458450327 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_59 : node_58+node_58=node_59 := by
  dsimp only [node_58,node_59]
  apply tangent_double (ℓ:=(48775222004144967926900012954859660921482696261 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_59 : (576460752303423488 : ℕ) • G=node_59 := by
  rw [show (576460752303423488 : ℕ)=288230376151711744+288230376151711744 by norm_num,
    add_nsmul,nsmul_58]
  exact add_59

private def node_60 : formulaCurve.Point :=
  .some (500476716033747488156618156736480545487217630197 : F) (65631120271142750594363385729417903934123323365 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_60 : node_59+node_59=node_60 := by
  dsimp only [node_59,node_60]
  apply tangent_double (ℓ:=(531768873077680253758966190587490479277641580168 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_60 : (1152921504606846976 : ℕ) • G=node_60 := by
  rw [show (1152921504606846976 : ℕ)=576460752303423488+576460752303423488 by norm_num,
    add_nsmul,nsmul_59]
  exact add_60

private def node_61 : formulaCurve.Point :=
  .some (301464775715525408498523745290872923019293063932 : F) (29978034291411107124102463714281554521047014640 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_61 : node_60+node_60=node_61 := by
  dsimp only [node_60,node_61]
  apply tangent_double (ℓ:=(133382115504822797356699449305966263761665177019 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_61 : (2305843009213693952 : ℕ) • G=node_61 := by
  rw [show (2305843009213693952 : ℕ)=1152921504606846976+1152921504606846976 by norm_num,
    add_nsmul,nsmul_60]
  exact add_61

private def node_62 : formulaCurve.Point :=
  .some (618519437477652344498764978115317797245203500420 : F) (8849758863654696163408796897309804975001281776 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_62 : node_61+node_61=node_62 := by
  dsimp only [node_61,node_62]
  apply tangent_double (ℓ:=(528037444127927754848678044635229173894442373572 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_62 : (4611686018427387904 : ℕ) • G=node_62 := by
  rw [show (4611686018427387904 : ℕ)=2305843009213693952+2305843009213693952 by norm_num,
    add_nsmul,nsmul_61]
  exact add_62

private def node_63 : formulaCurve.Point :=
  .some (710743883808347042452190068133199323151364928322 : F) (664548443513060356153104174692893809783008150740 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_63 : node_62+node_62=node_63 := by
  dsimp only [node_62,node_63]
  apply tangent_double (ℓ:=(60746306623331716192507581662736945711666005471 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_63 : (9223372036854775808 : ℕ) • G=node_63 := by
  rw [show (9223372036854775808 : ℕ)=4611686018427387904+4611686018427387904 by norm_num,
    add_nsmul,nsmul_62]
  exact add_63

private def node_64 : formulaCurve.Point :=
  .some (189012380878592118204956527804236017267721548320 : F) (539886064607824866546297693431616702872400441312 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_64 : node_63+node_63=node_64 := by
  dsimp only [node_63,node_64]
  apply tangent_double (ℓ:=(125955869186372007336455360587242143955420340415 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_64 : (18446744073709551616 : ℕ) • G=node_64 := by
  rw [show (18446744073709551616 : ℕ)=9223372036854775808+9223372036854775808 by norm_num,
    add_nsmul,nsmul_63]
  exact add_64

private def node_65 : formulaCurve.Point :=
  .some (423105802714950005115921003146697178825416031102 : F) (94490396892132625256250096442241153505666594776 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_65 : node_64+node_64=node_65 := by
  dsimp only [node_64,node_65]
  apply tangent_double (ℓ:=(61394233456656513290432559253492524745173634325 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_65 : (36893488147419103232 : ℕ) • G=node_65 := by
  rw [show (36893488147419103232 : ℕ)=18446744073709551616+18446744073709551616 by norm_num,
    add_nsmul,nsmul_64]
  exact add_65

private def node_66 : formulaCurve.Point :=
  .some (69506963250287722239086375961201363118494579785 : F) (465310903390612115384682004041978512803719933275 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_66 : node_65+node_65=node_66 := by
  dsimp only [node_65,node_66]
  apply tangent_double (ℓ:=(290059662263923538157328489907087995933573353574 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_66 : (73786976294838206464 : ℕ) • G=node_66 := by
  rw [show (73786976294838206464 : ℕ)=36893488147419103232+36893488147419103232 by norm_num,
    add_nsmul,nsmul_65]
  exact add_66

private def node_67 : formulaCurve.Point :=
  .some (26179572589562932698685392906136572631554957602 : F) (94861242248068994068343635810004835139252675964 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_67 : node_66+node_0=node_67 := by
  dsimp only [node_66,node_0,node_67]
  apply secant_add (ℓ:=(718992494830807449725608692546132153354464844234 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_67 : (73786976294838206465 : ℕ) • G=node_67 := by
  rw [show (73786976294838206465 : ℕ)=73786976294838206464+1 by norm_num,
    add_nsmul,nsmul_66,nsmul_0]
  exact add_67

private def node_68 : formulaCurve.Point :=
  .some (614347644156949845081410067464878010173201105726 : F) (394638492546415974870441053096143496388983807257 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_68 : node_67+node_67=node_68 := by
  dsimp only [node_67,node_68]
  apply tangent_double (ℓ:=(445821093882878362325838272271921451588565489731 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_68 : (147573952589676412930 : ℕ) • G=node_68 := by
  rw [show (147573952589676412930 : ℕ)=73786976294838206465+73786976294838206465 by norm_num,
    add_nsmul,nsmul_67]
  exact add_68

private def node_69 : formulaCurve.Point :=
  .some (414147500986473208070202045740312276156611686821 : F) (25721525147508093794390616369519648631553021088 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_69 : node_68+node_68=node_69 := by
  dsimp only [node_68,node_69]
  apply tangent_double (ℓ:=(210427311849919571854850622834672564952400710812 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_69 : (295147905179352825860 : ℕ) • G=node_69 := by
  rw [show (295147905179352825860 : ℕ)=147573952589676412930+147573952589676412930 by norm_num,
    add_nsmul,nsmul_68]
  exact add_69

private def node_70 : formulaCurve.Point :=
  .some (323065409216329559614291573513250755706407848466 : F) (476775824269320064796199129660595031177675011302 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_70 : node_69+node_69=node_70 := by
  dsimp only [node_69,node_70]
  apply tangent_double (ℓ:=(228991262397449497362960520246754811600859698407 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_70 : (590295810358705651720 : ℕ) • G=node_70 := by
  rw [show (590295810358705651720 : ℕ)=295147905179352825860+295147905179352825860 by norm_num,
    add_nsmul,nsmul_69]
  exact add_70

private def node_71 : formulaCurve.Point :=
  .some (604762431663491085122356314434127495284926615809 : F) (411965323532087348855276957974640093233193260277 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_71 : node_70+node_70=node_71 := by
  dsimp only [node_70,node_71]
  apply tangent_double (ℓ:=(547685431673396882762488601487090119535847062256 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_71 : (1180591620717411303440 : ℕ) • G=node_71 := by
  rw [show (1180591620717411303440 : ℕ)=590295810358705651720+590295810358705651720 by norm_num,
    add_nsmul,nsmul_70]
  exact add_71

private def node_72 : formulaCurve.Point :=
  .some (18725730906873592910183655218671474174726339878 : F) (178658027387635097256502677348327998608891574687 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_72 : node_71+node_0=node_72 := by
  dsimp only [node_71,node_0,node_72]
  apply secant_add (ℓ:=(57637991865881931242168246983946726502713670851 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_72 : (1180591620717411303441 : ℕ) • G=node_72 := by
  rw [show (1180591620717411303441 : ℕ)=1180591620717411303440+1 by norm_num,
    add_nsmul,nsmul_71,nsmul_0]
  exact add_72

private def node_73 : formulaCurve.Point :=
  .some (71905425904757228551048294756500199662955495518 : F) (626989184572668817869056362126257051064984048658 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_73 : node_72+node_72=node_73 := by
  dsimp only [node_72,node_73]
  apply tangent_double (ℓ:=(119119127110440698398211911561403466529420835315 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_73 : (2361183241434822606882 : ℕ) • G=node_73 := by
  rw [show (2361183241434822606882 : ℕ)=1180591620717411303441+1180591620717411303441 by norm_num,
    add_nsmul,nsmul_72]
  exact add_73

private def node_74 : formulaCurve.Point :=
  .some (482974203012889214895037051540101733239563968076 : F) (244990186531090384706650200076397595583561505567 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_74 : node_73+node_73=node_74 := by
  dsimp only [node_73,node_74]
  apply tangent_double (ℓ:=(482212525304808480338220556972569088401920115899 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_74 : (4722366482869645213764 : ℕ) • G=node_74 := by
  rw [show (4722366482869645213764 : ℕ)=2361183241434822606882+2361183241434822606882 by norm_num,
    add_nsmul,nsmul_73]
  exact add_74

private def node_75 : formulaCurve.Point :=
  .some (39974794897490646700261193491805185704254903408 : F) (720606487857659356845990686907545405260917935512 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_75 : node_74+node_0=node_75 := by
  dsimp only [node_74,node_0,node_75]
  apply secant_add (ℓ:=(471726663214270553520016176744859549009137719213 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_75 : (4722366482869645213765 : ℕ) • G=node_75 := by
  rw [show (4722366482869645213765 : ℕ)=4722366482869645213764+1 by norm_num,
    add_nsmul,nsmul_74,nsmul_0]
  exact add_75

private def node_76 : formulaCurve.Point :=
  .some (598522647617045979805349369588867106590009779718 : F) (417886300729250209210601692670341225361393306163 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_76 : node_75+node_75=node_76 := by
  dsimp only [node_75,node_76]
  apply tangent_double (ℓ:=(369938204806811601784511922815943840815622523581 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_76 : (9444732965739290427530 : ℕ) • G=node_76 := by
  rw [show (9444732965739290427530 : ℕ)=4722366482869645213765+4722366482869645213765 by norm_num,
    add_nsmul,nsmul_75]
  exact add_76

private def node_77 : formulaCurve.Point :=
  .some (634561859301115844370102429311577553302162298706 : F) (358614334945730904767490371573269397758173364193 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_77 : node_76+node_76=node_77 := by
  dsimp only [node_76,node_77]
  apply tangent_double (ℓ:=(394528701004738702626406130266102754017882336301 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_77 : (18889465931478580855060 : ℕ) • G=node_77 := by
  rw [show (18889465931478580855060 : ℕ)=9444732965739290427530+9444732965739290427530 by norm_num,
    add_nsmul,nsmul_76]
  exact add_77

private def node_78 : formulaCurve.Point :=
  .some (334521190615314898578980157976352132400586502673 : F) (382699132605236732049782228275770127965568199345 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_78 : node_77+node_0=node_78 := by
  dsimp only [node_77,node_0,node_78]
  apply secant_add (ℓ:=(60850948220101643722107199929368103354799113346 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_78 : (18889465931478580855061 : ℕ) • G=node_78 := by
  rw [show (18889465931478580855061 : ℕ)=18889465931478580855060+1 by norm_num,
    add_nsmul,nsmul_77,nsmul_0]
  exact add_78

private def node_79 : formulaCurve.Point :=
  .some (370286597772910817179879790934591777457893834343 : F) (487856519775125394522574561438436554711284228698 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_79 : node_78+node_78=node_79 := by
  dsimp only [node_78,node_79]
  apply tangent_double (ℓ:=(499725584880217429398322732357385828185256463146 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_79 : (37778931862957161710122 : ℕ) • G=node_79 := by
  rw [show (37778931862957161710122 : ℕ)=18889465931478580855061+18889465931478580855061 by norm_num,
    add_nsmul,nsmul_78]
  exact add_79

private def node_80 : formulaCurve.Point :=
  .some (124584594500429399081505540607254931549596503430 : F) (95662098780846506868319933112505669187532015189 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_80 : node_79+node_0=node_80 := by
  dsimp only [node_79,node_0,node_80]
  apply secant_add (ℓ:=(161605713762398355106730276965903655722647799350 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_80 : (37778931862957161710123 : ℕ) • G=node_80 := by
  rw [show (37778931862957161710123 : ℕ)=37778931862957161710122+1 by norm_num,
    add_nsmul,nsmul_79,nsmul_0]
  exact add_80

private def node_81 : formulaCurve.Point :=
  .some (695912684830259475936874464093856271788117334910 : F) (407495778029111223681575051557306021889147094710 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_81 : node_80+node_80=node_81 := by
  dsimp only [node_80,node_81]
  apply tangent_double (ℓ:=(69527910169801437399324528871174126855727308833 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_81 : (75557863725914323420246 : ℕ) • G=node_81 := by
  rw [show (75557863725914323420246 : ℕ)=37778931862957161710123+37778931862957161710123 by norm_num,
    add_nsmul,nsmul_80]
  exact add_81

private def node_82 : formulaCurve.Point :=
  .some (595986831514792615115103200317002284526268858199 : F) (76685277601413284732904471528747147385231856983 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_82 : node_81+node_0=node_82 := by
  dsimp only [node_81,node_0,node_82]
  apply secant_add (ℓ:=(604642571429594834726551000059547503220453593516 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_82 : (75557863725914323420247 : ℕ) • G=node_82 := by
  rw [show (75557863725914323420247 : ℕ)=75557863725914323420246+1 by norm_num,
    add_nsmul,nsmul_81,nsmul_0]
  exact add_82

private def node_83 : formulaCurve.Point :=
  .some (562900944715143245029610469327241458147271353897 : F) (513258113793304727939519982874269263216597104855 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_83 : node_82+node_82=node_83 := by
  dsimp only [node_82,node_83]
  apply tangent_double (ℓ:=(280402090375168166962084067953282946539978985072 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_83 : (151115727451828646840494 : ℕ) • G=node_83 := by
  rw [show (151115727451828646840494 : ℕ)=75557863725914323420247+75557863725914323420247 by norm_num,
    add_nsmul,nsmul_82]
  exact add_83

private def node_84 : formulaCurve.Point :=
  .some (723838248721612305060097684177023158986882917038 : F) (188853222465940449299906052380203828224964356166 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_84 : node_83+node_0=node_84 := by
  dsimp only [node_83,node_0,node_84]
  apply secant_add (ℓ:=(476093178295525959210392684076696697258634626950 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_84 : (151115727451828646840495 : ℕ) • G=node_84 := by
  rw [show (151115727451828646840495 : ℕ)=151115727451828646840494+1 by norm_num,
    add_nsmul,nsmul_83,nsmul_0]
  exact add_84

private def node_85 : formulaCurve.Point :=
  .some (483418425775816396789485644883732600759924110290 : F) (215745670205423025235283479602231819821308581608 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_85 : node_84+node_84=node_85 := by
  dsimp only [node_84,node_85]
  apply tangent_double (ℓ:=(581060380449427655395025638699788203486781603333 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_85 : (302231454903657293680990 : ℕ) • G=node_85 := by
  rw [show (302231454903657293680990 : ℕ)=151115727451828646840495+151115727451828646840495 by norm_num,
    add_nsmul,nsmul_84]
  exact add_85

private def node_86 : formulaCurve.Point :=
  .some (582210382172799916434386903975917684849590171407 : F) (181500263326930532450848663670204016120028296732 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_86 : node_85+node_0=node_86 := by
  dsimp only [node_85,node_0,node_86]
  apply secant_add (ℓ:=(190606533989005664489371476182702157993019542052 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_86 : (302231454903657293680991 : ℕ) • G=node_86 := by
  rw [show (302231454903657293680991 : ℕ)=302231454903657293680990+1 by norm_num,
    add_nsmul,nsmul_85,nsmul_0]
  exact add_86

private def node_87 : formulaCurve.Point :=
  .some (561228803405046758478633776061206720126788233292 : F) (612628940918339628198293846983196767664493164759 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_87 : node_86+node_86=node_87 := by
  dsimp only [node_86,node_87]
  apply tangent_double (ℓ:=(587004932296009441725255309531588541301789149507 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_87 : (604462909807314587361982 : ℕ) • G=node_87 := by
  rw [show (604462909807314587361982 : ℕ)=302231454903657293680991+302231454903657293680991 by norm_num,
    add_nsmul,nsmul_86]
  exact add_87

private def node_88 : formulaCurve.Point :=
  .some (593591894690004719394476096676021412983127334805 : F) (606403652650446096008425662412336044685338491253 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_88 : node_87+node_0=node_88 := by
  dsimp only [node_87,node_0,node_88]
  apply secant_add (ℓ:=(313382566263853291088581833483593767707117611882 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_88 : (604462909807314587361983 : ℕ) • G=node_88 := by
  rw [show (604462909807314587361983 : ℕ)=604462909807314587361982+1 by norm_num,
    add_nsmul,nsmul_87,nsmul_0]
  exact add_88

private def node_89 : formulaCurve.Point :=
  .some (182130781701984734064356524502534242442630104613 : F) (4296190200847046000995756432858762319907047354 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_89 : node_88+node_88=node_89 := by
  dsimp only [node_88,node_89]
  apply tangent_double (ℓ:=(450251984600845895246286395148924397674790074679 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_89 : (1208925819614629174723966 : ℕ) • G=node_89 := by
  rw [show (1208925819614629174723966 : ℕ)=604462909807314587361983+604462909807314587361983 by norm_num,
    add_nsmul,nsmul_88]
  exact add_89

private def node_90 : formulaCurve.Point :=
  .some (50823445823463901827849373737717480640340032062 : F) (29298371260721592676773063435532057530130728278 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_90 : node_89+node_0=node_90 := by
  dsimp only [node_89,node_0,node_90]
  apply secant_add (ℓ:=(372205354351746348338787416631627209022901981051 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_90 : (1208925819614629174723967 : ℕ) • G=node_90 := by
  rw [show (1208925819614629174723967 : ℕ)=1208925819614629174723966+1 by norm_num,
    add_nsmul,nsmul_89,nsmul_0]
  exact add_90

private def node_91 : formulaCurve.Point :=
  .some (69898910158574273707549234900824476664216843254 : F) (604590071299434004514926474641757961336840812678 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_91 : node_90+node_90=node_91 := by
  dsimp only [node_90,node_91]
  apply tangent_double (ℓ:=(308707439707756705803227884972073404718795791154 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_91 : (2417851639229258349447934 : ℕ) • G=node_91 := by
  rw [show (2417851639229258349447934 : ℕ)=1208925819614629174723967+1208925819614629174723967 by norm_num,
    add_nsmul,nsmul_90]
  exact add_91

private def node_92 : formulaCurve.Point :=
  .some (540713180554923007382175063170657694194753706615 : F) (414373885495471511379726233405940805518429043270 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_92 : node_91+node_0=node_92 := by
  dsimp only [node_91,node_0,node_92]
  apply secant_add (ℓ:=(261598067280338317021755481164136440651486594015 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_92 : (2417851639229258349447935 : ℕ) • G=node_92 := by
  rw [show (2417851639229258349447935 : ℕ)=2417851639229258349447934+1 by norm_num,
    add_nsmul,nsmul_91,nsmul_0]
  exact add_92

private def node_93 : formulaCurve.Point :=
  .some (552078489699523731855222236116157716379253141021 : F) (21402078134944936626353849703372247574445767563 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_93 : node_92+node_92=node_93 := by
  dsimp only [node_92,node_93]
  apply tangent_double (ℓ:=(304973531652664093766814048549027668145275260479 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_93 : (4835703278458516698895870 : ℕ) • G=node_93 := by
  rw [show (4835703278458516698895870 : ℕ)=2417851639229258349447935+2417851639229258349447935 by norm_num,
    add_nsmul,nsmul_92]
  exact add_93

private def node_94 : formulaCurve.Point :=
  .some (522436987279542546099315228519350212314284679850 : F) (335499260129298292582267707123021749382514886254 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_94 : node_93+node_0=node_94 := by
  dsimp only [node_93,node_0,node_94]
  apply secant_add (ℓ:=(117189505549616431509578269708492988634237422288 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_94 : (4835703278458516698895871 : ℕ) • G=node_94 := by
  rw [show (4835703278458516698895871 : ℕ)=4835703278458516698895870+1 by norm_num,
    add_nsmul,nsmul_93,nsmul_0]
  exact add_94

private def node_95 : formulaCurve.Point :=
  .some (700593842925028548320146038471362274632046476820 : F) (626595358396174616216643488783107179391307292928 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_95 : node_94+node_94=node_95 := by
  dsimp only [node_94,node_95]
  apply tangent_double (ℓ:=(657098778656753869495459014318533558320285100070 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_95 : (9671406556917033397791742 : ℕ) • G=node_95 := by
  rw [show (9671406556917033397791742 : ℕ)=4835703278458516698895871+4835703278458516698895871 by norm_num,
    add_nsmul,nsmul_94]
  exact add_95

private def node_96 : formulaCurve.Point :=
  .some (266891297881548693684872772666623087794194345869 : F) (133786439759741021058195393765476664887115568224 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_96 : node_95+node_95=node_96 := by
  dsimp only [node_95,node_96]
  apply tangent_double (ℓ:=(629542595639408941904223100805846027688161647870 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_96 : (19342813113834066795583484 : ℕ) • G=node_96 := by
  rw [show (19342813113834066795583484 : ℕ)=9671406556917033397791742+9671406556917033397791742 by norm_num,
    add_nsmul,nsmul_95]
  exact add_96

private def node_97 : formulaCurve.Point :=
  .some (196892250499030323411668874409903633107118954089 : F) (390104352656470700150338622643840265400569964004 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_97 : node_96+node_96=node_97 := by
  dsimp only [node_96,node_97]
  apply tangent_double (ℓ:=(536431189319133138951115912017375157521982181882 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_97 : (38685626227668133591166968 : ℕ) • G=node_97 := by
  rw [show (38685626227668133591166968 : ℕ)=19342813113834066795583484+19342813113834066795583484 by norm_num,
    add_nsmul,nsmul_96]
  exact add_97

private def node_98 : formulaCurve.Point :=
  .some (375080178690042548440525184638154603485895596293 : F) (204509549039735836168389658757783677929667767048 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_98 : node_97+node_0=node_98 := by
  dsimp only [node_97,node_0,node_98]
  apply secant_add (ℓ:=(193048906888340066159212410734294091342851139842 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_98 : (38685626227668133591166969 : ℕ) • G=node_98 := by
  rw [show (38685626227668133591166969 : ℕ)=38685626227668133591166968+1 by norm_num,
    add_nsmul,nsmul_97,nsmul_0]
  exact add_98

private def node_99 : formulaCurve.Point :=
  .some (280767422088782035673700526324507370217157821258 : F) (234533048571356970195242322057736871286846768153 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_99 : node_98+node_98=node_99 := by
  dsimp only [node_98,node_99]
  apply tangent_double (ℓ:=(277653416005863494582814027003521446196525011592 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_99 : (77371252455336267182333938 : ℕ) • G=node_99 := by
  rw [show (77371252455336267182333938 : ℕ)=38685626227668133591166969+38685626227668133591166969 by norm_num,
    add_nsmul,nsmul_98]
  exact add_99

private def node_100 : formulaCurve.Point :=
  .some (383152774551992500595092223553349109871001639804 : F) (234158671497039507363744100622250977763628988339 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_100 : node_99+node_0=node_100 := by
  dsimp only [node_99,node_0,node_100]
  apply secant_add (ℓ:=(636728888332327483308663871108109983359246928599 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_100 : (77371252455336267182333939 : ℕ) • G=node_100 := by
  rw [show (77371252455336267182333939 : ℕ)=77371252455336267182333938+1 by norm_num,
    add_nsmul,nsmul_99,nsmul_0]
  exact add_100

private def node_101 : formulaCurve.Point :=
  .some (520842488208100350081578415434978548716149958060 : F) (448296489958356919084427057016042788568005737625 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_101 : node_100+node_100=node_101 := by
  dsimp only [node_100,node_101]
  apply tangent_double (ℓ:=(398720711775155374446626278286174328243584116859 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_101 : (154742504910672534364667878 : ℕ) • G=node_101 := by
  rw [show (154742504910672534364667878 : ℕ)=77371252455336267182333939+77371252455336267182333939 by norm_num,
    add_nsmul,nsmul_100]
  exact add_101

private def node_102 : formulaCurve.Point :=
  .some (68398101511703823305222930683089554579687099175 : F) (126077866325214265969064952760268825466899541418 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_102 : node_101+node_0=node_102 := by
  dsimp only [node_101,node_0,node_102]
  apply secant_add (ℓ:=(288922879674076797300499509408613739088063141121 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_102 : (154742504910672534364667879 : ℕ) • G=node_102 := by
  rw [show (154742504910672534364667879 : ℕ)=154742504910672534364667878+1 by norm_num,
    add_nsmul,nsmul_101,nsmul_0]
  exact add_102

private def node_103 : formulaCurve.Point :=
  .some (368652346056573653452579870905712694867890920193 : F) (675059316289004447527658638115400794323683660477 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_103 : node_102+node_102=node_103 := by
  dsimp only [node_102,node_103]
  apply tangent_double (ℓ:=(174132843588619091408449997848929223283111679961 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_103 : (309485009821345068729335758 : ℕ) • G=node_103 := by
  rw [show (309485009821345068729335758 : ℕ)=154742504910672534364667879+154742504910672534364667879 by norm_num,
    add_nsmul,nsmul_102]
  exact add_103

private def node_104 : formulaCurve.Point :=
  .some (9540364316014548775428194177354594699530396412 : F) (225763492159956525441860436810742959582199925802 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_104 : node_103+node_0=node_104 := by
  dsimp only [node_103,node_0,node_104]
  apply secant_add (ℓ:=(611011541901033420008458823449152289469890219743 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_104 : (309485009821345068729335759 : ℕ) • G=node_104 := by
  rw [show (309485009821345068729335759 : ℕ)=309485009821345068729335758+1 by norm_num,
    add_nsmul,nsmul_103,nsmul_0]
  exact add_104

private def node_105 : formulaCurve.Point :=
  .some (76389355679918905701215126503659634379142495005 : F) (77153468639545090989305031788266871490608345531 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_105 : node_104+node_104=node_105 := by
  dsimp only [node_104,node_105]
  apply tangent_double (ℓ:=(37359331003306459363721741132552295165439883250 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_105 : (618970019642690137458671518 : ℕ) • G=node_105 := by
  rw [show (618970019642690137458671518 : ℕ)=309485009821345068729335759+309485009821345068729335759 by norm_num,
    add_nsmul,nsmul_104]
  exact add_105

private def node_106 : formulaCurve.Point :=
  .some (517278317753033394832643871177346868237836247256 : F) (44463615999321173016123273126335758304665606154 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_106 : node_105+node_105=node_106 := by
  dsimp only [node_105,node_106]
  apply tangent_double (ℓ:=(677173108002133777907866516411109301636698123334 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_106 : (1237940039285380274917343036 : ℕ) • G=node_106 := by
  rw [show (1237940039285380274917343036 : ℕ)=618970019642690137458671518+618970019642690137458671518 by norm_num,
    add_nsmul,nsmul_105]
  exact add_106

private def node_107 : formulaCurve.Point :=
  .some (67508271373209017328739330339856609555224590473 : F) (670143276862097146197085707109520406938596512019 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_107 : node_106+node_0=node_107 := by
  dsimp only [node_106,node_0,node_107]
  apply secant_add (ℓ:=(640601240400151806573679577345917632878153616623 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_107 : (1237940039285380274917343037 : ℕ) • G=node_107 := by
  rw [show (1237940039285380274917343037 : ℕ)=1237940039285380274917343036+1 by norm_num,
    add_nsmul,nsmul_106,nsmul_0]
  exact add_107

private def node_108 : formulaCurve.Point :=
  .some (372551431036203374304004378464278343747298834469 : F) (608873907183361745735239741649764475496748353844 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_108 : node_107+node_107=node_108 := by
  dsimp only [node_107,node_108]
  apply tangent_double (ℓ:=(513271796090326426339212397036543076436372670930 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_108 : (2475880078570760549834686074 : ℕ) • G=node_108 := by
  rw [show (2475880078570760549834686074 : ℕ)=1237940039285380274917343037+1237940039285380274917343037 by norm_num,
    add_nsmul,nsmul_107]
  exact add_108

private def node_109 : formulaCurve.Point :=
  .some (598320756789882512427707425384971881424817912633 : F) (27219363228051967767870029649825218887215197415 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_109 : node_108+node_0=node_109 := by
  dsimp only [node_108,node_0,node_109]
  apply secant_add (ℓ:=(501311679181173858320465875684874966464544273607 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_109 : (2475880078570760549834686075 : ℕ) • G=node_109 := by
  rw [show (2475880078570760549834686075 : ℕ)=2475880078570760549834686074+1 by norm_num,
    add_nsmul,nsmul_108,nsmul_0]
  exact add_109

private def node_110 : formulaCurve.Point :=
  .some (541502038188574422862443477168674738976247922092 : F) (109689483636432150045582743358510342608576727633 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_110 : node_109+node_109=node_110 := by
  dsimp only [node_109,node_110]
  apply tangent_double (ℓ:=(15513781782947579588094908323784118248667034350 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_110 : (4951760157141521099669372150 : ℕ) • G=node_110 := by
  rw [show (4951760157141521099669372150 : ℕ)=2475880078570760549834686075+2475880078570760549834686075 by norm_num,
    add_nsmul,nsmul_109]
  exact add_110

private def node_111 : formulaCurve.Point :=
  .some (122116953608733696192566596010636648653272499936 : F) (454915599558088539755491453388366287127483208276 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_111 : node_110+node_110=node_111 := by
  dsimp only [node_110,node_111]
  apply tangent_double (ℓ:=(690265975956951835081482986629269624896137631813 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_111 : (9903520314283042199338744300 : ℕ) • G=node_111 := by
  rw [show (9903520314283042199338744300 : ℕ)=4951760157141521099669372150+4951760157141521099669372150 by norm_num,
    add_nsmul,nsmul_110]
  exact add_111

private def node_112 : formulaCurve.Point :=
  .some (486371385045524893014589426545573980392302298590 : F) (624011918580758359952049390182762559569965341733 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_112 : node_111+node_111=node_112 := by
  dsimp only [node_111,node_112]
  apply tangent_double (ℓ:=(11559786823354777025147353246216876479242967804 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_112 : (19807040628566084398677488600 : ℕ) • G=node_112 := by
  rw [show (19807040628566084398677488600 : ℕ)=9903520314283042199338744300+9903520314283042199338744300 by norm_num,
    add_nsmul,nsmul_111]
  exact add_112

private def node_113 : formulaCurve.Point :=
  .some (124439640215526957339742617465208747055131370758 : F) (335930322062339056288514669788614479786553198352 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_113 : node_112+node_112=node_113 := by
  dsimp only [node_112,node_113]
  apply tangent_double (ℓ:=(23326942612934384831496265331337032418996735223 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_113 : (39614081257132168797354977200 : ℕ) • G=node_113 := by
  rw [show (39614081257132168797354977200 : ℕ)=19807040628566084398677488600+19807040628566084398677488600 by norm_num,
    add_nsmul,nsmul_112]
  exact add_113

private def node_114 : formulaCurve.Point :=
  .some (576011683905279123005925517894049022798669675485 : F) (15725045419079215284286788884578303565616372144 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_114 : node_113+node_113=node_114 := by
  dsimp only [node_113,node_114]
  apply tangent_double (ℓ:=(553435188598747742732595948588642787959321480832 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_114 : (79228162514264337594709954400 : ℕ) • G=node_114 := by
  rw [show (79228162514264337594709954400 : ℕ)=39614081257132168797354977200+39614081257132168797354977200 by norm_num,
    add_nsmul,nsmul_113]
  exact add_114

private def node_115 : formulaCurve.Point :=
  .some (702571842509316946285153155016460777220794919493 : F) (179673504540003481167901256759486457771540224637 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_115 : node_114+node_114=node_115 := by
  dsimp only [node_114,node_115]
  apply tangent_double (ℓ:=(504829653894816630141636282542515812794837937303 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_115 : (158456325028528675189419908800 : ℕ) • G=node_115 := by
  rw [show (158456325028528675189419908800 : ℕ)=79228162514264337594709954400+79228162514264337594709954400 by norm_num,
    add_nsmul,nsmul_114]
  exact add_115

private def node_116 : formulaCurve.Point :=
  .some (601666044598292915799367855534216523521903885808 : F) (641032651064956775186799695892606398411452919451 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_116 : node_115+node_115=node_116 := by
  dsimp only [node_115,node_116]
  apply tangent_double (ℓ:=(442155971683797887333329391946563460254725528009 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_116 : (316912650057057350378839817600 : ℕ) • G=node_116 := by
  rw [show (316912650057057350378839817600 : ℕ)=158456325028528675189419908800+158456325028528675189419908800 by norm_num,
    add_nsmul,nsmul_115]
  exact add_116

private def node_117 : formulaCurve.Point :=
  .some (507938681993202176426154316715595572798821122585 : F) (37446218357279654692462849103626836750294369221 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_117 : node_116+node_116=node_117 := by
  dsimp only [node_116,node_117]
  apply tangent_double (ℓ:=(366432448734646375929549898213273769509467253765 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_117 : (633825300114114700757679635200 : ℕ) • G=node_117 := by
  rw [show (633825300114114700757679635200 : ℕ)=316912650057057350378839817600+316912650057057350378839817600 by norm_num,
    add_nsmul,nsmul_116]
  exact add_117

private def node_118 : formulaCurve.Point :=
  .some (505294105753593767650224574997686658661512068664 : F) (676136978360178919561803102357019668469434481939 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_118 : node_117+node_0=node_118 := by
  dsimp only [node_117,node_0,node_118]
  apply secant_add (ℓ:=(679144949330890091098445572834075601852021109918 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_118 : (633825300114114700757679635201 : ℕ) • G=node_118 := by
  rw [show (633825300114114700757679635201 : ℕ)=633825300114114700757679635200+1 by norm_num,
    add_nsmul,nsmul_117,nsmul_0]
  exact add_118

private def node_119 : formulaCurve.Point :=
  .some (218860362698748630814917692966301072400102487787 : F) (369471618790895726168982927594813377137440040685 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_119 : node_118+node_118=node_119 := by
  dsimp only [node_118,node_119]
  apply tangent_double (ℓ:=(93715299219570200636910283767462830949223745483 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_119 : (1267650600228229401515359270402 : ℕ) • G=node_119 := by
  rw [show (1267650600228229401515359270402 : ℕ)=633825300114114700757679635201+633825300114114700757679635201 by norm_num,
    add_nsmul,nsmul_118]
  exact add_119

private def node_120 : formulaCurve.Point :=
  .some (197798839035833825714529331203303448223304437702 : F) (37445212584019180065599283728320520087874308001 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_120 : node_119+node_119=node_120 := by
  dsimp only [node_119,node_120]
  apply tangent_double (ℓ:=(129548852265161146778671561094568790805385618994 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_120 : (2535301200456458803030718540804 : ℕ) • G=node_120 := by
  rw [show (2535301200456458803030718540804 : ℕ)=1267650600228229401515359270402+1267650600228229401515359270402 by norm_num,
    add_nsmul,nsmul_119]
  exact add_120

private def node_121 : formulaCurve.Point :=
  .some (333687521404287357442487033683836146812364167822 : F) (403986294409241908951623633447567261205638390519 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_121 : node_120+node_0=node_121 := by
  dsimp only [node_120,node_0,node_121]
  apply secant_add (ℓ:=(454955492053026209675627820394320438190192547697 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_121 : (2535301200456458803030718540805 : ℕ) • G=node_121 := by
  rw [show (2535301200456458803030718540805 : ℕ)=2535301200456458803030718540804+1 by norm_num,
    add_nsmul,nsmul_120,nsmul_0]
  exact add_121

private def node_122 : formulaCurve.Point :=
  .some (665080848590474622785981510193903734929512206728 : F) (65841138259709714128270032091493634847694993689 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_122 : node_121+node_121=node_122 := by
  dsimp only [node_121,node_122]
  apply tangent_double (ℓ:=(150993894926816667758350315051026078149221654355 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_122 : (5070602400912917606061437081610 : ℕ) • G=node_122 := by
  rw [show (5070602400912917606061437081610 : ℕ)=2535301200456458803030718540805+2535301200456458803030718540805 by norm_num,
    add_nsmul,nsmul_121]
  exact add_122

private def node_123 : formulaCurve.Point :=
  .some (625820322181496574207565668992475327207642558183 : F) (197466658378339045424427775860205363209154034270 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_123 : node_122+node_122=node_123 := by
  dsimp only [node_122,node_123]
  apply tangent_double (ℓ:=(370168714505824542342213821645662404305783463287 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_123 : (10141204801825835212122874163220 : ℕ) • G=node_123 := by
  rw [show (10141204801825835212122874163220 : ℕ)=5070602400912917606061437081610+5070602400912917606061437081610 by norm_num,
    add_nsmul,nsmul_122]
  exact add_123

private def node_124 : formulaCurve.Point :=
  .some (92096764884040430123031215341299068730299841213 : F) (442603352009729001048247179334253262642003243217 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_124 : node_123+node_0=node_124 := by
  dsimp only [node_123,node_0,node_124]
  apply secant_add (ℓ:=(240593936458477549511681040426336193485489565136 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_124 : (10141204801825835212122874163221 : ℕ) • G=node_124 := by
  rw [show (10141204801825835212122874163221 : ℕ)=10141204801825835212122874163220+1 by norm_num,
    add_nsmul,nsmul_123,nsmul_0]
  exact add_124

private def node_125 : formulaCurve.Point :=
  .some (329016241460942454146481635958492638320195607263 : F) (275528941094019040387036265876012591839695254922 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_125 : node_124+node_124=node_125 := by
  dsimp only [node_124,node_125]
  apply tangent_double (ℓ:=(577917694434879100679212224839521820647784932833 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_125 : (20282409603651670424245748326442 : ℕ) • G=node_125 := by
  rw [show (20282409603651670424245748326442 : ℕ)=10141204801825835212122874163221+10141204801825835212122874163221 by norm_num,
    add_nsmul,nsmul_124]
  exact add_125

private def node_126 : formulaCurve.Point :=
  .some (456703521956043925841491309386164023535996362825 : F) (273053145424728675682532992250059494432594662588 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_126 : node_125+node_125=node_126 := by
  dsimp only [node_125,node_126]
  apply tangent_double (ℓ:=(653757132088412524275573493835322916420922631686 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_126 : (40564819207303340848491496652884 : ℕ) • G=node_126 := by
  rw [show (40564819207303340848491496652884 : ℕ)=20282409603651670424245748326442+20282409603651670424245748326442 by norm_num,
    add_nsmul,nsmul_125]
  exact add_126

private def node_127 : formulaCurve.Point :=
  .some (7515298875761085400186424098915338861808746555 : F) (437557432227714538908868100960137849268518449077 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_127 : node_126+node_0=node_127 := by
  dsimp only [node_126,node_0,node_127]
  apply secant_add (ℓ:=(514507492935332180603192795880174044475091044534 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_127 : (40564819207303340848491496652885 : ℕ) • G=node_127 := by
  rw [show (40564819207303340848491496652885 : ℕ)=40564819207303340848491496652884+1 by norm_num,
    add_nsmul,nsmul_126,nsmul_0]
  exact add_127

private def node_128 : formulaCurve.Point :=
  .some (411274385345531152720750554357129731771838042237 : F) (617189943801790239022100570648853835473360643240 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_128 : node_127+node_127=node_128 := by
  dsimp only [node_127,node_128]
  apply tangent_double (ℓ:=(649260226570325438872369507509182674055331066671 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_128 : (81129638414606681696982993305770 : ℕ) • G=node_128 := by
  rw [show (81129638414606681696982993305770 : ℕ)=40564819207303340848491496652885+40564819207303340848491496652885 by norm_num,
    add_nsmul,nsmul_127]
  exact add_128

private def node_129 : formulaCurve.Point :=
  .some (541782150446421198940495523567023787454346742187 : F) (455060928851677152664729616088006827491512513046 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_129 : node_128+node_128=node_129 := by
  dsimp only [node_128,node_129]
  apply tangent_double (ℓ:=(661205208631351866365465237416623105297592342007 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_129 : (162259276829213363393965986611540 : ℕ) • G=node_129 := by
  rw [show (162259276829213363393965986611540 : ℕ)=81129638414606681696982993305770+81129638414606681696982993305770 by norm_num,
    add_nsmul,nsmul_128]
  exact add_129

private def node_130 : formulaCurve.Point :=
  .some (51556874872681953724710525242888234936828589593 : F) (654310210341730116036542210811797823874087829462 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_130 : node_129+node_129=node_130 := by
  dsimp only [node_129,node_130]
  apply tangent_double (ℓ:=(663881873682715858779442714444526715629788929582 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_130 : (324518553658426726787931973223080 : ℕ) • G=node_130 := by
  rw [show (324518553658426726787931973223080 : ℕ)=162259276829213363393965986611540+162259276829213363393965986611540 by norm_num,
    add_nsmul,nsmul_129]
  exact add_130

private def node_131 : formulaCurve.Point :=
  .some (191498571807215669990948820740228122587496559009 : F) (316782939863145767933951150094144326249874196229 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_131 : node_130+node_0=node_131 := by
  dsimp only [node_130,node_0,node_131]
  apply secant_add (ℓ:=(665072364655910629748396251165239105491413662224 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_131 : (324518553658426726787931973223081 : ℕ) • G=node_131 := by
  rw [show (324518553658426726787931973223081 : ℕ)=324518553658426726787931973223080+1 by norm_num,
    add_nsmul,nsmul_130,nsmul_0]
  exact add_131

private def node_132 : formulaCurve.Point :=
  .some (293010726972986837805717802971198078497127609614 : F) (316591196957456476567050313325139170728305538776 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_132 : node_131+node_131=node_132 := by
  dsimp only [node_131,node_132]
  apply tangent_double (ℓ:=(408198410778782742090300554036590140247385997847 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_132 : (649037107316853453575863946446162 : ℕ) • G=node_132 := by
  rw [show (649037107316853453575863946446162 : ℕ)=324518553658426726787931973223081+324518553658426726787931973223081 by norm_num,
    add_nsmul,nsmul_131]
  exact add_132

private def node_133 : formulaCurve.Point :=
  .some (626295363361287689892849845633997581903556035879 : F) (695044225324114310709134434705290812830825334941 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_133 : node_132+node_0=node_133 := by
  dsimp only [node_132,node_0,node_133]
  apply secant_add (ℓ:=(9753930857586331791678366983761691286005255845 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_133 : (649037107316853453575863946446163 : ℕ) • G=node_133 := by
  rw [show (649037107316853453575863946446163 : ℕ)=649037107316853453575863946446162+1 by norm_num,
    add_nsmul,nsmul_132,nsmul_0]
  exact add_133

private def node_134 : formulaCurve.Point :=
  .some (578029863801011979140475431121335568939263712362 : F) (166015871721903544607186858069260369830532743847 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_134 : node_133+node_133=node_134 := by
  dsimp only [node_133,node_134]
  apply tangent_double (ℓ:=(580147842995879072147941305695353841106619582484 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_134 : (1298074214633706907151727892892326 : ℕ) • G=node_134 := by
  rw [show (1298074214633706907151727892892326 : ℕ)=649037107316853453575863946446163+649037107316853453575863946446163 by norm_num,
    add_nsmul,nsmul_133]
  exact add_134

private def node_135 : formulaCurve.Point :=
  .some (263844789786439559231445811795006792573776989264 : F) (413457064451020268433179176901172639004919017188 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_135 : node_134+node_134=node_135 := by
  dsimp only [node_134,node_135]
  apply tangent_double (ℓ:=(402317842821570562079297197111098751179719289600 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_135 : (2596148429267413814303455785784652 : ℕ) • G=node_135 := by
  rw [show (2596148429267413814303455785784652 : ℕ)=1298074214633706907151727892892326+1298074214633706907151727892892326 by norm_num,
    add_nsmul,nsmul_134]
  exact add_135

private def node_136 : formulaCurve.Point :=
  .some (124198028082039043863965326441677947278404597681 : F) (300809940195414823027628482079568654771168821102 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_136 : node_135+node_0=node_136 := by
  dsimp only [node_135,node_0,node_136]
  apply secant_add (ℓ:=(169352828630960057382842803100433072504719076247 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_136 : (2596148429267413814303455785784653 : ℕ) • G=node_136 := by
  rw [show (2596148429267413814303455785784653 : ℕ)=2596148429267413814303455785784652+1 by norm_num,
    add_nsmul,nsmul_135,nsmul_0]
  exact add_136

private def node_137 : formulaCurve.Point :=
  .some (259414729482800646092125193316894392388250439183 : F) (312413886375385504603532795472005859955780944010 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_137 : node_136+node_136=node_137 := by
  dsimp only [node_136,node_137]
  apply tangent_double (ℓ:=(568830172566517384179166503226290628974348993967 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_137 : (5192296858534827628606911571569306 : ℕ) • G=node_137 := by
  rw [show (5192296858534827628606911571569306 : ℕ)=2596148429267413814303455785784653+2596148429267413814303455785784653 by norm_num,
    add_nsmul,nsmul_136]
  exact add_137

private def node_138 : formulaCurve.Point :=
  .some (462095341010077201731088160940165622592161919591 : F) (168695149800618353893309724506830007354679756332 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_138 : node_137+node_137=node_138 := by
  dsimp only [node_137,node_138]
  apply tangent_double (ℓ:=(236343176749802147232433863587556772988222613468 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_138 : (10384593717069655257213823143138612 : ℕ) • G=node_138 := by
  rw [show (10384593717069655257213823143138612 : ℕ)=5192296858534827628606911571569306+5192296858534827628606911571569306 by norm_num,
    add_nsmul,nsmul_137]
  exact add_138

private def node_139 : formulaCurve.Point :=
  .some (142025609548978055642843151965393753420018827865 : F) (571522396037195670664836182721451575357509921627 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_139 : node_138+node_138=node_139 := by
  dsimp only [node_138,node_139]
  apply tangent_double (ℓ:=(588633856173782332835479657219138872462393177436 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_139 : (20769187434139310514427646286277224 : ℕ) • G=node_139 := by
  rw [show (20769187434139310514427646286277224 : ℕ)=10384593717069655257213823143138612+10384593717069655257213823143138612 by norm_num,
    add_nsmul,nsmul_138]
  exact add_139

private def node_140 : formulaCurve.Point :=
  .some (7970077940554630268428647259477717922740813777 : F) (383362752797773977452195617867029554185387788676 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_140 : node_139+node_139=node_140 := by
  dsimp only [node_139,node_140]
  apply tangent_double (ℓ:=(148933731827421087066461338842995263712052733334 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_140 : (41538374868278621028855292572554448 : ℕ) • G=node_140 := by
  rw [show (41538374868278621028855292572554448 : ℕ)=20769187434139310514427646286277224+20769187434139310514427646286277224 by norm_num,
    add_nsmul,nsmul_139]
  exact add_140

private def node_141 : formulaCurve.Point :=
  .some (173452776217548744369688451181006844236712565837 : F) (712894654072734878250250848567393514480374636221 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_141 : node_140+node_0=node_141 := by
  dsimp only [node_140,node_0,node_141]
  apply secant_add (ℓ:=(319555934577698524690205310573703611244965921254 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_141 : (41538374868278621028855292572554449 : ℕ) • G=node_141 := by
  rw [show (41538374868278621028855292572554449 : ℕ)=41538374868278621028855292572554448+1 by norm_num,
    add_nsmul,nsmul_140,nsmul_0]
  exact add_141

private def node_142 : formulaCurve.Point :=
  .some (405372419825347766167488341938945577788569030318 : F) (221405288590144114309816845582625518084690476100 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_142 : node_141+node_141=node_142 := by
  dsimp only [node_141,node_142]
  apply tangent_double (ℓ:=(620773487041319254909707536497303008279539570287 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_142 : (83076749736557242057710585145108898 : ℕ) • G=node_142 := by
  rw [show (83076749736557242057710585145108898 : ℕ)=41538374868278621028855292572554449+41538374868278621028855292572554449 by norm_num,
    add_nsmul,nsmul_141]
  exact add_142

private def node_143 : formulaCurve.Point :=
  .some (328809595304692079376626897026987527621679944710 : F) (487933362479650102131452880244117412994994664052 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_143 : node_142+node_0=node_143 := by
  dsimp only [node_142,node_0,node_143]
  apply secant_add (ℓ:=(460067921352646292104601838979586064015423883182 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_143 : (83076749736557242057710585145108899 : ℕ) • G=node_143 := by
  rw [show (83076749736557242057710585145108899 : ℕ)=83076749736557242057710585145108898+1 by norm_num,
    add_nsmul,nsmul_142,nsmul_0]
  exact add_143

private def node_144 : formulaCurve.Point :=
  .some (475674157413141069273096704570255560251893319283 : F) (298906807131029425950542204279935253814897078268 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_144 : node_143+node_143=node_144 := by
  dsimp only [node_143,node_144]
  apply tangent_double (ℓ:=(559958552556042649727287777353240077053087093354 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_144 : (166153499473114484115421170290217798 : ℕ) • G=node_144 := by
  rw [show (166153499473114484115421170290217798 : ℕ)=83076749736557242057710585145108899+83076749736557242057710585145108899 by norm_num,
    add_nsmul,nsmul_143]
  exact add_144

private def node_145 : formulaCurve.Point :=
  .some (449990366005545859609978277421829251704687813825 : F) (605642064727356554520548337809307503322996418975 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_145 : node_144+node_144=node_145 := by
  dsimp only [node_144,node_145]
  apply tangent_double (ℓ:=(47011143421959452347624492168881232141058788127 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_145 : (332306998946228968230842340580435596 : ℕ) • G=node_145 := by
  rw [show (332306998946228968230842340580435596 : ℕ)=166153499473114484115421170290217798+166153499473114484115421170290217798 by norm_num,
    add_nsmul,nsmul_144]
  exact add_145

private def node_146 : formulaCurve.Point :=
  .some (131694467897578709432715496589514077923900623503 : F) (217894910083951637932915039007294365221255288659 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_146 : node_145+node_0=node_146 := by
  dsimp only [node_145,node_0,node_146]
  apply secant_add (ℓ:=(348730485873148938320156205588103671346962389319 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_146 : (332306998946228968230842340580435597 : ℕ) • G=node_146 := by
  rw [show (332306998946228968230842340580435597 : ℕ)=332306998946228968230842340580435596+1 by norm_num,
    add_nsmul,nsmul_145,nsmul_0]
  exact add_146

private def node_147 : formulaCurve.Point :=
  .some (537596099684117789439820803651744427382361602114 : F) (512440098028481576084923569509181331725607136671 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_147 : node_146+node_146=node_147 := by
  dsimp only [node_146,node_147]
  apply tangent_double (ℓ:=(17288427335306020845962468062072457009821734660 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_147 : (664613997892457936461684681160871194 : ℕ) • G=node_147 := by
  rw [show (664613997892457936461684681160871194 : ℕ)=332306998946228968230842340580435597+332306998946228968230842340580435597 by norm_num,
    add_nsmul,nsmul_146]
  exact add_147

private def node_148 : formulaCurve.Point :=
  .some (45309172439337212235563001486933839830820867281 : F) (99190761674538622621611954969091093081876291621 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_148 : node_147+node_147=node_148 := by
  dsimp only [node_147,node_148]
  apply tangent_double (ℓ:=(676409314360272947793172545923035619765922342873 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_148 : (1329227995784915872923369362321742388 : ℕ) • G=node_148 := by
  rw [show (1329227995784915872923369362321742388 : ℕ)=664613997892457936461684681160871194+664613997892457936461684681160871194 by norm_num,
    add_nsmul,nsmul_147]
  exact add_148

private def node_149 : formulaCurve.Point :=
  .some (244728169687358853006632958259606835342090476232 : F) (240377239479957652889009582534364299492231884422 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_149 : node_148+node_148=node_149 := by
  dsimp only [node_148,node_149]
  apply tangent_double (ℓ:=(655084048957539652444244595992780352524576609842 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_149 : (2658455991569831745846738724643484776 : ℕ) • G=node_149 := by
  rw [show (2658455991569831745846738724643484776 : ℕ)=1329227995784915872923369362321742388+1329227995784915872923369362321742388 by norm_num,
    add_nsmul,nsmul_148]
  exact add_149

private def node_150 : formulaCurve.Point :=
  .some (216434476212098568503749611632759911583072148878 : F) (526297955481744263362057377680949916949570656715 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_150 : node_149+node_0=node_150 := by
  dsimp only [node_149,node_0,node_150]
  apply secant_add (ℓ:=(416307372707964112084963930078325317755359613725 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_150 : (2658455991569831745846738724643484777 : ℕ) • G=node_150 := by
  rw [show (2658455991569831745846738724643484777 : ℕ)=2658455991569831745846738724643484776+1 by norm_num,
    add_nsmul,nsmul_149,nsmul_0]
  exact add_150

private def node_151 : formulaCurve.Point :=
  .some (92287653581600179176949982088688856363067696387 : F) (312913715456936569931809784141470531826066507555 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_151 : node_150+node_150=node_151 := by
  dsimp only [node_150,node_151]
  apply tangent_double (ℓ:=(728569238249922050769821035455422675562830420559 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_151 : (5316911983139663491693477449286969554 : ℕ) • G=node_151 := by
  rw [show (5316911983139663491693477449286969554 : ℕ)=2658455991569831745846738724643484777+2658455991569831745846738724643484777 by norm_num,
    add_nsmul,nsmul_150]
  exact add_151

private def node_152 : formulaCurve.Point :=
  .some (498246905507532040969287495010166465636902083853 : F) (539567406118692711597074066426460640823900929845 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_152 : node_151+node_151=node_152 := by
  dsimp only [node_151,node_152]
  apply tangent_double (ℓ:=(67248649710493359489678674290418022824996168190 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_152 : (10633823966279326983386954898573939108 : ℕ) • G=node_152 := by
  rw [show (10633823966279326983386954898573939108 : ℕ)=5316911983139663491693477449286969554+5316911983139663491693477449286969554 by norm_num,
    add_nsmul,nsmul_151]
  exact add_152

private def node_153 : formulaCurve.Point :=
  .some (272850000767905132407942739755644398287338067077 : F) (47410351569193221278409520389598849323675902452 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_153 : node_152+node_0=node_153 := by
  dsimp only [node_152,node_0,node_153]
  apply secant_add (ℓ:=(195762926815993825414859698217694322597762209646 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_153 : (10633823966279326983386954898573939109 : ℕ) • G=node_153 := by
  rw [show (10633823966279326983386954898573939109 : ℕ)=10633823966279326983386954898573939108+1 by norm_num,
    add_nsmul,nsmul_152,nsmul_0]
  exact add_153

private def node_154 : formulaCurve.Point :=
  .some (195983477495305579887153256360626368227902413497 : F) (60053176827656699664239844565242504207145454745 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_154 : node_153+node_153=node_154 := by
  dsimp only [node_153,node_154]
  apply tangent_double (ℓ:=(465939893700835685395084374633700972037069783177 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_154 : (21267647932558653966773909797147878218 : ℕ) • G=node_154 := by
  rw [show (21267647932558653966773909797147878218 : ℕ)=10633823966279326983386954898573939109+10633823966279326983386954898573939109 by norm_num,
    add_nsmul,nsmul_153]
  exact add_154

private def node_155 : formulaCurve.Point :=
  .some (439640069789240306586726899401766792333047647860 : F) (192552963272470032382341782718858950856793464134 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_155 : node_154+node_0=node_155 := by
  dsimp only [node_154,node_0,node_155]
  apply secant_add (ℓ:=(349751679835708001249215133389877204866827777231 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_155 : (21267647932558653966773909797147878219 : ℕ) • G=node_155 := by
  rw [show (21267647932558653966773909797147878219 : ℕ)=21267647932558653966773909797147878218+1 by norm_num,
    add_nsmul,nsmul_154,nsmul_0]
  exact add_155

private def node_156 : formulaCurve.Point :=
  .some (525434602973319946311689246205372746737297551539 : F) (650769069536420273421029895451614137983473159019 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_156 : node_155+node_155=node_156 := by
  dsimp only [node_155,node_156]
  apply tangent_double (ℓ:=(326054895190223238227894283322326127108729701255 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_156 : (42535295865117307933547819594295756438 : ℕ) • G=node_156 := by
  rw [show (42535295865117307933547819594295756438 : ℕ)=21267647932558653966773909797147878219+21267647932558653966773909797147878219 by norm_num,
    add_nsmul,nsmul_155]
  exact add_156

private def node_157 : formulaCurve.Point :=
  .some (309129033437298294008258406577207509553766687642 : F) (608969275855797816455819427618717316104398917710 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_157 : node_156+node_0=node_157 := by
  dsimp only [node_156,node_0,node_157]
  apply secant_add (ℓ:=(692690461561398061523957160324035307238852771924 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_157 : (42535295865117307933547819594295756439 : ℕ) • G=node_157 := by
  rw [show (42535295865117307933547819594295756439 : ℕ)=42535295865117307933547819594295756438+1 by norm_num,
    add_nsmul,nsmul_156,nsmul_0]
  exact add_157

private def node_158 : formulaCurve.Point :=
  .some (693180741991710871785101286925893273182037113403 : F) (435413440073213789328487412463265297064618173537 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_158 : node_157+node_157=node_158 := by
  dsimp only [node_157,node_158]
  apply tangent_double (ℓ:=(485842521484275969040715356876749444474143632940 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_158 : (85070591730234615867095639188591512878 : ℕ) • G=node_158 := by
  rw [show (85070591730234615867095639188591512878 : ℕ)=42535295865117307933547819594295756439+42535295865117307933547819594295756439 by norm_num,
    add_nsmul,nsmul_157]
  exact add_158

private def node_159 : formulaCurve.Point :=
  .some (267500663254887556358144489518218131529078012574 : F) (109152595418433354884497686213389047980531009167 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_159 : node_158+node_158=node_159 := by
  dsimp only [node_158,node_159]
  apply tangent_double (ℓ:=(337327581639572455791628593353056283072062480488 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_159 : (170141183460469231734191278377183025756 : ℕ) • G=node_159 := by
  rw [show (170141183460469231734191278377183025756 : ℕ)=85070591730234615867095639188591512878+85070591730234615867095639188591512878 by norm_num,
    add_nsmul,nsmul_158]
  exact add_159

private def node_160 : formulaCurve.Point :=
  .some (274067528241016469494569628030086513660671344169 : F) (379492689866205565000590378021385871894254400791 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_160 : node_159+node_159=node_160 := by
  dsimp only [node_159,node_160]
  apply tangent_double (ℓ:=(668469085979842183622439325585287099735236397303 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_160 : (340282366920938463468382556754366051512 : ℕ) • G=node_160 := by
  rw [show (340282366920938463468382556754366051512 : ℕ)=170141183460469231734191278377183025756+170141183460469231734191278377183025756 by norm_num,
    add_nsmul,nsmul_159]
  exact add_160

private def node_161 : formulaCurve.Point :=
  .some (627748906494374512332226501228770495351334110707 : F) (281415833500276424304972801211598698637027014057 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_161 : node_160+node_0=node_161 := by
  dsimp only [node_160,node_0,node_161]
  apply secant_add (ℓ:=(287899852082607671020351878586293839021241988396 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_161 : (340282366920938463468382556754366051513 : ℕ) • G=node_161 := by
  rw [show (340282366920938463468382556754366051513 : ℕ)=340282366920938463468382556754366051512+1 by norm_num,
    add_nsmul,nsmul_160,nsmul_0]
  exact add_161

private def node_162 : formulaCurve.Point :=
  .some (491015847773524461025108262284937449530597936717 : F) (334614949996754197043600210561052464223212575938 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_162 : node_161+node_161=node_162 := by
  dsimp only [node_161,node_162]
  apply tangent_double (ℓ:=(57730262932652323594137879741170762158639457956 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_162 : (680564733841876926936765113508732103026 : ℕ) • G=node_162 := by
  rw [show (680564733841876926936765113508732103026 : ℕ)=340282366920938463468382556754366051513+340282366920938463468382556754366051513 by norm_num,
    add_nsmul,nsmul_161]
  exact add_162

private def node_163 : formulaCurve.Point :=
  .some (540186879455630641633532102826191966810843574126 : F) (258387402498122592549328684275309051163146999326 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_163 : node_162+node_0=node_163 := by
  dsimp only [node_162,node_0,node_163]
  apply secant_add (ℓ:=(337813437158268138408277395745347515055008481267 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_163 : (680564733841876926936765113508732103027 : ℕ) • G=node_163 := by
  rw [show (680564733841876926936765113508732103027 : ℕ)=680564733841876926936765113508732103026+1 by norm_num,
    add_nsmul,nsmul_162,nsmul_0]
  exact add_163

private def node_164 : formulaCurve.Point :=
  .some (179177045864173672856126928659221249063490293579 : F) (285015840122059088716643679848095933506119880713 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_164 : node_163+node_163=node_164 := by
  dsimp only [node_163,node_164]
  apply tangent_double (ℓ:=(583615994160560280088704298034433935377441212906 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_164 : (1361129467683753853873530227017464206054 : ℕ) • G=node_164 := by
  rw [show (1361129467683753853873530227017464206054 : ℕ)=680564733841876926936765113508732103027+680564733841876926936765113508732103027 by norm_num,
    add_nsmul,nsmul_163]
  exact add_164

private def node_165 : formulaCurve.Point :=
  .some (248694351697224167462352724345669271408336982883 : F) (696328954970126392552233398505871082545587054062 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_165 : node_164+node_164=node_165 := by
  dsimp only [node_164,node_165]
  apply tangent_double (ℓ:=(448119872856538985605526213822427776124487958133 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_165 : (2722258935367507707747060454034928412108 : ℕ) • G=node_165 := by
  rw [show (2722258935367507707747060454034928412108 : ℕ)=1361129467683753853873530227017464206054+1361129467683753853873530227017464206054 by norm_num,
    add_nsmul,nsmul_164]
  exact add_165

private def node_166 : formulaCurve.Point :=
  .some (577544817308698872706364656738293087186861271304 : F) (629314508923112553356542873402486675014232534066 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_166 : node_165+node_0=node_166 := by
  dsimp only [node_165,node_0,node_166]
  apply secant_add (ℓ:=(722647579269407503689650227623499697538397251535 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_166 : (2722258935367507707747060454034928412109 : ℕ) • G=node_166 := by
  rw [show (2722258935367507707747060454034928412109 : ℕ)=2722258935367507707747060454034928412108+1 by norm_num,
    add_nsmul,nsmul_165,nsmul_0]
  exact add_166

private def node_167 : formulaCurve.Point :=
  .some (649846654792629983063109697064452932491805226173 : F) (272819409183322728078400710532216202539771776881 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_167 : node_166+node_166=node_167 := by
  dsimp only [node_166,node_167]
  apply tangent_double (ℓ:=(157117046903250886690598074213667586122894286795 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_167 : (5444517870735015415494120908069856824218 : ℕ) • G=node_167 := by
  rw [show (5444517870735015415494120908069856824218 : ℕ)=2722258935367507707747060454034928412109+2722258935367507707747060454034928412109 by norm_num,
    add_nsmul,nsmul_166]
  exact add_167

private def node_168 : formulaCurve.Point :=
  .some (151259387288213219350866273503776918415555774923 : F) (170558123665977546749438789598342559918643929259 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_168 : node_167+node_0=node_168 := by
  dsimp only [node_167,node_0,node_168]
  apply secant_add (ℓ:=(283158048921548023395826652900782467135281264212 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_168 : (5444517870735015415494120908069856824219 : ℕ) • G=node_168 := by
  rw [show (5444517870735015415494120908069856824219 : ℕ)=5444517870735015415494120908069856824218+1 by norm_num,
    add_nsmul,nsmul_167,nsmul_0]
  exact add_168

private def node_169 : formulaCurve.Point :=
  .some (246923505797707050256347529326527436318447741687 : F) (652778515920072387289426823900103545072246713192 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_169 : node_168+node_168=node_169 := by
  dsimp only [node_168,node_169]
  apply tangent_double (ℓ:=(143018339171661879413043594910055530565021207709 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_169 : (10889035741470030830988241816139713648438 : ℕ) • G=node_169 := by
  rw [show (10889035741470030830988241816139713648438 : ℕ)=5444517870735015415494120908069856824219+5444517870735015415494120908069856824219 by norm_num,
    add_nsmul,nsmul_168]
  exact add_169

private def node_170 : formulaCurve.Point :=
  .some (479466851109046652423578630002482337309178765096 : F) (666813494933165305636678381727790709842614403222 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_170 : node_169+node_0=node_170 := by
  dsimp only [node_169,node_0,node_170]
  apply secant_add (ℓ:=(390309871157898110845132632227922219608569910502 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_170 : (10889035741470030830988241816139713648439 : ℕ) • G=node_170 := by
  rw [show (10889035741470030830988241816139713648439 : ℕ)=10889035741470030830988241816139713648438+1 by norm_num,
    add_nsmul,nsmul_169,nsmul_0]
  exact add_170

private def node_171 : formulaCurve.Point :=
  .some (721730477224861842266265660152739743457852505689 : F) (477028159918509299420741681030914629091794815309 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_171 : node_170+node_170=node_171 := by
  dsimp only [node_170,node_171]
  apply tangent_double (ℓ:=(434593325030697278791303533447404068502695754122 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_171 : (21778071482940061661976483632279427296878 : ℕ) • G=node_171 := by
  rw [show (21778071482940061661976483632279427296878 : ℕ)=10889035741470030830988241816139713648439+10889035741470030830988241816139713648439 by norm_num,
    add_nsmul,nsmul_170]
  exact add_171

private def node_172 : formulaCurve.Point :=
  .some (632387850034662104202945557055784561643596959085 : F) (515218891961099252713950968760113749660668431869 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_172 : node_171+node_171=node_172 := by
  dsimp only [node_171,node_172]
  apply tangent_double (ℓ:=(45998291038880343410073387939384759094483182259 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_172 : (43556142965880123323952967264558854593756 : ℕ) • G=node_172 := by
  rw [show (43556142965880123323952967264558854593756 : ℕ)=21778071482940061661976483632279427296878+21778071482940061661976483632279427296878 by norm_num,
    add_nsmul,nsmul_171]
  exact add_172

private def node_173 : formulaCurve.Point :=
  .some (29626733206566141627281710497331428828629529703 : F) (550262030925158005787588483721246604576969268837 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_173 : node_172+node_0=node_173 := by
  dsimp only [node_172,node_0,node_173]
  apply secant_add (ℓ:=(193919409368628174251177283215061656347378598346 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_173 : (43556142965880123323952967264558854593757 : ℕ) • G=node_173 := by
  rw [show (43556142965880123323952967264558854593757 : ℕ)=43556142965880123323952967264558854593756+1 by norm_num,
    add_nsmul,nsmul_172,nsmul_0]
  exact add_173

private def node_174 : formulaCurve.Point :=
  .some (391203062122783450150267343329312648996942830672 : F) (348532017511225707581845493435010706479436266127 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_174 : node_173+node_173=node_174 := by
  dsimp only [node_173,node_174]
  apply tangent_double (ℓ:=(481517030041568138629069474320646921816888456342 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_174 : (87112285931760246647905934529117709187514 : ℕ) • G=node_174 := by
  rw [show (87112285931760246647905934529117709187514 : ℕ)=43556142965880123323952967264558854593757+43556142965880123323952967264558854593757 by norm_num,
    add_nsmul,nsmul_173]
  exact add_174

private def node_175 : formulaCurve.Point :=
  .some (441787167523956632340231938056712285473065863109 : F) (81451274649838488295527832909091923963808501834 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_175 : node_174+node_0=node_175 := by
  dsimp only [node_174,node_0,node_175]
  apply secant_add (ℓ:=(281882093677013287144849092776369500286935578169 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_175 : (87112285931760246647905934529117709187515 : ℕ) • G=node_175 := by
  rw [show (87112285931760246647905934529117709187515 : ℕ)=87112285931760246647905934529117709187514+1 by norm_num,
    add_nsmul,nsmul_174,nsmul_0]
  exact add_175

private def node_176 : formulaCurve.Point :=
  .some (480509440340306971412773736412719303826995809821 : F) (251038521487661190195829891841233892223832913397 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_176 : node_175+node_175=node_176 := by
  dsimp only [node_175,node_176]
  apply tangent_double (ℓ:=(103031541980113950151823089684743064129155354405 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_176 : (174224571863520493295811869058235418375030 : ℕ) • G=node_176 := by
  rw [show (174224571863520493295811869058235418375030 : ℕ)=87112285931760246647905934529117709187515+87112285931760246647905934529117709187515 by norm_num,
    add_nsmul,nsmul_175]
  exact add_176

private def node_177 : formulaCurve.Point :=
  .some (339000702492439733266241668765224404084518675411 : F) (511678585602828160072214925044693506683362555886 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_177 : node_176+node_0=node_177 := by
  dsimp only [node_176,node_0,node_177]
  apply secant_add (ℓ:=(15788979345061682502254535282541629232422970437 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_177 : (174224571863520493295811869058235418375031 : ℕ) • G=node_177 := by
  rw [show (174224571863520493295811869058235418375031 : ℕ)=174224571863520493295811869058235418375030+1 by norm_num,
    add_nsmul,nsmul_176,nsmul_0]
  exact add_177

private def node_178 : formulaCurve.Point :=
  .some (191556166270305646655180370332466273497528092621 : F) (10718435118926002860532924776470817484725722008 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_178 : node_177+node_177=node_178 := by
  dsimp only [node_177,node_178]
  apply tangent_double (ℓ:=(497962320890545320874425514515964639768774366599 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_178 : (348449143727040986591623738116470836750062 : ℕ) • G=node_178 := by
  rw [show (348449143727040986591623738116470836750062 : ℕ)=174224571863520493295811869058235418375031+174224571863520493295811869058235418375031 by norm_num,
    add_nsmul,nsmul_177]
  exact add_178

private def node_179 : formulaCurve.Point :=
  .some (433858634573741225140444628139617895343662042140 : F) (558838843670582512572264864493622830030685974685 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_179 : node_178+node_178=node_179 := by
  dsimp only [node_178,node_179]
  apply tangent_double (ℓ:=(599152099993812638964955892972980622818449753086 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_179 : (696898287454081973183247476232941673500124 : ℕ) • G=node_179 := by
  rw [show (696898287454081973183247476232941673500124 : ℕ)=348449143727040986591623738116470836750062+348449143727040986591623738116470836750062 by norm_num,
    add_nsmul,nsmul_178]
  exact add_179

private def node_180 : formulaCurve.Point :=
  .some (259573573885886697459343177728727774879284621704 : F) (374720850019739440761908095717605109050018241414 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_180 : node_179+node_0=node_180 := by
  dsimp only [node_179,node_0,node_180]
  apply secant_add (ℓ:=(100219634773380552181100450985954518546526902322 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_180 : (696898287454081973183247476232941673500125 : ℕ) • G=node_180 := by
  rw [show (696898287454081973183247476232941673500125 : ℕ)=696898287454081973183247476232941673500124+1 by norm_num,
    add_nsmul,nsmul_179,nsmul_0]
  exact add_180

private def node_181 : formulaCurve.Point :=
  .some (10322711217384438238631529969370676302671200889 : F) (475177483732545712048524337169141299193318871795 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_181 : node_180+node_180=node_181 := by
  dsimp only [node_180,node_181]
  apply tangent_double (ℓ:=(256383047316039759144423267570056925210557535717 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_181 : (1393796574908163946366494952465883347000250 : ℕ) • G=node_181 := by
  rw [show (1393796574908163946366494952465883347000250 : ℕ)=696898287454081973183247476232941673500125+696898287454081973183247476232941673500125 by norm_num,
    add_nsmul,nsmul_180]
  exact add_181

private def node_182 : formulaCurve.Point :=
  .some (134061444026991273620137989330581009823715811480 : F) (537722734640827828799131147899600891660891375820 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_182 : node_181+node_181=node_182 := by
  dsimp only [node_181,node_182]
  apply tangent_double (ℓ:=(311953398095304806929433782702519087540067697990 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_182 : (2787593149816327892732989904931766694000500 : ℕ) • G=node_182 := by
  rw [show (2787593149816327892732989904931766694000500 : ℕ)=1393796574908163946366494952465883347000250+1393796574908163946366494952465883347000250 by norm_num,
    add_nsmul,nsmul_181]
  exact add_182

private def node_183 : formulaCurve.Point :=
  .some (529891841364367869931133082390795838693676189126 : F) (231310553940143008690924334484298411078685424225 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_183 : node_182+node_0=node_183 := by
  dsimp only [node_182,node_0,node_183]
  apply secant_add (ℓ:=(144546790701620598994983887392885300104986607317 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_183 : (2787593149816327892732989904931766694000501 : ℕ) • G=node_183 := by
  rw [show (2787593149816327892732989904931766694000501 : ℕ)=2787593149816327892732989904931766694000500+1 by norm_num,
    add_nsmul,nsmul_182,nsmul_0]
  exact add_183

private def node_184 : formulaCurve.Point :=
  .some (286661366552785840134080467750722613856982572159 : F) (597738043518756276576064461222982037885053843424 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_184 : node_183+node_183=node_184 := by
  dsimp only [node_183,node_184]
  apply tangent_double (ℓ:=(19450528933310730092535642981371726434713471980 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_184 : (5575186299632655785465979809863533388001002 : ℕ) • G=node_184 := by
  rw [show (5575186299632655785465979809863533388001002 : ℕ)=2787593149816327892732989904931766694000501+2787593149816327892732989904931766694000501 by norm_num,
    add_nsmul,nsmul_183]
  exact add_184

private def node_185 : formulaCurve.Point :=
  .some (110018724279854778533177227884292875029967134277 : F) (226590572754214878893208120341859238537359576731 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_185 : node_184+node_0=node_185 := by
  dsimp only [node_184,node_0,node_185]
  apply secant_add (ℓ:=(711928645134379098962941058815113001412852964878 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_185 : (5575186299632655785465979809863533388001003 : ℕ) • G=node_185 := by
  rw [show (5575186299632655785465979809863533388001003 : ℕ)=5575186299632655785465979809863533388001002+1 by norm_num,
    add_nsmul,nsmul_184,nsmul_0]
  exact add_185

private def node_186 : formulaCurve.Point :=
  .some (563849035824858621289105065814788478929881057523 : F) (557369356875683735693508074452914542376356276920 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_186 : node_185+node_185=node_186 := by
  dsimp only [node_185,node_186]
  apply tangent_double (ℓ:=(293227135163274211500469385346069447906215672385 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_186 : (11150372599265311570931959619727066776002006 : ℕ) • G=node_186 := by
  rw [show (11150372599265311570931959619727066776002006 : ℕ)=5575186299632655785465979809863533388001003+5575186299632655785465979809863533388001003 by norm_num,
    add_nsmul,nsmul_185]
  exact add_186

private def node_187 : formulaCurve.Point :=
  .some (664973413598253485069293683018153425040348795868 : F) (248610313801954467534900378749834256743074607567 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_187 : node_186+node_186=node_187 := by
  dsimp only [node_186,node_187]
  apply tangent_double (ℓ:=(635399349455359606071912760455676337155157828903 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_187 : (22300745198530623141863919239454133552004012 : ℕ) • G=node_187 := by
  rw [show (22300745198530623141863919239454133552004012 : ℕ)=11150372599265311570931959619727066776002006+11150372599265311570931959619727066776002006 by norm_num,
    add_nsmul,nsmul_186]
  exact add_187

private def node_188 : formulaCurve.Point :=
  .some (487579111867838370815446540431868329795141500245 : F) (475788523050885862146126096529227233415323909884 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_188 : node_187+node_187=node_188 := by
  dsimp only [node_187,node_188]
  apply tangent_double (ℓ:=(186308616300052962957229455179200333034571869290 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_188 : (44601490397061246283727838478908267104008024 : ℕ) • G=node_188 := by
  rw [show (44601490397061246283727838478908267104008024 : ℕ)=22300745198530623141863919239454133552004012+22300745198530623141863919239454133552004012 by norm_num,
    add_nsmul,nsmul_187]
  exact add_188

private def node_189 : formulaCurve.Point :=
  .some (405833651940810979830931422543665645356498958246 : F) (232755291883887913444294597167632270864982731565 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_189 : node_188+node_188=node_189 := by
  dsimp only [node_188,node_189]
  apply tangent_double (ℓ:=(83435419689200956450665374833014846039638452201 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_189 : (89202980794122492567455676957816534208016048 : ℕ) • G=node_189 := by
  rw [show (89202980794122492567455676957816534208016048 : ℕ)=44601490397061246283727838478908267104008024+44601490397061246283727838478908267104008024 by norm_num,
    add_nsmul,nsmul_188]
  exact add_189

private def node_190 : formulaCurve.Point :=
  .some (262931995294310194557143265059968220175945175210 : F) (141243916879838191567386537007163889799476973725 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_190 : node_189+node_189=node_190 := by
  dsimp only [node_189,node_190]
  apply tangent_double (ℓ:=(369195414012756771567119503428795892418503265195 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_190 : (178405961588244985134911353915633068416032096 : ℕ) • G=node_190 := by
  rw [show (178405961588244985134911353915633068416032096 : ℕ)=89202980794122492567455676957816534208016048+89202980794122492567455676957816534208016048 by norm_num,
    add_nsmul,nsmul_189]
  exact add_190

private def node_191 : formulaCurve.Point :=
  .some (686109114223515629621513381960695570436471685651 : F) (597275420849766897368110782243375543046686027289 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_191 : node_190+node_0=node_191 := by
  dsimp only [node_190,node_0,node_191]
  apply secant_add (ℓ:=(282732944220447338114600190218508228931141132023 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_191 : (178405961588244985134911353915633068416032097 : ℕ) • G=node_191 := by
  rw [show (178405961588244985134911353915633068416032097 : ℕ)=178405961588244985134911353915633068416032096+1 by norm_num,
    add_nsmul,nsmul_190,nsmul_0]
  exact add_191

private def node_192 : formulaCurve.Point :=
  .some (462080715978420256010197573882189815970699012291 : F) (411671094703552954192733705330923412186387564603 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_192 : node_191+node_191=node_192 := by
  dsimp only [node_191,node_192]
  apply tangent_double (ℓ:=(596998414502107082221820096402561836983301830758 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_192 : (356811923176489970269822707831266136832064194 : ℕ) • G=node_192 := by
  rw [show (356811923176489970269822707831266136832064194 : ℕ)=178405961588244985134911353915633068416032097+178405961588244985134911353915633068416032097 by norm_num,
    add_nsmul,nsmul_191]
  exact add_192

private def node_193 : formulaCurve.Point :=
  .some (118205129556237536099041215535272257011611139429 : F) (115863297532377883612608985353892702433484163632 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_193 : node_192+node_0=node_193 := by
  dsimp only [node_192,node_0,node_193]
  apply secant_add (ℓ:=(181938556500646558634996610634824211152526131664 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_193 : (356811923176489970269822707831266136832064195 : ℕ) • G=node_193 := by
  rw [show (356811923176489970269822707831266136832064195 : ℕ)=356811923176489970269822707831266136832064194+1 by norm_num,
    add_nsmul,nsmul_192,nsmul_0]
  exact add_193

private def node_194 : formulaCurve.Point :=
  .some (186105015443678090066104278589908595791154916423 : F) (58130253176219689402586091468626428050629871247 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_194 : node_193+node_193=node_194 := by
  dsimp only [node_193,node_194]
  apply tangent_double (ℓ:=(335735750107712255849566414555072661305453535212 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_194 : (713623846352979940539645415662532273664128390 : ℕ) • G=node_194 := by
  rw [show (713623846352979940539645415662532273664128390 : ℕ)=356811923176489970269822707831266136832064195+356811923176489970269822707831266136832064195 by norm_num,
    add_nsmul,nsmul_193]
  exact add_194

private def node_195 : formulaCurve.Point :=
  .some (574941075761808609898773186593373154070582883420 : F) (397013971868372819014870541099823984547288789437 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_195 : node_194+node_194=node_195 := by
  dsimp only [node_194,node_195]
  apply tangent_double (ℓ:=(267499242053381550045572024002882314411824350101 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_195 : (1427247692705959881079290831325064547328256780 : ℕ) • G=node_195 := by
  rw [show (1427247692705959881079290831325064547328256780 : ℕ)=713623846352979940539645415662532273664128390+713623846352979940539645415662532273664128390 by norm_num,
    add_nsmul,nsmul_194]
  exact add_195

private def node_196 : formulaCurve.Point :=
  .some (566848391744584071190542829140130503427710371891 : F) (36666359709664508294590862616672450226814250751 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_196 : node_195+node_195=node_196 := by
  dsimp only [node_195,node_196]
  apply tangent_double (ℓ:=(242007631784233877211329071340875749114214781697 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_196 : (2854495385411919762158581662650129094656513560 : ℕ) • G=node_196 := by
  rw [show (2854495385411919762158581662650129094656513560 : ℕ)=1427247692705959881079290831325064547328256780+1427247692705959881079290831325064547328256780 by norm_num,
    add_nsmul,nsmul_195]
  exact add_196

private def node_197 : formulaCurve.Point :=
  .some (253887441132108702270847800033619021470037370898 : F) (48590636366157339722302043660997544619351946947 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_197 : node_196+node_0=node_197 := by
  dsimp only [node_196,node_0,node_197]
  apply secant_add (ℓ:=(652514810259585237151254614086152515868294914844 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_197 : (2854495385411919762158581662650129094656513561 : ℕ) • G=node_197 := by
  rw [show (2854495385411919762158581662650129094656513561 : ℕ)=2854495385411919762158581662650129094656513560+1 by norm_num,
    add_nsmul,nsmul_196,nsmul_0]
  exact add_197

private def node_198 : formulaCurve.Point :=
  .some (128590084838190098164237800335063851717228664529 : F) (57413169091151844734884160719946067049058011309 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_198 : node_197+node_197=node_198 := by
  dsimp only [node_197,node_198]
  apply tangent_double (ℓ:=(416421486707284145327087905877724020576037569423 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_198 : (5708990770823839524317163325300258189313027122 : ℕ) • G=node_198 := by
  rw [show (5708990770823839524317163325300258189313027122 : ℕ)=2854495385411919762158581662650129094656513561+2854495385411919762158581662650129094656513561 by norm_num,
    add_nsmul,nsmul_197]
  exact add_198

private def node_199 : formulaCurve.Point :=
  .some (473078978745879122198750584957920683832213258043 : F) (61324722018015032057878968761425446759155178330 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_199 : node_198+node_198=node_199 := by
  dsimp only [node_198,node_199]
  apply tangent_double (ℓ:=(312521295018095590782582823474953575186351770143 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_199 : (11417981541647679048634326650600516378626054244 : ℕ) • G=node_199 := by
  rw [show (11417981541647679048634326650600516378626054244 : ℕ)=5708990770823839524317163325300258189313027122+5708990770823839524317163325300258189313027122 by norm_num,
    add_nsmul,nsmul_198]
  exact add_199

private def node_200 : formulaCurve.Point :=
  .some (267787552157073219802038488123122609678800434698 : F) (500324288340841518045043969785382604364958197319 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_200 : node_199+node_0=node_200 := by
  dsimp only [node_199,node_0,node_200]
  apply secant_add (ℓ:=(150528001794010570586873265698325762096951541411 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_200 : (11417981541647679048634326650600516378626054245 : ℕ) • G=node_200 := by
  rw [show (11417981541647679048634326650600516378626054245 : ℕ)=11417981541647679048634326650600516378626054244+1 by norm_num,
    add_nsmul,nsmul_199,nsmul_0]
  exact add_200

private def node_201 : formulaCurve.Point :=
  .some (462597760496461634735372724410383844632996873404 : F) (532098655113806506199987192101207378354214375513 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_201 : node_200+node_200=node_201 := by
  dsimp only [node_200,node_201]
  apply tangent_double (ℓ:=(576459174724254638165648903589328008701329373621 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_201 : (22835963083295358097268653301201032757252108490 : ℕ) • G=node_201 := by
  rw [show (22835963083295358097268653301201032757252108490 : ℕ)=11417981541647679048634326650600516378626054245+11417981541647679048634326650600516378626054245 by norm_num,
    add_nsmul,nsmul_200]
  exact add_201

private def node_202 : formulaCurve.Point :=
  .some (431846302869219464963737833741577708318133907931 : F) (258053398640646269478767949237098896914734569585 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_202 : node_201+node_0=node_202 := by
  dsimp only [node_201,node_0,node_202]
  apply secant_add (ℓ:=(299368388818496182932778648363617085991377760431 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_202 : (22835963083295358097268653301201032757252108491 : ℕ) • G=node_202 := by
  rw [show (22835963083295358097268653301201032757252108491 : ℕ)=22835963083295358097268653301201032757252108490+1 by norm_num,
    add_nsmul,nsmul_201,nsmul_0]
  exact add_202

private def node_203 : formulaCurve.Point :=
  .some (194335820672852298408437801871993953548339627219 : F) (186628213060576980772504521077414856381080202377 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_203 : node_202+node_202=node_203 := by
  dsimp only [node_202,node_203]
  apply tangent_double (ℓ:=(387629757084433110034185248512792207555408207151 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_203 : (45671926166590716194537306602402065514504216982 : ℕ) • G=node_203 := by
  rw [show (45671926166590716194537306602402065514504216982 : ℕ)=22835963083295358097268653301201032757252108491+22835963083295358097268653301201032757252108491 by norm_num,
    add_nsmul,nsmul_202]
  exact add_203

private def node_204 : formulaCurve.Point :=
  .some (283571332481635732116098411899268527267363141614 : F) (116972815611115452847616916653771119837659517256 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_204 : node_203+node_203=node_204 := by
  dsimp only [node_203,node_204]
  apply tangent_double (ℓ:=(542116980804672006856711263441212699827017237612 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_204 : (91343852333181432389074613204804131029008433964 : ℕ) • G=node_204 := by
  rw [show (91343852333181432389074613204804131029008433964 : ℕ)=45671926166590716194537306602402065514504216982+45671926166590716194537306602402065514504216982 by norm_num,
    add_nsmul,nsmul_203]
  exact add_204

private def node_205 : formulaCurve.Point :=
  .some (618250321023028733786651453221511317640723764557 : F) (390815865602875610094458181876553428763251820250 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_205 : node_204+node_0=node_205 := by
  dsimp only [node_204,node_0,node_205]
  apply secant_add (ℓ:=(123429331069473228866906783270462197317133849528 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_205 : (91343852333181432389074613204804131029008433965 : ℕ) • G=node_205 := by
  rw [show (91343852333181432389074613204804131029008433965 : ℕ)=91343852333181432389074613204804131029008433964+1 by norm_num,
    add_nsmul,nsmul_204,nsmul_0]
  exact add_205

private def node_206 : formulaCurve.Point :=
  .some (720998918175692808873190038978455193662203842683 : F) (122429379203115167663185316900328515277649007220 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_206 : node_205+node_205=node_206 := by
  dsimp only [node_205,node_206]
  apply tangent_double (ℓ:=(266256039478843544005060524118764784635347651559 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_206 : (182687704666362864778149226409608262058016867930 : ℕ) • G=node_206 := by
  rw [show (182687704666362864778149226409608262058016867930 : ℕ)=91343852333181432389074613204804131029008433965+91343852333181432389074613204804131029008433965 by norm_num,
    add_nsmul,nsmul_205]
  exact add_206

private def node_207 : formulaCurve.Point :=
  .some (687813709064381171296003550328135506184923732920 : F) (680141074064218914847817940190300131930844300333 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_207 : node_206+node_206=node_207 := by
  dsimp only [node_206,node_207]
  apply tangent_double (ℓ:=(208584892864261225159717762458118725267523554947 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_207 : (365375409332725729556298452819216524116033735860 : ℕ) • G=node_207 := by
  rw [show (365375409332725729556298452819216524116033735860 : ℕ)=182687704666362864778149226409608262058016867930+182687704666362864778149226409608262058016867930 by norm_num,
    add_nsmul,nsmul_206]
  exact add_207

private def node_208 : formulaCurve.Point :=
  .some (373654958340159215284670181762443975470117347830 : F) (475045086830537499221535934589573026783478895781 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_208 : node_207+node_0=node_208 := by
  dsimp only [node_207,node_0,node_208]
  apply secant_add (ℓ:=(682613215324405161446398549544280525541923320631 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_208 : (365375409332725729556298452819216524116033735861 : ℕ) • G=node_208 := by
  rw [show (365375409332725729556298452819216524116033735861 : ℕ)=365375409332725729556298452819216524116033735860+1 by norm_num,
    add_nsmul,nsmul_207,nsmul_0]
  exact add_208

private def node_209 : formulaCurve.Point :=
  .some (1 : F) (420214349725551765393634551299436392850699902703 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_209 : node_208+node_208=node_209 := by
  dsimp only [node_208,node_209]
  apply tangent_double (ℓ:=(495573237460681968187371450828980927417298029778 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_209 : (730750818665451459112596905638433048232067471722 : ℕ) • G=node_209 := by
  rw [show (730750818665451459112596905638433048232067471722 : ℕ)=365375409332725729556298452819216524116033735861+365375409332725729556298452819216524116033735861 by norm_num,
    add_nsmul,nsmul_208]
  exact add_209

private def node_210 : formulaCurve.Point :=
  0

private theorem add_210 : node_209+node_0=node_210 := by
  dsimp only [node_209,node_0,node_210]
  apply WeierstrassCurve.Affine.Point.add_of_Y_eq
  · rfl
  · simp only [formulaCurve,WeierstrassCurve.Affine.negY]
    unfold F p
    reduce_mod_char

private theorem nsmul_210 : (730750818665451459112596905638433048232067471723 : ℕ) • G=node_210 := by
  rw [show (730750818665451459112596905638433048232067471723 : ℕ)=730750818665451459112596905638433048232067471722+1 by norm_num,
    add_nsmul,nsmul_209,nsmul_0]
  exact add_210

private def node_211 : formulaCurve.Point :=
  .some (64411170002682093523156567771413718466443096837 : F) (136099325766419635169994741219640178503526480744 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_211 : node_3+node_0=node_211 := by
  dsimp only [node_3,node_0,node_211]
  apply secant_add (ℓ:=(654255064993373910870715056478265960114729201529 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_211 : (9 : ℕ) • G=node_211 := by
  rw [show (9 : ℕ)=8+1 by norm_num,
    add_nsmul,nsmul_3,nsmul_0]
  exact add_211

private def node_212 : formulaCurve.Point :=
  .some (206135768517470219199640577933617657405686171665 : F) (688708350018329125124692204654825629359243532694 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_212 : node_211+node_211=node_212 := by
  dsimp only [node_211,node_212]
  apply tangent_double (ℓ:=(331227025140575751658612640794834941872955363091 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_212 : (18 : ℕ) • G=node_212 := by
  rw [show (18 : ℕ)=9+9 by norm_num,
    add_nsmul,nsmul_211]
  exact add_212

private def node_213 : formulaCurve.Point :=
  .some (180916227418363184591743118799851010449742538799 : F) (269683912631112131166656633881303420903547016779 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_213 : node_212+node_212=node_213 := by
  dsimp only [node_212,node_213]
  apply tangent_double (ℓ:=(164890738910072091351238362663840679651531445024 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_213 : (36 : ℕ) • G=node_213 := by
  rw [show (36 : ℕ)=18+18 by norm_num,
    add_nsmul,nsmul_212]
  exact add_213

private def node_214 : formulaCurve.Point :=
  .some (616974833709121204844928542698471200079234094318 : F) (259620432950677032261599742595512883860491937112 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_214 : node_213+node_213=node_214 := by
  dsimp only [node_213,node_214]
  apply tangent_double (ℓ:=(504149023852112336053104268800914107078637332433 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_214 : (72 : ℕ) • G=node_214 := by
  rw [show (72 : ℕ)=36+36 by norm_num,
    add_nsmul,nsmul_213]
  exact add_214

private def node_215 : formulaCurve.Point :=
  .some (512661292726615500272144661085188520799475482830 : F) (145363080251366438409832557801871133606531739181 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_215 : node_214+node_0=node_215 := by
  dsimp only [node_214,node_0,node_215]
  apply secant_add (ℓ:=(716648407087023010844501084701675720416733830360 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_215 : (73 : ℕ) • G=node_215 := by
  rw [show (73 : ℕ)=72+1 by norm_num,
    add_nsmul,nsmul_214,nsmul_0]
  exact add_215

private def node_216 : formulaCurve.Point :=
  .some (195810589931461511032100325569550890570917278080 : F) (135456574536539342306042457962441717889957344073 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_216 : node_215+node_215=node_216 := by
  dsimp only [node_215,node_216]
  apply tangent_double (ℓ:=(132426312740540854246206116704999629711280522300 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_216 : (146 : ℕ) • G=node_216 := by
  rw [show (146 : ℕ)=73+73 by norm_num,
    add_nsmul,nsmul_215]
  exact add_216

private def node_217 : formulaCurve.Point :=
  .some (255428822934863271210945136186609828614362127513 : F) (214611883718145554446129706975486061189886486159 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_217 : node_216+node_0=node_217 := by
  dsimp only [node_216,node_0,node_217]
  apply secant_add (ℓ:=(183661331830334831486783059023763181987191517295 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_217 : (147 : ℕ) • G=node_217 := by
  rw [show (147 : ℕ)=146+1 by norm_num,
    add_nsmul,nsmul_216,nsmul_0]
  exact add_217

private def node_218 : formulaCurve.Point :=
  .some (267369980236312690351514313871985997224325725201 : F) (569489270074330001539260011179505768200831242135 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_218 : node_217+node_217=node_218 := by
  dsimp only [node_217,node_218]
  apply tangent_double (ℓ:=(47300144987979939004859699555530197023028714781 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_218 : (294 : ℕ) • G=node_218 := by
  rw [show (294 : ℕ)=147+147 by norm_num,
    add_nsmul,nsmul_217]
  exact add_218

private def node_219 : formulaCurve.Point :=
  .some (199698973521428649771461916377718084116574416212 : F) (378517417395862819171718014200646882086517993887 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_219 : node_218+node_218=node_219 := by
  dsimp only [node_218,node_219]
  apply tangent_double (ℓ:=(150858939029354543343803982232806854228733814106 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_219 : (588 : ℕ) • G=node_219 := by
  rw [show (588 : ℕ)=294+294 by norm_num,
    add_nsmul,nsmul_218]
  exact add_219

private def node_220 : formulaCurve.Point :=
  .some (685974348919291906510284736768591875362216033390 : F) (201346708572422419972433867779340196950489842520 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_220 : node_219+node_0=node_220 := by
  dsimp only [node_219,node_0,node_220]
  apply secant_add (ℓ:=(500252034749954017826208958054976423646150394345 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_220 : (589 : ℕ) • G=node_220 := by
  rw [show (589 : ℕ)=588+1 by norm_num,
    add_nsmul,nsmul_219,nsmul_0]
  exact add_220

private def node_221 : formulaCurve.Point :=
  .some (88603541373445955359921127930195691904334685005 : F) (295715549946444182434867834241103130342802603394 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_221 : node_220+node_220=node_221 := by
  dsimp only [node_220,node_221]
  apply tangent_double (ℓ:=(211034546386018967737876105249588349515374633074 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_221 : (1178 : ℕ) • G=node_221 := by
  rw [show (1178 : ℕ)=589+589 by norm_num,
    add_nsmul,nsmul_220]
  exact add_221

private def node_222 : formulaCurve.Point :=
  .some (539783378105349109304557030083814431194093124797 : F) (338599238799580730550224636696162956352236638334 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_222 : node_221+node_0=node_222 := by
  dsimp only [node_221,node_0,node_222]
  apply secant_add (ℓ:=(295942278826905758753986601028092319398754880057 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_222 : (1179 : ℕ) • G=node_222 := by
  rw [show (1179 : ℕ)=1178+1 by norm_num,
    add_nsmul,nsmul_221,nsmul_0]
  exact add_222

private def node_223 : formulaCurve.Point :=
  .some (531266810111975660682718817928416860721409166755 : F) (340266938194155859705526139924152628023057548874 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_223 : node_222+node_222=node_223 := by
  dsimp only [node_222,node_223]
  apply tangent_double (ℓ:=(320644644247693794493938065667639230158792852097 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_223 : (2358 : ℕ) • G=node_223 := by
  rw [show (2358 : ℕ)=1179+1179 by norm_num,
    add_nsmul,nsmul_222]
  exact add_223

private def node_224 : formulaCurve.Point :=
  .some (174925506580454189290786225545350127848393029566 : F) (178931967713744866911937604346802526589537218150 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_224 : node_223+node_223=node_224 := by
  dsimp only [node_223,node_224]
  apply tangent_double (ℓ:=(10049218114319066564508108750836877796507199154 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_224 : (4716 : ℕ) • G=node_224 := by
  rw [show (4716 : ℕ)=2358+2358 by norm_num,
    add_nsmul,nsmul_223]
  exact add_224

private def node_225 : formulaCurve.Point :=
  .some (465343182583613210441846481491800695339319659216 : F) (78533310070460794404724087782610776868503655636 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_225 : node_224+node_0=node_225 := by
  dsimp only [node_224,node_0,node_225]
  apply secant_add (ℓ:=(568735264715460724131963894213599128074024424230 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_225 : (4717 : ℕ) • G=node_225 := by
  rw [show (4717 : ℕ)=4716+1 by norm_num,
    add_nsmul,nsmul_224,nsmul_0]
  exact add_225

private def node_226 : formulaCurve.Point :=
  .some (532492879183663647073904839584187377092224043570 : F) (482377700591874327401032911368785307137259644674 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_226 : node_225+node_225=node_226 := by
  dsimp only [node_225,node_226]
  apply tangent_double (ℓ:=(410241797304764456806445751583587620504840379484 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_226 : (9434 : ℕ) • G=node_226 := by
  rw [show (9434 : ℕ)=4717+4717 by norm_num,
    add_nsmul,nsmul_225]
  exact add_226

private def node_227 : formulaCurve.Point :=
  .some (652127362487206878055676472325830204075641326462 : F) (64280494679382155660116042079569902219554460049 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_227 : node_226+node_226=node_227 := by
  dsimp only [node_226,node_227]
  apply tangent_double (ℓ:=(665296732110713511136345058578991665500376823228 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_227 : (18868 : ℕ) • G=node_227 := by
  rw [show (18868 : ℕ)=9434+9434 by norm_num,
    add_nsmul,nsmul_226]
  exact add_227

private def node_228 : formulaCurve.Point :=
  .some (471251650958092271256616126049811370454587137343 : F) (675761858752383827485103921428577607166214557381 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_228 : node_227+node_227=node_228 := by
  dsimp only [node_227,node_228]
  apply tangent_double (ℓ:=(709183035458199715084717619664594538288819795636 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_228 : (37736 : ℕ) • G=node_228 := by
  rw [show (37736 : ℕ)=18868+18868 by norm_num,
    add_nsmul,nsmul_227]
  exact add_228

private def node_229 : formulaCurve.Point :=
  .some (258780849498513430669267801208195274601126220911 : F) (643188133151063398622122987501823094436079438142 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_229 : node_228+node_0=node_229 := by
  dsimp only [node_228,node_0,node_229]
  apply secant_add (ℓ:=(191276083133210994647352920441860026527447793874 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_229 : (37737 : ℕ) • G=node_229 := by
  rw [show (37737 : ℕ)=37736+1 by norm_num,
    add_nsmul,nsmul_228,nsmul_0]
  exact add_229

private def node_230 : formulaCurve.Point :=
  .some (458526176145059084068941983230256700519950358208 : F) (546670620439038921809661031630466611532944712366 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_230 : node_229+node_229=node_230 := by
  dsimp only [node_229,node_230]
  apply tangent_double (ℓ:=(714631697675186318868138990924961036602875515542 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_230 : (75474 : ℕ) • G=node_230 := by
  rw [show (75474 : ℕ)=37737+37737 by norm_num,
    add_nsmul,nsmul_229]
  exact add_230

private def node_231 : formulaCurve.Point :=
  .some (685255049633754227846433334113173055097613767385 : F) (89044412375991104218582735061763292919139444333 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_231 : node_230+node_230=node_231 := by
  dsimp only [node_230,node_231]
  apply tangent_double (ℓ:=(116337019428900442703110830637357101366157360443 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_231 : (150948 : ℕ) • G=node_231 := by
  rw [show (150948 : ℕ)=75474+75474 by norm_num,
    add_nsmul,nsmul_230]
  exact add_231

private def node_232 : formulaCurve.Point :=
  .some (606154601173535870393138329234707681959895070457 : F) (540551622993260427056725619558661142486890610159 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_232 : node_231+node_0=node_232 := by
  dsimp only [node_231,node_0,node_232]
  apply secant_add (ℓ:=(551447089442502410337877217800733718827424232451 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_232 : (150949 : ℕ) • G=node_232 := by
  rw [show (150949 : ℕ)=150948+1 by norm_num,
    add_nsmul,nsmul_231,nsmul_0]
  exact add_232

private def node_233 : formulaCurve.Point :=
  .some (496648009445642446298076389781666651729650600899 : F) (574461917070554643673290688975542501564000248211 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_233 : node_232+node_232=node_233 := by
  dsimp only [node_232,node_233]
  apply tangent_double (ℓ:=(11396511090674887931141621003498378861842995721 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_233 : (301898 : ℕ) • G=node_233 := by
  rw [show (301898 : ℕ)=150949+150949 by norm_num,
    add_nsmul,nsmul_232]
  exact add_233

private def node_234 : formulaCurve.Point :=
  .some (18646713446966950068396400693581209260507298230 : F) (463134942469837918670429504953098241648561369340 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_234 : node_233+node_0=node_234 := by
  dsimp only [node_233,node_0,node_234]
  apply secant_add (ℓ:=(89010224958508555250403150983500897281145823359 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_234 : (301899 : ℕ) • G=node_234 := by
  rw [show (301899 : ℕ)=301898+1 by norm_num,
    add_nsmul,nsmul_233,nsmul_0]
  exact add_234

private def node_235 : formulaCurve.Point :=
  .some (282571212158201611778386349773667263811562716109 : F) (236599628014781951184123662808582552020242004717 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_235 : node_234+node_234=node_235 := by
  dsimp only [node_234,node_235]
  apply tangent_double (ℓ:=(590751711984197321413557919324351141451390578281 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_235 : (603798 : ℕ) • G=node_235 := by
  rw [show (603798 : ℕ)=301899+301899 by norm_num,
    add_nsmul,nsmul_234]
  exact add_235

private def node_236 : formulaCurve.Point :=
  .some (13855093748175526960115322776225325657447511232 : F) (86849547504518232481121797976754649261946812843 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_236 : node_235+node_0=node_236 := by
  dsimp only [node_235,node_0,node_236]
  apply secant_add (ℓ:=(367625994953692540310692220947903135644330619056 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_236 : (603799 : ℕ) • G=node_236 := by
  rw [show (603799 : ℕ)=603798+1 by norm_num,
    add_nsmul,nsmul_235,nsmul_0]
  exact add_236

private def node_237 : formulaCurve.Point :=
  .some (399076087210167499460703574675247503152325594628 : F) (361216318288249960007444597433298991358512756132 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_237 : node_236+node_236=node_237 := by
  dsimp only [node_236,node_237]
  apply tangent_double (ℓ:=(141033244954240692254330053636483154890509318131 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_237 : (1207598 : ℕ) • G=node_237 := by
  rw [show (1207598 : ℕ)=603799+603799 by norm_num,
    add_nsmul,nsmul_236]
  exact add_237

private def node_238 : formulaCurve.Point :=
  .some (82591587726622216722224673923521641305696958837 : F) (195159046497568087623284439030268698764318907180 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_238 : node_237+node_237=node_238 := by
  dsimp only [node_237,node_238]
  apply tangent_double (ℓ:=(37212776351279955646418896131399139339011018598 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_238 : (2415196 : ℕ) • G=node_238 := by
  rw [show (2415196 : ℕ)=1207598+1207598 by norm_num,
    add_nsmul,nsmul_237]
  exact add_238

private def node_239 : formulaCurve.Point :=
  .some (592543401006561819383957732745025532397930388254 : F) (332823967404797543752618301168756481125690716187 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_239 : node_238+node_238=node_239 := by
  dsimp only [node_238,node_239]
  apply tangent_double (ℓ:=(314646769259531803097450922306572917438384993438 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_239 : (4830392 : ℕ) • G=node_239 := by
  rw [show (4830392 : ℕ)=2415196+2415196 by norm_num,
    add_nsmul,nsmul_238]
  exact add_239

private def node_240 : formulaCurve.Point :=
  .some (25621050070336581870228772139318713336209923687 : F) (110445467430472480588742132697092590935996116766 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_240 : node_239+node_239=node_240 := by
  dsimp only [node_239,node_240]
  apply tangent_double (ℓ:=(621370372374617115216997426459308688260938920241 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_240 : (9660784 : ℕ) • G=node_240 := by
  rw [show (9660784 : ℕ)=4830392+4830392 by norm_num,
    add_nsmul,nsmul_239]
  exact add_240

private def node_241 : formulaCurve.Point :=
  .some (392359586675451588236215926238750710861564349253 : F) (570868950118085626001562383339150159048126711514 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_241 : node_240+node_0=node_241 := by
  dsimp only [node_240,node_0,node_241]
  apply secant_add (ℓ:=(696499513612715405027193617292972638365401786370 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_241 : (9660785 : ℕ) • G=node_241 := by
  rw [show (9660785 : ℕ)=9660784+1 by norm_num,
    add_nsmul,nsmul_240,nsmul_0]
  exact add_241

private def node_242 : formulaCurve.Point :=
  .some (398895614129923605088761361215347570269212954887 : F) (333419197499786011276933424891184133816124936577 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_242 : node_241+node_241=node_242 := by
  dsimp only [node_241,node_242]
  apply tangent_double (ℓ:=(473226593115314163492290305139312079344432350215 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_242 : (19321570 : ℕ) • G=node_242 := by
  rw [show (19321570 : ℕ)=9660785+9660785 by norm_num,
    add_nsmul,nsmul_241]
  exact add_242

private def node_243 : formulaCurve.Point :=
  .some (239072076039567448438963076816936149202752336904 : F) (307144942486827986211989017905092322791342333833 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_243 : node_242+node_0=node_243 := by
  dsimp only [node_242,node_0,node_243]
  apply secant_add (ℓ:=(638117386008255194486910677084390857886502778120 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_243 : (19321571 : ℕ) • G=node_243 := by
  rw [show (19321571 : ℕ)=19321570+1 by norm_num,
    add_nsmul,nsmul_242,nsmul_0]
  exact add_243

private def node_244 : formulaCurve.Point :=
  .some (217056916694768349939382146239985513559279711771 : F) (446702702004451977934280447022613661519948680869 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_244 : node_243+node_243=node_244 := by
  dsimp only [node_243,node_244]
  apply tangent_double (ℓ:=(96901305590919653271344223966745280749399308791 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_244 : (38643142 : ℕ) • G=node_244 := by
  rw [show (38643142 : ℕ)=19321571+19321571 by norm_num,
    add_nsmul,nsmul_243]
  exact add_244

private def node_245 : formulaCurve.Point :=
  .some (48856577009163002299298912427587735036938239667 : F) (226974057410261602140860014282132253315186698924 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_245 : node_244+node_0=node_245 := by
  dsimp only [node_244,node_0,node_245]
  apply secant_add (ℓ:=(485416595127166610026891382620472730622710478237 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_245 : (38643143 : ℕ) • G=node_245 := by
  rw [show (38643143 : ℕ)=38643142+1 by norm_num,
    add_nsmul,nsmul_244,nsmul_0]
  exact add_245

private def node_246 : formulaCurve.Point :=
  .some (190950896880308434164106639708353386135128612185 : F) (218168727426899574090422769226191645584378350132 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_246 : node_245+node_245=node_246 := by
  dsimp only [node_245,node_246]
  apply tangent_double (ℓ:=(299754807326350121358196189482226953310041039137 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_246 : (77286286 : ℕ) • G=node_246 := by
  rw [show (77286286 : ℕ)=38643143+38643143 by norm_num,
    add_nsmul,nsmul_245]
  exact add_246

private def node_247 : formulaCurve.Point :=
  .some (653328618843442253053144937348873744326869672914 : F) (204109321726868650540544220419708968951551193356 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_247 : node_246+node_0=node_247 := by
  dsimp only [node_246,node_0,node_247]
  apply secant_add (ℓ:=(284083714604813428935286678095561215552977757006 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_247 : (77286287 : ℕ) • G=node_247 := by
  rw [show (77286287 : ℕ)=77286286+1 by norm_num,
    add_nsmul,nsmul_246,nsmul_0]
  exact add_247

private def node_248 : formulaCurve.Point :=
  .some (574505618977435585643611455366004278119146849221 : F) (203698313429282667754961375532992923360215077469 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_248 : node_247+node_247=node_248 := by
  dsimp only [node_247,node_248]
  apply tangent_double (ℓ:=(269751619778931044590396688671359468135507190618 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_248 : (154572574 : ℕ) • G=node_248 := by
  rw [show (154572574 : ℕ)=77286287+77286287 by norm_num,
    add_nsmul,nsmul_247]
  exact add_248

private def node_249 : formulaCurve.Point :=
  .some (721210849235350852380509100689331261783638357701 : F) (446038168637937975303138612751957905148170509438 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_249 : node_248+node_0=node_249 := by
  dsimp only [node_248,node_0,node_249]
  apply secant_add (ℓ:=(136449771713650543303210319175628021315000590432 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_249 : (154572575 : ℕ) • G=node_249 := by
  rw [show (154572575 : ℕ)=154572574+1 by norm_num,
    add_nsmul,nsmul_248,nsmul_0]
  exact add_249

private def node_250 : formulaCurve.Point :=
  .some (316696805655481299721516391616597670054457187050 : F) (705038738442564369907652579114737722952172900555 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_250 : node_249+node_249=node_250 := by
  dsimp only [node_249,node_250]
  apply tangent_double (ℓ:=(55889133778606545804337620286299239452545822710 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_250 : (309145150 : ℕ) • G=node_250 := by
  rw [show (309145150 : ℕ)=154572575+154572575 by norm_num,
    add_nsmul,nsmul_249]
  exact add_250

private def node_251 : formulaCurve.Point :=
  .some (177225152372401499650984098789088547658023852104 : F) (206862267525544090028734948796857909265355332175 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_251 : node_250+node_0=node_251 := by
  dsimp only [node_250,node_0,node_251]
  apply secant_add (ℓ:=(482435656782989362529637762250499858901790659016 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_251 : (309145151 : ℕ) • G=node_251 := by
  rw [show (309145151 : ℕ)=309145150+1 by norm_num,
    add_nsmul,nsmul_250,nsmul_0]
  exact add_251

private def node_252 : formulaCurve.Point :=
  .some (578874282512970953534350282079354795609374854736 : F) (126119034253575389110235546309104591337348874454 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_252 : node_251+node_251=node_252 := by
  dsimp only [node_251,node_252]
  apply tangent_double (ℓ:=(506148621830390472920314366729398793603445473241 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_252 : (618290302 : ℕ) • G=node_252 := by
  rw [show (618290302 : ℕ)=309145151+309145151 by norm_num,
    add_nsmul,nsmul_251]
  exact add_252

private def node_253 : formulaCurve.Point :=
  .some (686207272229407309411917694606227220698508624432 : F) (76425681176236946660057909487283749392895703482 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_253 : node_252+node_0=node_253 := by
  dsimp only [node_252,node_0,node_253]
  apply secant_add (ℓ:=(717784688366490585041987683577062913385936944069 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_253 : (618290303 : ℕ) • G=node_253 := by
  rw [show (618290303 : ℕ)=618290302+1 by norm_num,
    add_nsmul,nsmul_252,nsmul_0]
  exact add_253

private def node_254 : formulaCurve.Point :=
  .some (259630372236519151595503780112464303599190265063 : F) (82804909992999273169988022408615283877864861832 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_254 : node_253+node_253=node_254 := by
  dsimp only [node_253,node_254]
  apply tangent_double (ℓ:=(730282997862220449261129393698302774437439059240 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_254 : (1236580606 : ℕ) • G=node_254 := by
  rw [show (1236580606 : ℕ)=618290303+618290303 by norm_num,
    add_nsmul,nsmul_253]
  exact add_254

private def node_255 : formulaCurve.Point :=
  .some (527582271072821577905914128331553005280835610064 : F) (446665123228582783221250472003287294802106068852 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_255 : node_254+node_0=node_255 := by
  dsimp only [node_254,node_0,node_255]
  apply secant_add (ℓ:=(251019772569459440002556164065138691436302573126 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_255 : (1236580607 : ℕ) • G=node_255 := by
  rw [show (1236580607 : ℕ)=1236580606+1 by norm_num,
    add_nsmul,nsmul_254,nsmul_0]
  exact add_255

private def node_256 : formulaCurve.Point :=
  .some (69055178628527419178054701810100322105883563352 : F) (403174919816258071545635332683643409753382036721 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_256 : node_255+node_255=node_256 := by
  dsimp only [node_255,node_256]
  apply tangent_double (ℓ:=(27875181021246642964277754441272884103401774224 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_256 : (2473161214 : ℕ) • G=node_256 := by
  rw [show (2473161214 : ℕ)=1236580607+1236580607 by norm_num,
    add_nsmul,nsmul_255]
  exact add_256

private def node_257 : formulaCurve.Point :=
  .some (143675340858363161999692787902404255544440865122 : F) (151787759980930329633963611878551821318106873297 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_257 : node_256+node_256=node_257 := by
  dsimp only [node_256,node_257]
  apply tangent_double (ℓ:=(30349418434853904337428449676510552225532731317 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_257 : (4946322428 : ℕ) • G=node_257 := by
  rw [show (4946322428 : ℕ)=2473161214+2473161214 by norm_num,
    add_nsmul,nsmul_256]
  exact add_257

private def node_258 : formulaCurve.Point :=
  .some (207846191836967714765176281892515178489702933499 : F) (361888734492814175705778202926019078552132511418 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_258 : node_257+node_0=node_258 := by
  dsimp only [node_257,node_0,node_258]
  apply secant_add (ℓ:=(384096182667402120928637537883086031581599795481 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_258 : (4946322429 : ℕ) • G=node_258 := by
  rw [show (4946322429 : ℕ)=4946322428+1 by norm_num,
    add_nsmul,nsmul_257,nsmul_0]
  exact add_258

private def node_259 : formulaCurve.Point :=
  .some (68265638230881487589568018945976381235050812021 : F) (458762923418144888508685981961322523093558220670 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_259 : node_258+node_258=node_259 := by
  dsimp only [node_258,node_259]
  apply tangent_double (ℓ:=(523458742488113500252121440636565303648044216788 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_259 : (9892644858 : ℕ) • G=node_259 := by
  rw [show (9892644858 : ℕ)=4946322429+4946322429 by norm_num,
    add_nsmul,nsmul_258]
  exact add_259

private def node_260 : formulaCurve.Point :=
  .some (163333238629768201053281444955952091717549384384 : F) (643042587201727548416738829801259597638545899868 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_260 : node_259+node_259=node_260 := by
  dsimp only [node_259,node_260]
  apply tangent_double (ℓ:=(527838387049791588169305279294143884965110635012 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_260 : (19785289716 : ℕ) • G=node_260 := by
  rw [show (19785289716 : ℕ)=9892644858+9892644858 by norm_num,
    add_nsmul,nsmul_259]
  exact add_260

private def node_261 : formulaCurve.Point :=
  .some (499356738111560972696246157147099256591510226015 : F) (272048098395832108374922510030896937424734184143 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_261 : node_260+node_260=node_261 := by
  dsimp only [node_260,node_261]
  apply tangent_double (ℓ:=(261797436499380330260083269612191157023500504596 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_261 : (39570579432 : ℕ) • G=node_261 := by
  rw [show (39570579432 : ℕ)=19785289716+19785289716 by norm_num,
    add_nsmul,nsmul_260]
  exact add_261

private def node_262 : formulaCurve.Point :=
  .some (627614191004047431706139994038508014370247307004 : F) (313162793288108902225630456004433436350440404498 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_262 : node_261+node_261=node_262 := by
  dsimp only [node_261,node_262]
  apply tangent_double (ℓ:=(510810159105828658531583223368764594633221070395 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_262 : (79141158864 : ℕ) • G=node_262 := by
  rw [show (79141158864 : ℕ)=39570579432+39570579432 by norm_num,
    add_nsmul,nsmul_261]
  exact add_262

private def node_263 : formulaCurve.Point :=
  .some (554094374914105834980114047382463037452387597321 : F) (577978702140580806450299389919803788337913752441 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_263 : node_262+node_0=node_263 := by
  dsimp only [node_262,node_0,node_263]
  apply secant_add (ℓ:=(11975702360745098563573635945558018320373373740 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_263 : (79141158865 : ℕ) • G=node_263 := by
  rw [show (79141158865 : ℕ)=79141158864+1 by norm_num,
    add_nsmul,nsmul_262,nsmul_0]
  exact add_263

private def node_264 : formulaCurve.Point :=
  .some (640199737577556945805531678055586312614360358743 : F) (133337315298755961394154884909801426342163244000 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_264 : node_263+node_263=node_264 := by
  dsimp only [node_263,node_264]
  apply tangent_double (ℓ:=(491584849219717871338217577766566676261205733982 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_264 : (158282317730 : ℕ) • G=node_264 := by
  rw [show (158282317730 : ℕ)=79141158865+79141158865 by norm_num,
    add_nsmul,nsmul_263]
  exact add_264

private def node_265 : formulaCurve.Point :=
  .some (136203781446377398789351801369034747285410724209 : F) (458484913920548986967257645306110984700182934878 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_265 : node_264+node_0=node_265 := by
  dsimp only [node_264,node_0,node_265]
  apply secant_add (ℓ:=(110697904922052255447394101885423954367771158303 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_265 : (158282317731 : ℕ) • G=node_265 := by
  rw [show (158282317731 : ℕ)=158282317730+1 by norm_num,
    add_nsmul,nsmul_264,nsmul_0]
  exact add_265

private def node_266 : formulaCurve.Point :=
  .some (450335951777359317716199752105604051050641886221 : F) (102682900660429538069690829256398513908977972235 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_266 : node_265+node_265=node_266 := by
  dsimp only [node_265,node_266]
  apply tangent_double (ℓ:=(608905088003749582805407848433986797386035701212 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_266 : (316564635462 : ℕ) • G=node_266 := by
  rw [show (316564635462 : ℕ)=158282317731+158282317731 by norm_num,
    add_nsmul,nsmul_265]
  exact add_266

private def node_267 : formulaCurve.Point :=
  .some (636834395837778060682038053651339096683820730764 : F) (634796926802414857342407968749678972475212113099 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_267 : node_266+node_0=node_267 := by
  dsimp only [node_266,node_0,node_267]
  apply secant_add (ℓ:=(118886469695994622960788642855753802440664067441 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_267 : (316564635463 : ℕ) • G=node_267 := by
  rw [show (316564635463 : ℕ)=316564635462+1 by norm_num,
    add_nsmul,nsmul_266,nsmul_0]
  exact add_267

private def node_268 : formulaCurve.Point :=
  .some (581307553112362944581786157074816497882177123357 : F) (595486526034628635010902238048817106020624786157 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_268 : node_267+node_267=node_268 := by
  dsimp only [node_267,node_268]
  apply tangent_double (ℓ:=(324946340969417956077701972974487543666780852375 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_268 : (633129270926 : ℕ) • G=node_268 := by
  rw [show (633129270926 : ℕ)=316564635463+316564635463 by norm_num,
    add_nsmul,nsmul_267]
  exact add_268

private def node_269 : formulaCurve.Point :=
  .some (70386553154886032928084055254351104769803422048 : F) (509902262976492269150806507958803124579577833833 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_269 : node_268+node_268=node_269 := by
  dsimp only [node_268,node_269]
  apply tangent_double (ℓ:=(219536435264519201032129517588173225167295384272 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_269 : (1266258541852 : ℕ) • G=node_269 := by
  rw [show (1266258541852 : ℕ)=633129270926+633129270926 by norm_num,
    add_nsmul,nsmul_268]
  exact add_269

private def node_270 : formulaCurve.Point :=
  .some (98114830262247330160569604056425317918321239580 : F) (113102089589234493624176416571093951038505896553 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_270 : node_269+node_0=node_270 := by
  dsimp only [node_269,node_0,node_270]
  apply secant_add (ℓ:=(439855694348219412976420297236284934483228315377 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_270 : (1266258541853 : ℕ) • G=node_270 := by
  rw [show (1266258541853 : ℕ)=1266258541852+1 by norm_num,
    add_nsmul,nsmul_269,nsmul_0]
  exact add_270

private def node_271 : formulaCurve.Point :=
  .some (383401701250372797688717399503058954645572960926 : F) (388543111083862123910856546278143064999300039874 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_271 : node_270+node_270=node_271 := by
  dsimp only [node_270,node_271]
  apply tangent_double (ℓ:=(175780701716254517962235779863587611448986202151 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_271 : (2532517083706 : ℕ) • G=node_271 := by
  rw [show (2532517083706 : ℕ)=1266258541853+1266258541853 by norm_num,
    add_nsmul,nsmul_270]
  exact add_271

private def node_272 : formulaCurve.Point :=
  .some (376042420755864465259139066819197240112158106889 : F) (252094336374912132704568430935378748281949734548 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_272 : node_271+node_271=node_272 := by
  dsimp only [node_271,node_272]
  apply tangent_double (ℓ:=(153982935514379233053131789246381681484875933909 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_272 : (5065034167412 : ℕ) • G=node_272 := by
  rw [show (5065034167412 : ℕ)=2532517083706+2532517083706 by norm_num,
    add_nsmul,nsmul_271]
  exact add_272

private def node_273 : formulaCurve.Point :=
  .some (413385291978904631408578011610589945612380686962 : F) (475664298089957630996292537093806554666798894254 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_273 : node_272+node_0=node_273 := by
  dsimp only [node_272,node_0,node_273]
  apply secant_add (ℓ:=(133148354028010307278388927614373919582954765331 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_273 : (5065034167413 : ℕ) • G=node_273 := by
  rw [show (5065034167413 : ℕ)=5065034167412+1 by norm_num,
    add_nsmul,nsmul_272,nsmul_0]
  exact add_273

private def node_274 : formulaCurve.Point :=
  .some (261480851649280744141814037076561049306679638575 : F) (722923549516919018341592906765586350530234699494 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_274 : node_273+node_273=node_274 := by
  dsimp only [node_273,node_274]
  apply tangent_double (ℓ:=(290631831578647091023349848268832123502325750187 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_274 : (10130068334826 : ℕ) • G=node_274 := by
  rw [show (10130068334826 : ℕ)=5065034167413+5065034167413 by norm_num,
    add_nsmul,nsmul_273]
  exact add_274

private def node_275 : formulaCurve.Point :=
  .some (251939330515062843093475688753459958938002592085 : F) (438678597790074904859725104445254662535016450243 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_275 : node_274+node_274=node_275 := by
  dsimp only [node_274,node_275]
  apply tangent_double (ℓ:=(541312438145612511809753353546835452744598179454 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_275 : (20260136669652 : ℕ) • G=node_275 := by
  rw [show (20260136669652 : ℕ)=10130068334826+10130068334826 by norm_num,
    add_nsmul,nsmul_274]
  exact add_275

private def node_276 : formulaCurve.Point :=
  .some (219723068763748257890384644601314852767239539962 : F) (86654708303305006703363787405888375442681381586 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_276 : node_275+node_0=node_276 := by
  dsimp only [node_275,node_0,node_276]
  apply secant_add (ℓ:=(49942087090849925197777701068723911416657713289 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_276 : (20260136669653 : ℕ) • G=node_276 := by
  rw [show (20260136669653 : ℕ)=20260136669652+1 by norm_num,
    add_nsmul,nsmul_275,nsmul_0]
  exact add_276

private def node_277 : formulaCurve.Point :=
  .some (417042709519179900331607043412682602117580617142 : F) (184564977336468145876833489545443923853390838586 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_277 : node_276+node_276=node_277 := by
  dsimp only [node_276,node_277]
  apply tangent_double (ℓ:=(183001831089196506365587076262573152103345019297 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_277 : (40520273339306 : ℕ) • G=node_277 := by
  rw [show (40520273339306 : ℕ)=20260136669653+20260136669653 by norm_num,
    add_nsmul,nsmul_276]
  exact add_277

private def node_278 : formulaCurve.Point :=
  .some (157805859383879328293122497057910356296071400995 : F) (412212651669157904794019977312146146397664042947 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_278 : node_277+node_0=node_278 := by
  dsimp only [node_277,node_0,node_278]
  apply secant_add (ℓ:=(185957156736100670821431377698919002298749532228 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_278 : (40520273339307 : ℕ) • G=node_278 := by
  rw [show (40520273339307 : ℕ)=40520273339306+1 by norm_num,
    add_nsmul,nsmul_277,nsmul_0]
  exact add_278

private def node_279 : formulaCurve.Point :=
  .some (717756763092661900201890469876635115038451659792 : F) (512949280800704981285437759053259758730670029272 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_279 : node_278+node_278=node_279 := by
  dsimp only [node_278,node_279]
  apply tangent_double (ℓ:=(15781593060121242289125680660289554156042723524 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_279 : (81040546678614 : ℕ) • G=node_279 := by
  rw [show (81040546678614 : ℕ)=40520273339307+40520273339307 by norm_num,
    add_nsmul,nsmul_278]
  exact add_279

private def node_280 : formulaCurve.Point :=
  .some (152984302278924080705522909321245107375017893795 : F) (38079773084718684984463756857925833782186364867 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_280 : node_279+node_0=node_280 := by
  dsimp only [node_279,node_0,node_280]
  apply secant_add (ℓ:=(369681981357449187338545722169930907121480756745 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_280 : (81040546678615 : ℕ) • G=node_280 := by
  rw [show (81040546678615 : ℕ)=81040546678614+1 by norm_num,
    add_nsmul,nsmul_279,nsmul_0]
  exact add_280

private def node_281 : formulaCurve.Point :=
  .some (235958558785649721190724485442558210251969354497 : F) (202944055428653271620973308820502212157720139149 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_281 : node_280+node_280=node_281 := by
  dsimp only [node_280,node_281]
  apply tangent_double (ℓ:=(435460206520479303645239479036528474317867015060 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_281 : (162081093357230 : ℕ) • G=node_281 := by
  rw [show (162081093357230 : ℕ)=81040546678615+81040546678615 by norm_num,
    add_nsmul,nsmul_280]
  exact add_281

private def node_282 : formulaCurve.Point :=
  .some (512364575341759740484451753296748019353504755706 : F) (291667237086333599774386777742409860120127408926 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_282 : node_281+node_0=node_282 := by
  dsimp only [node_281,node_0,node_282]
  apply secant_add (ℓ:=(409064414104393039847588575122290787679306418364 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_282 : (162081093357231 : ℕ) • G=node_282 := by
  rw [show (162081093357231 : ℕ)=162081093357230+1 by norm_num,
    add_nsmul,nsmul_281,nsmul_0]
  exact add_282

private def node_283 : formulaCurve.Point :=
  .some (394214268413050418943205290129213482082791816194 : F) (278610042868545335553369420616618012793211928892 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_283 : node_282+node_282=node_283 := by
  dsimp only [node_282,node_283]
  apply tangent_double (ℓ:=(66595180476272191687184042719573347749562002452 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_283 : (324162186714462 : ℕ) • G=node_283 := by
  rw [show (324162186714462 : ℕ)=162081093357231+162081093357231 by norm_num,
    add_nsmul,nsmul_282]
  exact add_283

private def node_284 : formulaCurve.Point :=
  .some (75801363926847289623196818088044080562213694095 : F) (386685321302824766676002430375605856854920021912 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_284 : node_283+node_283=node_284 := by
  dsimp only [node_283,node_284]
  apply tangent_double (ℓ:=(607559505698106145710311592748873016816033419035 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_284 : (648324373428924 : ℕ) • G=node_284 := by
  rw [show (648324373428924 : ℕ)=324162186714462+324162186714462 by norm_num,
    add_nsmul,nsmul_283]
  exact add_284

private def node_285 : formulaCurve.Point :=
  .some (259683638596418409073514368706223438511201669464 : F) (516855462770593328996015894465281603076248739790 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_285 : node_284+node_0=node_285 := by
  dsimp only [node_284,node_0,node_285]
  apply secant_add (ℓ:=(317532110371736757283534412086573058488204212089 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_285 : (648324373428925 : ℕ) • G=node_285 := by
  rw [show (648324373428925 : ℕ)=648324373428924+1 by norm_num,
    add_nsmul,nsmul_284,nsmul_0]
  exact add_285

private def node_286 : formulaCurve.Point :=
  .some (606753507613748491471264745016929878660659205312 : F) (386856983131329196541831805673766415746470764940 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_286 : node_285+node_285=node_286 := by
  dsimp only [node_285,node_286]
  apply tangent_double (ℓ:=(548745682040783102338661820233041029051707680737 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_286 : (1296648746857850 : ℕ) • G=node_286 := by
  rw [show (1296648746857850 : ℕ)=648324373428925+648324373428925 by norm_num,
    add_nsmul,nsmul_285]
  exact add_286

private def node_287 : formulaCurve.Point :=
  .some (103215776449483166304329830950261676414497661203 : F) (533610238416478013558532251733645897136484076144 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_287 : node_286+node_0=node_287 := by
  dsimp only [node_286,node_0,node_287]
  apply secant_add (ℓ:=(482181675327599586118876559515246910117377530869 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_287 : (1296648746857851 : ℕ) • G=node_287 := by
  rw [show (1296648746857851 : ℕ)=1296648746857850+1 by norm_num,
    add_nsmul,nsmul_286,nsmul_0]
  exact add_287

private def node_288 : formulaCurve.Point :=
  .some (479422388971369147200740444738632046748228476277 : F) (605485049839049332653073449088649689996216765551 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_288 : node_287+node_287=node_288 := by
  dsimp only [node_287,node_288]
  apply tangent_double (ℓ:=(177988634488682658804328910046334557676966178781 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_288 : (2593297493715702 : ℕ) • G=node_288 := by
  rw [show (2593297493715702 : ℕ)=1296648746857851+1296648746857851 by norm_num,
    add_nsmul,nsmul_287]
  exact add_288

private def node_289 : formulaCurve.Point :=
  .some (390768357451555299530113541871472630883798942826 : F) (546047199541735698837835907069827944563695844421 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_289 : node_288+node_0=node_289 := by
  dsimp only [node_288,node_0,node_289]
  apply secant_add (ℓ:=(103363290646820119541500812618016672685004171276 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_289 : (2593297493715703 : ℕ) • G=node_289 := by
  rw [show (2593297493715703 : ℕ)=2593297493715702+1 by norm_num,
    add_nsmul,nsmul_288,nsmul_0]
  exact add_289

private def node_290 : formulaCurve.Point :=
  .some (177102593494644348134184613506774935423228271936 : F) (225022751039839033559773821803590656170331371714 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_290 : node_289+node_289=node_290 := by
  dsimp only [node_289,node_290]
  apply tangent_double (ℓ:=(503719690522414240527620112578634784114721692353 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_290 : (5186594987431406 : ℕ) • G=node_290 := by
  rw [show (5186594987431406 : ℕ)=2593297493715703+2593297493715703 by norm_num,
    add_nsmul,nsmul_289]
  exact add_290

private def node_291 : formulaCurve.Point :=
  .some (728753183949079828460253322165301460802696801269 : F) (163086228475400364749435406065874030783250512528 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_291 : node_290+node_290=node_291 := by
  dsimp only [node_290,node_291]
  apply tangent_double (ℓ:=(313778593600765836082363121595801488348789569630 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_291 : (10373189974862812 : ℕ) • G=node_291 := by
  rw [show (10373189974862812 : ℕ)=5186594987431406+5186594987431406 by norm_num,
    add_nsmul,nsmul_290]
  exact add_291

private def node_292 : formulaCurve.Point :=
  .some (332763479283887029551008053426791579395480497584 : F) (504246079210801715563058906126915147963811872734 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_292 : node_291+node_0=node_292 := by
  dsimp only [node_291,node_0,node_292]
  apply secant_add (ℓ:=(624660515322987346126216790242180551045883197261 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_292 : (10373189974862813 : ℕ) • G=node_292 := by
  rw [show (10373189974862813 : ℕ)=10373189974862812+1 by norm_num,
    add_nsmul,nsmul_291,nsmul_0]
  exact add_292

private def node_293 : formulaCurve.Point :=
  .some (443729481115832476719350120109075192854154499519 : F) (147126797215932503982385921346068713826980955556 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_293 : node_292+node_292=node_293 := by
  dsimp only [node_292,node_293]
  apply tangent_double (ℓ:=(391959292756264624618468782633963988337527740603 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_293 : (20746379949725626 : ℕ) • G=node_293 := by
  rw [show (20746379949725626 : ℕ)=10373189974862813+10373189974862813 by norm_num,
    add_nsmul,nsmul_292]
  exact add_293

private def node_294 : formulaCurve.Point :=
  .some (128829347676412737713502741588681337606745553809 : F) (188313112761267432241160590612333751828253532502 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_294 : node_293+node_293=node_294 := by
  dsimp only [node_293,node_294]
  apply tangent_double (ℓ:=(200935065420165104737960283538683973846247740 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_294 : (41492759899451252 : ℕ) • G=node_294 := by
  rw [show (41492759899451252 : ℕ)=20746379949725626+20746379949725626 by norm_num,
    add_nsmul,nsmul_293]
  exact add_294

private def node_295 : formulaCurve.Point :=
  .some (116500312569923093326593794987143144232177352538 : F) (167775572338852383503209544256149468348875251419 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_295 : node_294+node_0=node_295 := by
  dsimp only [node_294,node_0,node_295]
  apply secant_add (ℓ:=(450933270606042811588570297209131818280025042952 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_295 : (41492759899451253 : ℕ) • G=node_295 := by
  rw [show (41492759899451253 : ℕ)=41492759899451252+1 by norm_num,
    add_nsmul,nsmul_294,nsmul_0]
  exact add_295

private def node_296 : formulaCurve.Point :=
  .some (118935579561595638167318169850469154768765604574 : F) (560634091776363309407866357720459851399619910630 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_296 : node_295+node_295=node_296 := by
  dsimp only [node_295,node_296]
  apply tangent_double (ℓ:=(587826103265100930641189031355936999398403143790 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_296 : (82985519798902506 : ℕ) • G=node_296 := by
  rw [show (82985519798902506 : ℕ)=41492759899451253+41492759899451253 by norm_num,
    add_nsmul,nsmul_295]
  exact add_296

private def node_297 : formulaCurve.Point :=
  .some (232818269143596398632033082919775535569529371318 : F) (223565927394568275210875580831710678728700598966 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_297 : node_296+node_0=node_297 := by
  dsimp only [node_296,node_0,node_297]
  apply secant_add (ℓ:=(349934304367416679575207404107698174226384662972 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_297 : (82985519798902507 : ℕ) • G=node_297 := by
  rw [show (82985519798902507 : ℕ)=82985519798902506+1 by norm_num,
    add_nsmul,nsmul_296,nsmul_0]
  exact add_297

private def node_298 : formulaCurve.Point :=
  .some (2516766027024258036372521363011209823484405298 : F) (484009129464653239771706697274845533543182538042 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_298 : node_297+node_297=node_298 := by
  dsimp only [node_297,node_298]
  apply tangent_double (ℓ:=(166535830079810616970004777709828037433388458033 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_298 : (165971039597805014 : ℕ) • G=node_298 := by
  rw [show (165971039597805014 : ℕ)=82985519798902507+82985519798902507 by norm_num,
    add_nsmul,nsmul_297]
  exact add_298

private def node_299 : formulaCurve.Point :=
  .some (695910225981782030355006819621596076560457972519 : F) (589598698925396383242959742473398548818670469061 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_299 : node_298+node_298=node_299 := by
  dsimp only [node_298,node_299]
  apply tangent_double (ℓ:=(108828306164971851786067449137416588834262642523 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_299 : (331942079195610028 : ℕ) • G=node_299 := by
  rw [show (331942079195610028 : ℕ)=165971039597805014+165971039597805014 by norm_num,
    add_nsmul,nsmul_298]
  exact add_299

private def node_300 : formulaCurve.Point :=
  .some (617098788271658169107749363884718776458878324702 : F) (364717423010218744060842845207422768330851852892 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_300 : node_299+node_0=node_300 := by
  dsimp only [node_299,node_0,node_300]
  apply secant_add (ℓ:=(250665665049928916557474455345119276389697727376 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_300 : (331942079195610029 : ℕ) • G=node_300 := by
  rw [show (331942079195610029 : ℕ)=331942079195610028+1 by norm_num,
    add_nsmul,nsmul_299,nsmul_0]
  exact add_300

private def node_301 : formulaCurve.Point :=
  .some (320372311423352996832444423705949686008988965296 : F) (318452621401522077258082793453287134002691332187 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_301 : node_300+node_300=node_301 := by
  dsimp only [node_300,node_301]
  apply tangent_double (ℓ:=(456208320014768187315006950842996570072003613073 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_301 : (663884158391220058 : ℕ) • G=node_301 := by
  rw [show (663884158391220058 : ℕ)=331942079195610029+331942079195610029 by norm_num,
    add_nsmul,nsmul_300]
  exact add_301

private def node_302 : formulaCurve.Point :=
  .some (147271033860472741487766825129189028696393050429 : F) (484552467585436713092070032483499462409282033247 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_302 : node_301+node_301=node_302 := by
  dsimp only [node_301,node_302]
  apply tangent_double (ℓ:=(210620674457846438675989778094261956870545734449 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_302 : (1327768316782440116 : ℕ) • G=node_302 := by
  rw [show (1327768316782440116 : ℕ)=663884158391220058+663884158391220058 by norm_num,
    add_nsmul,nsmul_301]
  exact add_302

private def node_303 : formulaCurve.Point :=
  .some (433706891064489991065256865396831028206101807791 : F) (516255367291754166189327224787065102771883755952 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_303 : node_302+node_0=node_303 := by
  dsimp only [node_302,node_0,node_303]
  apply secant_add (ℓ:=(244703394636806054776834271495486233314156224870 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_303 : (1327768316782440117 : ℕ) • G=node_303 := by
  rw [show (1327768316782440117 : ℕ)=1327768316782440116+1 by norm_num,
    add_nsmul,nsmul_302,nsmul_0]
  exact add_303

private def node_304 : formulaCurve.Point :=
  .some (620663358293124810246531205109729593516408776118 : F) (502072842460459615538911104361557709353698683191 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_304 : node_303+node_303=node_304 := by
  dsimp only [node_303,node_304]
  apply tangent_double (ℓ:=(584930969975333645376243561988509672118954142976 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_304 : (2655536633564880234 : ℕ) • G=node_304 := by
  rw [show (2655536633564880234 : ℕ)=1327768316782440117+1327768316782440117 by norm_num,
    add_nsmul,nsmul_303]
  exact add_304

private def node_305 : formulaCurve.Point :=
  .some (80031234402989024637709644638542618544358331589 : F) (533602860659669073737251385696602322011361730430 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_305 : node_304+node_0=node_305 := by
  dsimp only [node_304,node_0,node_305]
  apply secant_add (ℓ:=(634267334249893713566894659990682208548302230111 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_305 : (2655536633564880235 : ℕ) • G=node_305 := by
  rw [show (2655536633564880235 : ℕ)=2655536633564880234+1 by norm_num,
    add_nsmul,nsmul_304,nsmul_0]
  exact add_305

private def node_306 : formulaCurve.Point :=
  .some (419285114094070399409441927349222292688470433566 : F) (720551618703474072073863402204782271321052110172 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_306 : node_305+node_305=node_306 := by
  dsimp only [node_305,node_306]
  apply tangent_double (ℓ:=(599182597520412050838812487218763278051343316144 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_306 : (5311073267129760470 : ℕ) • G=node_306 := by
  rw [show (5311073267129760470 : ℕ)=2655536633564880235+2655536633564880235 by norm_num,
    add_nsmul,nsmul_305]
  exact add_306

private def node_307 : formulaCurve.Point :=
  .some (433290822336552009634569075299162919584358016421 : F) (43967154034881825553823772222439944512104795786 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_307 : node_306+node_306=node_307 := by
  dsimp only [node_306,node_307]
  apply tangent_double (ℓ:=(610594691414264245423367506850533534035122252366 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_307 : (10622146534259520940 : ℕ) • G=node_307 := by
  rw [show (10622146534259520940 : ℕ)=5311073267129760470+5311073267129760470 by norm_num,
    add_nsmul,nsmul_306]
  exact add_307

private def node_308 : formulaCurve.Point :=
  .some (267658906682482213384575833033684341949708027563 : F) (24134055078514795490867391240525933054340628119 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_308 : node_307+node_307=node_308 := by
  dsimp only [node_307,node_308]
  apply tangent_double (ℓ:=(558909941320127799940847342985131496102213577127 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_308 : (21244293068519041880 : ℕ) • G=node_308 := by
  rw [show (21244293068519041880 : ℕ)=10622146534259520940+10622146534259520940 by norm_num,
    add_nsmul,nsmul_307]
  exact add_308

private def node_309 : formulaCurve.Point :=
  .some (170606875014709710601442634036905289116034928926 : F) (211340694078562667041717590218899708008676169916 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_309 : node_308+node_0=node_309 := by
  dsimp only [node_308,node_0,node_309]
  apply secant_add (ℓ:=(650693767065832671509826151400780963638706692711 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_309 : (21244293068519041881 : ℕ) • G=node_309 := by
  rw [show (21244293068519041881 : ℕ)=21244293068519041880+1 by norm_num,
    add_nsmul,nsmul_308,nsmul_0]
  exact add_309

private def node_310 : formulaCurve.Point :=
  .some (325690404592572067046595049106580138592630327753 : F) (28530548622927344550870732267439489415112290395 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_310 : node_309+node_309=node_310 := by
  dsimp only [node_309,node_310]
  apply tangent_double (ℓ:=(548258048108303786798947214707627871117775869924 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_310 : (42488586137038083762 : ℕ) • G=node_310 := by
  rw [show (42488586137038083762 : ℕ)=21244293068519041881+21244293068519041881 by norm_num,
    add_nsmul,nsmul_309]
  exact add_310

private def node_311 : formulaCurve.Point :=
  .some (202914954705673038020436281210790393181370344011 : F) (622073615837880722028561975117487495145705624118 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_311 : node_310+node_310=node_311 := by
  dsimp only [node_310,node_311]
  apply tangent_double (ℓ:=(398362905994775832844343462075027947060520323232 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_311 : (84977172274076167524 : ℕ) • G=node_311 := by
  rw [show (84977172274076167524 : ℕ)=42488586137038083762+42488586137038083762 by norm_num,
    add_nsmul,nsmul_310]
  exact add_311

private def node_312 : formulaCurve.Point :=
  .some (116125011318217117702064652299237841342629692509 : F) (221941030297373677164453178595649404684870121710 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_312 : node_311+node_311=node_312 := by
  dsimp only [node_311,node_312]
  apply tangent_double (ℓ:=(412399212397688703471525890831589782530202314355 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_312 : (169954344548152335048 : ℕ) • G=node_312 := by
  rw [show (169954344548152335048 : ℕ)=84977172274076167524+84977172274076167524 by norm_num,
    add_nsmul,nsmul_311]
  exact add_312

private def node_313 : formulaCurve.Point :=
  .some (725087571381148905182592336275275339279886616781 : F) (324536641295045809619805804068671418374207136484 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_313 : node_312+node_0=node_313 := by
  dsimp only [node_312,node_0,node_313]
  apply secant_add (ℓ:=(602068960962077421307938043493424600586305217194 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_313 : (169954344548152335049 : ℕ) • G=node_313 := by
  rw [show (169954344548152335049 : ℕ)=169954344548152335048+1 by norm_num,
    add_nsmul,nsmul_312,nsmul_0]
  exact add_313

private def node_314 : formulaCurve.Point :=
  .some (579242832192040616166732434751091789910727042532 : F) (67323882656422108378124007962150139956810816310 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_314 : node_313+node_313=node_314 := by
  dsimp only [node_313,node_314]
  apply tangent_double (ℓ:=(93943887707876805866265062070609968248497165002 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_314 : (339908689096304670098 : ℕ) • G=node_314 := by
  rw [show (339908689096304670098 : ℕ)=169954344548152335049+169954344548152335049 by norm_num,
    add_nsmul,nsmul_313]
  exact add_314

private def node_315 : formulaCurve.Point :=
  .some (411064600436363413695677434758467553269805591076 : F) (309003711214742442368811286680033160098036219620 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_315 : node_314+node_314=node_315 := by
  dsimp only [node_314,node_315]
  apply tangent_double (ℓ:=(657858585918008431274346418565249457692275785271 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_315 : (679817378192609340196 : ℕ) • G=node_315 := by
  rw [show (679817378192609340196 : ℕ)=339908689096304670098+339908689096304670098 by norm_num,
    add_nsmul,nsmul_314]
  exact add_315

private def node_316 : formulaCurve.Point :=
  .some (420536385614617844482909573833125868016272046278 : F) (181749705684522408769512662374260514499348680734 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_316 : node_315+node_315=node_316 := by
  dsimp only [node_315,node_316]
  apply tangent_double (ℓ:=(654493506206517347861883094969984020729171088775 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_316 : (1359634756385218680392 : ℕ) • G=node_316 := by
  rw [show (1359634756385218680392 : ℕ)=679817378192609340196+679817378192609340196 by norm_num,
    add_nsmul,nsmul_315]
  exact add_316

private def node_317 : formulaCurve.Point :=
  .some (721363387204593350684272897679963099383276606021 : F) (637312898175905043566576147558390770769173250495 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_317 : node_316+node_0=node_317 := by
  dsimp only [node_316,node_0,node_317]
  apply secant_add (ℓ:=(59749884845663158547523129927834098619556483279 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_317 : (1359634756385218680393 : ℕ) • G=node_317 := by
  rw [show (1359634756385218680393 : ℕ)=1359634756385218680392+1 by norm_num,
    add_nsmul,nsmul_316,nsmul_0]
  exact add_317

private def node_318 : formulaCurve.Point :=
  .some (52976401566275437387491665342854407279462348783 : F) (17684675856954076854029716830898450840294147287 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_318 : node_317+node_317=node_318 := by
  dsimp only [node_317,node_318]
  apply tangent_double (ℓ:=(111540880929874961988070656944681683767569402544 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_318 : (2719269512770437360786 : ℕ) • G=node_318 := by
  rw [show (2719269512770437360786 : ℕ)=1359634756385218680393+1359634756385218680393 by norm_num,
    add_nsmul,nsmul_317]
  exact add_318

private def node_319 : formulaCurve.Point :=
  .some (173827069315605499133635320651121368197129346213 : F) (22216650872820349715726313568139231200000069682 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_319 : node_318+node_318=node_319 := by
  dsimp only [node_318,node_319]
  apply tangent_double (ℓ:=(249681625080143991190539477595920332060043975715 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_319 : (5438539025540874721572 : ℕ) • G=node_319 := by
  rw [show (5438539025540874721572 : ℕ)=2719269512770437360786+2719269512770437360786 by norm_num,
    add_nsmul,nsmul_318]
  exact add_319

private def node_320 : formulaCurve.Point :=
  .some (359527458559098187858519147608391532525859732891 : F) (729140089479762704660780792628701823752648225853 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_320 : node_319+node_319=node_320 := by
  dsimp only [node_319,node_320]
  apply tangent_double (ℓ:=(17488986692274698885914716701302712633999352538 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_320 : (10877078051081749443144 : ℕ) • G=node_320 := by
  rw [show (10877078051081749443144 : ℕ)=5438539025540874721572+5438539025540874721572 by norm_num,
    add_nsmul,nsmul_319]
  exact add_320

private def node_321 : formulaCurve.Point :=
  .some (506572122576934788098699254772031317672692867721 : F) (369092292777142623240328564616376086924244118724 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_321 : node_320+node_0=node_321 := by
  dsimp only [node_320,node_0,node_321]
  apply secant_add (ℓ:=(495987001286593997120469481318817337368653044431 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_321 : (10877078051081749443145 : ℕ) • G=node_321 := by
  rw [show (10877078051081749443145 : ℕ)=10877078051081749443144+1 by norm_num,
    add_nsmul,nsmul_320,nsmul_0]
  exact add_321

private def node_322 : formulaCurve.Point :=
  .some (507873217654155564164623754210036179433428935596 : F) (465376222103837635666757586168099238756724752557 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_322 : node_321+node_321=node_322 := by
  dsimp only [node_321,node_322]
  apply tangent_double (ℓ:=(341867699780251737255388632112626681188240305000 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_322 : (21754156102163498886290 : ℕ) • G=node_322 := by
  rw [show (21754156102163498886290 : ℕ)=10877078051081749443145+10877078051081749443145 by norm_num,
    add_nsmul,nsmul_321]
  exact add_322

private def node_323 : formulaCurve.Point :=
  .some (493777503198055981364566150822768151733202229552 : F) (94419299715206113175030557735438907974168520320 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_323 : node_322+node_322=node_323 := by
  dsimp only [node_322,node_323]
  apply tangent_double (ℓ:=(699145109623987211309194502557764446730114029447 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_323 : (43508312204326997772580 : ℕ) • G=node_323 := by
  rw [show (43508312204326997772580 : ℕ)=21754156102163498886290+21754156102163498886290 by norm_num,
    add_nsmul,nsmul_322]
  exact add_323

private def node_324 : formulaCurve.Point :=
  .some (358342669601767251222711970078216784349076568502 : F) (457376008712710666797079086698599302293164035526 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_324 : node_323+node_0=node_324 := by
  dsimp only [node_323,node_0,node_324]
  apply secant_add (ℓ:=(635367123184801072456750687058635098297021532680 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_324 : (43508312204326997772581 : ℕ) • G=node_324 := by
  rw [show (43508312204326997772581 : ℕ)=43508312204326997772580+1 by norm_num,
    add_nsmul,nsmul_323,nsmul_0]
  exact add_324

private def node_325 : formulaCurve.Point :=
  .some (224924074254221656681506279697413761512039069676 : F) (536803551103524952071693601683680126813339647425 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_325 : node_324+node_324=node_325 := by
  dsimp only [node_324,node_325]
  apply tangent_double (ℓ:=(145527949819386917468593707674656109288166812769 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_325 : (87016624408653995545162 : ℕ) • G=node_325 := by
  rw [show (87016624408653995545162 : ℕ)=43508312204326997772581+43508312204326997772581 by norm_num,
    add_nsmul,nsmul_324]
  exact add_325

private def node_326 : formulaCurve.Point :=
  .some (136643288949841595393742833767017854457443987881 : F) (333351407756274727382913722220560366815338634803 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_326 : node_325+node_325=node_326 := by
  dsimp only [node_325,node_326]
  apply tangent_double (ℓ:=(312931555111711337327017337900007669461112077803 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_326 : (174033248817307991090324 : ℕ) • G=node_326 := by
  rw [show (174033248817307991090324 : ℕ)=87016624408653995545162+87016624408653995545162 by norm_num,
    add_nsmul,nsmul_325]
  exact add_326

private def node_327 : formulaCurve.Point :=
  .some (179645528477555587147563078217499923481826173040 : F) (410257468957107516334621991127304654837682197193 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_327 : node_326+node_0=node_327 := by
  dsimp only [node_326,node_0,node_327]
  apply secant_add (ℓ:=(670427870456301692756121990820697317784376964988 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_327 : (174033248817307991090325 : ℕ) • G=node_327 := by
  rw [show (174033248817307991090325 : ℕ)=174033248817307991090324+1 by norm_num,
    add_nsmul,nsmul_326,nsmul_0]
  exact add_327

private def node_328 : formulaCurve.Point :=
  .some (590447812955565076251628477844916426709663241706 : F) (515163544500555553629298306405215008119576327966 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_328 : node_327+node_327=node_328 := by
  dsimp only [node_327,node_328]
  apply tangent_double (ℓ:=(621806202219633184718817883419975775442320420705 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_328 : (348066497634615982180650 : ℕ) • G=node_328 := by
  rw [show (348066497634615982180650 : ℕ)=174033248817307991090325+174033248817307991090325 by norm_num,
    add_nsmul,nsmul_327]
  exact add_328

private def node_329 : formulaCurve.Point :=
  .some (433176530990107001345596081783552042765490813199 : F) (70857744898811060377323194726057359747401101961 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_329 : node_328+node_328=node_329 := by
  dsimp only [node_328,node_329]
  apply tangent_double (ℓ:=(498357070329258411366421163668715698948876544906 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_329 : (696132995269231964361300 : ℕ) • G=node_329 := by
  rw [show (696132995269231964361300 : ℕ)=348066497634615982180650+348066497634615982180650 by norm_num,
    add_nsmul,nsmul_328]
  exact add_329

private def node_330 : formulaCurve.Point :=
  .some (487505965374605380790622390971168661396375074597 : F) (20261591128577776991658103979019445457001650066 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_330 : node_329+node_329=node_330 := by
  dsimp only [node_329,node_330]
  apply tangent_double (ℓ:=(448420113652868956251399070625673513047169040307 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_330 : (1392265990538463928722600 : ℕ) • G=node_330 := by
  rw [show (1392265990538463928722600 : ℕ)=696132995269231964361300+696132995269231964361300 by norm_num,
    add_nsmul,nsmul_329]
  exact add_330

private def node_331 : formulaCurve.Point :=
  .some (224447058355874999128344654409871152622195550064 : F) (419539190760847557732216926777620240685993350642 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_331 : node_330+node_0=node_331 := by
  dsimp only [node_330,node_0,node_331]
  apply secant_add (ℓ:=(492066190713702863916076422237558510832505824266 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_331 : (1392265990538463928722601 : ℕ) • G=node_331 := by
  rw [show (1392265990538463928722601 : ℕ)=1392265990538463928722600+1 by norm_num,
    add_nsmul,nsmul_330,nsmul_0]
  exact add_331

private def node_332 : formulaCurve.Point :=
  .some (385023415340488023792710574288265447084290106356 : F) (503502468477808328869392829379888442470612116743 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_332 : node_331+node_331=node_332 := by
  dsimp only [node_331,node_332]
  apply tangent_double (ℓ:=(323291307165253516626474578827393006914153929982 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_332 : (2784531981076927857445202 : ℕ) • G=node_332 := by
  rw [show (2784531981076927857445202 : ℕ)=1392265990538463928722601+1392265990538463928722601 by norm_num,
    add_nsmul,nsmul_331]
  exact add_332

private def node_333 : formulaCurve.Point :=
  .some (394129262435073940754648789948491342035036600047 : F) (347915753903688638072371129599294242465298214946 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_333 : node_332+node_332=node_333 := by
  dsimp only [node_332,node_333]
  apply tangent_double (ℓ:=(123850417571883185913745795799540901309396980993 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_333 : (5569063962153855714890404 : ℕ) • G=node_333 := by
  rw [show (5569063962153855714890404 : ℕ)=2784531981076927857445202+2784531981076927857445202 by norm_num,
    add_nsmul,nsmul_332]
  exact add_333

private def node_334 : formulaCurve.Point :=
  .some (721986644907262516940827758836022386526512923268 : F) (249513032921096888120914978779965707437848889591 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_334 : node_333+node_0=node_334 := by
  dsimp only [node_333,node_0,node_334]
  apply secant_add (ℓ:=(144214649961153706108893069382490601271585800529 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_334 : (5569063962153855714890405 : ℕ) • G=node_334 := by
  rw [show (5569063962153855714890405 : ℕ)=5569063962153855714890404+1 by norm_num,
    add_nsmul,nsmul_333,nsmul_0]
  exact add_334

private def node_335 : formulaCurve.Point :=
  .some (447707110917411570323749815594492045016070234570 : F) (479807968633477724033710020649321585984295182378 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_335 : node_334+node_334=node_335 := by
  dsimp only [node_334,node_335]
  apply tangent_double (ℓ:=(191688781495344904153247832029527303277190088239 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_335 : (11138127924307711429780810 : ℕ) • G=node_335 := by
  rw [show (11138127924307711429780810 : ℕ)=5569063962153855714890405+5569063962153855714890405 by norm_num,
    add_nsmul,nsmul_334]
  exact add_335

private def node_336 : formulaCurve.Point :=
  .some (713305163329734663151631311461474966263185632079 : F) (537658900155848162628928753437853606396244166807 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_336 : node_335+node_335=node_336 := by
  dsimp only [node_335,node_336]
  apply tangent_double (ℓ:=(485982815779534134171397761616976496660203472450 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_336 : (22276255848615422859561620 : ℕ) • G=node_336 := by
  rw [show (22276255848615422859561620 : ℕ)=11138127924307711429780810+11138127924307711429780810 by norm_num,
    add_nsmul,nsmul_335]
  exact add_336

private def node_337 : formulaCurve.Point :=
  .some (435467839864798814523129716754268682819748507766 : F) (356055828984944178486235584476988358488527451758 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_337 : node_336+node_0=node_337 := by
  dsimp only [node_336,node_0,node_337]
  apply secant_add (ℓ:=(437255922682019327826600790079320271985310914046 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_337 : (22276255848615422859561621 : ℕ) • G=node_337 := by
  rw [show (22276255848615422859561621 : ℕ)=22276255848615422859561620+1 by norm_num,
    add_nsmul,nsmul_336,nsmul_0]
  exact add_337

private def node_338 : formulaCurve.Point :=
  .some (664243200356623923696863298358632613571126030304 : F) (256669853056434733714653322766417397310830799814 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_338 : node_337+node_337=node_338 := by
  dsimp only [node_337,node_338]
  apply tangent_double (ℓ:=(160438297282715186367215546538736722878882567753 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_338 : (44552511697230845719123242 : ℕ) • G=node_338 := by
  rw [show (44552511697230845719123242 : ℕ)=22276255848615422859561621+22276255848615422859561621 by norm_num,
    add_nsmul,nsmul_337]
  exact add_338

private def node_339 : formulaCurve.Point :=
  .some (691747202696251223607277808103368480482999709597 : F) (214994655134564162761012577293258189370636789250 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_339 : node_338+node_338=node_339 := by
  dsimp only [node_338,node_339]
  apply tangent_double (ℓ:=(62033178491696310111826174299872135988310601913 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_339 : (89105023394461691438246484 : ℕ) • G=node_339 := by
  rw [show (89105023394461691438246484 : ℕ)=44552511697230845719123242+44552511697230845719123242 by norm_num,
    add_nsmul,nsmul_338]
  exact add_339

private def node_340 : formulaCurve.Point :=
  .some (134525227540773987165471342985781347242277598787 : F) (536360019198549095365326219370096346348102895274 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_340 : node_339+node_339=node_340 := by
  dsimp only [node_339,node_340]
  apply tangent_double (ℓ:=(178075681803032445390357786472024406553730191 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_340 : (178210046788923382876492968 : ℕ) • G=node_340 := by
  rw [show (178210046788923382876492968 : ℕ)=89105023394461691438246484+89105023394461691438246484 by norm_num,
    add_nsmul,nsmul_339]
  exact add_340

private def node_341 : formulaCurve.Point :=
  .some (42307626117146843819519596801965200803528748737 : F) (359051474535550581327488734604224104677693630584 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_341 : node_340+node_340=node_341 := by
  dsimp only [node_340,node_341]
  apply tangent_double (ℓ:=(415947956470439024950135492737548144898699010425 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_341 : (356420093577846765752985936 : ℕ) • G=node_341 := by
  rw [show (356420093577846765752985936 : ℕ)=178210046788923382876492968+178210046788923382876492968 by norm_num,
    add_nsmul,nsmul_340]
  exact add_341

private def node_342 : formulaCurve.Point :=
  .some (16017813271059124260886090915182595771237770569 : F) (315683026087127425436428095610678401308520970814 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_342 : node_341+node_341=node_342 := by
  dsimp only [node_341,node_342]
  apply tangent_double (ℓ:=(78690422231852153396017344251036914315827578642 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_342 : (712840187155693531505971872 : ℕ) • G=node_342 := by
  rw [show (712840187155693531505971872 : ℕ)=356420093577846765752985936+356420093577846765752985936 by norm_num,
    add_nsmul,nsmul_341]
  exact add_342

private def node_343 : formulaCurve.Point :=
  .some (72618934968832343020587065191792000287754462628 : F) (663149050509026974423318074996176863711087564824 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_343 : node_342+node_342=node_343 := by
  dsimp only [node_342,node_343]
  apply tangent_double (ℓ:=(217248199470871372829244114252422101521441071964 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_343 : (1425680374311387063011943744 : ℕ) • G=node_343 := by
  rw [show (1425680374311387063011943744 : ℕ)=712840187155693531505971872+712840187155693531505971872 by norm_num,
    add_nsmul,nsmul_342]
  exact add_343

private def node_344 : formulaCurve.Point :=
  .some (422493751339181466472794978421347073027486644695 : F) (656587553018253006849308920842933875733717999976 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_344 : node_343+node_0=node_344 := by
  dsimp only [node_343,node_0,node_344]
  apply secant_add (ℓ:=(564458655420002265495486324996977592183854271567 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_344 : (1425680374311387063011943745 : ℕ) • G=node_344 := by
  rw [show (1425680374311387063011943745 : ℕ)=1425680374311387063011943744+1 by norm_num,
    add_nsmul,nsmul_343,nsmul_0]
  exact add_344

private def node_345 : formulaCurve.Point :=
  .some (699457378625610686336415662063888287000511943839 : F) (419974100774933557313418952552511064220348367483 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_345 : node_344+node_344=node_345 := by
  dsimp only [node_344,node_345]
  apply tangent_double (ℓ:=(255985992593583719579603610766758603470547596301 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_345 : (2851360748622774126023887490 : ℕ) • G=node_345 := by
  rw [show (2851360748622774126023887490 : ℕ)=1425680374311387063011943745+1425680374311387063011943745 by norm_num,
    add_nsmul,nsmul_344]
  exact add_345

private def node_346 : formulaCurve.Point :=
  .some (617228779759301469105868850803118187115271096408 : F) (694459025628604795140749292455098175441594965263 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_346 : node_345+node_0=node_346 := by
  dsimp only [node_345,node_0,node_346]
  apply secant_add (ℓ:=(616386689630670106248807435355115674023060633107 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_346 : (2851360748622774126023887491 : ℕ) • G=node_346 := by
  rw [show (2851360748622774126023887491 : ℕ)=2851360748622774126023887490+1 by norm_num,
    add_nsmul,nsmul_345,nsmul_0]
  exact add_346

private def node_347 : formulaCurve.Point :=
  .some (146204410174776581885658797331215486110885700676 : F) (283222467243727531779924776838561303672848793302 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_347 : node_346+node_346=node_347 := by
  dsimp only [node_346,node_347]
  apply tangent_double (ℓ:=(26860958560714463151834695953173498056413036352 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_347 : (5702721497245548252047774982 : ℕ) • G=node_347 := by
  rw [show (5702721497245548252047774982 : ℕ)=2851360748622774126023887491+2851360748622774126023887491 by norm_num,
    add_nsmul,nsmul_346]
  exact add_347

private def node_348 : formulaCurve.Point :=
  .some (545750412193295728786229493160413846880330725260 : F) (240710620172242230695854130887525645670516920802 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_348 : node_347+node_347=node_348 := by
  dsimp only [node_347,node_348]
  apply tangent_double (ℓ:=(446050738005049834672842826551645398766929594340 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_348 : (11405442994491096504095549964 : ℕ) • G=node_348 := by
  rw [show (11405442994491096504095549964 : ℕ)=5702721497245548252047774982+5702721497245548252047774982 by norm_num,
    add_nsmul,nsmul_347]
  exact add_348

private def node_349 : formulaCurve.Point :=
  .some (13081643054779921368386570275686665436048194634 : F) (220965161904644407689840529540447967690463308090 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_349 : node_348+node_0=node_349 := by
  dsimp only [node_348,node_0,node_349]
  apply secant_add (ℓ:=(473852371513208983870808908318598364395642913806 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_349 : (11405442994491096504095549965 : ℕ) • G=node_349 := by
  rw [show (11405442994491096504095549965 : ℕ)=11405442994491096504095549964+1 by norm_num,
    add_nsmul,nsmul_348,nsmul_0]
  exact add_349

private def node_350 : formulaCurve.Point :=
  .some (468191703138681727569445378297262538720575019757 : F) (503093189512952764588933130668763782355490607866 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_350 : node_349+node_349=node_350 := by
  dsimp only [node_349,node_350]
  apply tangent_double (ℓ:=(352982125889279225114827030702465829008462277494 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_350 : (22810885988982193008191099930 : ℕ) • G=node_350 := by
  rw [show (22810885988982193008191099930 : ℕ)=11405442994491096504095549965+11405442994491096504095549965 by norm_num,
    add_nsmul,nsmul_349]
  exact add_350

private def node_351 : formulaCurve.Point :=
  .some (298064750483768127351955210059692649700806836944 : F) (695796094513376749641836908779872116701031560429 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_351 : node_350+node_350=node_351 := by
  dsimp only [node_350,node_351]
  apply tangent_double (ℓ:=(519863976632296272326043184068269132989306781520 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_351 : (45621771977964386016382199860 : ℕ) • G=node_351 := by
  rw [show (45621771977964386016382199860 : ℕ)=22810885988982193008191099930+22810885988982193008191099930 by norm_num,
    add_nsmul,nsmul_350]
  exact add_351

private def node_352 : formulaCurve.Point :=
  .some (509351916671745013097131107713957424627439057078 : F) (423860019573882803658855276418755736711162222954 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_352 : node_351+node_0=node_352 := by
  dsimp only [node_351,node_0,node_352]
  apply secant_add (ℓ:=(332319697837105339809951231674296610683095829549 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_352 : (45621771977964386016382199861 : ℕ) • G=node_352 := by
  rw [show (45621771977964386016382199861 : ℕ)=45621771977964386016382199860+1 by norm_num,
    add_nsmul,nsmul_351,nsmul_0]
  exact add_352

private def node_353 : formulaCurve.Point :=
  .some (75234967356468043505571293451238734242301594530 : F) (462503976003541608469560048077582521086795523381 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_353 : node_352+node_352=node_353 := by
  dsimp only [node_352,node_353]
  apply tangent_double (ℓ:=(284751233944489159816975380697220709016867088730 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_353 : (91243543955928772032764399722 : ℕ) • G=node_353 := by
  rw [show (91243543955928772032764399722 : ℕ)=45621771977964386016382199861+45621771977964386016382199861 by norm_num,
    add_nsmul,nsmul_352]
  exact add_353

private def node_354 : formulaCurve.Point :=
  .some (680466737602806057166865361667527809960983607679 : F) (282881738592131329374027118553267354096712459506 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_354 : node_353+node_0=node_354 := by
  dsimp only [node_353,node_0,node_354]
  apply secant_add (ℓ:=(568314904867957103592487823084453095794057897826 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_354 : (91243543955928772032764399723 : ℕ) • G=node_354 := by
  rw [show (91243543955928772032764399723 : ℕ)=91243543955928772032764399722+1 by norm_num,
    add_nsmul,nsmul_353,nsmul_0]
  exact add_354

private def node_355 : formulaCurve.Point :=
  .some (179748234408818123389176893873394694803258139898 : F) (573161809396712488644105015958817019058817574384 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_355 : node_354+node_354=node_355 := by
  dsimp only [node_354,node_355]
  apply tangent_double (ℓ:=(94653325999720507637424716599486757651898644653 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_355 : (182487087911857544065528799446 : ℕ) • G=node_355 := by
  rw [show (182487087911857544065528799446 : ℕ)=91243543955928772032764399723+91243543955928772032764399723 by norm_num,
    add_nsmul,nsmul_354]
  exact add_355

private def node_356 : formulaCurve.Point :=
  .some (327240973155397788472033305593686896700182121543 : F) (55405323845582629461990420710826676192735624105 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_356 : node_355+node_355=node_356 := by
  dsimp only [node_355,node_356]
  apply tangent_double (ℓ:=(280636583997266574028930339147604065029305505898 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_356 : (364974175823715088131057598892 : ℕ) • G=node_356 := by
  rw [show (364974175823715088131057598892 : ℕ)=182487087911857544065528799446+182487087911857544065528799446 by norm_num,
    add_nsmul,nsmul_355]
  exact add_356

private def node_357 : formulaCurve.Point :=
  .some (481962534553537744658367596911274458388951009718 : F) (614996776104754776718620697231129437020621293588 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_357 : node_356+node_0=node_357 := by
  dsimp only [node_356,node_0,node_357]
  apply secant_add (ℓ:=(577409505216441293739077391128637722749874517586 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_357 : (364974175823715088131057598893 : ℕ) • G=node_357 := by
  rw [show (364974175823715088131057598893 : ℕ)=364974175823715088131057598892+1 by norm_num,
    add_nsmul,nsmul_356,nsmul_0]
  exact add_357

private def node_358 : formulaCurve.Point :=
  .some (438762021595580119149636427251908405381854261012 : F) (587185167303321966798952657215542531843300431872 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_358 : node_357+node_357=node_358 := by
  dsimp only [node_357,node_358]
  apply tangent_double (ℓ:=(19676241921987455110033717242253370475385165093 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_358 : (729948351647430176262115197786 : ℕ) • G=node_358 := by
  rw [show (729948351647430176262115197786 : ℕ)=364974175823715088131057598893+364974175823715088131057598893 by norm_num,
    add_nsmul,nsmul_357]
  exact add_358

private def node_359 : formulaCurve.Point :=
  .some (307698114530407721841733762232986916648104968216 : F) (181355768397088397320257064279192184135198507747 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_359 : node_358+node_358=node_359 := by
  dsimp only [node_358,node_359]
  apply tangent_double (ℓ:=(623346573686648378206916366976656730036998574228 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_359 : (1459896703294860352524230395572 : ℕ) • G=node_359 := by
  rw [show (1459896703294860352524230395572 : ℕ)=729948351647430176262115197786+729948351647430176262115197786 by norm_num,
    add_nsmul,nsmul_358]
  exact add_359

private def node_360 : formulaCurve.Point :=
  .some (639965444037713876336048641049887097806792178932 : F) (600808563133581364269955991326708797650066247415 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_360 : node_359+node_359=node_360 := by
  dsimp only [node_359,node_360]
  apply tangent_double (ℓ:=(701773690929794091698021552185489910849227021456 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_360 : (2919793406589720705048460791144 : ℕ) • G=node_360 := by
  rw [show (2919793406589720705048460791144 : ℕ)=1459896703294860352524230395572+1459896703294860352524230395572 by norm_num,
    add_nsmul,nsmul_359]
  exact add_360

private def node_361 : formulaCurve.Point :=
  .some (336683173855077088392931908336854419381841625519 : F) (612802472741274535875980894684376772624826851021 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_361 : node_360+node_360=node_361 := by
  dsimp only [node_360,node_361]
  apply tangent_double (ℓ:=(670016707382597245546517955275543768083286939622 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_361 : (5839586813179441410096921582288 : ℕ) • G=node_361 := by
  rw [show (5839586813179441410096921582288 : ℕ)=2919793406589720705048460791144+2919793406589720705048460791144 by norm_num,
    add_nsmul,nsmul_360]
  exact add_361

private def node_362 : formulaCurve.Point :=
  .some (501703339634787094027067469810474634417556055152 : F) (237674026824563557676611825884689580046416045766 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_362 : node_361+node_0=node_362 := by
  dsimp only [node_361,node_0,node_362]
  apply secant_add (ℓ:=(19420610201119524942212065046433591745875102436 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_362 : (5839586813179441410096921582289 : ℕ) • G=node_362 := by
  rw [show (5839586813179441410096921582289 : ℕ)=5839586813179441410096921582288+1 by norm_num,
    add_nsmul,nsmul_361,nsmul_0]
  exact add_362

private def node_363 : formulaCurve.Point :=
  .some (557442692059256289655794007776908303928028024690 : F) (87639148657519204805865502277339837470510578516 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_363 : node_362+node_362=node_363 := by
  dsimp only [node_362,node_363]
  apply tangent_double (ℓ:=(191880896381670700330760042330227834815588643051 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_363 : (11679173626358882820193843164578 : ℕ) • G=node_363 := by
  rw [show (11679173626358882820193843164578 : ℕ)=5839586813179441410096921582289+5839586813179441410096921582289 by norm_num,
    add_nsmul,nsmul_362]
  exact add_363

private def node_364 : formulaCurve.Point :=
  .some (604355101041445734109082848327443939423167882534 : F) (714244619847932204743937235884777579629582229235 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_364 : node_363+node_363=node_364 := by
  dsimp only [node_363,node_364]
  apply tangent_double (ℓ:=(729952264816957651145638749848425729144768528732 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_364 : (23358347252717765640387686329156 : ℕ) • G=node_364 := by
  rw [show (23358347252717765640387686329156 : ℕ)=11679173626358882820193843164578+11679173626358882820193843164578 by norm_num,
    add_nsmul,nsmul_363]
  exact add_364

private def node_365 : formulaCurve.Point :=
  .some (711268689634732996338348659786063062144347422556 : F) (97344118483569308520826593514549484899395097162 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_365 : node_364+node_0=node_365 := by
  dsimp only [node_364,node_0,node_365]
  apply secant_add (ℓ:=(214497691415552908965443609457898115857343304605 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_365 : (23358347252717765640387686329157 : ℕ) • G=node_365 := by
  rw [show (23358347252717765640387686329157 : ℕ)=23358347252717765640387686329156+1 by norm_num,
    add_nsmul,nsmul_364,nsmul_0]
  exact add_365

private def node_366 : formulaCurve.Point :=
  .some (126663451387357054926177286597104293313456283062 : F) (200876275181582899954115917010772132199217370835 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_366 : node_365+node_365=node_366 := by
  dsimp only [node_365,node_366]
  apply tangent_double (ℓ:=(236576259924006875426553114118604294430829998650 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_366 : (46716694505435531280775372658314 : ℕ) • G=node_366 := by
  rw [show (46716694505435531280775372658314 : ℕ)=23358347252717765640387686329157+23358347252717765640387686329157 by norm_num,
    add_nsmul,nsmul_365]
  exact add_366

private def node_367 : formulaCurve.Point :=
  .some (189359272869044714035309198567621253319318240859 : F) (73163142565168479156758101675339171849619522695 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_367 : node_366+node_366=node_367 := by
  dsimp only [node_366,node_367]
  apply tangent_double (ℓ:=(538435795974076511746075424881449873299393271682 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_367 : (93433389010871062561550745316628 : ℕ) • G=node_367 := by
  rw [show (93433389010871062561550745316628 : ℕ)=46716694505435531280775372658314+46716694505435531280775372658314 by norm_num,
    add_nsmul,nsmul_366]
  exact add_367

private def node_368 : formulaCurve.Point :=
  .some (710544526145648380635070529858307861093566397838 : F) (204690772371078337078094176310250481167135907245 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_368 : node_367+node_367=node_368 := by
  dsimp only [node_367,node_368]
  apply tangent_double (ℓ:=(202601533174100943408226429369473983477426974722 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_368 : (186866778021742125123101490633256 : ℕ) • G=node_368 := by
  rw [show (186866778021742125123101490633256 : ℕ)=93433389010871062561550745316628+93433389010871062561550745316628 by norm_num,
    add_nsmul,nsmul_367]
  exact add_368

private def node_369 : formulaCurve.Point :=
  .some (185296063872383815586383924543559306262205973141 : F) (144731525954698666718282173373652078240864737556 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_369 : node_368+node_0=node_369 := by
  dsimp only [node_368,node_0,node_369]
  apply secant_add (ℓ:=(575935724302334192802600749990630361029561967357 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_369 : (186866778021742125123101490633257 : ℕ) • G=node_369 := by
  rw [show (186866778021742125123101490633257 : ℕ)=186866778021742125123101490633256+1 by norm_num,
    add_nsmul,nsmul_368,nsmul_0]
  exact add_369

private def node_370 : formulaCurve.Point :=
  .some (464871999406020177054343877579193121235485643637 : F) (200618836867221388720266372157492174055978739863 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_370 : node_369+node_369=node_370 := by
  dsimp only [node_369,node_370]
  apply tangent_double (ℓ:=(237380649919413933431365878627022971348654477624 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_370 : (373733556043484250246202981266514 : ℕ) • G=node_370 := by
  rw [show (373733556043484250246202981266514 : ℕ)=186866778021742125123101490633257+186866778021742125123101490633257 by norm_num,
    add_nsmul,nsmul_369]
  exact add_370

private def node_371 : formulaCurve.Point :=
  .some (650994956688702044669003644512552381516373100485 : F) (471124250712048950984953507113276061696086855336 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_371 : node_370+node_370=node_371 := by
  dsimp only [node_370,node_371]
  apply tangent_double (ℓ:=(550740286956006720237815538003987149136158796730 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_371 : (747467112086968500492405962533028 : ℕ) • G=node_371 := by
  rw [show (747467112086968500492405962533028 : ℕ)=373733556043484250246202981266514+373733556043484250246202981266514 by norm_num,
    add_nsmul,nsmul_370]
  exact add_371

private def node_372 : formulaCurve.Point :=
  .some (333972527336410261639656372231841704777745469445 : F) (135003200249552108086636166015148787572650943913 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_372 : node_371+node_0=node_372 := by
  dsimp only [node_371,node_0,node_372]
  apply secant_add (ℓ:=(408762036473995587619683670612236039314942241229 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_372 : (747467112086968500492405962533029 : ℕ) • G=node_372 := by
  rw [show (747467112086968500492405962533029 : ℕ)=747467112086968500492405962533028+1 by norm_num,
    add_nsmul,nsmul_371,nsmul_0]
  exact add_372

private def node_373 : formulaCurve.Point :=
  .some (671159458040819424793073084778618963892975287546 : F) (574060465433638169995635071749065696071657943167 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_373 : node_372+node_372=node_373 := by
  dsimp only [node_372,node_373]
  apply tangent_double (ℓ:=(362252929336659870195747163371539567544240317478 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_373 : (1494934224173937000984811925066058 : ℕ) • G=node_373 := by
  rw [show (1494934224173937000984811925066058 : ℕ)=747467112086968500492405962533029+747467112086968500492405962533029 by norm_num,
    add_nsmul,nsmul_372]
  exact add_373

private def node_374 : formulaCurve.Point :=
  .some (375091366228138542585871370578539133468265011139 : F) (120138796543698181953563121499602796628594332440 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_374 : node_373+node_0=node_374 := by
  dsimp only [node_373,node_0,node_374]
  apply secant_add (ℓ:=(564885549872097482609396186384230581822292741780 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_374 : (1494934224173937000984811925066059 : ℕ) • G=node_374 := by
  rw [show (1494934224173937000984811925066059 : ℕ)=1494934224173937000984811925066058+1 by norm_num,
    add_nsmul,nsmul_373,nsmul_0]
  exact add_374

private def node_375 : formulaCurve.Point :=
  .some (78425179066663486705967321263885635540189702568 : F) (668912665931499465512316349454635174646656893498 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_375 : node_374+node_374=node_375 := by
  dsimp only [node_374,node_375]
  apply tangent_double (ℓ:=(366281452755972992413662313471794876688725693770 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_375 : (2989868448347874001969623850132118 : ℕ) • G=node_375 := by
  rw [show (2989868448347874001969623850132118 : ℕ)=1494934224173937000984811925066059+1494934224173937000984811925066059 by norm_num,
    add_nsmul,nsmul_374]
  exact add_375

private def node_376 : formulaCurve.Point :=
  .some (591686671795696445264425508730870618240561000625 : F) (64182643718752517503047916855710383916008140175 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_376 : node_375+node_375=node_376 := by
  dsimp only [node_375,node_376]
  apply tangent_double (ℓ:=(671546944778549309878401299542069753463746452705 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_376 : (5979736896695748003939247700264236 : ℕ) • G=node_376 := by
  rw [show (5979736896695748003939247700264236 : ℕ)=2989868448347874001969623850132118+2989868448347874001969623850132118 by norm_num,
    add_nsmul,nsmul_375]
  exact add_376

private def node_377 : formulaCurve.Point :=
  .some (549190918425313686137431324870422528362456649895 : F) (409523492060827208681840306725442813055624746600 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_377 : node_376+node_376=node_377 := by
  dsimp only [node_376,node_377]
  apply tangent_double (ℓ:=(448792684251210992541212022935728183171578214954 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_377 : (11959473793391496007878495400528472 : ℕ) • G=node_377 := by
  rw [show (11959473793391496007878495400528472 : ℕ)=5979736896695748003939247700264236+5979736896695748003939247700264236 by norm_num,
    add_nsmul,nsmul_376]
  exact add_377

private def node_378 : formulaCurve.Point :=
  .some (392718704636155120068561157862717924992714997379 : F) (29535899618999176564959434728068090781082824415 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_378 : node_377+node_377=node_378 := by
  dsimp only [node_377,node_378]
  apply tangent_double (ℓ:=(144719798176705584737434339125616510093706191551 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_378 : (23918947586782992015756990801056944 : ℕ) • G=node_378 := by
  rw [show (23918947586782992015756990801056944 : ℕ)=11959473793391496007878495400528472+11959473793391496007878495400528472 by norm_num,
    add_nsmul,nsmul_377]
  exact add_378

private def node_379 : formulaCurve.Point :=
  .some (376756024887961059585742574916842994829552381967 : F) (385114367179460426961572948561130726123202708433 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_379 : node_378+node_0=node_379 := by
  dsimp only [node_378,node_0,node_379]
  apply secant_add (ℓ:=(590773436057033507226875683067127164720034494402 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_379 : (23918947586782992015756990801056945 : ℕ) • G=node_379 := by
  rw [show (23918947586782992015756990801056945 : ℕ)=23918947586782992015756990801056944+1 by norm_num,
    add_nsmul,nsmul_378,nsmul_0]
  exact add_379

private def node_380 : formulaCurve.Point :=
  .some (194252981688404094294545254724385324317965577869 : F) (260471831581760007359526601268731029809147363590 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_380 : node_379+node_379=node_380 := by
  dsimp only [node_379,node_380]
  apply tangent_double (ℓ:=(195752206436669843839304907514408773171983104483 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_380 : (47837895173565984031513981602113890 : ℕ) • G=node_380 := by
  rw [show (47837895173565984031513981602113890 : ℕ)=23918947586782992015756990801056945+23918947586782992015756990801056945 by norm_num,
    add_nsmul,nsmul_379]
  exact add_380

private def node_381 : formulaCurve.Point :=
  .some (93932549058223674982088598412253129027218033231 : F) (454668457844714929891510847211145954026594191003 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_381 : node_380+node_380=node_381 := by
  dsimp only [node_380,node_381]
  apply tangent_double (ℓ:=(243754022356935102655228291175651757886389639678 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_381 : (95675790347131968063027963204227780 : ℕ) • G=node_381 := by
  rw [show (95675790347131968063027963204227780 : ℕ)=47837895173565984031513981602113890+47837895173565984031513981602113890 by norm_num,
    add_nsmul,nsmul_380]
  exact add_381

private def node_382 : formulaCurve.Point :=
  .some (197657978988037182575592261915131657789613865344 : F) (300615324008660352927017225527815038313217649876 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_382 : node_381+node_381=node_382 := by
  dsimp only [node_381,node_382]
  apply tangent_double (ℓ:=(513675703564593241653230260674220374398421724478 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_382 : (191351580694263936126055926408455560 : ℕ) • G=node_382 := by
  rw [show (191351580694263936126055926408455560 : ℕ)=95675790347131968063027963204227780+95675790347131968063027963204227780 by norm_num,
    add_nsmul,nsmul_381]
  exact add_382

private def node_383 : formulaCurve.Point :=
  .some (119991854729987542568105579067710458372384623258 : F) (22211580608451116577561964768369266115881509331 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_383 : node_382+node_0=node_383 := by
  dsimp only [node_382,node_0,node_383]
  apply secant_add (ℓ:=(646547705416403007376022680108120035154333609767 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_383 : (191351580694263936126055926408455561 : ℕ) • G=node_383 := by
  rw [show (191351580694263936126055926408455561 : ℕ)=191351580694263936126055926408455560+1 by norm_num,
    add_nsmul,nsmul_382,nsmul_0]
  exact add_383

private def node_384 : formulaCurve.Point :=
  .some (258871169806737249368006746771777455157443140156 : F) (63472543594109118759349505389253140962945706897 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_384 : node_383+node_383=node_384 := by
  dsimp only [node_383,node_384]
  apply tangent_double (ℓ:=(336160376988026141859407306049334942321187837401 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_384 : (382703161388527872252111852816911122 : ℕ) • G=node_384 := by
  rw [show (382703161388527872252111852816911122 : ℕ)=191351580694263936126055926408455561+191351580694263936126055926408455561 by norm_num,
    add_nsmul,nsmul_383]
  exact add_384

private def node_385 : formulaCurve.Point :=
  .some (312756921518621529091781422765613417917825949891 : F) (513913197627931201220013010811692732926878357227 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_385 : node_384+node_0=node_385 := by
  dsimp only [node_384,node_0,node_385]
  apply secant_add (ℓ:=(376913324404063358155523645123890403799973624497 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_385 : (382703161388527872252111852816911123 : ℕ) • G=node_385 := by
  rw [show (382703161388527872252111852816911123 : ℕ)=382703161388527872252111852816911122+1 by norm_num,
    add_nsmul,nsmul_384,nsmul_0]
  exact add_385

private def node_386 : formulaCurve.Point :=
  .some (575226347413776145005131089959875290697487445598 : F) (19732239267922425996551340994699907410053907748 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_386 : node_385+node_385=node_386 := by
  dsimp only [node_385,node_386]
  apply tangent_double (ℓ:=(269134927589128686434323006823018014259240819020 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_386 : (765406322777055744504223705633822246 : ℕ) • G=node_386 := by
  rw [show (765406322777055744504223705633822246 : ℕ)=382703161388527872252111852816911123+382703161388527872252111852816911123 by norm_num,
    add_nsmul,nsmul_385]
  exact add_386

private def node_387 : formulaCurve.Point :=
  .some (481925117489703156343563854619356594531896899151 : F) (455869368307826200506338018717500174207050799672 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_387 : node_386+node_0=node_387 := by
  dsimp only [node_386,node_0,node_387]
  apply secant_add (ℓ:=(418115309651752501986513352593512508760339284203 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_387 : (765406322777055744504223705633822247 : ℕ) • G=node_387 := by
  rw [show (765406322777055744504223705633822247 : ℕ)=765406322777055744504223705633822246+1 by norm_num,
    add_nsmul,nsmul_386,nsmul_0]
  exact add_387

private def node_388 : formulaCurve.Point :=
  .some (335361746742811976675227109611496612213541293453 : F) (9544364843137531908005336311830018011472334203 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_388 : node_387+node_387=node_388 := by
  dsimp only [node_387,node_388]
  apply tangent_double (ℓ:=(176692141964114833293303730767840795685864570229 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_388 : (1530812645554111489008447411267644494 : ℕ) • G=node_388 := by
  rw [show (1530812645554111489008447411267644494 : ℕ)=765406322777055744504223705633822247+765406322777055744504223705633822247 by norm_num,
    add_nsmul,nsmul_387]
  exact add_388

private def node_389 : formulaCurve.Point :=
  .some (663576825736245707584625425561164558674355807119 : F) (196092393342732832222822922419939376245349492082 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_389 : node_388+node_0=node_389 := by
  dsimp only [node_388,node_0,node_389]
  apply secant_add (ℓ:=(545067543131330423235084082486134412987615245158 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_389 : (1530812645554111489008447411267644495 : ℕ) • G=node_389 := by
  rw [show (1530812645554111489008447411267644495 : ℕ)=1530812645554111489008447411267644494+1 by norm_num,
    add_nsmul,nsmul_388,nsmul_0]
  exact add_389

private def node_390 : formulaCurve.Point :=
  .some (588675460883043532665257422499814010504969602223 : F) (287965903162493480099155957578156248366150215967 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_390 : node_389+node_389=node_390 := by
  dsimp only [node_389,node_390]
  apply tangent_double (ℓ:=(606117886533726687746829685976920432135278834728 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_390 : (3061625291108222978016894822535288990 : ℕ) • G=node_390 := by
  rw [show (3061625291108222978016894822535288990 : ℕ)=1530812645554111489008447411267644495+1530812645554111489008447411267644495 by norm_num,
    add_nsmul,nsmul_389]
  exact add_390

private def node_391 : formulaCurve.Point :=
  .some (263366078086298829059438349602316592150949659373 : F) (527419810568639314139681148075186009434423561394 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_391 : node_390+node_0=node_391 := by
  dsimp only [node_390,node_0,node_391]
  apply secant_add (ℓ:=(235164727231793276198712651092738032896401623751 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_391 : (3061625291108222978016894822535288991 : ℕ) • G=node_391 := by
  rw [show (3061625291108222978016894822535288991 : ℕ)=3061625291108222978016894822535288990+1 by norm_num,
    add_nsmul,nsmul_390,nsmul_0]
  exact add_391

private def node_392 : formulaCurve.Point :=
  .some (658616220024496403525361800833486991130653349824 : F) (249963911387784289318354857388751411925177678554 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_392 : node_391+node_391=node_392 := by
  dsimp only [node_391,node_392]
  apply tangent_double (ℓ:=(689881791277176459045688365236082902164814029884 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_392 : (6123250582216445956033789645070577982 : ℕ) • G=node_392 := by
  rw [show (6123250582216445956033789645070577982 : ℕ)=3061625291108222978016894822535288991+3061625291108222978016894822535288991 by norm_num,
    add_nsmul,nsmul_391]
  exact add_392

private def node_393 : formulaCurve.Point :=
  .some (126974666161815164118442131118069357554513557944 : F) (694071207201147800912004787377905789247236145998 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_393 : node_392+node_0=node_393 := by
  dsimp only [node_392,node_0,node_393]
  apply secant_add (ℓ:=(245846810332296353752937219208709619256178155867 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_393 : (6123250582216445956033789645070577983 : ℕ) • G=node_393 := by
  rw [show (6123250582216445956033789645070577983 : ℕ)=6123250582216445956033789645070577982+1 by norm_num,
    add_nsmul,nsmul_392,nsmul_0]
  exact add_393

private def node_394 : formulaCurve.Point :=
  .some (462330508879645310552148134068822081030810215668 : F) (425297347364211223973755031251388807074533241026 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_394 : node_393+node_393=node_394 := by
  dsimp only [node_393,node_394]
  apply tangent_double (ℓ:=(512303003783497699081589201216119799119269713227 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_394 : (12246501164432891912067579290141155966 : ℕ) • G=node_394 := by
  rw [show (12246501164432891912067579290141155966 : ℕ)=6123250582216445956033789645070577983+6123250582216445956033789645070577983 by norm_num,
    add_nsmul,nsmul_393]
  exact add_394

private def node_395 : formulaCurve.Point :=
  .some (360738389152508504598146708345135834583779976001 : F) (704981031028950283165386186718831942727333082360 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_395 : node_394+node_394=node_395 := by
  dsimp only [node_394,node_395]
  apply tangent_double (ℓ:=(28159273029288459051549537349187992033743560895 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_395 : (24493002328865783824135158580282311932 : ℕ) • G=node_395 := by
  rw [show (24493002328865783824135158580282311932 : ℕ)=12246501164432891912067579290141155966+12246501164432891912067579290141155966 by norm_num,
    add_nsmul,nsmul_394]
  exact add_395

private def node_396 : formulaCurve.Point :=
  .some (293870057066843133634898389718535930464248056570 : F) (542327039306734347694460555473983982483999875481 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_396 : node_395+node_0=node_396 := by
  dsimp only [node_395,node_0,node_396]
  apply secant_add (ℓ:=(685598280294283794134365143957594529286036836653 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_396 : (24493002328865783824135158580282311933 : ℕ) • G=node_396 := by
  rw [show (24493002328865783824135158580282311933 : ℕ)=24493002328865783824135158580282311932+1 by norm_num,
    add_nsmul,nsmul_395,nsmul_0]
  exact add_396

private def node_397 : formulaCurve.Point :=
  .some (449787229135295411108103722419414089333247866624 : F) (186160638567116450550296994342807706948838306919 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_397 : node_396+node_396=node_397 := by
  dsimp only [node_396,node_397]
  apply tangent_double (ℓ:=(53309688831208882777752673744610417588879377195 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_397 : (48986004657731567648270317160564623866 : ℕ) • G=node_397 := by
  rw [show (48986004657731567648270317160564623866 : ℕ)=24493002328865783824135158580282311933+24493002328865783824135158580282311933 by norm_num,
    add_nsmul,nsmul_396]
  exact add_397

private def node_398 : formulaCurve.Point :=
  .some (340430010189360743735939162741432626971578638438 : F) (192689539943771936641946273779059098777095883412 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_398 : node_397+node_0=node_398 := by
  dsimp only [node_397,node_0,node_398]
  apply secant_add (ℓ:=(606858181579904378646465417334292733707921405368 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_398 : (48986004657731567648270317160564623867 : ℕ) • G=node_398 := by
  rw [show (48986004657731567648270317160564623867 : ℕ)=48986004657731567648270317160564623866+1 by norm_num,
    add_nsmul,nsmul_397,nsmul_0]
  exact add_398

private def node_399 : formulaCurve.Point :=
  .some (675636027895116481559032039884601601568091349632 : F) (23757924155181274817706738870262483683383454347 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_399 : node_398+node_398=node_399 := by
  dsimp only [node_398,node_399]
  apply tangent_double (ℓ:=(548653514850335687464925613135503502807042281137 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_399 : (97972009315463135296540634321129247734 : ℕ) • G=node_399 := by
  rw [show (97972009315463135296540634321129247734 : ℕ)=48986004657731567648270317160564623867+48986004657731567648270317160564623867 by norm_num,
    add_nsmul,nsmul_398]
  exact add_399

private def node_400 : formulaCurve.Point :=
  .some (715012637222352885721908629696743258600619635310 : F) (404101088001350327681221635098868703744838929952 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_400 : node_399+node_399=node_400 := by
  dsimp only [node_399,node_400]
  apply tangent_double (ℓ:=(38986543192061609290666410234658784566026641586 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_400 : (195944018630926270593081268642258495468 : ℕ) • G=node_400 := by
  rw [show (195944018630926270593081268642258495468 : ℕ)=97972009315463135296540634321129247734+97972009315463135296540634321129247734 by norm_num,
    add_nsmul,nsmul_399]
  exact add_400

private def node_401 : formulaCurve.Point :=
  .some (450119493495294572788380344916442631496921823294 : F) (599677506130911510655058996661150205167725846665 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_401 : node_400+node_0=node_401 := by
  dsimp only [node_400,node_0,node_401]
  apply secant_add (ℓ:=(619216495707236581484799164238037523092788175850 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_401 : (195944018630926270593081268642258495469 : ℕ) • G=node_401 := by
  rw [show (195944018630926270593081268642258495469 : ℕ)=195944018630926270593081268642258495468+1 by norm_num,
    add_nsmul,nsmul_400,nsmul_0]
  exact add_401

private def node_402 : formulaCurve.Point :=
  .some (101861467794188914834348379180078059464235560132 : F) (262004500819623349107422192869870446016124105212 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_402 : node_401+node_401=node_402 := by
  dsimp only [node_401,node_402]
  apply tangent_double (ℓ:=(311943967401169541766080544061016954819238059457 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_402 : (391888037261852541186162537284516990938 : ℕ) • G=node_402 := by
  rw [show (391888037261852541186162537284516990938 : ℕ)=195944018630926270593081268642258495469+195944018630926270593081268642258495469 by norm_num,
    add_nsmul,nsmul_401]
  exact add_402

private def node_403 : formulaCurve.Point :=
  .some (723293795990166095790856695317387091471244705811 : F) (269567543551076195614375756504714821603409452122 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_403 : node_402+node_0=node_403 := by
  dsimp only [node_402,node_0,node_403]
  apply secant_add (ℓ:=(221739476932999790654140547663819527090738139739 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_403 : (391888037261852541186162537284516990939 : ℕ) • G=node_403 := by
  rw [show (391888037261852541186162537284516990939 : ℕ)=391888037261852541186162537284516990938+1 by norm_num,
    add_nsmul,nsmul_402,nsmul_0]
  exact add_403

private def node_404 : formulaCurve.Point :=
  .some (271562755929956652806825824528198565493639379941 : F) (54583023176751588433946593234204541466759391026 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_404 : node_403+node_403=node_404 := by
  dsimp only [node_403,node_404]
  apply tangent_double (ℓ:=(629794826889685238400705537573411785994348410008 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_404 : (783776074523705082372325074569033981878 : ℕ) • G=node_404 := by
  rw [show (783776074523705082372325074569033981878 : ℕ)=391888037261852541186162537284516990939+391888037261852541186162537284516990939 by norm_num,
    add_nsmul,nsmul_403]
  exact add_404

private def node_405 : formulaCurve.Point :=
  .some (431778032405446927395856958446868564871316308694 : F) (414685122165850621894270127964516182584157814990 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_405 : node_404+node_404=node_405 := by
  dsimp only [node_404,node_405]
  apply tangent_double (ℓ:=(664844524204311621502810743961222990463256666680 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_405 : (1567552149047410164744650149138067963756 : ℕ) • G=node_405 := by
  rw [show (1567552149047410164744650149138067963756 : ℕ)=783776074523705082372325074569033981878+783776074523705082372325074569033981878 by norm_num,
    add_nsmul,nsmul_404]
  exact add_405

private def node_406 : formulaCurve.Point :=
  .some (616014143562477156235489254275875394920346754217 : F) (575742719607744309645153305524135112738306767694 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_406 : node_405+node_0=node_406 := by
  dsimp only [node_405,node_0,node_406]
  apply secant_add (ℓ:=(253091222159381458847314438653167227473141389359 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_406 : (1567552149047410164744650149138067963757 : ℕ) • G=node_406 := by
  rw [show (1567552149047410164744650149138067963757 : ℕ)=1567552149047410164744650149138067963756+1 by norm_num,
    add_nsmul,nsmul_405,nsmul_0]
  exact add_406

private def node_407 : formulaCurve.Point :=
  .some (424565305081086521955722357720794507151493610455 : F) (205884387172207917043245089534910107721150116570 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_407 : node_406+node_406=node_407 := by
  dsimp only [node_406,node_407]
  apply tangent_double (ℓ:=(495960467432921690951313090826154522922354657861 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_407 : (3135104298094820329489300298276135927514 : ℕ) • G=node_407 := by
  rw [show (3135104298094820329489300298276135927514 : ℕ)=1567552149047410164744650149138067963757+1567552149047410164744650149138067963757 by norm_num,
    add_nsmul,nsmul_406]
  exact add_407

private def node_408 : formulaCurve.Point :=
  .some (521793321676139403694914466588923787043679856847 : F) (240610990989632439636125217662628564306919369395 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_408 : node_407+node_407=node_408 := by
  dsimp only [node_407,node_408]
  apply tangent_double (ℓ:=(527867631128919703101695100366329544664857758662 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_408 : (6270208596189640658978600596552271855028 : ℕ) • G=node_408 := by
  rw [show (6270208596189640658978600596552271855028 : ℕ)=3135104298094820329489300298276135927514+3135104298094820329489300298276135927514 by norm_num,
    add_nsmul,nsmul_407]
  exact add_408

private def node_409 : formulaCurve.Point :=
  .some (442083316108421672027155529433718181259539360684 : F) (291828038112581463311970042031309455115442807135 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_409 : node_408+node_0=node_409 := by
  dsimp only [node_408,node_0,node_409]
  apply secant_add (ℓ:=(650412788564325221355017150260399382271992131250 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_409 : (6270208596189640658978600596552271855029 : ℕ) • G=node_409 := by
  rw [show (6270208596189640658978600596552271855029 : ℕ)=6270208596189640658978600596552271855028+1 by norm_num,
    add_nsmul,nsmul_408,nsmul_0]
  exact add_409

private def node_410 : formulaCurve.Point :=
  .some (179961043894123026389627868641523510745135268047 : F) (2252478632131724948133500627951901241642816294 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_410 : node_409+node_409=node_410 := by
  dsimp only [node_409,node_410]
  apply tangent_double (ℓ:=(669093948754416272250540627223307491726486713612 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_410 : (12540417192379281317957201193104543710058 : ℕ) • G=node_410 := by
  rw [show (12540417192379281317957201193104543710058 : ℕ)=6270208596189640658978600596552271855029+6270208596189640658978600596552271855029 by norm_num,
    add_nsmul,nsmul_409]
  exact add_410

private def node_411 : formulaCurve.Point :=
  .some (263071781711684832793908897229277376663752638454 : F) (418024716196849887726214804992042990291891531596 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_411 : node_410+node_410=node_411 := by
  dsimp only [node_410,node_411]
  apply tangent_double (ℓ:=(529259975258306919340565660723773502736634768569 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_411 : (25080834384758562635914402386209087420116 : ℕ) • G=node_411 := by
  rw [show (25080834384758562635914402386209087420116 : ℕ)=12540417192379281317957201193104543710058+12540417192379281317957201193104543710058 by norm_num,
    add_nsmul,nsmul_410]
  exact add_411

private def node_412 : formulaCurve.Point :=
  .some (474553951666455706094527144890817311787866785537 : F) (216276235354971743672525147543094421691288950107 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_412 : node_411+node_411=node_412 := by
  dsimp only [node_411,node_412]
  apply tangent_double (ℓ:=(180291944042883138367260852203913226007352586774 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_412 : (50161668769517125271828804772418174840232 : ℕ) • G=node_412 := by
  rw [show (50161668769517125271828804772418174840232 : ℕ)=25080834384758562635914402386209087420116+25080834384758562635914402386209087420116 by norm_num,
    add_nsmul,nsmul_411]
  exact add_412

private def node_413 : formulaCurve.Point :=
  .some (525386396969122027531229188375087826131358464108 : F) (448455918753436510693764527656399960957789226111 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_413 : node_412+node_412=node_413 := by
  dsimp only [node_412,node_413]
  apply tangent_double (ℓ:=(15196559226365083580143029362550061841861391375 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_413 : (100323337539034250543657609544836349680464 : ℕ) • G=node_413 := by
  rw [show (100323337539034250543657609544836349680464 : ℕ)=50161668769517125271828804772418174840232+50161668769517125271828804772418174840232 by norm_num,
    add_nsmul,nsmul_412]
  exact add_413

private def node_414 : formulaCurve.Point :=
  .some (200271544317703136294869513197544614497614072178 : F) (178591436798182792010608699645817252051346838891 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_414 : node_413+node_413=node_414 := by
  dsimp only [node_413,node_414]
  apply tangent_double (ℓ:=(451354720120645388429045172484572628400574145417 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_414 : (200646675078068501087315219089672699360928 : ℕ) • G=node_414 := by
  rw [show (200646675078068501087315219089672699360928 : ℕ)=100323337539034250543657609544836349680464+100323337539034250543657609544836349680464 by norm_num,
    add_nsmul,nsmul_413]
  exact add_414

private def node_415 : formulaCurve.Point :=
  .some (593415115187734545248830282855864374852294356275 : F) (364664723795408536116757231170807311966997794331 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_415 : node_414+node_0=node_415 := by
  dsimp only [node_414,node_0,node_415]
  apply secant_add (ℓ:=(118623675474668382420099188949522750745478354347 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_415 : (200646675078068501087315219089672699360929 : ℕ) • G=node_415 := by
  rw [show (200646675078068501087315219089672699360929 : ℕ)=200646675078068501087315219089672699360928+1 by norm_num,
    add_nsmul,nsmul_414,nsmul_0]
  exact add_415

private def node_416 : formulaCurve.Point :=
  .some (442702949426686394968369041918940953469006671944 : F) (600244552359337644596359841858792035416351099481 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_416 : node_415+node_415=node_416 := by
  dsimp only [node_415,node_416]
  apply tangent_double (ℓ:=(343216667277701786313658965280930374791688674316 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_416 : (401293350156137002174630438179345398721858 : ℕ) • G=node_416 := by
  rw [show (401293350156137002174630438179345398721858 : ℕ)=200646675078068501087315219089672699360929+200646675078068501087315219089672699360929 by norm_num,
    add_nsmul,nsmul_415]
  exact add_416

private def node_417 : formulaCurve.Point :=
  .some (186032098475879433021402140989588113644227766984 : F) (697353887985885106972181767618871110736786798290 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_417 : node_416+node_416=node_417 := by
  dsimp only [node_416,node_417]
  apply tangent_double (ℓ:=(199787922483739776152442165727183917499391845061 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_417 : (802586700312274004349260876358690797443716 : ℕ) • G=node_417 := by
  rw [show (802586700312274004349260876358690797443716 : ℕ)=401293350156137002174630438179345398721858+401293350156137002174630438179345398721858 by norm_num,
    add_nsmul,nsmul_416]
  exact add_417

private def node_418 : formulaCurve.Point :=
  .some (489782871141230840485352377130124093496700003240 : F) (38223252583083505473307391769887092014097695150 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_418 : node_417+node_417=node_418 := by
  dsimp only [node_417,node_418]
  apply tangent_double (ℓ:=(266235167823659147955567706294613040952139816724 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_418 : (1605173400624548008698521752717381594887432 : ℕ) • G=node_418 := by
  rw [show (1605173400624548008698521752717381594887432 : ℕ)=802586700312274004349260876358690797443716+802586700312274004349260876358690797443716 by norm_num,
    add_nsmul,nsmul_417]
  exact add_418

private def node_419 : formulaCurve.Point :=
  .some (239549658368015133476589427724673975924882658384 : F) (152366594847131403531344150300192144314983001877 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_419 : node_418+node_418=node_419 := by
  dsimp only [node_418,node_419]
  apply tangent_double (ℓ:=(681373656809692809581844540672725224388271089246 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_419 : (3210346801249096017397043505434763189774864 : ℕ) • G=node_419 := by
  rw [show (3210346801249096017397043505434763189774864 : ℕ)=1605173400624548008698521752717381594887432+1605173400624548008698521752717381594887432 by norm_num,
    add_nsmul,nsmul_418]
  exact add_419

private def node_420 : formulaCurve.Point :=
  .some (120436643480710126928772040418557768381410145595 : F) (276269901597613144926025177988640625395586049321 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_420 : node_419+node_419=node_420 := by
  dsimp only [node_419,node_420]
  apply tangent_double (ℓ:=(17555563598778543077359591246160307984554696489 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_420 : (6420693602498192034794087010869526379549728 : ℕ) • G=node_420 := by
  rw [show (6420693602498192034794087010869526379549728 : ℕ)=3210346801249096017397043505434763189774864+3210346801249096017397043505434763189774864 by norm_num,
    add_nsmul,nsmul_419]
  exact add_420

private def node_421 : formulaCurve.Point :=
  .some (406370158773228987492775489981518126004095524482 : F) (726024931965456391235151113763961870092102329006 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_421 : node_420+node_0=node_421 := by
  dsimp only [node_420,node_0,node_421]
  apply secant_add (ℓ:=(146474434087147535565252709855394006890490265872 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_421 : (6420693602498192034794087010869526379549729 : ℕ) • G=node_421 := by
  rw [show (6420693602498192034794087010869526379549729 : ℕ)=6420693602498192034794087010869526379549728+1 by norm_num,
    add_nsmul,nsmul_420,nsmul_0]
  exact add_421

private def node_422 : formulaCurve.Point :=
  .some (319818561163772966143635037548266791313568384908 : F) (541288386561083264804137164849596268448683003290 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_422 : node_421+node_421=node_422 := by
  dsimp only [node_421,node_422]
  apply tangent_double (ℓ:=(133247766876851476013708129488763547083931102113 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_422 : (12841387204996384069588174021739052759099458 : ℕ) • G=node_422 := by
  rw [show (12841387204996384069588174021739052759099458 : ℕ)=6420693602498192034794087010869526379549729+6420693602498192034794087010869526379549729 by norm_num,
    add_nsmul,nsmul_421]
  exact add_422

private def node_423 : formulaCurve.Point :=
  .some (504662743620559093393472281884064771945737004484 : F) (128683699657049591390610180775068555456465106847 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_423 : node_422+node_422=node_423 := by
  dsimp only [node_422,node_423]
  apply tangent_double (ℓ:=(401353056966446891496752127080542013982938521042 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_423 : (25682774409992768139176348043478105518198916 : ℕ) • G=node_423 := by
  rw [show (25682774409992768139176348043478105518198916 : ℕ)=12841387204996384069588174021739052759099458+12841387204996384069588174021739052759099458 by norm_num,
    add_nsmul,nsmul_422]
  exact add_423

private def node_424 : formulaCurve.Point :=
  .some (400264726356262040834477018343807756386291203965 : F) (375110465964215655759093711974024231956757693685 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_424 : node_423+node_0=node_424 := by
  dsimp only [node_423,node_0,node_424]
  apply secant_add (ℓ:=(169496881293641619048713984656440907683931158061 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_424 : (25682774409992768139176348043478105518198917 : ℕ) • G=node_424 := by
  rw [show (25682774409992768139176348043478105518198917 : ℕ)=25682774409992768139176348043478105518198916+1 by norm_num,
    add_nsmul,nsmul_423,nsmul_0]
  exact add_424

private def node_425 : formulaCurve.Point :=
  .some (597044963176922789992538481145536336279676590628 : F) (83878985530891068417066588654961666225976273582 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_425 : node_424+node_424=node_425 := by
  dsimp only [node_424,node_425]
  apply tangent_double (ℓ:=(117409024316511835939434184044735285026271453345 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_425 : (51365548819985536278352696086956211036397834 : ℕ) • G=node_425 := by
  rw [show (51365548819985536278352696086956211036397834 : ℕ)=25682774409992768139176348043478105518198917+25682774409992768139176348043478105518198917 by norm_num,
    add_nsmul,nsmul_424]
  exact add_425

private def node_426 : formulaCurve.Point :=
  .some (613484347015269463940775654559558621894839912831 : F) (16745763676300631264567531354323818949315481733 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_426 : node_425+node_0=node_426 := by
  dsimp only [node_425,node_0,node_426]
  apply secant_add (ℓ:=(328884102082417374763060911494143505687998749341 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_426 : (51365548819985536278352696086956211036397835 : ℕ) • G=node_426 := by
  rw [show (51365548819985536278352696086956211036397835 : ℕ)=51365548819985536278352696086956211036397834+1 by norm_num,
    add_nsmul,nsmul_425,nsmul_0]
  exact add_426

private def node_427 : formulaCurve.Point :=
  .some (126457021169111529164087287535382696203361030091 : F) (533980086201822276049293560915483386501160188353 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_427 : node_426+node_426=node_427 := by
  dsimp only [node_426,node_427]
  apply tangent_double (ℓ:=(520503026308808049101141346604526866727861038534 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_427 : (102731097639971072556705392173912422072795670 : ℕ) • G=node_427 := by
  rw [show (102731097639971072556705392173912422072795670 : ℕ)=51365548819985536278352696086956211036397835+51365548819985536278352696086956211036397835 by norm_num,
    add_nsmul,nsmul_426]
  exact add_427

private def node_428 : formulaCurve.Point :=
  .some (304172685540800682626759356179065258060233758362 : F) (505894881446707285110121711070266936173322534369 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_428 : node_427+node_0=node_428 := by
  dsimp only [node_427,node_0,node_428]
  apply secant_add (ℓ:=(111916431341939177983446112869215978694141670185 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_428 : (102731097639971072556705392173912422072795671 : ℕ) • G=node_428 := by
  rw [show (102731097639971072556705392173912422072795671 : ℕ)=102731097639971072556705392173912422072795670+1 by norm_num,
    add_nsmul,nsmul_427,nsmul_0]
  exact add_428

private def node_429 : formulaCurve.Point :=
  .some (646040072282245938213942093212081001566773651092 : F) (52307016868443832443188161679681442698023738696 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_429 : node_428+node_428=node_429 := by
  dsimp only [node_428,node_429]
  apply tangent_double (ℓ:=(301531057530952154545952454784616177919780338148 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_429 : (205462195279942145113410784347824844145591342 : ℕ) • G=node_429 := by
  rw [show (205462195279942145113410784347824844145591342 : ℕ)=102731097639971072556705392173912422072795671+102731097639971072556705392173912422072795671 by norm_num,
    add_nsmul,nsmul_428]
  exact add_429

private def node_430 : formulaCurve.Point :=
  .some (230111788128967377009128094169081291010603627283 : F) (327179519815318669886291954965630497317685853188 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_430 : node_429+node_0=node_430 := by
  dsimp only [node_429,node_0,node_430]
  apply secant_add (ℓ:=(573223501664091553289477293416432915892368507238 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_430 : (205462195279942145113410784347824844145591343 : ℕ) • G=node_430 := by
  rw [show (205462195279942145113410784347824844145591343 : ℕ)=205462195279942145113410784347824844145591342+1 by norm_num,
    add_nsmul,nsmul_429,nsmul_0]
  exact add_430

private def node_431 : formulaCurve.Point :=
  .some (203100273241872066821618260505998029055123635736 : F) (659764649145302911632109021023025036059563522337 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_431 : node_430+node_430=node_431 := by
  dsimp only [node_430,node_431]
  apply tangent_double (ℓ:=(200306243172972553069109015948154689320957373700 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_431 : (410924390559884290226821568695649688291182686 : ℕ) • G=node_431 := by
  rw [show (410924390559884290226821568695649688291182686 : ℕ)=205462195279942145113410784347824844145591343+205462195279942145113410784347824844145591343 by norm_num,
    add_nsmul,nsmul_430]
  exact add_431

private def node_432 : formulaCurve.Point :=
  .some (720334624765046204244846323564265925446598860846 : F) (516910538963221156545465130691321589450524812678 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_432 : node_431+node_0=node_432 := by
  dsimp only [node_431,node_0,node_432]
  apply secant_add (ℓ:=(556516905210499575369079895166112331704471313099 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_432 : (410924390559884290226821568695649688291182687 : ℕ) • G=node_432 := by
  rw [show (410924390559884290226821568695649688291182687 : ℕ)=410924390559884290226821568695649688291182686+1 by norm_num,
    add_nsmul,nsmul_431,nsmul_0]
  exact add_432

private def node_433 : formulaCurve.Point :=
  .some (430376216918149202624187938635714344168216394526 : F) (547219296622751142767370281471494830727772899563 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_433 : node_432+node_432=node_433 := by
  dsimp only [node_432,node_433]
  apply tangent_double (ℓ:=(158440516491360902622271660701693983414837497184 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_433 : (821848781119768580453643137391299376582365374 : ℕ) • G=node_433 := by
  rw [show (821848781119768580453643137391299376582365374 : ℕ)=410924390559884290226821568695649688291182687+410924390559884290226821568695649688291182687 by norm_num,
    add_nsmul,nsmul_432]
  exact add_433

private def node_434 : formulaCurve.Point :=
  .some (159956557480851788168216474418984391607243125413 : F) (575921684962613102334349491493761768054960134521 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_434 : node_433+node_0=node_434 := by
  dsimp only [node_433,node_0,node_434]
  apply secant_add (ℓ:=(236745782962637834957824107350190515729479441644 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_434 : (821848781119768580453643137391299376582365375 : ℕ) • G=node_434 := by
  rw [show (821848781119768580453643137391299376582365375 : ℕ)=821848781119768580453643137391299376582365374+1 by norm_num,
    add_nsmul,nsmul_433,nsmul_0]
  exact add_434

private def node_435 : formulaCurve.Point :=
  .some (527006715521674663359923195217320426327584562122 : F) (566692069801177406380498891477426840840597342992 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_435 : node_434+node_434=node_435 := by
  dsimp only [node_434,node_435]
  apply tangent_double (ℓ:=(213482130820157028294043195391453426739130467488 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_435 : (1643697562239537160907286274782598753164730750 : ℕ) • G=node_435 := by
  rw [show (1643697562239537160907286274782598753164730750 : ℕ)=821848781119768580453643137391299376582365375+821848781119768580453643137391299376582365375 by norm_num,
    add_nsmul,nsmul_434]
  exact add_435

private def node_436 : formulaCurve.Point :=
  .some (391088687679482492857834540651001524637524074995 : F) (568224238651736094837526616418836130105744670689 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_436 : node_435+node_0=node_436 := by
  dsimp only [node_435,node_0,node_436]
  apply secant_add (ℓ:=(327069292553084555185592454050638617927207968367 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_436 : (1643697562239537160907286274782598753164730751 : ℕ) • G=node_436 := by
  rw [show (1643697562239537160907286274782598753164730751 : ℕ)=1643697562239537160907286274782598753164730750+1 by norm_num,
    add_nsmul,nsmul_435,nsmul_0]
  exact add_436

private def node_437 : formulaCurve.Point :=
  .some (692027449798196469369660943450747020255225179049 : F) (540084096095253278105066674794651134621027373067 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_437 : node_436+node_436=node_437 := by
  dsimp only [node_436,node_437]
  apply tangent_double (ℓ:=(466433753327502129488390089772382073323288124576 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_437 : (3287395124479074321814572549565197506329461502 : ℕ) • G=node_437 := by
  rw [show (3287395124479074321814572549565197506329461502 : ℕ)=1643697562239537160907286274782598753164730751+1643697562239537160907286274782598753164730751 by norm_num,
    add_nsmul,nsmul_436]
  exact add_437

private def node_438 : formulaCurve.Point :=
  .some (239160252719240576754240933536229305889122728039 : F) (633573668393003517826118091937338430003544727576 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_438 : node_437+node_437=node_438 := by
  dsimp only [node_437,node_438]
  apply tangent_double (ℓ:=(334780735363211498683646165722106774299378187630 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_438 : (6574790248958148643629145099130395012658923004 : ℕ) • G=node_438 := by
  rw [show (6574790248958148643629145099130395012658923004 : ℕ)=3287395124479074321814572549565197506329461502+3287395124479074321814572549565197506329461502 by norm_num,
    add_nsmul,nsmul_437]
  exact add_438

private def node_439 : formulaCurve.Point :=
  .some (7859430180186065406825903265899117381581648180 : F) (483783381205651795398532540546023079478582212375 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_439 : node_438+node_0=node_439 := by
  dsimp only [node_438,node_0,node_439]
  apply secant_add (ℓ:=(561194596292356960506662676470780317691713354270 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_439 : (6574790248958148643629145099130395012658923005 : ℕ) • G=node_439 := by
  rw [show (6574790248958148643629145099130395012658923005 : ℕ)=6574790248958148643629145099130395012658923004+1 by norm_num,
    add_nsmul,nsmul_438,nsmul_0]
  exact add_439

private def node_440 : formulaCurve.Point :=
  .some (517696635930025282142792176081868490961942941595 : F) (176536276318969994106399280560301060926753541967 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_440 : node_439+node_439=node_440 := by
  dsimp only [node_439,node_440]
  apply tangent_double (ℓ:=(247281591009407942643523529585645793978155613612 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_440 : (13149580497916297287258290198260790025317846010 : ℕ) • G=node_440 := by
  rw [show (13149580497916297287258290198260790025317846010 : ℕ)=6574790248958148643629145099130395012658923005+6574790248958148643629145099130395012658923005 by norm_num,
    add_nsmul,nsmul_439]
  exact add_440

private def node_441 : formulaCurve.Point :=
  .some (110813798174042929627904389084539495603396924425 : F) (180428035815802352175974906103952632334954141224 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_441 : node_440+node_0=node_441 := by
  dsimp only [node_440,node_0,node_441]
  apply secant_add (ℓ:=(136382529441696569893930454949490142927659650170 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_441 : (13149580497916297287258290198260790025317846011 : ℕ) • G=node_441 := by
  rw [show (13149580497916297287258290198260790025317846011 : ℕ)=13149580497916297287258290198260790025317846010+1 by norm_num,
    add_nsmul,nsmul_440,nsmul_0]
  exact add_441

private def node_442 : formulaCurve.Point :=
  .some (181109855364250095998838445903736104176255069219 : F) (511645851584275974616115135874421381017294046655 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_442 : node_441+node_441=node_442 := by
  dsimp only [node_441,node_442]
  apply tangent_double (ℓ:=(276850105030347518146406591961320143021339722274 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_442 : (26299160995832594574516580396521580050635692022 : ℕ) • G=node_442 := by
  rw [show (26299160995832594574516580396521580050635692022 : ℕ)=13149580497916297287258290198260790025317846011+13149580497916297287258290198260790025317846011 by norm_num,
    add_nsmul,nsmul_441]
  exact add_442

private def node_443 : formulaCurve.Point :=
  .some (181398508288355630747851919554856919012256736369 : F) (598326478328139208056436489595613468461686995228 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_443 : node_442+node_442=node_443 := by
  dsimp only [node_442,node_443]
  apply tangent_double (ℓ:=(85640520376692231296366265600239228227876723872 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_443 : (52598321991665189149033160793043160101271384044 : ℕ) • G=node_443 := by
  rw [show (52598321991665189149033160793043160101271384044 : ℕ)=26299160995832594574516580396521580050635692022+26299160995832594574516580396521580050635692022 by norm_num,
    add_nsmul,nsmul_442]
  exact add_443

private def node_444 : formulaCurve.Point :=
  .some (384990024000825071861159268398641750596794649191 : F) (439339922263818440492012759606543239950007492378 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_444 : node_443+node_443=node_444 := by
  dsimp only [node_443,node_444]
  apply tangent_double (ℓ:=(6334021730227741738319729198463834408434021384 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_444 : (105196643983330378298066321586086320202542768088 : ℕ) • G=node_444 := by
  rw [show (105196643983330378298066321586086320202542768088 : ℕ)=52598321991665189149033160793043160101271384044+52598321991665189149033160793043160101271384044 by norm_num,
    add_nsmul,nsmul_443]
  exact add_444

private def node_445 : formulaCurve.Point :=
  .some (443455821991718217719458578923151842647725092474 : F) (176573771147450543599107283358475142735318312272 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_445 : node_444+node_0=node_445 := by
  dsimp only [node_444,node_0,node_445]
  apply secant_add (ℓ:=(289477473264832210000239495805001475057910570705 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_445 : (105196643983330378298066321586086320202542768089 : ℕ) • G=node_445 := by
  rw [show (105196643983330378298066321586086320202542768089 : ℕ)=105196643983330378298066321586086320202542768088+1 by norm_num,
    add_nsmul,nsmul_444,nsmul_0]
  exact add_445

private def node_446 : formulaCurve.Point :=
  .some (661763578747278442569138171326635646103480054969 : F) (678818123500113454589622099026902848611458542839 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_446 : node_445+node_445=node_446 := by
  dsimp only [node_445,node_446]
  apply tangent_double (ℓ:=(49545385893341304543166181352576670362744055051 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_446 : (210393287966660756596132643172172640405085536178 : ℕ) • G=node_446 := by
  rw [show (210393287966660756596132643172172640405085536178 : ℕ)=105196643983330378298066321586086320202542768089+105196643983330378298066321586086320202542768089 by norm_num,
    add_nsmul,nsmul_445]
  exact add_446

private def node_447 : formulaCurve.Point :=
  .some (259479713158273757050367190534858072594223227627 : F) (39414799304662693276020416800594933604368475811 : F) (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      formula_discriminant_ne_zero).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [formulaCurve]
    unfold F p
    reduce_mod_char)

private theorem add_447 : node_446+node_0=node_447 := by
  dsimp only [node_446,node_0,node_447]
  apply secant_add (ℓ:=(168279764783477901168262575549274114986732014776 : F))
  all_goals try simp only [formulaCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY]
  all_goals unfold F p
  all_goals reduce_mod_char
  all_goals decide +kernel

private theorem nsmul_447 : (210393287966660756596132643172172640405085536179 : ℕ) • G=node_447 := by
  rw [show (210393287966660756596132643172172640405085536179 : ℕ)=210393287966660756596132643172172640405085536178+1 by norm_num,
    add_nsmul,nsmul_446,nsmul_0]
  exact add_447

def Q : formulaCurve.Point := node_447
def knownScalar : ℕ := 210393287966660756596132643172172640405085536179
theorem G_ne_zero : G≠0 := by simp [G,node_0]
theorem p_nsmul_G : p • G=0 := by
  exact nsmul_210
theorem G_order : addOrderOf G=p := by
  apply addOrderOf_eq_prime
  · exact p_nsmul_G
  · exact G_ne_zero
theorem known_answer_relation : knownScalar • G=Q := by
  exact nsmul_447
theorem large_order_audit_certificate :
    Nat.Prime p ∧ 2^159≤p ∧ p<2^160 ∧ formulaCurve.Δ≠0 ∧
    addOrderOf G=p ∧ G≠0 ∧ knownScalar • G=Q ∧
    formulaCurve.j=(-32768 : F) ∧ printedCurve.j≠(-32768 : F) := by
  exact ⟨p_prime,p_bit_bounds.1,p_bit_bounds.2,formula_discriminant_ne_zero,
    G_order,G_ne_zero,known_answer_relation,formula_j,printed_j_not_claimed⟩
#print axioms G_order
#print axioms known_answer_relation
#print axioms large_order_audit_certificate
end
end AnomalousPointTraceAudit
