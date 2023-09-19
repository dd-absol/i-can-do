import qualified Data.Set as Set

main = do
    input <- lines <$> readFile "inputs/day15.txt"
    print $ day1 input

type Position = (Int, Int)
type Range = (Int, Int)

manDist :: Position -> Position -> Int
manDist (x1, y1) (x2, y2) = abs (x1 - x2) + abs (y1 - y2)

maxLength = 20

day1 input = 10
    -- let y = filter (maxLength == length $ nub $ concatMap (flip takenXrange.parseLine) input) [0..maxLength]

parseLine :: String -> (Position, Position)
parseLine l =
    let w = map (read.init.drop 2) $ words l
    in ((w !! 2, w !! 3), (w !! 8, read $ drop 2 $ words l !! 9))

circleFromTuple (sensor, beacon) = 
    let distCenter = manDist sensor
        maxDist = distCenter beacon
        xmax = abs (fst sensor) + maxDist
    in Set.fromList [(x, 2000000) | x <- [-xmax..xmax], distCenter (x, 2000000) <= maxDist]

takenXrange :: (Position, Position) -> Int -> Maybe [Int]
takenXrange (sensor@(xs, ys), beacon) y = 
    let distCenter = manDist sensor
        maxDist = distCenter beacon
        yDist = abs (y - ys)
        xDist = maxDist - yDist
    in if yDist > maxDist then Nothing
        else Just [xs - xDist .. xs + xDist]