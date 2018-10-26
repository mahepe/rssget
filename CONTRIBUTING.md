# Instructions for contributing

## Getting started

1. Look for an issue to contribute to or discuss with the maintainers about the update.

1. Fork this repo.

2. Clone your fork of this repo and set the remotes as follows:
```
git clone https://github.com/YOUR_USERNAME/rssget.git
git remote add upstream https://github.com/mahepe/rssget.git
```
afterwards the output of `git remote -v` should be
```
origin	git@github.com:YOUR_USERNAME/rssget.git (fetch)
origin	git@github.com:YOUR_USERNAME/rssget.git (push)
upstream	git@github.com:mahepe/rssget.git (fetch)
upstream	git@github.com:mahepe/rssget.git (push)
```
4. It's recommended to keep the master branch up to date with upstream/master and make custom changes to a development branch:
```
git checkout -b EXAMPLE
```

## Making a PR

1. After you are happy with the changes you have made, squeeze them to a single commit by running (in branch EXAMPLE)
```
git rebase -i master
```
2. Push the changes to your remote
```
git push origin EXAMPLE
```
3. Make a pull request in GitHub.
