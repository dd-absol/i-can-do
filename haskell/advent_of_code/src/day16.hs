import qualified Data.Map as Map
import Data.Maybe
import Data.List
import Debug.Trace

main = do
    input <- lines <$> readFile "inputs/day16.txt"
    let vs = Map.fromList $ map parseValve input
    print $ day16 vs $ originalState vs

type ValveId = String

data Valve = Valve {
    name :: ValveId,
    flow :: Int,
    tunnels :: [ValveId]
} deriving (Show, Eq)

type ValveSystem = Map.Map ValveId Valve

parseValve :: String -> (ValveId, Valve)
parseValve l = 
    let w = words l
        id = w !! 1
        flow = fst.head.reads $ drop 5 $ w !! 4
        tunnels = map (take 2) $ drop 9 w
    in (id, Valve {
        name = id,
        flow = flow,
        tunnels = tunnels
    })

-- part 1

data State = State {
    position :: ValveId,
    closedValves :: [ValveId],
    timeLeft :: Int,
    pressure :: Int
} deriving (Show)

originalState :: ValveSystem -> State
originalState vs = State {
    position = "AA",
    closedValves = Map.keys $ Map.filter ((>0).flow) vs,
    timeLeft = 30,
    pressure = 0
}

shortestPath :: (Ord a, Eq a) => Map.Map a [a] -> a -> a -> Int
shortestPath graph start finish = bfs (== finish) (fromJust.flip Map.lookup graph) [Node { state = start, cost = 0 }] []

data Node a = Node {
    state :: a,
    cost :: Int
}

bfs :: (Eq a) => (a -> Bool) -> (a -> [a]) -> [Node a] -> [a] -> Int
bfs _ _ [] _= error "terminal state not accessible"
bfs terminal successors (h:t) explored
    | terminal $ state h = cost h
    | state h `elem` explored = aux t explored
    | otherwise = 
        let nodes = map (\s -> Node { state = s, cost = cost h + 1 }) $ successors $ state h
        in aux (t ++ nodes) (state h:explored)
    where aux = bfs terminal successors

type Graph a = Map.Map a (Map.Map a Int)

graph :: (Eq a, Ord a) => Map.Map a [a] -> Graph a
graph g =
    let k = Map.keys g
    in Map.fromList $ map (\v1 -> (v1, Map.fromList $ map (\v2 -> (v2, shortestPath g v1 v2)) k)) k

dist :: (Eq a, Ord a) => Graph a -> a -> a -> Maybe Int
dist g v1 v2 = Map.lookup v1 g >>= Map.lookup v2

data Move = Move {
    finalPosition :: ValveId,
    duration :: Int,
    score :: Int
}

getMoves :: ValveSystem -> State -> [State]
getMoves vs s = filter ((>=0).timeLeft) $ map (move vs s) $ closedValves s

move :: ValveSystem -> State -> ValveId -> State
move vs s finish =
    let start = position s
        valve = fromJust $ Map.lookup finish vs
        graph = Map.map tunnels vs
        newTimeLeft = timeLeft s - (shortestPath graph start finish + 1)
    in State {
        position = finish,
        closedValves = delete finish $ closedValves s,
        timeLeft = newTimeLeft,
        pressure = pressure s + (newTimeLeft * flow valve) 
    }

day16 :: ValveSystem -> State -> Int
day16 vs s
    | trace (show $ timeLeft s) False = undefined
    | timeLeft s <= 0 || null (closedValves s) || null moves = pressure s
    | otherwise = maximum $ map (day16 vs) moves
    where moves = getMoves vs s

-- part 2

data State' = State' {
    humanPosition :: ValveId,
    closedValves' :: [ValveId],
    humanTimeLeft :: Int,
    pressure' :: Int,
    elephantPosition :: ValveId,
    elephantTimeLeft :: Int
} deriving (Show, Eq)

originalState' :: ValveSystem -> State'
originalState' vs = State' {
    humanPosition = "AA",
    elephantPosition = "AA",
    closedValves' = Map.keys $ Map.filter ((>0).flow) vs,
    humanTimeLeft = 26,
    pressure' = 0,
    elephantTimeLeft = 26
}

moveHuman :: ValveSystem -> State' -> ValveId -> State'
moveHuman vs s finish =
    let start = humanPosition s
        valve = fromJust $ Map.lookup finish vs
        graph = Map.map tunnels vs
        newTimeLeft = humanTimeLeft s - (shortestPath graph start finish + 1)
    in s {
        humanPosition = finish,
        closedValves' = delete finish $ closedValves' s,
        humanTimeLeft = newTimeLeft,
        pressure' = pressure' s + (newTimeLeft * flow valve) 
    }

moveElephant :: ValveSystem -> State' -> ValveId -> State'
moveElephant vs s finish =
    let start = elephantPosition s
        valve = fromJust $ Map.lookup finish vs
        graph = Map.map tunnels vs
        newTimeLeft = elephantTimeLeft s - (shortestPath graph start finish + 1)
    in s {
        elephantPosition = finish,
        closedValves' = delete finish $ closedValves' s,
        elephantTimeLeft = newTimeLeft,
        pressure' = pressure' s + (newTimeLeft * flow valve) 
    }

getMoves' :: ValveSystem -> State' -> [State']
getMoves' vs s = filter ((>=0).elephantTimeLeft) (map (moveElephant vs s) $ closedValves' s) ++ filter ((>=0).humanTimeLeft) (map (moveHuman vs s) $ closedValves' s)

day16' :: ValveSystem -> State' -> Int
day16' vs s
    | trace (show (humanTimeLeft s) ++ " " ++ show (elephantTimeLeft s)) False = undefined
    | humanTimeLeft s <= 0 && elephantTimeLeft s <= 0 || null (closedValves' s) || null moves = pressure' s
    | otherwise = maximum $ map (day16' vs) moves
    where moves = getMoves' vs s