#! /bin/bash
cargo test

if [ $? ]
then
    cd system_test
    ./run_tests
fi
