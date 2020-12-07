main  = do xs <- IOT.readFile "dataset.txt" <&> fmap (map unpack . T.lines) . T.splitOn "\n\n"
           print (sum $ fmap (length . foldr1 union) xs)
           print (sum $ fmap (length . foldr1 intersect) xs)