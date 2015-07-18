#!/bin/bash
echo -e "Publishing RustDoc...\n"

cp -R target/doc $HOME/api/

cd $HOME
git config --global user.email "travis@travis-ci.org"
git config --global user.name "travis-ci"
git clone --quiet --branch=gh-pages https://${GH_TOKEN}@github.com/hawkw/seax_svm gh-pages > /dev/null

cd gh-pages
git rm -rf ./api
cp -Rf $HOME/api .
git add -f .
git commit -m "Lastest RustDoc for version $TRAVIS_TAG on successful travis build $TRAVIS_BUILD_NUMBER auto-pushed to gh-pages"
git push -fq origin gh-pages > /dev/null

echo -e "Published RustDoc to gh-pages.\n"
