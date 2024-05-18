# Depy

Depy is a dependency manager based on scoop. It automatically installs any program from a collection of over 3000+ packages.

Depy automatically manages creates virtual environemnt similar to `python -m venv` that helps you manage app versions of apps cleanly.\

## Compilation

**Compilation dependencies:**
- only rust :)

First install rust from [rustup](https://rustup.rs/). After that download the source code from github. Finally open the project in your preferred code editor and run `cargo build`. The project will be compiled under target/Debug/depy.exe

## Usage

First we need to create a `depy.json` file that will declare all of our desired programs.

`depy.json`
```
[
    {
        "name": "python",
        "version": "3.11.0",
        "bucket_url": "main",
        "bucket_name": "main"
    },
    {
        "name": "firefox",
        "version": "latest",
        "bucket_url": "extras",
        "bucket_name": "extras"
    }
]
```

In this depy.json file we declared that we need to install python version 3.11.0 from the `main` bucket and firefox the latest version from the `extras` bucket. Buckets are just collections of packages, the `main`, `extras` and `versions` buckets are official buckets provided by scoop. Usually you can find a lot of programs in the `main` bucket but for most programs, they  don't fit the scoop main bucket criteria,  so you can find them in `extras` bucket. If you want to you can provide your own bucket [more info](#costume-buckets).

After we define our `depy.json` we can just run:
```
depy
``` 
And everything will install automatically without polluting the environment PATH(in most cases).

Last thing we need to do is run:
```
.depyvenv\activate
```
Now all of our apps will be available to us and `(CURENTLY IN DEV SHELL)` should appear in front of the prompt. Until you close the shell you will have access to all the specified versions of the programs. On subsequent runs you will only need o run the activate script.

## Some more important info

**This installation system will have conflicts if you are using the "use_isolated_path" config, very recomended you unset it if you are using it.**

Depy may require you to set 

```
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

This application does not cause conficts with existing scoop installed programs.

## Costume buckets