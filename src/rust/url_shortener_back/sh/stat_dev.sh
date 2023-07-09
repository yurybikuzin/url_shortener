#!/usr/bin/env bash
set -e
cmd=(
    curl 
    -i
    -w "\n" 
    -X GET 
    'https://u2h.ru/dev/url_shortener_back/stat/N:FY5' 
)
echo "${cmd[@]}"
"${cmd[@]}" 
