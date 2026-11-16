# Readable finite proof

Let C(T)=product_{q=-4}^3(T-q). Excluding the zero factor gives
C'(0)=4*3*2*1*(-1)*(-2)*(-3)=-144. Similarly C'(1)=240.

Define W_delta(T) by omitting the factor T-delta from C and dividing by
C'(0). For delta1, evaluation at T=1 gives C'(1)/C'(0)=-5/3.
For delta2, the remaining product still includes T-1, so the chain rule at
T=1+8z gives coefficient1 equal to
8*C'(1)/(C'(0)*(1-2))=40/3. Lean computes these finite products and
polynomial derivatives directly, rather than assuming that quotient formula.

Both coordinates of x=(0,0),y=(2,1) lie in[0,4). Their total two-bit
popcounts are0 and2, respectively, so both lie in the even parity code.
The first-axis low bits are equal, while y2-x2=1, the selected local
translation. Therefore the coefficient of z1 in the product kernel with
its second chart at z2=0 is (40/3)*(-5/3)=-200/9, which is nonzero.
The finite code distance alone has not killed this passive channel.

Connecting this product entry to Gamma's one-particle sector is the
standard determinant-of-1x1 identity and is explained in the written review;
it is not separately formalized here. The general passive-rank family is
also outside this finite formalization.
