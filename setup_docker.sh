#!/usr/bin/env bash

source .env

docker build -t autogoblin --build-arg DISCORD_TOKEN=$DISCORD_TOKEN .

echo "Creating autogoblin network, please make sure you run all your other containers in this network."

docker network create autogoblin_network

echo "Setup complete!"