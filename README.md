# Ginger-releaser

Releaser is a CLI tool for managing 
1. changelog - By generating CHANGELOG.md
2. Versioning



Commands : 

```sh
ginger-releaser init
```

This will initialize a releaser.toml file, this conatins a starting semantic version number. 

Then after adding few commits with commit messaged conforming to `commitzen` message format you can run

```sh
ginger-releaser release ?
```
This has many sub commands. 

a. major
b. minor
c. patch
d. channel
    This is interactive as it will ask you to choose the new channel to switch to. The options are 
        1. Final <-- this is for the prod release
        2. Alpha
        3. Beta
        4. Nighly <-- this is like dev changes on daily basis
e. revision

This should be called when we have a merge request to a specific branch. 

Test change 11

