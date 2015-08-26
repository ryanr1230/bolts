# bolts
a rust static site generator


### how to use:
```bash
 $ sudo make install
 $ cd /directory/for/your/project/to/be/in
 $ bolts init PROJECT_NAME
 $ cd PROJECT_NAME
 $ bolts run
```

### All commands:
 - `bolts init PROJECT_NAME`: initializes a static site with basic settings
 - `bolts run`: runs a daemon that updates your site as you write it
 - `bolts update`: updates bolts library for your local site
 - `bolts compile`: recompiles your configuration files to take effect
 - `bolts -h`: --help lists these commands

### To update the default config and bolts command line tool:
 - redownload
 - redo sudo make install
 - Should probably add a script to do this at some point
