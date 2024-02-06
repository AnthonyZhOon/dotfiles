#!/usr/bin/env bash

#
# basic map-reduce test
#

#RACE=

# comment this to run the tests without the Go race detector.
RACE=-race

# run the test in a fresh sub-directory.
rm -rf mr-tmp
mkdir mr-tmp || exit 1
cd mr-tmp || exit 1
rm -f mr-*

# make sure software is freshly built.
(cd ../../apps/wc && go build $RACE -buildmode=plugin wc.go) || exit 1
(cd ../../apps/indexer && go build $RACE -buildmode=plugin indexer.go) || exit 1
(cd ../../apps/mtiming && go build $RACE -buildmode=plugin mtiming.go) || exit 1
(cd ../../apps/rtiming && go build $RACE -buildmode=plugin rtiming.go) || exit 1
(cd ../../apps/jobcount && go build $RACE -buildmode=plugin jobcount.go) || exit 1
(cd ../../apps/early_exit && go build $RACE -buildmode=plugin early_exit.go) || exit 1
(cd ../../apps/crash && go build $RACE -buildmode=plugin crash.go) || exit 1
(cd ../../apps/nocrash && go build $RACE -buildmode=plugin nocrash.go) || exit 1
(cd ../../coordinator && go build $RACE mrcoordinator.go) || exit 1
(cd ../../worker && go build $RACE mrworker.go) || exit 1
(cd ../../sequential && go build $RACE mrsequential.go) || exit 1

failed_any=0