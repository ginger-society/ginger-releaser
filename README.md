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
e. revision

This should be called when we have a merge request to a specific branch. 


```sh
ginger-releaser bump ?
```
When initialized the project is in nightly release. Everytime you `bump` it. It will go to next version. Nightly -> Alpha -> Beta -> Final 

Once the project is in Final stage, you should user major/minor/patch releases. 

The philosiphy here is that once a product is released. You can not make breaking changes. Every version after 1.0.0 should always be in release state with all scenarios covered. 