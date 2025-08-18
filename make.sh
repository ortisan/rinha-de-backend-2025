#!/bin/sh

#https://stackoverflow.com/questions/2870992/automatic-exit-from-bash-shell-script-on-error
abort()
{
    echo >&2 '
***************
*** ABORTED ***
***************
'
    echo "An error occurred. Exiting..." >&2
    exit 1
}

trap 'abort' 0

set -e

YOUR_DOCKER_HUB_USER=marceloorsa

docker build -t $YOUR_DOCKER_HUB_USER/rinha-rust:1.0.0 -f Dockerfile .
docker push $YOUR_DOCKER_HUB_USER/rinha-rust:1.0.0

trap : 0

echo >&2 '
************
*** DONE ***
************
'