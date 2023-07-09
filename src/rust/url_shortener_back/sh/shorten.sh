#!/usr/bin/env bash
set -e
cmd=(
    curl 
    -i
    -w "\n" 
    -X POST 
    'https://u2h.ru/url_shortener_back/shorten' 
    -H 'Content-Type: application/json'
    -d '{ "url": "https://github.com/yurybikuzin/url_shortener" }'
)
echo "${cmd[@]}"
"${cmd[@]}" 
