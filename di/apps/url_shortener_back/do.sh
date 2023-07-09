#!/usr/bin/env bash
set -e

source "$(realpath "$(dirname "${BASH_SOURCE[0]}")/../../sh/core/do.common.sh")"

src_rust_dir="$proj_dir/src/rust"
target="x86_64-unknown-linux-musl"
exe="target/$target/release/./$app"
dependencies_for_deploy=(
    "$src_rust_dir/$exe" 
    "$src_rust_dir/$app/./.env"
    "$src_rust_dir/$app/./settings.toml"
)

case $cmd in
    build )
        pushd "$src_rust_dir" 
        x rustup target add "$target"
        x sudo apt install -y musl-tools
        x cargo build --release --target $target -p $app 
        x ls -lah $exe 
        popd 
    ;;
    get-dependencies-for-deploy )
        echo "${dependencies_for_deploy[@]}"
    ;;
    deploy )
        [[ $dry_run ]] || set -e
        x $dry_run $src_rust_dir/$exe -w "$src_rust_dir/$app" -t 
        x $dry_run ssh "$host" "mkdir -p $proj/$kind/$app" 
        x $dry_run rsync -avz --relative "${dependencies_for_deploy[@]}" $host:$proj/$kind/$app/ 
    ;;
    after-deploy )
        service_name="${app}_$kind"
        if [[ $(ssh $host "ls /etc/systemd/system/${service_name}.service") ]]; then
            if [[ $BUILD != "NO" ]]; then
                cmd="sudo systemctl restart ${app}_$kind && sudo systemctl enable ${app}_$kind"
                x $dry_run time ssh $host "cd $proj/$kind/$app/ && $cmd"
            fi
            url="https://u2h.ru"
            route=/$app
            prefix=
            if [[ $kind == 'prod' ]]; then
                prefix=""
            else
                prefix="/$kind"
            fi
            url="$url$prefix$route/about"
            sleep  1
            x $dry_run curl "$url"
            cat << EOM
== DID DEPLOY AND $cmd
EOM
        elif [[ -e $di_dir/apps/$app/systemd.service ]]; then
            cat << EOM
== AFTER DEPLOY NOTE:
    run $di_dir/$kind/$app/systemd.sh
EOM
        else
            cat << EOM
== DID DEPLOY 
EOM
        fi
    ;;
esac

