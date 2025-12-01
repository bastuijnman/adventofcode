import Data.List

vwls :: String -> Bool
vwls i = length (intersect i "aeiou") >= 3 

grp :: String -> Bool
grp i = length (filter ((>= 2) . length) (group i)) > 0

prs :: String -> Bool
prs i = length (intersect ["ab", "cd", "pq", "xy"] (zipWith (\a b -> [a,b]) i (tail i))) == 0

main = do
    contents <- readFile "input.txt"
    putStrLn $ "Answer part one: " ++ show (length $ filter prs $ filter grp $ filter vwls $ lines contents)
