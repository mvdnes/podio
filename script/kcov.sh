set -e
shopt -s nullglob

PROJECT=podio
TARGET=$HOME/kcov

wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
tar xzf master.tar.gz
mkdir kcov-master/build
cd kcov-master/build
cmake .. -DCMAKE_INSTALL_PREFIX=$TARGET
make
make install
cd ../..
export PATH=$TARGET/bin:$PATH

#actual coverage testing:

cargo rustc -- --test -C 'link-args=-Wl,--no-gc-sections'
kcov --exclude-pattern=/.cargo target/kcov-lib target/debug/$PROJECT

for d in tests/*; do
    basename=$(basename $d);
    testname=${basename%.*};

    cargo rustc --test $testname -- -C 'link-args=-Wl,--no-gc-sections'
    kcov --exclude-pattern=/.cargo target/kcov-$testname target/debug/$testname-*
done

kcov --merge --coveralls-id=$TRAVIS_JOB_ID target/kcov target/kcov-*
