det a b c = b^2 - 4*a*c
quadsol1 a b c = (-b - sqrt (det a b c))/2*a
quadsol2 a b c = (-b + sqrt (det a b c))/2*a

--"!! n" takes the (n+1)th element from the list starting at 0
--"!! 2" would take the 3rd element. Since the list starts at 0 1 2 3 ...
third_a xs = xs !! 2

--Returns the third element (first:second:x(third):rest)
third_b (_:_:x:_) = x

--Factorial using if-else & recursion
fact n
 | n < 0     = error "Undefined"
 | n == 0    = 1
 | otherwise = n * fact (n-1)

--Hailstone: even number -> n/2, odd number -> 3*n + 1
hailstone n 
 | even n = n `div` 2
 | odd n  = 3 * n + 1

--HailLen using if-else & recursion
hailLen n
 | n == 1    = 0
 | otherwise = 1 + hailLen (hailstone n)

process [] = 8
process (x:xs) = x - (process $ xs)

--types = return $ map succ [2,3,4]