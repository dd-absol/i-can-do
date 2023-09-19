import qualified Data.Set as Set
import Data.Maybe
import Debug.Trace

main = do
    file <- readFile "inputs/day14.txt"
    let input = map parseLine $ lines file
        initialCave = foldl Set.union Set.empty $ map (\l -> foldl fromLine Set.empty $ zip l $ drop 1 l) input
        bottom = maximum $ map snd $ Set.toList initialCave
    print input
    print initialCave
    print bottom
    print $ length $ takeWhile isJust $ iterate (>>= dropSand' (bottom + 2) (500, 0)) $ return initialCave 
    
type Position = (Int, Int)

parseLine :: [Char] -> [Position]
parseLine = map (parseTuple.snd).filter (even.fst).zip [0..].words

parseTuple :: [Char] -> Position
parseTuple s = 
    let t = break (== ',') s
    in ((read.fst) t, (read.tail.snd) t)

type Cave = Set.Set Position

fromLine :: Cave -> (Position, Position) -> Cave
fromLine cave ((x1, y1), (x2, y2))
    | x1 == x2 = cave `Set.union` Set.fromList [(x1, y)| y <- [min y1 y2..max y1 y2]]
    | y1 == y2 = cave `Set.union` Set.fromList [(x, y1)| x <- [min x1 x2.. max x1 x2]]

dropSand :: Int -> Position -> Cave -> Maybe Cave
dropSand bottom (x, y) cave
    | y > bottom = Nothing
    | not $ Set.member (x, y + 1) cave = dropSand bottom (x, y + 1) cave
    | not $ Set.member (x - 1, y + 1) cave = dropSand bottom (x - 1, y + 1) cave
    | not $ Set.member (x + 1, y + 1) cave = dropSand bottom (x + 1, y + 1) cave 
    | otherwise = Just $ Set.insert (x, y) cave

dropSand' :: Int -> Position -> Cave -> Maybe Cave
dropSand' bottom (x, y) cave
    | y + 1 == bottom = Just $ Set.insert (x, y) cave
    | not $ Set.member (x, y + 1) cave = dropSand' bottom (x, y + 1) cave
    | not $ Set.member (x - 1, y + 1) cave = dropSand' bottom (x - 1, y + 1) cave
    | not $ Set.member (x + 1, y + 1) cave = dropSand' bottom (x + 1, y + 1) cave
    | y == 0 = Nothing
    | otherwise = Just $ Set.insert (x, y) cave

