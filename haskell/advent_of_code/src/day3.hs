import Data.List
import Data.Char

main = do
    input <- readFile "inputs/day3.txt"
    let inputLines = lines input
    print $ coreDay3 inputLines

coreDay3 :: [[Char]] -> Int
coreDay3 l = sum $ map auxDay3 $ prepDay3 l

prepDay3 :: [[Char]] -> [([Char], [Char], [Char])]
prepDay3 [] = []
prepDay3 (a:b:c:t) = (a, b, c) : prepDay3 t
prepDay3 l = []


auxDay3 :: ([Char], [Char], [Char])-> Int
auxDay3 (a, b, c) = 
    score $ head $ intersect a $ intersect b c

score :: Char -> Int
score c
    | isUpper c = ord c - 64 + 26
    | otherwise = ord c - 96






