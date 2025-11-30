import Data.List

convert :: Char -> Int
convert '>' = 1
convert '<' = -1
convert '^' = -10000 -- TODO: rework magic number
convert 'v' = 10000 
convert _ = 0

main = do
    contents <- readFile "input.txt"
    putStrLn $ "Answer part one: " ++ show (length $ nub $ scanl (+) 0 $ map convert contents)

    let santa = scanl (+) 0 $ map snd $ filter (even . fst) $ zip [0..] $ map convert contents
    let robo = scanl (+) 0 $ map snd $ filter (odd. fst) $ zip [0..] $ map convert contents
    putStrLn $ "Answer part two: " ++ show (length $ nub (santa ++ robo))
