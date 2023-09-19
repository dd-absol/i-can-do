main = do
    contents <- readFile "inputs/day5.txt"
    let input = lines contents
    let dock = parseDock $ init $ takeWhile (/="") input
    let instr = concatMap parseInstr $ tail $ dropWhile (/="") input
    print $ map head $ filter (/= []) $ foldl move dock instr

type Crate = Char
type Dock = [[Crate]]

parseDock :: [[Char]] -> Dock
parseDock l = 
    let
        aux line = zipWith (parseStack line) [0..]
    in
        foldr aux (replicate 9 []) l

parseStack :: [Crate] -> Int -> [Char] -> [Crate]
parseStack line index stack
    | c == ' ' = stack
    | otherwise = c:stack
    where
        c = line !! (1 + 4*index)

parseInstr :: [Char] -> [(Int, Int)]
parseInstr s =
    let
        w = words s
        n = read $ w !! 1
        start = read $ w !! 3
        finish = read $ w !! 5
    in
        if start /= 1 && finish /= 1 then replicate n (start - 1, 0) ++ replicate n (0, finish - 1)
        else if start /= 2 && finish /= 2 then replicate n (start - 1, 1) ++ replicate n (1, finish - 1)
        else replicate n (start - 1, 2) ++ replicate n (2, finish - 1)

move :: Dock -> (Int, Int) -> Dock
move dock (start, finish) =
    let 
        crate = head $ dock !! start
        aux index l
            | index == finish = crate : l
            | index == start = tail l
            | otherwise = l
    in
        zipWith aux [0 .. ] dock
         


