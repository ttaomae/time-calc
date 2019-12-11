# Time Calculator
This is a simple command line tool and desktop application which performs calculations on times.
In English, the word "time" can refer to two distinct concepts. It can refer to a specific point in
time (e.g. "what time is it?") or it can refer to a duration between two points in time (e.g. "how
much time will it take?"). This calculator is for the second type.


## Use
### Operations and Syntax
The following table describes the supported operations.

| Left Operand | Operator(s) | Right Operand | Result |
|--------------|-------------|---------------|--------|
| Number       | +, -, \*, / | Number        | Number |
| Number       | \*          | Time          | Time   |
| Time         | \*, /       | Number        | Time   |
| Time         | +, -        | Time          | Time   |
| Time         | /           | Time          | Number |

Operations are evaluated in standard order. That is, multiplication and division, followed by
addition and subtraction, with operators of the same precedence evaluated left to right.
Additionally, parentheses can be used to group sub-expressions to override the normal order or
operations. No precedence is given to operations involving times versus numbers.

A time can be expressed in one of the following formats:
* *h*:*mm*:*ss*[.*sss*]
* *mm*:*ss*[.*sss*]
* *ss*[.*sss*]s

where
* *h* - represent a one or more digit hours component.
* *mm* - represents a two digit minutes component.
* *ss* - represents a two digit seconds component.
* [.*sss*] - represents an optional fractional seconds component, up to nanosecond precision.
* s - is the character `s`.
* : - is the character `:`.


### Command Line
The command line tool has three modes of operation. First, there is single expression mode where you
provide an expression as command line arguments. Depending on your shell, you may need to escape or
quote certain characters, or you can just quote the entire expression. However, for simplicity, the
following examples will not do this.
```bash
$ time-calc 9.8 + 7.6 - 5.4 * 3.2 / 1.1
1.690909091
$ time-calc 24:36 + 48s
25:24
$ time-calc 12:34:56 / 3
2:05:49.333333333
$ time-calc (55:55 / 2.5)
22:22
```

If you do not provide any arguments, you will enter into interactive mode. In this mode you can
enter multiple expressions and they will each be evaluted. To exit interactive mode, send an EOF
character (usually `Ctrl-D`). For clarity, in the example below, lines entered by the user are
prefixed with `> `. Howver, this will not actually be output and you should not type this.
```bash
$ time-calc
> 9.8 + 7.6 - 5.4 * 3.2 / 1.1
1.690909091
> 24:36 + 48s
25:24
> 12:34:56 / 3
2:05:49.333333333
> (55:55 / 2.5)
22:22
^D
```

The third mode is batch mode. In this mode, you can provide a list of expressions, delimited by line
breaks, to the stdin of the process. Each expression will be evaluated in order and the results will
be written to stdout. (This is technically the same as interactive mode, except rather than typing
in expressions directly, they are provided all at once.)
```bash
$ echo "9.8 + 7.6 - 5.4 * 3.2 / 1.1" > expressions
$ echo "24:36 + 48s"                >> expressions
$ echo "12:34:56 / 3"               >> expressions
$ echo "(55:55 / 2.5)"              >> expressions
$ cat expressions | time-calc
1.690909091
25:24
2:05:49.333333333
22:22
```

### Desktop Application
Values are automatically formatted as you type, so formatting characters such as `s` and `:` are not
necessary. By default, values are formatted as times, but you can toggle between times and numbers
clicking on the `#` key or typing `#` or `n`.

![Example desktop application usage](screenshots/demo.gif)

## Build
### Requirements
Builds should work on Windows and Linux with the following minimum requirements:
* [Java](https://jdk.java.net/) 11
* [Rust](https://www.rust-lang.org/) 1.39.0
* [Maven](https://maven.apache.org/) 3.6.3

Older or newer versions may work, but they have not been tested. Similarly, builds may work on macOS
since both Java and Rust have good cross-platform support, but this has not been tested.

### Command Line Tool
The `core` module is written in Rust and can be built using Cargo.
First navigate to the `core` directory then run one of the following commands.
```bash
# Faster compile time; slower, larger binary.
$ cargo build
# Slower compile time; faster, smaller binary.
$ cargo build --release
```
It will produce the command line executable at `core/target/{release,debug}/time-calc`, depending
on which build you performed.

The `core` module is also a Java project which can be built using Maven. It essentially just bundles
the executable in a JAR so that it can be used by the GUI. Use one of the following commands to
build the executable in the same location, plus a JAR file in the `core/target` directory.
```bash
# Debug build.
$ mvn package
# Release build.
$ mvn -P release package
```

### Desktop Application
The `gui` module is a [JavaFX](https://openjfx.io/) project which can be built using Maven.

The simplest way to build it is to build the entire project with the `bundle` profile. From the root
of the project, run one of the following commands.
```bash
# Use debug build of `core` executable in bundled application.
$ mvn -P bundle package
# Use release build in bundled application.
$ mvn -P release -P bundle package
```

This will automatically build both the `core` and `gui` modules. The application will be built in
the `gui/target/time-calc/` directory. It can be launched by running the
`gui/target/time-calc/bin/time-calc.bat` script.

Alternatively, you can install the `core` module to your local Maven repository. Then as long as you
don't make any changes to the `core` module, you only need to re-build the `gui` module. From the
root of the project, run the following commands.
```bash
# NOTE: Run only one of the following, depending on whether
# you want debug or release builds of the `core` executable.
$ mvn -pl core -am install
$ mvn -pl core -am -P release install
$ cd gui
$ mvn -P bundle package
```
Subsequent builds will only require you to run the last command.

If you have installed the `core` module, you can also run the application directly by navigating to
the `gui` directory and running the following command.
```bash
$ mvn compile javafx:run
```
