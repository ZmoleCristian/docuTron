# docuTron 🤖
![](logo.png)
## docuTron is a simple program that uses OpenAI's GPT to generate markdown documentation for code.

## ⭐ Features

- Automatically generate documentation in markdown format
- Customizable options allow you to choose the model used for generation
- Specify desired extensions for the files to be processed
- Read code files from stdin or from the current directory

## 🧰 Requirements

- Rust 2021 edition
- Cargo
- Dependencies:
  - serde_json 1.0
  - reqwest 0.11 with "json" feature
  - dotenv 0.15.0
  - tokio 1 with "full" feature
  - structopt 0.3
  - walkdir 2.3
  - async-recursion 1.0

## 💾 Installation

To install and build the project, run:

```
git clone https://github.com/tragDate/docuTron.git
cd docuTron
cargo build --release
```

## ⌨️ Usage

1. Set the `OPENAI_API_KEY` environment variable to your OpenAI API key.
2. Run the program with the desired options:

To read all the code files from the current directory:

```
cd your_project
docuTron -h
```

To read code files with specific extensions:
```
docuTron -h -e "rs,toml"
```

To use a custom GPT model:
```
./target/release/docuTron -h -m "gpt-4"
```

To read code files from stdin:
```
cat input_code.txt | ./target/release/docuTron
```

## 🥷 Author

[@tragDate](https://tragdate.ninja)

## License

GPL-3.0
