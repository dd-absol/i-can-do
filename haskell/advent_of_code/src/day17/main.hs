main = do
    input <- readFile "inputs/day17.txt"
    print input

type Direction = Char

jet :: Direction -> Int -> Int
jet dir x
    | dir == '>' = min (x + 1) 6
    | dir == '<' = max (x - 1) 0

 
