# catrina-bundler
A mini-bundler for [catrina](https://github.com/PROMUEVETE-QUERETARO/catrina)

## Start project

```
$ catrina init
```

This command create a project with actual directory name, and install catrina package whit npm or yarn. Then the tool will question you if you wish to start the wizard; if you input "n", the new project will have the standard configuration.

```
$ catrina init -Y
```
With the `-Y` flag to use 'yarn' in the project.

```
$ catrina init -s
```

With the `-s` flag you create a new project with the standard configuration.

### Configuration

The catrina's configuration is read in **catrina.config.json** file. You can set this configuration using the wizard setup was run after `catrina init` command or make yourself the file.

The file's structure is the next (this values are the _standard configuration_):

```json
{
  "input_js": "input.js",
  "input_css": "input.css",
  "deploy_path": "./deploy",
  "out_js": "myProyectName.main.js",
  "out_css": "myProyectName.styles.css",
  "server_port": ":9095",
  "location_lib": "node_modules/catrina",
  "module": false
}
```

* input_js: initial javascript file where you write your code.
* input_css: initial CSS file where you write your code.
* deploy_path: directory where you want build the final files.
* out_js: name of final javascript file generated by catrina.
* out_css: name of final CSS file generated by catrina.
* server_port: port where proof server will be started (command `catrina run`).
* location_lib: the standard library location
* module: indicates if the output file will be a module

***Important***: *deploy path and input path can be the same directory, but the final files must have a different names than the inputs files.*

### Wizard

This tool runs after the `new` command ─ if you don't use the` -s`─ flag, and is used to manually configure the project settings. The fields that are not explicitly modified will be taken from the standard configuration.

## Build project (pending)

Write the output files

```
$ catrina build
```

## Start server (pending)

Run a proof server in deploy path defined in configuration file.

```
$ catrina run
```

---
## Compile catrina bundler
### Using make
Prepare the environment:
```
$ make prepare
```
Then, compile develop version
```
$ make dev
```

Or, compile release version. **Note** this command is not recommended, because this project is work in progress  
```
$ make
```