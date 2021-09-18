import RainbowAssign
import qualified Data.Map as Map
import Data.Maybe as Maybe

--type Hash = Int32
--type Passwd = String

pwLength, nLetters, width, height :: Int
pwLength = 8            -- length of each password
nLetters = 5            -- number of letters to use in passwords: 5 -> a-e
width = 40              -- length of each chain in the table
height = 1000           -- number of "rows" in the table
filename = "table.txt"  -- filename to store the table

--Hashing & Reducing Part
--Base Conversion Method: Recursively store n mod nLetters, n = n/nLetters
pwReduce :: Hash -> Passwd
pwReduce hashValue = reverse(map toLetter(take pwLength(baseConversion (fromEnum hashValue))))
 where baseConversion n = [n `mod` nLetters] ++ baseConversion (n `div` nLetters)
 
--Building Table Part
finalValue :: Int -> [Passwd] -> [Hash] 
finalValue 0 passwords = map pwHash passwords
finalValue n passwords = finalValue (n-1) (map pwReduce (map pwHash passwords)) 

rainbowTable :: Int -> [Passwd] -> Map.Map Hash Passwd
rainbowTable tableWidth passwdList = Map.fromList(zip (finalValue tableWidth passwdList) passwdList)

--Generate Table
generateTable :: IO ()
generateTable = do
 table <- buildTable rainbowTable nLetters pwLength width height
 writeTable table filename

--Read Table
--If item found returns Just "password_letters", otherwise return "Nothing"
test1 :: IO (Maybe Passwd)
test1 = do
 table <- readTable filename
 return (Map.lookup (-1404040874) table)

--Reversing Hashes Part
--Search Table to find out if hash value is a key (Must check every row of the table)
searchTable :: Map.Map Hash Passwd -> Int -> Hash -> Maybe Passwd -> Maybe Passwd
searchTable table t_width hash result
 | t_width < 0                      = result
 | Map.lookup hash table == Nothing = searchTable table (t_width-1) (pwHash (pwReduce hash)) result
 | otherwise                        = searchTable table (t_width-1) (pwHash (pwReduce hash)) (Map.lookup hash table)

--If key-hash not found -> return nothing
--Otherwise -> iterate pwHash and pwReduce and return the password
iteratePassword :: Maybe Passwd -> Int -> Hash -> Maybe Passwd
iteratePassword Nothing _  _ = Nothing
iteratePassword (Just key_hash) t_width given_hash
 | t_width < 0                     = Nothing
 | (pwHash key_hash) == given_hash = Just key_hash
 | otherwise                       = iteratePassword (Just (pwReduce (pwHash key_hash))) (t_width-1) given_hash

--Order of finding password: 1.Search table for key-hash 2.Iterate key-hash to find the actual password
findPassword :: Map.Map Hash Passwd -> Int -> Hash -> Maybe Passwd
findPassword table t_width hash = iteratePassword table_key_hash t_width hash
 where table_key_hash = searchTable table t_width hash Nothing

--Test2
test2 :: Int -> IO ([Passwd], Int)
test2 n = do
  table <- readTable filename
  pws <- randomPasswords nLetters pwLength n
  let hs = map pwHash pws
  let result = Maybe.mapMaybe (findPassword table width) hs
  return (result, length result)
  
--Compile Code
main :: IO ()
main = do
  generateTable
  res <- test2 10000
  print res