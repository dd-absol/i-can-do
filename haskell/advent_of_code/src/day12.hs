import Data.Char
import Data.List

import Data.Maybe
import Debug.Trace
import Data.Function


main = do
    file <- readFile "inputs/day12.txt"
    let input = lines file
        heightMap = parseHeightMap input
        endPos = charPos 'E' input
        startPos = charPos 'S' input

    print $ bfs' heightMap [Node {state = endPos, cost = 0}] []


type Position = (Int, Int)
data Direction = North | South | West | East deriving (Enum, Bounded, Ord, Eq, Show)

getActions = take 4 [North ..]

addPos :: Position -> Position -> Position
addPos (x1 ,y1) (x2, y2) = (x1 + x2, y1 + y2)

manDist :: Position -> Position -> Int
manDist (x1, y1) (x2, y2) = abs (x1 - x2) + abs (y1 - y2)

directionVector :: Direction -> Position
directionVector North  = (0, 1)
directionVector South  = (0, -1)
directionVector West = (-1, 0)
directionVector East = (1, 0)

parseHeightMap :: [[Char]] -> [[Int]]
parseHeightMap = 
    let aux 'E' = 'z'
        aux 'S' = 'a'
        aux c = c
    in map (map ((+ (-97)).ord.aux))

charPos :: Char -> [[Char]] -> Position
charPos c input =
    let line = find (c `elem`) input
    in (fromJust $ line >>= (`elemIndex`input), fromJust $ line >>= elemIndex c)

type HeightMap = [[Int]]

readHMap hmap (x, y) = hmap !! x !! y

getLegalActions :: HeightMap -> Position -> [Direction]
getLegalActions hmap pos =
    let currentHeight = readHMap hmap pos
    in filter ((>= -1).(currentHeight-).readHMap hmap.addPos pos.directionVector) $ filter ((\(x, y) -> x >= 0 && y >= 0 && x < length hmap && y < length (head hmap)).addPos pos.directionVector) getActions

getLegalActions' :: HeightMap -> Position -> [Direction]
getLegalActions' hmap pos =
    let currentHeight = readHMap hmap pos
    in filter ((<= 1).(currentHeight-).readHMap hmap.addPos pos.directionVector) $ filter ((\(x, y) -> x >= 0 && y >= 0 && x < length hmap && y < length (head hmap)).addPos pos.directionVector) getActions

data Node = Node {
    state :: Position,
    -- action :: Maybe Direction,
    cost :: Int
} deriving (Show)

successorState node action = Node {
    state = state node `addPos` directionVector action,
    -- action = Just action,
    cost = cost node + 1
}

bfs :: HeightMap -> Position -> [Node] -> [Position] -> Int
bfs _ _ [] _ = 1000
bfs hmap endPos (n:t) explored | trace ("myfun " ++ show n) False = undefined
bfs hmap endPos (n:t) explored
    | state n == endPos = cost n
    | state n `elem` explored = bfs hmap endPos t explored
    | otherwise = bfs hmap endPos (foldl (flip $ insertBy (compare `on` (\s -> (manDist endPos.state) s + cost s))) t (map (successorState n) (getLegalActions hmap $ state n))) $ state n:explored

bfs' :: HeightMap -> [Node] -> [Position] -> Int
bfs' _ [] _ = 1000
bfs' hmap (n:t) explored | trace ("myfun " ++ show n) False = undefined
bfs' hmap (n:t) explored
    | readHMap hmap (state n) == 0 = cost n
    | state n `elem` explored = bfs' hmap t explored
    | otherwise = bfs' hmap (foldl (flip $ insertBy (compare `on` (\s -> readHMap hmap (state s) + cost s))) t (map (successorState n) (getLegalActions' hmap $ state n))) $ state n:explored
    



