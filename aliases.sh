apiroot=localhost:8000/api

apy () {
    curl $apiroot/pretty
}

ast () {
    curl $apiroot/start/$1
}

asp () {
    curl $apiroot/stopall
}