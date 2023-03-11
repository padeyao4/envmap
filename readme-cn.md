# envmap

envmap 是一个用 rust 编写的命令行工具，可以用来代替 envsubst¹，用来递归处理目录中所有文本的变量替换为环境变量。

## 安装

你可以从[这里](https://github.com/user/envmap/releases)下载 envmap 的二进制文件，或者使用 cargo 安装：

```bash
cargo install envmap
```

## 用法

envmap 的基本用法如下：

```bash
envmap path [options]
```

其中`path`是要处理的目录或文件的地址，可以是相对路径或绝对路径。

可选参数有：

- `-e`或`--env-file`：指定环境变量的文本文件地址，该文件应该包含一行或多行以`key=value`格式定义的环境变量。如果不指定该参数，则使用当前系统的环境变量。
- `--include`：指定只处理特定类型的文件，可以是一个或多个以逗号分隔的扩展名或正则表达式。例如：`--include .txt,.md`
- `--exclude`：指定排除特定类型的文件，可以是一个或多个以逗号分隔的扩展名或正则表达式。例如：`--exclude .png,.jpg`
- `-r`或`--regex`：指定可以替换的变量类型，一共有四个可选择类型，分别是 LINUX,PYTHON,MYBATIS,WINDOWS。LINUX 表示可以替换的变量格式为`${var}`或者 `$var`, WINDOWS 表示可以替换的变量格式为 `%var%`, PYTHON 表示可以替换的变量格式为 `{{var}}`, MYBATIS 表示可以替换的变量格式为 `#{var}` 或者 `#var`. 默认使用 LINUX 变量格式。

注意：只能使用其中一个参数 `--include` 或者 `--exclude`, 如果同时使用两个参数，则会报错。

## 示例

假设有一个目录结构如下：

```text
test/
├── config.txt
├── data.json
├── image.png
└── sub/
    ├── env.txt
    └── script.py
```

其中 config.txt 内容如下：

```text
name=${NAME}
age=$AGE
gender={{GENDER}}
```

data.json 内容如下：

```text
{
  "name": "#{NAME}",
  "age": #AGE,
  "gender": "%GENDER%"
}
```

script.py 内容如下：

```text
print("Hello, {{NAME}}!")
print("You are {{AGE}} years old.")
print("Your gender is {{GENDER}}.")
```

env.txt 内容如下：

```text
NAME=Bob
AGE=25
GENDER=male
```

如果我们想要把 test 目录中所有文本文件中出现在 env.txt 中定义过得环境变量进行替换，并且只处理.py 和.json 类型得文件，则我们可以执行以下命令：

```bash
envmap test --env-file test/sub/env.txt --include .py,.json --regex PYTHON
```

执行后得到结果如下：

config.txt 内容不变（因为没有被处理）:

```text
name=${NAME}
age=$AGE
gender={{GENDER}}
```

data.json 内容被修改为（因为被处理了）:

```text
{
  "name": "Bob",
  "age": 25,
  "gender": "male"
}
```

script.py 内容被修改为（因为被处理了）:

```text
print("Hello, Bob!")
print("You are 25 years old.")
print("Your gender is male.")
```

image.png 内容不变（因为没有被处理）.
