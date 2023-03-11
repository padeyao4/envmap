# envmap

envmap is a command-line tool written in rust that can be used to replace envsubst¹, which can recursively process all text variables in a directory and replace them with environment variables.

## Installation

You can download the binary file of envmap from [here](https://github.com/padeyao4/envmap/releases), or use cargo to install:

```bash
cargo install envmap
```

## Usage

The basic usage of envmap is as follows:

```bash
envmap path [options]
```

Where `path` is the address of the directory or file to be processed, which can be a relative or absolute path.

Optional parameters are:

- `-e` or `--env-file`: Specify the text file address of the environment variables, which should contain one or more lines of environment variables defined in `key=value` format. If this parameter is not specified, the current system environment variables are used.
- `--include`: Specify only files of a specific type to be processed, which can be one or more extensions or regular expressions separated by commas. For example: `--include .txt,.md`
- `--exclude`: Specify files of a specific type to be excluded, which can be one or more extensions or regular expressions separated by commas. For example: `--exclude .png,.jpg`
- `-r` or `--regex`: Specify the type of variable that can be replaced, there are four types to choose from: LINUX,PYTHON,MYBATIS,WINDOWS. LINUX means that the variable format that can be replaced is `${var}`or `$var`, WINDOWS means that the variable format that can be replaced is `%var%`, PYTHON means that the variable format that can be replaced is `{{var}}`, MYBATIS means that the variable format that can be replaced is `#{var}`or `#var`. The default is LINUX variable format.

Note: Only one parameter `--include`or `--exclude`can be used. If both parameters are used at the same time, an error will occur.

## Example

Assume there is a directory structure as follows:

```
test/
├── config.txt
├── data.json
├── image.png
└── sub/
    ├── env.txt
    └── script.py
```

The contents of config.txt are as follows:

```
name=${NAME}
age=$AGE
gender={{GENDER}}
```

The contents of data.json are as follows:

```
{
  "name": "#{NAME}",
  "age": #AGE,
  "gender": "%GENDER%"
}
```

The contents of script.py are as follows:

```
print("Hello, {{NAME}}!")
print("You are {{AGE}} years old.")
print("Your gender is {{GENDER}}.")
```

The contents of env.txt are as follows:

```
NAME=Bob
AGE=25
GENDER=male
```

If we want to replace all text files in test directory with environment variables defined in env.txt and only process .py and .json types files then we can execute following command:

```bash
envmap test --env-file test/sub/env.txt --include .py,.json --regex PYTHON
```

After execution result like this：

config.txt content unchanged (because it was not processed):

```
name=${NAME}
age=$AGE
gender={{GENDER}}
```

data.json content modified (because it was processed):

```
{
  "name": "Bob",
  "age": 25,
  "gender": "male"
}
```

script.py content modified (because it was processed):

```
print("Hello, Bob!")
print("You are 25 years old.")
print("Your gender is male.")
```

image.png content unchanged (because it was not processed).
