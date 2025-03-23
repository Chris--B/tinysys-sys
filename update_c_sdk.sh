#!/bin/bash

function do_loudly() {
    echo "+ $*"
    $*
}

set -e

mkdir -pv tinysys_c_sdk
OUTDIR=$(realpath tinysys_c_sdk)

REPO=target/tinysys_repo
if [ ! -d "$REPO" ]; then
    do_loudly git clone https://github.com/ecilasun/tinysys.git \
        --no-checkout $REPO                                     \
        --depth       1
    pushd $REPO > /dev/null

    pwd
    do_loudly git sparse-checkout init --cone
    do_loudly git sparse-checkout set software/SDK/
    do_loudly git checkout
else
    echo "Found $REPO, attempting to update"
    pushd $REPO > /dev/null
    do_loudly git pull
fi

echo $(git rev-parse HEAD)      > $OUTDIR/git-HEAD.txt
echo                           >> $OUTDIR/git-HEAD.txt
echo $(git log --oneline HEAD) >> $OUTDIR/git-HEAD.txt

popd > /dev/null

file $REPO/software/SDK
file $OUTDIR
do_loudly cp -rv $REPO/software/SDK $OUTDIR/

ls $OUTDIR/SDK/*.h
