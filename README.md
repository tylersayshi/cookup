# cookup 🧑‍🍳

A little tool for building my cookbook.

![visual gif of cookup showing features listed below](./docs/demo.gif)

## Setup

Set an open ai api key as `$OPENAI_API_KEY`: https://platform.openai.com/docs/quickstart

```sh
$ cargo build --release
$ ln -sF ./target/release/cookup ~/bin/cookup # ~/bin will need to exist in $PATH
$ cookup
```

## How it works

This is using typescript for storing [the cookbook](./cookbook.ts) to make it easy to edit. I use [swc.rs](https://swc.rs) to parse the cookbook.

I built this for myself and personally really enjoy editing/reading the cookbook in the typescript file in my editor. Playing with swc in rust was fun too!

## What it does

- Helps with generating recipes and new recipe ideas
- Parses and saves generated recipes
- Supports authoring new recipes with your terminal `$EDITOR`
- Displays existing recipes in the terminal for reference
