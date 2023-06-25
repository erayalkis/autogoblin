
# Only use this for development purposes! The docker image should be used for *actually* hosting the app

source .env

DISCORD_TOKEN=$DISCORD_TOKEN cargo run 
