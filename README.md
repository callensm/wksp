#helloworld wksp

## Usage

Create the configuration directory for `wksp` to look for template files in:

```sh
# Windows -> C:\Users\{USER}\.wksp
# Linux -> /home/{USER}/.wksp
# MacOS -> ~/.wksp OR /Users/{USER}/.wksp
```

Inside of the `wksp` configuration directory, each template is a JSON file using the following schema:

```json
{
  "folders": [
    {
      "name": "myfolder",
      "template": {
        "folders": ["..."],
        "files": ["..."]
      }
    }
  ],
  "files": ["file1.rs", "file2.js", "file3.go"]
}
```

> `folder` and `files` fields do not need to be defined if they will be empty

## Example

Running the command:

```sh
wksp --name new_workspace --template example
```

Will look for a template file `$HOME/.wksp/example.json` and build the template in the current directory named whatever you passed to the `-n/--name` argument.

Given a template defined as:

```json
# javascript_git.json
{
  "folders": [
    {
      "name": "src",
      "template": {
        "folders": [{ "name": "lib" }],
        "files": ["index.js"]
      }
    },
    {
      "name": "tests",
      "template": {
        "files": ["index.test.js"]
      }
    }
  ],
  "files": [".gitignore", "README.md", "LICENSE"]
}
```

Running the command `wksp -n js-project -t javascript_git` will produce the following file tree:

```
js-project
├── .gitignore
├── LICENSE
├── README.md
├── src
│   ├── index.js
│   └── lib
└── tests
    └── index.test.js

3 directories, 5 files
```
