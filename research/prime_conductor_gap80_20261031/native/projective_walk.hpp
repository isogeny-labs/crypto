#pragma once
// Internal helper: Point, Z, p, a, mod(), inverse(), add() and require() are
// supplied by complete_pipeline.cpp. General short-Weierstrass, p > 3.
// Formulas: EFD jacobian madd-2004-hmv and dbl-2007-bl (general a).
struct ProjectivePoint { Z x=0,y=1,z=0; };
static ProjectivePoint projective_lift(const Point& u) {
 return u.inf?ProjectivePoint{}:ProjectivePoint{u.x,u.y,1};
}
static ProjectivePoint projective_double(const ProjectivePoint& u) {
 if(u.z==0||u.y==0)return {};
 Z xx=mod(u.x*u.x),yy=mod(u.y*u.y),yyyy=mod(yy*yy),zz=mod(u.z*u.z);
 Z s=mod(2*((u.x+yy)*(u.x+yy)-xx-yyyy)),m=mod(3*xx+a*zz*zz);
 Z x=mod(m*m-2*s),y=mod(m*(s-x)-8*yyyy),z=mod((u.y+u.z)*(u.y+u.z)-yy-zz);
 return {x,y,z};
}
static ProjectivePoint projective_mixed_add(const ProjectivePoint& u,const Point& v) {
 if(v.inf)return u;
 if(u.z==0)return projective_lift(v);
 Z zz=mod(u.z*u.z),h=mod(v.x*zz-u.x),r=mod(v.y*u.z*zz-u.y);
 if(h==0)return r==0?projective_double(u):ProjectivePoint{};
 Z hh=mod(h*h),hhh=mod(h*hh),xh=mod(u.x*hh),x=mod(r*r-2*xh-hhh);
 return {x,mod(r*(xh-x)-u.y*hhh),mod(u.z*h)};
}
// Full conversion is only used by the small arithmetic controls.
static Point projective_affine(const ProjectivePoint& u) {
 if(u.z==0)return {};
 Z zi=inverse(u.z),zi2=mod(zi*zi);
 return {mod(u.x*zi2),mod(u.y*zi2*zi),false};
}
struct WalkDiagnostics {
 Z batch_inverse_calls=0,projective_additions=0,projective_points=0;
 unsigned long peak_batch_points=0;
};
template<class Emit> static void for_each_walk_x(Point start,const Point& step,
 unsigned long count,unsigned long batch,WalkDiagnostics& diagnostic,Emit emit) {
 if(batch==0){
   for(unsigned long k=0;k<count;k++){
     require(!start.inf,"affine walk infinity");emit(k,start.x);
     if(k+1<count)start=add(start,step);
   }
   return;
 }
 require(batch<=65536,"projective batch cap exceeds 65536");
 unsigned long capacity=std::min(batch,count);
 std::vector<Z> xs(capacity),zs(capacity),prefix(capacity);
 ProjectivePoint current=projective_lift(start);
 for(unsigned long offset=0;offset<count;){
   unsigned long size=std::min(batch,count-offset);Z accumulated=1;
   for(unsigned long k=0;k<size;k++){
     require(current.z!=0,"projective walk infinity");
     xs[k]=current.x;zs[k]=current.z;prefix[k]=accumulated;
     accumulated=mod(accumulated*zs[k]);
     if(offset+k+1<count){current=projective_mixed_add(current,step);diagnostic.projective_additions++;}
   }
   Z remaining_inverse=inverse(accumulated);diagnostic.batch_inverse_calls++;
   for(unsigned long k=size;k-->0;){
     Z zi=mod(remaining_inverse*prefix[k]);remaining_inverse=mod(remaining_inverse*zs[k]);
     xs[k]=mod(xs[k]*zi*zi);
   }
   diagnostic.projective_points+=size;
   diagnostic.peak_batch_points=std::max(diagnostic.peak_batch_points,size);
   for(unsigned long k=0;k<size;k++)emit(offset+k,xs[k]);
   offset+=size;
 }
}
