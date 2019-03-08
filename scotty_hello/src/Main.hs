{-# LANGUAGE OverloadedStrings #-}

import Web.Scotty

import Data.Monoid (mconcat)

main = scotty 8080 $
    get "/hello/:name" $ do
        name <- param "name"
        text $ mconcat ["Hello, ", name, "!"]