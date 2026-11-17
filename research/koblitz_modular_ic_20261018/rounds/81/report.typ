#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R81: four explicit candidate F4 maps
For each R80 prime quotient, the native program chose a generator $g$ with $g^2+g+1=0$, printed all 22 variable images in $\{0,1,g,g+1\}$, and verified the resulting map ideal equals the prime ideal in both directions.

#image("candidate81.svg", width: 100%)

The source coordinate triple $(x,y,p)$ is the same for both primes over each component: $(g,0,0)$ at $y=0$ and $(0,1,0)$ at $y=1$. Five auxiliary coordinates distinguish the first pair; six distinguish the second. These are exact maps of the rejected R50 candidate algebra. Source-field equality and local normality must be checked before the split is used as branch geometry.
