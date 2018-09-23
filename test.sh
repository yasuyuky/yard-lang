#!/bin/bash
try() {
  expected="$1"
  input="$2"

  echo "$input" | ./target/debug/yard-lang > tmp.ll
  clang -target x86_64-apple-darwin17.6.0 -o tmp tmp.ll
  ./tmp
  actual="$?"

  if [ "$actual" == "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input expected, but got $actual"
    exit 1
  fi
}

try 0 0
try 42 42

echo OK
