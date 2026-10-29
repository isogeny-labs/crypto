#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);cd "$here"
sha256sum -c map-inputs.sha256
mkdir -p source-data
cat > source-data/mapbuild-input-template.sing <<'INNER'
ring mapControl27=2,(u,v),dp;
int curveA;poly sourceEquation;poly targetEquation;
for(curveA=0;curveA<=1;curveA++){
 sourceEquation=v^2+u^2*v+u^7+curveA*u^4+u;
 targetEquation=(u*v+1)^2+u^3*(u*v+1)+u^9+curveA*u^6+1;
 if(targetEquation!=u^2*sourceEquation){ERROR("actual samefield control identity failed");}
 if(((u*v+1+u)^2+u^3*(u*v+1+u)+u^9+curveA*u^6+1)+targetEquation!=u^2+u^4){ERROR("corrupt samefield control failed");}
}
print("MAPBUILD27_SAMEFIELD_POSITIVE_CORRUPT_CONTROLS_PASS");
ring mapBuild27=2,(w),lp;
INNER
cat runs/action27-01/sigma-matrix.sing runs/action27-01/basis-jets389.sing runs/action27-01/invariant-coefficients.sing runs/action27-01/original-coordinate-coefficients.sing MAPBUILD27.sing >> source-data/mapbuild-input-template.sing
sha256sum source-data/mapbuild-input-template.sing > source-data/mapbuild-input.sha256
tar -cJf checkpoint-live.tar.xz runs source-data
