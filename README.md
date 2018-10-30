# Nand2Tetris [![Build Status](https://travis-ci.org/mitoma/nand2tetris.svg?branch=master)](https://travis-ci.org/mitoma/nand2tetris)

コンピュータシステムの理論と実装を読んでRustで実装しているリポジトリです。

## テストの実行

cargo test

## hack コンピュータの起動の仕方

```bash
cargo run --bin computer_runner [hack binary path] [max_cycle] [wait_ms]
```

実行例

```text
cargo run --bin computer_runner test/MyRect.hack 1000
```

## assembler の実行

```text
cargo run ../hack_computer/test/MyRect.asm
```