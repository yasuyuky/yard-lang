#!/bin/bash
try() {
  expected="$1"
  input="$2"

  echo "$input" | ./target/debug/yard-lang > tmp.ll
  clang -Wno-override-module -o tmp tmp.ll
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
