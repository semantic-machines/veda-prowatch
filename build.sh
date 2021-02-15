BUILD_PATH=$PWD

#!/bin/sh
mkdir ./bin
BIN=$BUILD_PATH/bin

export CARGO_TARGET_DIR=$HOME/target

    echo BUILD VEDA-PROWATCH
    cd veda-prowatch
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-prowatch $BIN

