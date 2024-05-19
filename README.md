# Depy

Depy is a dependency manager based on scoop. It automatically installs any program from a collection of over 3000+ packages.

Depy automatically manages and creates virtual environemnts similar to `python -m venv` that help you manage versions of apps cleanly.

## Instalation

Open up powershell by searching "powershell" in you windows search bar.

If you already have scoop installed skip to the next set of commands, if you don't just copy paste these and hit enter until all commands are run. If powershell asks you for any sort of confirmation type y:
```
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
```

If you are on a 32 bit system you need to install [git](https://git-scm.com/downloads) manually. Otherwise just run:

```
scoop install git
```

After git is installed you just need to run:
```
scoop install https://raw.githubusercontent.com/insertokername/depy/main/manifest/depy.json
```

If you get an error about not finding `VCRUNTIME140.dll` you need to install the visual studio redistributable from [here](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#visual-studio-2015-2017-2019-and-2022) 

You are done! Just reopen powershell and you can now use the depy command!

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

In this depy.json file we declared that we need to install python version 3.11.0 from the `main` bucket and firefox the latest version from the `extras` bucket. Buckets are just collections of packages, the `main`, `extras` and `versions` buckets are official buckets provided by scoop. Usually you can find a lot of programs in the `main` bucket but for any other programs there is a high chance that they are in the `extras` bucket. If you want to you can provide your own bucket. More information about depy.json format [here](#depyjson)

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

**This installation system will have conflicts if you are using the scoop "use_isolated_path" config, very recomended you unset it if you are using it.**

This application does not cause conficts with existing scoop installed programs.

## depy.json

The format of a depy.json looks like this:

```
[   //an array of "packages"
    
    {   //each package is defined by:
        "name":"generic_package",   //its name
        "version":"1.0",            //its version
        "bucket_url":"main"         //the github repository in which it is stored
        "bucket_name":"main"        //an identifier for that repository
    }
]
```

- The name and version section are self explanatory. 
- The `bucket_url` represents the repository in which to search for the package in. There are 3 built in bucket url shortcuts `main`, `extras` and `versions` these are official scoop repositories and are the most used when working with scoop. The rule of thumb is that you will either find the package you desire in the `main` repository and if not in the `extras` repository. The `versions` repository is for much older versions of programs like `python3.9` or others. 
- The `bucket_name` represents an arbitrary label to that bucket. This is used so that depy doesn't have to add buckets for each separate program. **It's very important that you name the main bucket `main` and no other bucket `main`.** This is because the main bucket is added by default for performance reasons and any conflicts with that will result in a crash.

## Adding a costume bucket:

First you should read a bit about scoop buckets from [here](https://github.com/ScoopInstaller/Scoop/wiki/Buckets). The proceed to use the [bucket template repo](https://github.com/ScoopInstaller/BucketTemplate) and **make sure your bucket directory is on the master branch.** After that you can just put the link to you repository in the `bucket_url` argument of the program that requires it (something like this: `"bucket_url":"https://github.com/ScoopInstaller/Extras"`)

## Compilation

**Compilation dependencies:**
- only rust 😁

First install rust from [rustup](https://rustup.rs/). After that download the source code from github. Finally open the project in your preferred code editor and run `cargo build`. The project will be compiled under target/Debug/depy.exe
