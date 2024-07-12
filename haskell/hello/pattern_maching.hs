-- `defval` デフォルト値
-- `maybe` Maybe型
getValue defval maybe =
  case maybe of
    Nothing -> defval
    Just x  -> x

myabs x = if x < 0 then -x else x

main = do
  print (getValue 0 Nothing)   -- 出力: 0
  print (getValue 0 (Just 5))  -- 出力: 5

