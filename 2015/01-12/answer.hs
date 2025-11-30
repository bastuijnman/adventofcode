import Data.List (findIndex)

convert :: Char -> Int 
convert '(' = 1
convert ')' = -1
convert _ = 0

main = do
    contents <- readFile "input.txt"
    let nums = map convert contents
    putStrLn("Answer part 1: " ++ (show (sum nums)))
    
    case findIndex (== -1) (scanl1 (+) nums) of 
        Just index -> putStrLn $ "Answer part 2: " ++ show (index + 1)
        Nothing -> putStrLn $ "No answer for part 2"
