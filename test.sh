#! /bin/bash
cargo test

if [ $? ]
then
    cd system_test
    ./run_tests
fi

if [[ $? -eq 0 ]]
then
    echo "$(tput setaf 2)all tests OK"
else
    echo "$(tput setaf 1)some tests Failed"
    exit 1
fi

exit 0
