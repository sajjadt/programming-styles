import Data.List.Split
import Data.Char
import qualified Data.Set as Set  
import qualified Data.Map as Map  
import Data.List  
import Data.List (sortBy)
import Data.Function (on)
import System.Environment
import Text.Regex.Posix
import System.IO
import Control.Monad

import Data.Map (fromListWith, toList)


frequency :: (Ord a) => [a] -> [(a, Int)]
frequency xs = toList (fromListWith (+) [(x, 1) | x <- xs])

-------------------------------
--        I/O Functions
-------------------------------
showOp :: [(String, Int)] -> String
showOp [] = []
showOp (o:os) =  fst(o)   ++ " - " ++  show(snd(o)) ++   ('\n' : showOp os)

printOp :: [(String, Int)] -> IO ()
printOp xs = putStr $ showOp xs

main = do
    let n = 25
    [ft] <- getArgs
    ft_content <- readFile ft     
    fs_content <- readFile "../stop_words.txt"

    let non_alpha =['0'..'9'] ++ "?!-;'&*{}[]+#:(),. $_\n\""
    let t = splitOneOf non_alpha $ map toLower ft_content
    let input_words = filter ((> 1) . length) t 
    
    let stop_words = splitOneOf  "," fs_content
    let valid_words = filter (\x -> not $ elem x stop_words) input_words
    
    let freq = frequency valid_words 
    let worted_words = sortBy (flip compare `on` snd) freq
    
    printOp $ take n worted_words
    
