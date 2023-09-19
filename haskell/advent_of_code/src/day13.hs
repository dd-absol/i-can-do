import Data.Char
import Data.List
import Data.Maybe
import Data.Function

main = do
    file <- readFile "inputs/day13.txt"
    let input = lines file
        sortedList = sortBy (comparePackets `on` snd) $ zip [0::Int ..] $ parseInput' input
    print $ head sortedList
    print $ filter ((\c -> c == 0 || c == 1).fst.snd) $ zip [1..] sortedList

data PacketData = PList [PacketData] | PInt Int deriving (Show, Eq)

parseInput :: [[Char]] -> [(PacketData, PacketData)]
parseInput [] = []
parseInput [l1, l2] = [(parseLine (tail l1) [], parseLine (tail l2) [])]
parseInput (l1:l2:t) = (parseLine (tail l1) [], parseLine (tail l2) []):parseInput (tail t)

parseInput' :: [[Char]] -> [PacketData]
parseInput' [] = []
parseInput' (h:t)
    | h == "" = parseInput' t
    | otherwise = parseLine (tail h) []:parseInput' t

parseLine :: [Char] -> [PacketData] -> PacketData
parseLine [] acc = PList $ reverse acc
parseLine (h:t) acc
    | h == '[' = 
        let (plist, rest) = spanBracket t [] 0 
        in parseLine rest (parseLine plist []:acc)
    | isNumber h = 
        let (number, rest) = break (\c -> c == ',' || c == ']') (h:t)
        in parseLine rest (PInt (read number):acc)
    | otherwise = parseLine t acc

spanBracket :: [Char] -> [Char] -> Int -> ([Char], [Char])
spanBracket [] acc depth = (acc, [])
spanBracket (h:t) acc depth
    | h == ']' && depth == 0 = (reverse acc, t)
    | h == '[' = spanBracket t (h:acc) (depth + 1)
    | h == ']' = spanBracket t (h:acc) (depth - 1)
    | otherwise = spanBracket t (h:acc) depth

comparePackets :: PacketData -> PacketData -> Ordering
comparePackets (PInt x) (PInt y) = compare x y
comparePackets (PList []) (PList []) = EQ
comparePackets (PList []) _ = LT
comparePackets _ (PList []) = GT
comparePackets (PList (h1:t1)) (PList (h2:t2)) = case comparePackets h1 h2 of
    EQ -> comparePackets (PList t1) (PList t2)
    b -> b
comparePackets (PInt x) pl = comparePackets (PList [PInt x]) pl
comparePackets pl (PInt x) = comparePackets pl (PList [PInt x])




