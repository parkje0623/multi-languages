import Data.Ratio

myIterate :: (a -> a) -> a -> [a]
myIterate f x = [x] ++ myIterate f (f x)

mySplitAt :: Int -> [a] -> ([a], [a])
mySplitAt n xs = (take n xs, drop n xs)

rationalSum :: (Integral a) => a -> [Ratio a]
rationalSum a = [(x%y)|x<-[1..a], y<-[1..a], y == (a-x)]

rationalSumLowest :: (Integral a) => a -> [Ratio a]
rationalSumLowest a = [(x%y)|x<-[1..a], y<-[1..a], y == (a-x), gcd x y == 1]

rationals :: (Integral a) => [Ratio a]
rationals = concat $ map rationalSumLowest[2..]

splitAtSeparator :: String -> [String]
splitAtSeparator [] = []
splitAtSeparator content = first : splitAtSeparator rest
 where
  first    = takeWhile(/= '\n') content
  rest     = dropWhile (== '\n') content


readInt :: [String] -> [Int]
readInt = map read

sumFile :: IO ()
sumFile = do
 nums <- readFile "input.txt"
 let a = splitAtSeparator nums
 let b = readInt a
 print (sum b)
 
