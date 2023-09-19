main = do
    contents <- readFile "inputs/day4.txt"
    let input = lines contents
    print $ length $ filter core_day4 $ map aux_day4 input

aux_day4 :: [Char] -> ((Int, Int), (Int, Int))
aux_day4 s = 
    let
        (a, b) = span (/=',') s
        ((c, d), (e, f)) = (span (/='-') a, span (/='-') $ tail b)
    in
        ((read c, read $ tail d), (read e, read $ tail f))


core_day4 :: ((Int, Int), (Int, Int)) -> Bool
core_day4 ((a, b), (c, d)) = 
    ((a <= c) && (d <= b)) 
    || ((c <= a) && (b <= d))
    || ((a <= d) && (c <= b))
    || ((c <= b) && (a <= d))



