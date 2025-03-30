#!/bin/bash

pushd "$(dirname "$0")"/.. > /dev/null

# First argument is a git branch to checkout. We default to `main`, but any branch, tag, or commit should work.
branchtag=${1:-main}

function do_loudly() {
    echo "+ $*"
    $*
}

set -e

mkdir -pv c_sdk
OUTDIR=$(realpath c_sdk)

REPO=target/tinysys_repo
if [ ! -d "$REPO" ]; then
    do_loudly git clone https://github.com/ecilasun/tinysys.git \
        --branch $branchtag                                     \
        --no-checkout                                           \
        --depth 1                                               \
        $REPO
    pushd $REPO > /dev/null

    pwd
    do_loudly git sparse-checkout init --cone
    do_loudly git sparse-checkout set software/SDK/
    do_loudly git checkout $branchtag
else
    echo "[~] Found $(realpath $REPO), attempting to update"
    pushd $REPO > /dev/null
    git fetch origin $branchtag --depth 1
    git checkout FETCH_HEAD
fi

git show --summary | head -n 15
echo $(git rev-parse HEAD)      > $OUTDIR/git-HEAD.txt
echo                           >> $OUTDIR/git-HEAD.txt
echo $(git log --oneline HEAD | head -n 1) >> $OUTDIR/git-HEAD.txt

popd > /dev/null

SDK_PATH=$REPO/software/
mv -v $OUTDIR/SDK $OUTDIR/SDK.old
do_loudly cp -r $SDK_PATH/SDK $OUTDIR/

echo "[~] SDK H files"
for file in $(find $SDK_PATH/SDK -type f -name "*.h" | sort); do
    if [[ -f "$file" ]]; then
        echo "    + ${file#$SDK_PATH/}"
    fi
done
