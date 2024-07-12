main :: IO ()
main = do
  let add x y = x + y
    in
      -- print (add 3 5) -- 8
      print $ add 3 5 -- 8
