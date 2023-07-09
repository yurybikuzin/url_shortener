#!/usr/bin/env bash
set -e
cmd=(
    curl 
    'https://u2h.ru/url_shortener_back/about' 
)
echo "${cmd[@]}"
"${cmd[@]}" 
