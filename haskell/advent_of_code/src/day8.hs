import Data.Char (digitToInt)

main = do
    file <- readFile "inputs/day8.txt"
    let heightMap = map (map digitToInt) $ lines file
    let input = zipWith (\x l -> map (\(y, _) -> (x, y)) l) [0..] $ map (zip [0..]) heightMap
    print $ maximum $ map (maximum.map (check heightMap)) input

type Height = Int

check :: [[Height]] -> (Int, Int) -> Int
check treeMap (a, b) =
    let
        height = treeMap !! a !! b
        xMax = length treeMap
        yMax = length treeMap
        west = reverse [treeMap !! a !! y | y <- [0..b], y < b]
        east = [treeMap !! a !! y | y <- [b + 1..yMax], y < yMax]
        south = [treeMap !! x !! b | x <- [a + 1 .. xMax], x < xMax]
        north = reverse [treeMap !! x !! b | x <- [0..a], x < a]
    in
        product $ map (aux height) [west, east, north, south]

aux height [] = 0
aux height (h:t)
    | h < height = 1 + aux height t
    | otherwise = 1

         

