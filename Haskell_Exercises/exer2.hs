divisors :: Int -> [Int]
divisors n = [i | i<-[2..(n `div` 2)], n `mod` i == 0]
primes :: Int -> [Int]
primes n = [i | i<-[2..n], divisors i == []]

pythagorean :: Int -> [(Int, Int, Int)]
pythagorean n = [(a,b,c) | a<-[1..n], b<-[1..n], c<-[1..n], a^2 + b^2 == c^2, a<b]

join :: String -> [String] -> String
join _ [] = ""
join _ [a] = a
join a (x:xs) = (x ++ a) ++ join a xs

fact' :: Integer -> Integer
fact' n = foldl (*) 1 [1..n]

--For hailLen (Grabbed from exercise 1)
hailstone n 
 | even n = n `div` 2
 | odd n  = 3 * n + 1

hailLen :: Int -> Int
hailLen n = hailTail 0 n
 where
  hailTail a 1 = a
  hailTail a n = hailTail (a+1) (hailstone n)