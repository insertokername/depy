This application does not cause conficts with existing scoop installed programs

items i couldn't get to work with multiple versions installed due to inconsistent installers

may require to set 

```
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

This installation system will have conflicts if you are using the "use_isolated_path" config, very recomended you unset it if you are using it