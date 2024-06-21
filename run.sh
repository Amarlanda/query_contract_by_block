#!/bin/bash

reset ; clear 

# Function to check if .env file exists
function check_env_file() {
    if [ ! -f .env ]; then
        echo ".env file does not exist!"
        exit 1
    fi
    echo ".env file exists."
}

check_env_file

cargo run