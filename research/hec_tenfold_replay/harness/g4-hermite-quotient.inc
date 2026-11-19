       // f - w² = z³(z-t)^m Q. Read the coefficients of Q from high to low.
       constexpr int mB=CONTACT, wdeg=mB+2;
       int q[5]{};
       int t2=mod(t*t,P),t3=mod(t2*t,P);
       for(int j=4;j>=0;j--){
         int k=j+3+mB;
         int coeff=k<=9?f[k]:0;
         for(int i=k-wdeg;i<=wdeg;i++)if(i>=0 && k-i<=wdeg)coeff-=w[i]*w[k-i];
         if(j<4)coeff+=mB*t*q[j+1];
         if(j<3)coeff-=(mB==3?3:1)*t2*q[j+2];
         if(mB==3 && j<2)coeff+=t3*q[j+3];
         q[j]=mod(coeff,P);
       }
       int d=q[4]?4:3;
       assert(q[d]);
       QUOTIENT_AUDIT
       // Scan the retained rational abscissae. Once the quotient is quadratic,
       // its discriminant supplies both remaining roots, including repetitions.
       int roots_found[4]{},nr=0;
       for(auto point:coordinates){
         int z=point[0];
         int value=q[0]+z*q[1]+point[1]*q[2]+point[2]*q[3]+point[3]*q[4];
         if(value%P)continue;
         do{
           int lead=q[d];
           for(int i=d-1;i>=0;i--){int next=q[i];q[i]=lead;lead=mod(next+lead*z,P);}
           assert(!lead);
           q[d--]=0;roots_found[nr++]=z;
           if(d==2)break;
         }while(!mod(q[0]+z*q[1]+point[1]*q[2]+point[2]*q[3],P));
         if(d==2)break;
       }
       if(d!=2)continue;
       int disc=mod(q[1]*q[1]-4*q[2]*q[0],P);
       if(sqrt_table[disc]<0)continue;
       int inv=inv_table[mod(2*q[2],P)];
       roots_found[nr++]=mod((-q[1]+sqrt_table[disc])*inv,P);
       roots_found[nr++]=mod((-q[1]-sqrt_table[disc])*inv,P);
       std::vector<int> row(n);row[aj]=3;row[bj]=mB*sign;
       for(int i=0;i<nr;i++){
         int z=roots_found[i],x=mod(z+h,P);
         int value=0;for(int j=wdeg;j>=0;j--)value=mod(value*z+w[j],P);
         int index=by_x[x];assert(index>=0);
         int canon=ys[index];assert(value==canon || mod(-value,P)==canon);
         if(canon)row[cols[index]]+=value==canon?1:-1;
       }
       split++;insert(row);
