import Data.List

main = do
    input <- readFile "inputs/day6.txt"
    print $ coreDay6 (reverse $ take 14 input) (drop 14 input) 14

coreDay6 :: [Char] -> [Char] -> Int -> Int
coreDay6 _ [] index = index
coreDay6 acc rest index
    | length (nub acc) == 14 = index
    | otherwise = coreDay6 (head rest:init acc) (tail rest) (index + 1)