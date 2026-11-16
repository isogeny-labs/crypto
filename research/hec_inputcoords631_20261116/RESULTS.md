# Original-input coordinates over F631

Frozen source ecdb66820c19 and lock supplement bc4367af25ba preceded checkpoint 20261116T022453Z and the single execution. Exit status is zero, stderr empty. The registered source setup and anchors01 matrix were reused without refitting or rerunning calibration.

Both actual public inputs D1 and B admit the first fixed translation offset, one times the calibration base. Their transformed primary zero-index coordinates are respectively 162 and 442, so the required inverse chart is available. Both adjacent translates also have nonzero coordinate vectors. All four vectors and both offsets are serialized in coordinates/ and printed in RUN.txt. No exclusions or policy amendments were needed.

This is a supported-coordinate gate, conditional on prior finite alignment evidence. It does not recover target classes, verify a global map, or establish a runtime gain. The next gate must port the inverse generator's F127 constants and its fixed positive control points to the actual F631 target basis, then solve and verify original and adjacent equations. The tenfold objective and genus-2/4 coverage remain open.
