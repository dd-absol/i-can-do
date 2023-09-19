
main = do
    file <- readFile "inputs/day10.txt"
    let input = lines file
    let l = map coreDay9' $ scanl coreDay9 (1, 1) $ concatMap (parseDay9.words) input
    mapM (putStrLn . (\n -> take 40 $ drop n l)) [0, 40, 80, 120, 160, 200]

type Instruction = Maybe Int

parseDay9 :: [[Char]] -> [Instruction]
parseDay9 words = case head words of
    "noop" -> [Nothing]
    "addx" -> [Nothing, Just $ read (words !! 1)]

coreDay9 :: (Int, Int) -> Instruction -> (Int, Int)
coreDay9 (acc, cycle) Nothing = (acc, cycle + 1)
coreDay9 (acc, cycle) (Just x) = (acc + x, cycle + 1)

coreDay9' :: (Int, Int) -> Char
coreDay9' (x, y) = if abs (x - mod (y - 1) 40) <= 1 then '#' else '.'