# Minimal-prefix implication

Write each recorded row as [U, -b, -a], with equation Ux-bk-a=0 over F_34147. Node zero is fixed at zero and omitted. The width is fixed to the terminal nuisance-node inventory; coordinates absent from an early prefix are free and cannot affect the set of attainable target coordinates.

At the claimed first prefix j, the direct checker verifies a row combination [0,...,0,1,-k0] and a particular solution with target k0. This proves consistency and uniqueness of the target at j. At prefix j-1 it verifies a particular point with target zero and a homogeneous direction with target one. Their affine combinations provide a solution for every target coordinate. Removing more equations preserves these solutions, so every prefix before j is undetermined. This proves minimality without trusting binary search or Gaussian elimination.

For a terminal undetermined certificate, the same affine-family argument applies to every earlier prefix. It does not assert that arbitrary nuisance assignments are logarithms of geometric points; the conclusion concerns precisely the recorded linear equations.

An empty prefix has free target coordinate, including with zero nuisance columns. Immediate determination at j=1 is valid and its predecessor is that empty prefix. The synthetic controls exercise both conditions. No observed native stream had immediate determination. Inconsistent terminal systems are rejected by this packet rather than misclassified.
