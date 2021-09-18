import Data.Maybe

addPrevValue :: [Integer] -> [Integer]
addPrevValue prev = [1] ++ map (uncurry (+)) (zip prev (tail prev)) ++ [1]

pascal :: Integer -> [Integer]
pascal 0 = [1]
pascal n = combineList (n)
 where combineList n = addPrevValue (pascal (n-1))
  
addPair :: (Integer, Integer) -> Integer
addPair = (uncurry (+))

withoutZeros :: (Eq a, Num a) => [a] -> [a]
withoutZeros = filter(/= 0)

searchList :: Eq a => a -> [a] -> Int -> Maybe Int
searchList a [] _ = Nothing
searchList a (x:xs) index
 | a == x    = Just index
 | otherwise = searchList a xs (index+1)
 
findElt :: Eq a => a -> [a] -> Maybe Int
findElt a [] = Nothing
findElt a xs = searchList a xs 0
