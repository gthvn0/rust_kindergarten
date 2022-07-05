# Rust Kindergarten

![Rust Logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

My [Rust](https://www.rust-lang.org/) playground

## bm

- Create a tool to manage directoris as bookmark.
- It will return a string that will be the directory to move on.
  - We will create an alias in the shell to *cd* into the directory
  - So we renamed the binary **bmr** to avoir confusion

### TODO

- [X] Create the project
```
cargo new --vcs none bm
```
- [X] Start by following the [Rust CLI tutorial](https://rust-cli.github.io/book/tutorial/setup.html)
- [X] Update CLI to do:
  - `bmr -l`: list all available bookmarks
  - `bmr -d <bookmark name>`: delete the bookmark
  - `bmr -a <bookmark name>`: add the current directory as a bookmark named *named*
- [ ] Implement the function using file like for bookmarks **.bm.yml**.

## Codingame

- Created using:
```
cargo new --vcs none codingame
```

- Most of exercices are taken from [coding game](https://www.codingame.com)
- Maybe have a look to [code wars](https://www.codewars.com)

## Monads

- Created using:
```
cargo new --vcs none monads
```
- It is a video found on youtube that explains monads by example but I don't
  remember which one it is.


## Notebook

- Give a try to [evcxr](https://github.com/google/evcxr/blob/main/evcxr_jupyter/README.md) for Jupyter
