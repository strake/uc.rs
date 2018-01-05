import Data.ByteString.Lazy as BS
import Data.Char

main :: IO ()
main = BS.putStr . pack $ fromIntegral . fromEnum . generalCategory <$> ['\0'..]
