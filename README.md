# MokaRes 
## A resource manager for MoKa Reads

> This application is under the [GPLv2 License](LICENSE.md)

This application is built to manage the resources that are used in the MoKa Reads web service.
This is built to serve different functionalities that will make it easier to manage the resources, 
such as creating, updating, and indexing resources. 

## How to Install 
This application is not yet available on [crates.io](https://crates.io), but there are plans to do so once more work has been put in 
and I can stop relying on the bleeding edge commits on projects. At the current moment to install do the following: 

```bash
$ cargo install --git https://github.com/Moka-Reads/MokaRes.git 
```

## Features to Implement 
- [X] Create a resource
  - [X] Cheatsheets
  - [X] Articles 
  - [X] Guides 
- [ ] Update a resource (migrates to new spec)
  - [ ] Cheatsheets
  - [ ] Articles 
- [X] Indexing Resources 
- [X] Indexing Awesome Lists 
- [ ] Providing Suggestions