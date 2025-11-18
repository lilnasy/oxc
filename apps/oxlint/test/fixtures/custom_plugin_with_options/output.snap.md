# Exit code
1

# stdout
```
  x test-plugin-options(check-options): Expected value to be production, got disabled
   ,-[files/index.js:2:1]
 1 | // Test file with debugger statement
 2 | debugger;
   : ^^^^^^^^^
 3 | 
   `----

Found 0 warnings and 1 error.
Finished in Xms on 1 file using X threads.
```

# stderr
```
WARNING: JS plugins are experimental and not subject to semver.
Breaking changes are possible while JS plugins support is under development.
```
