import Data.List

main = do
    file <- readFile "inputs/day11.txt"
    let input = lines file
    let monkeys = map (parseMonkey.reverse) $ parseInput input []
    print $ product $ take 2 $ reverse $ sort $ map inspectionsCount $ iterate monkeyRound monkeys !! 10000
    -- print $ map itemStack $ iterate monkeyRound monkeys !! 10000


data Monkey = Monkey {
    monkeyId :: Int,
    itemStack :: [Integer],
    operation :: Integer -> Integer,
    test :: Integer -> Int,
    divider :: Integer,
    inspectionsCount :: Integer
}

operatorFromChar :: Char -> Integer -> Integer -> Integer
operatorFromChar '*' = (*)
operatorFromChar '+' = (+)

testFrom :: Integer -> Int -> Int -> Integer -> Int
testFrom divider monkeyTrue monkeyFalse x = if mod x divider == 0 then monkeyTrue else monkeyFalse

operationFrom :: (Integer -> Integer -> Integer) -> Maybe Integer -> Integer -> Integer
operationFrom operator (Just right) x = operator right x
operationFrom operator Nothing x = operator x x

parseInput :: [[Char]] -> [[Char]] -> [[[Char]]]
parseInput [] res = [res]
parseInput (h:t) res
    | null h = res:parseInput t []
    | otherwise = parseInput t (h:res)

parseMonkey :: [[Char]] -> Monkey
parseMonkey input =
    let
        monkeyId = read $ init $ last $ words $ head input
        itemStack = map (read.take 2) $ drop 2 $ words $ input !! 1
        operator = operatorFromChar $ head (words (input !! 2) !! 4)
        right = case last $ words (input !! 2) of
            "old" -> Nothing
            x -> Just $ read x
        divider = read $ last $ words (input !! 3)
        monkeyTrue = read $ last $ words (input !! 4)
        monkeyFalse = read $ last $ words (input !! 5)
    in
        Monkey {
            monkeyId = monkeyId,
            itemStack = itemStack,
            operation = operationFrom operator right,
            test = testFrom divider monkeyTrue monkeyFalse,
            divider = divider,
            inspectionsCount = 0
        }


monkeyRound :: [Monkey] -> [Monkey]
monkeyRound initialMonkeys = foldl turn initialMonkeys [0..length initialMonkeys - 1]

turn :: [Monkey] -> Int -> [Monkey]
turn monkeys throwerId = foldl (throw throwerId) monkeys (itemStack (monkeys !! throwerId))

throw :: Int -> [Monkey] -> Integer -> [Monkey]
throw throwerId monkeys initialWorryLevel =
    let
        thrower = monkeys !! throwerId
        worryLevel = newWorryLevel monkeys thrower initialWorryLevel
        receiverId = test thrower worryLevel
    in
        map (updateStacks throwerId receiverId worryLevel) monkeys

updateStacks :: Int -> Int -> Integer -> Monkey -> Monkey
updateStacks throwerId receiverId worryLevel monkey
    | throwerId == receiverId = incrMonkey $ replaceStack monkey $ tail (itemStack monkey ++ [worryLevel])
    | monkeyId monkey == throwerId = incrMonkey $ replaceStack monkey $ tail $ itemStack monkey
    | monkeyId monkey == receiverId = replaceStack monkey (itemStack monkey ++ [worryLevel])
    | otherwise = monkey

replaceStack monkey stack = Monkey {
    monkeyId = monkeyId monkey,
    itemStack = stack,
    operation = operation monkey,
    test = test monkey,
    divider = divider monkey,
    inspectionsCount = inspectionsCount monkey
}

incrMonkey monkey = Monkey {
    monkeyId = monkeyId monkey,
    itemStack = itemStack monkey,
    operation = operation monkey,
    test = test monkey,
    divider = divider monkey,
    inspectionsCount = inspectionsCount monkey + 1
}

newWorryLevel :: [Monkey] -> Monkey -> Integer -> Integer
newWorryLevel monkeys monkey worryLevel = operation monkey worryLevel `mod` product (map divider monkeys)
