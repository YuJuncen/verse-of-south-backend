import System.IO
import System.Environment
import Control.Monad.Trans.Writer.Strict
import Control.Monad.IO.Class
import Control.Exception(bracket)
import Data.Char

safeHead [] = Nothing
safeHead (x:_) = Just x
hWriteAsEnv handler name val = hPutStrLn handler $ name ++ "=" ++ val
fileName = do
    name <- safeHead <$> getArgs
    return $ maybe ".env" id name

hGetPassword handler = bracket (hSetEcho handler False) (const (hSetEcho handler True)) (\_ -> getLine)
getPassword = hGetPassword stdin

defaultString def s = if null s then def else s

getDataBaseUrl :: IO String
getDataBaseUrl = execWriterT $ do
    liftIO $ putStrLn "Now, we only support postgresql..."
    tell "postgres://"
    liftIO $ putStrLn "Please, Input the database user: (default: postgre)"
    defaultString "postgre" <$> liftIO getLine >>= tell
    tell ":"
    liftIO $ putStrLn "Please, Input Your Password..."
    defaultString (error "You must provide database password!") <$> liftIO getPassword >>= tell
    tell "@"
    liftIO $ putStrLn "Please, tell me where the database is? (default: localhost)"
    defaultString "localhost" <$> liftIO getLine >>= tell
    tell "/"
    liftIO $ putStrLn "Please, tell me which database(or, scheme) you want to use? (default: postgre)"
    defaultString "postgre" <$> liftIO getLine >>= tell

promptChoiceWithDefault def str = str ++ if def then "(Y/n)" else "(y/N)"

choiceByUser def prompt action other = do
    putStr $ promptChoiceWithDefault def prompt
    let loop = do
        a <- getChar
        case toLower a of 
            'y' -> action
            'n' -> other
            _ -> do
                getLine
                putStrLn "Could you please input Y or N..."
                loop
    loop

whenUserSayYes = \def prompt action -> choiceByUser def prompt action (return ())
whenUserSayYesWithLust = whenUserSayYes True
whenUserSayYesWithDepress = whenUserSayYes False
main = do
    name <- fileName
    withFile name WriteMode $ \f -> do
        putStrLn "Database url?"
        getDataBaseUrl >>= hWriteAsEnv f "DATABASE_URL"
        "Do you want to enable recaptcha?" `whenUserSayYesWithLust` do
            putStrLn "Please, give me your recaptch V2 secret..."
            getPassword >>= hWriteAsEnv f "RECAPTCHA_SECRET"
        "Do you want to enable CORS, usually, it will be just use on swarm or development." `whenUserSayYesWithDepress` do
            hWriteAsEnv f "ENABLE_CORS" "true"

        