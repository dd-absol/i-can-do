module Search
( State
, Node
, search
) where

import qualified Data.Set as S
import Data.List (insertBy)
import Data.Function (on)

class Move m where
    cost :: m -> c

class (Eq s, Ord s) => State s where
    terminal :: s -> Bool
    moves :: s -> [m]
    successor :: s -> m -> s
    heuristic :: s -> c

data Node s m c = Node {
    state :: s,
    totalCost :: c,
    move :: Maybe m,
    parent :: Maybe (Node s m c)
}

search :: (State s, Move m, Num c, Ord c) => S.Set s -> [Node s m c] -> Maybe (Node s m c)
search _ [] = Nothing
search explored (h:t)
    | terminal $ state h = Just h
    | state h `S.member` explored = search explored t
    | otherwise = search (S.insert (state h) explored) 
        $ foldl (flip $ insertBy (compare `on` (\n -> totalCost n + heuristic (state n)))) t 
        $ map (successorNode h) (moves $ state h)

successorNode :: (State s, Move m, Num c) => Node s m c -> m -> Node s m c
successorNode node m = node {
    state = successor (state node) m,
    totalCost = totalCost node + cost m,
    move = Just m,
    parent = Just node
}




