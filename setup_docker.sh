#!/usr/bin/env bash

source .env

docker build -t autogoblin --build-arg DISCORD_TOKEN=$DISCORD_TOKEN .