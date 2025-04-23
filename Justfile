
list:
    just --list

ship:
    release-plz update
    git add .
    git commit -m "Update version"
    git push
    just release

release:
    release-plz release --backend github --git-token $(gh auth token)
