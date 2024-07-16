#!/bin/bash

echo "Setting up sanity_check_with_rspec fixture"
echo "Copying files into temp dir $1..."
rsync -r tests/fixtures/sanity_check_with_rspec/* $1 --exclude=setup.sh
echo "Done"

echo "Installing dependencies..."
bundle install --gemfile=$1/Gemfile
echo "Initializing git repository..."

if ! test -f ~/.gitconfig; then
  touch ~/.gitconfig
  git config --global user.name 'Your name'
  git config --global user.email 'you@example.com'
  git config --global init.defaultBranch main
fi

git init $1
echo "Committing files..."
git -C $1 add init.rb spec/init_spec.rb Gemfile Gemfile.lock
git -C $1 commit -m "init commit"
git -C $1 add foo.rb spec/foo_spec.rb
git -C $1 commit -m "add foo"
git -C $1 checkout -b new-branch
git -C $1 add bar.rb spec/bar_spec.rb
git -C $1 commit -m "add bar"
git -C $1 add baz.rb spec/baz_spec.rb
git -C $1 commit -m "add baz"
git -C $1 add fizz.rb spec/fizz_spec.rb
git -C $1 commit -m "add fizz"
echo "Done"
