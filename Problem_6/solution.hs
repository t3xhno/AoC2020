import Data.List
import Data.List.Split

main  = do xs <- (fmap (lines) . splitOn "\n\n") <$> (readFile "dataset.txt")
           print (sum $ fmap (length . foldr1 union) xs)
           print (sum $ fmap (length . foldr1 intersect) xs)