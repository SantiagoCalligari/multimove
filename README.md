# multimove

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)


<center><img></img></center>
## Description

`multimove` is a tool that moves files listed in a source file to a specified destination location.

## Installation

1. Clone this repository:

```git clone https://github.com/santiagocalligari/multimove.git```


2. Navigate to the project directory:

```cd multimove```


3. Build the project:

```cargo install --path .```


## Usage

Run the program by specifying the path to the source file and the destination location:

```multimove [OPTIONS] <filepath> <destination> ```

The program will move all the files listed in the source file to the specified destination location.

Additional options:

- `-t, --threads <count>`: Specify the number of threads to use for moving the files. By default, all available threads will be used.

## Examples

- Move files using all available threads:

```multimove <filepath> <destination> ```

- Move files using only 2 threads:

```multimove -t 2 <filepath> <destination> ```


## Contribution

Contributions are welcome. If you find any issues or have any suggestions, feel free to contribute.

🔧👥 Enjoy moving your files with `multimove`! 😊🚚

