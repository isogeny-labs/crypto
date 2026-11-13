# From cofactor equations to the nonzero normalized norm

Pure local proof continuation, 2026-11-13. Parent component source01e312a94;
PR4059 is pending. Do not mutate that maintained PR while working here.

Target: under the actual quotient-moment and public branch chart hypotheses,
and the actual reduced coefficient cofactor equations for G, derive

    (C alpha*F)^2 - f*(C beta*H)^2 = C (alpha^2)*G,
    alpha^2 != 0.

Use the even chart deg F=deg H+2, deg f<=3, G monic of degree2 deg F.
No norm identity, nonzero norm normalization, numeric rank or witness-coordinate
oracle may be supplied as an assumption in this theorem. Suitable F,H,V and
cofactor equations remain inputs; no efficient finder or bit bound follows.

Route: choose an actual nonzero diagonal cofactor, use its actual adjugate
column, prove that column lies in the actual radical, transfer it to the
reduced known polynomial direction, and cancel nonzero field scaling in the
actual coefficient quadrics. Degree and leading coefficient imply alpha^2
is the actual leading norm coefficient; coefficient extensionality gives the
full identity. Do not use the old normOfWitnessLine theorem circularly: it
already assumes the norm identity being sought here.

Sources checked (KB unavailable): actual public branch/corank/cofactor source,
RankOneExtraction.pivotFactor and NormDualPencil.scaledPairing/minorsOfRank,
actual NormMomentMatrix.quadraticCoefficient, NormLift.norm, and pinned
polynomial Monic/natDegree/coefficient lemmas. Existing normOfWitnessLine
inspected specifically to avoid its circular use. Standard certificate
algebra, no priority/novelty claim [pcc:unverified].
