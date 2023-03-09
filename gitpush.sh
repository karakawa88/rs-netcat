#!/bin/bash

git checkout main
git merge dev
git checkout dev
git push github main
git push github dev

exit 0

