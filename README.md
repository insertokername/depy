This application does not cause conficts with existing scoop installed programs

items i couldn't get to work with multiple versions installed due to inconsistent installers

may require to set 

```
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

This installation system will have conflicts if you are using the "use_isolated_path" config, very recomended you unset it if you are using it

```
acmesharp.json
azure-ps.json
boost.json
cacert.json
clink-completions.json
clink-flex-prompt.json
git-up.json
glfw.json
importexcel.json
jdtls.json
modern7z.json
nunit-extension-nunit-project-loader.json
nunit-extension-nunit-v2-driver.json
nunit-extension-nunit-v2-result-writer.json
nunit-extension-teamcity-event-listener.json
nunit-extension-vs-project-loader.json
oracle-instant-client-odbc.json
oracle-instant-client-sdk.json
oracle-instant-client-sqlplus.json
pester.json
powershell-beautifier.json
powershell-yaml.json
psgithub.json
rtools.json
scoop-shim.json
terraform-provider-ibm.json
tesseract-languages.json
winget-ps.json
xpdf-tools-lsp.json
z.lua.json
```
