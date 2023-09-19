import qualified Data.Map as M
import Data.Maybe
import Distribution.Simple.Utils (currentDir)
import Data.ByteString (sort)

main = do
    contents <- readFile "inputs/day7.txt"
    let input = lines contents
    print $ coreDay7 $ auxDay7 input M.empty []

data Item = File Int | Dir [Char] deriving (Show)
type System = M.Map [Char] [Item]

size :: System -> Item -> Int
size _ (File size) = size
size sys (Dir id) = sum $ map (size sys) (fromJust $ M.lookup id sys)

auxDay7 :: [[Char]] -> System -> [[Char]] -> System
auxDay7 [] sys _ = sys
auxDay7 (h:t) sys currentDir =
    let
        id = drop 5 h
        (ls, rest) = span (\line -> line == "$ cd .." || take 5 line /= "$ cd ") t
        aux (dir, cd) line
            | line == "$ cd .." = (dir, tail cd)
            | commandType == "$" = (dir, cd)
            | commandType == "dir" = (Dir (path ++ value):dir, cd)
            | otherwise = (File (read commandType):dir, cd)
            where
                commandType = head $ words line
                value = words line !! 1
                path = concat $ reverse cd
        (res, newcd) = foldl aux ([], id:currentDir) ls
    in
        auxDay7 rest (M.insert (concat (reverse currentDir) ++ id) res sys) newcd

coreDay7 sys = 
    let
        unusedSpace = (70000000 - (sum.map (size sys) $ fromJust $ M.lookup "/" sys))
    in
        minimum $ filter (>= 30000000 - unusedSpace) $ map (sum.map (size sys)) $ M.elems sys
