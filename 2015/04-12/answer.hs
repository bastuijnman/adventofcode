import Data.Hash.MD5
import Data.List
import Control.Parallel.Strategies

key :: Int -> String 
key i = md5s (Str ("INSERT_KEY_HERE" ++ show i))

main = do
    -- Just brute-force it -.-
   putStrLn $ "Answer part one: " ++ show (findIndex (== "00000") $ map (take 5) $ map key [0..])
   putStrLn $ "Answer part one: " ++ show (findIndex (== "000000") $ map (take 6) $ map key [0..])
