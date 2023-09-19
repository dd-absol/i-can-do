import Data.List
import Data.Semigroup (diff)

main = do
    file <- readFile "inputs/day9.txt"
    let input = concatMap parseDay9 $ lines file
    print $ length $ nub $ map (!!9) $ scanl coreDay9 (replicate 10 (0, 0)) input

directionFromChar :: Char -> Direction
directionFromChar 'U'  = (0, 1)
directionFromChar 'D'  = (0, -1)
directionFromChar 'L' = (-1, 0)
directionFromChar 'R' = (1, 0)

type Position = (Int, Int)
type Direction = (Int, Int)

addPos :: Position -> Position -> Position
addPos (x1 ,y1) (x2, y2) = (x1 + x2, y1 + y2)

diffPos :: Position -> Position -> Position
diffPos (x1 ,y1) (x2, y2) = (x1 - x2, y1 - y2)

type Knot = (Position, Position)
type Rope = [Position]

checkRope :: Knot -> Bool
checkRope (head, tail) =
    let
        (x, y) = diffPos head tail
    in
        abs x <= 1 && abs y <= 1

normalize :: Position -> Position
normalize (x, y) =
    let
        newx = if abs x > 1 then quot x $ abs x else x
        newy = if abs y > 1 then quot y $ abs y else y
    in
        (newx, newy)

parseDay9 :: [Char] -> [Direction]
parseDay9 line =
    let
        w = words line
        dir = directionFromChar $ head $ head w
        n = read $ head $ tail w
    in
        replicate n dir

coreDay9 :: Rope -> Direction -> Rope
coreDay9 [] _ = []
coreDay9 [h] dir = [addPos h dir]
coreDay9 (h:t) dir@(xd, yd) = 
    let
        ropeTail = head t
        newHead = addPos h dir
        potentialDirection = normalize $ diffPos newHead ropeTail
    in
        if checkRope (newHead, ropeTail) then
            newHead:t
        else
            newHead:coreDay9 t potentialDirection