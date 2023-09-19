import Data.List

main = do
    input <- getContents
    print (core_day1' (aux_day1 (lines input) []))

aux_day1 :: [[Char]] -> [Int] -> [[Int]]
aux_day1 [] res = [res]
aux_day1 (h:t) res
    | null h = res:aux_day1 t []
    | otherwise = aux_day1 t (read h:res)

core_day1 :: [[Int]] -> Int
core_day1 [] = 0
core_day1 (h:t)
    | head_sum > max_tail_sum = head_sum
    | otherwise = max_tail_sum
    where 
        head_sum = sum h
        max_tail_sum = core_day1 t

-- that's haskell baby
core_day1' :: [[Int]] -> Int
core_day1' l = sum $ take 3 $ reverse $ sort $ map sum l

    

