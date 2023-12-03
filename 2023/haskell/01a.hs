import Data.Char
import Data.List (elemIndex)
import Data.Maybe (fromJust)

main :: IO ()
main = do
  fileContent <- readFile "01.txt"
  print $ solve1 $ lines fileContent

onlyCharDigit :: [Char] -> [Char]
onlyCharDigit xs = [n | n <- xs, isDigit n]

solve1 :: [String] -> Int
solve1 contents = sum $ map (\d -> digitToInt (head d) * 10 + digitToInt (last d)) digits
  where
    digits = map onlyCharDigit contents

wordDigits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

rs :: String -> String
rs xs = show $ fromJust $ elemIndex (take 5 xs) wordDigits

solve2 :: [String] -> Int
solve2 xs = 10