#!/usr/bin/env python3
"""CT74: input-uniform local Hensel lift of the 11a1 F_29 3-torsion.
All checks run under -O. Only standard-library integer arithmetic.
"""
import json

P = 29

def require(ok, why):
    if not ok:
        raise ArithmeticError(why)

def psi3(x):
    return 3*x**4 - 4*x**3 - 60*x*x - 237*x - 21

def psi3p(x):
    return 12*x**3 - 12*x*x - 120*x - 237

def rhs(x):
    return x*x*x-x*x-10*x-20

def on_curve(x, y, m):
    return (y*y+y-rhs(x)) % m == 0

def dbl(x, y, m):
    denom = (2*y+1)%m
    require(denom%P != 0, "3-torsion doubling denominator not invertible")
    slope = ((3*x*x-2*x-10)*pow(denom, -1, m))%m
    newx = (slope*slope+1-2*x)%m
    newy = (-slope*newx-y+slope*x-1)%m
    return (newx,newy)

def lift_root(x, k):
    modulus = P**k
    return (x - psi3(x)*pow(psi3p(x),-1,modulus))%modulus

def lift_y(x, old_y, k):
    modulus = P**k
    y = old_y
    return (y - (y*y+y-rhs(x))*pow(2*y+1,-1,modulus))%modulus

def main():
    require(psi3(18)%P==0 and psi3p(18)%P!=0, "simple torsion root")
    require(on_curve(18,5,P) and on_curve(18,23,P), "source points")
    require(dbl(18,5,P)==(18,23),"original order-three point")
    x=18
    y=5
    y_neg=23
    for k in range(1,7):
        m=P**k
        if k>1:
            x=lift_root(x,k)
            y=lift_y(x,y,k)
            y_neg=lift_y(x,y_neg,k)
        require(psi3(x)%m==0,"not a 3-division root")
        require(on_curve(x,y,m) and on_curve(x,y_neg,m), "curve equation")
        require(y_neg==(-y-1)%m,"negation")
        require(dbl(x,y,m)==(x,y_neg),"three torsion fails")
        require(dbl(x,y_neg,m)==(x,y),"inverse three torsion fails")
        require((x%P,y%P)==(18,5) and y_neg%P==23,"source labels")
        wrong=5
        for j in range(2,k+1):
            wrong=lift_y(18,wrong,j)
        require(on_curve(18,wrong,m),"naive point not on the curve")
        wrong_order=(dbl(18,wrong,m)==(18,(-wrong-1)%m))
        require(wrong_order==(k==1),"naive coordinate lift unexpectedly torsion")
        if k==2:
            require(len({lift_root(18+P*d,k) for d in range(P)})==1,"nonunique Hensel lift")
        out={"N":k,"modulus":str(m),"negative_curve_ok":True,"negative_order3":wrong_order,
             "point_3":[str(x),str(y)],"point_3_neg":[str(x),str(y_neg)],
             "point_lift_3":True,"reduction_ok":True}
        print(json.dumps(out,sort_keys=True,separators=(",",":")))

if __name__=="__main__":
    main()
