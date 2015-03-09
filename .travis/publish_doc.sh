#!/bin/bash

if  [ "$TRAVIS_BRANCH" = "master" ] &&
    [ "$TRAVIS_PULL_REQUEST" = "false" ] &&
    [ "$TRAVIS_REPO_SLUG" = "mvdnes/podio" ]
then
    echo "Publishing documentation..."

    cd $HOME
    git clone --quiet --branch=gh-pages https://${GH_TOKEN}@github.com/mvdnes/podio gh-pages > /dev/null

    cd gh-pages
    git config user.email "travis@travis-ci.org"
    git config user.name "travis-ci"

    git rm -rf . > /dev/null
    cp -Rf $TRAVIS_BUILD_DIR/target/doc/* .

    git reset HEAD -- index.html > /dev/null
    git checkout -- index.html > /dev/null

    git add -f .
    git commit -m "Auto doc upload from travis"
    git push -fq origin gh-pages > /dev/null

    echo "Published documentation"
fi
