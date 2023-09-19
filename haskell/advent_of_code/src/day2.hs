main = do
    input <- getContents
    print $ core_day2 $ aux_day2 $ lines input

aux_day2 :: [[Char]] -> [(Char, Char)]
aux_day2 l = map str_to_tuple' l

str_to_tuple :: [Char] -> (Char, Char)
str_to_tuple s = (head s, x_to_a (last s))

str_to_tuple' :: [Char] -> (Char, Char)
str_to_tuple' s = (head s, x_to_a' (head s, last s))

x_to_a :: Char -> Char
x_to_a 'X' = 'A'
x_to_a 'Y' = 'B'
x_to_a 'Z' = 'C'
x_to_a c = ' '

x_to_a' :: (Char, Char) -> Char
x_to_a' (c, 'X') = next $ next c
x_to_a' (c, 'Y') = c
x_to_a' (c, 'Z') = next c
x_to_a' (c, d) = ' '

next :: Char -> Char
next 'A' = 'B'
next 'B' = 'C'
next 'C' = 'A'
next c = c

core_day2 :: [(Char, Char)] -> Int
core_day2 l = sum (map evaluate_matchup l) + sum (map (value) (map snd l))

evaluate_matchup :: (Char, Char) -> Int
evaluate_matchup ('A', 'B') = 6
evaluate_matchup ('B', 'C') = 6
evaluate_matchup ('C', 'A') = 6
evaluate_matchup (a, x)
    | a == x = 3
    | otherwise = 0

value :: Char -> Int
value 'A' = 1
value 'B' = 2
value 'C' = 3
value c = 0