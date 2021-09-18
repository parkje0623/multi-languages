import Data.Time.Calendar
import Data.Time.Calendar.OrdinalDate

merge :: (Ord a) => [a] -> [a] -> [a]
merge a [] = a
merge [] b = b
merge (a:alst) (b:blst) 
 | a < b = [a] ++ (merge alst (b:blst))
 | otherwise = [b] ++ (merge (a:alst) blst)
 
mergeSort :: Ord a => [a] -> [a]
mergeSort [] = []
mergeSort [x] = [x]
mergeSort xs = merge (mergeSort leftHalf) (mergeSort rightHalf)
 where
  (leftHalf, rightHalf) = splitAt ((div) (length xs) 2) xs
  
daysInYear :: Integer -> [Day]
daysInYear y = [jan1..dec31]
 where 
  jan1 = fromGregorian y 1 1
  dec31 = fromGregorian y 12 31
  
isFriday :: Day -> Bool
isFriday day
 | snd (mondayStartWeek day) == 5 = True
 | otherwise                      = False

--divisors function from exercise 2
divisors :: Int -> [Int]
divisors n = [i | i<-[2..(n `div` 2)], n `mod` i == 0]

isPrimeDay :: Day -> Bool
isPrimeDay day
 | divisors (getDay (toGregorian(day))) == [] = True
 | otherwise                                  = False
 
getDay (y,m,d) = d

primeFridays :: Integer -> [Day]
primeFridays x = [day | day <- (daysInYear x), (isPrimeDay day), (isFriday day)]