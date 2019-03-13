{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE DeriveGeneric #-}

import Data.List (find)
import Data.Aeson (ToJSON)
import GHC.Generics
import Web.Scotty

data User = User { userId :: Int, userName :: String } deriving (Show, Generic)
instance ToJSON User

bob :: User
bob = User { userId = 1, userName = "bob" }

alice :: User
alice = User { userId = 2, userName = "alice" }

allUsers :: [User]
allUsers = [bob, alice]

matchesId :: Int -> User -> Bool
matchesId id user = userId user == id

main :: IO ()
main = scotty 8080 $ do
    get "/hello/:name" $ do
        name <- param "name"
        text $ "Hello, " <> name <> "!"

    get "/users/:id" $ do
        id <- param "id"
        json $ find (matchesId id) allUsers
