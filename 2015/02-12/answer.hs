import Data.List

-- Modified words function
split :: String -> [String]
split s = case dropWhile (=='x') s of 
    "" -> []
    s' -> w : split s''
        where (w, s'') = break (=='x') s'

-- Calculate surface
surface :: [String] -> Int
surface dims =
    let [l, w, h] = map read dims
        sides = [l*w, w*h, h*l]
    in 2 * (sum sides) + minimum sides

ribbon :: [String] -> Int
ribbon dims = 
    let t = map read dims
    in (product $ t) + (sum $ take 2 $ sort t)*2 

main = do
    contents <- readFile "input.txt"
    let partOne = sum $ map (surface . split) (lines contents)
    let partTwo = sum $ map (ribbon . split) (lines contents)
    putStrLn $ "Answer part one: " ++ show partOne 
    putStrLn $ "Answer part two: " ++ show partTwo
