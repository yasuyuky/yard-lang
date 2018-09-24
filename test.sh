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
    echo "$expected expected, but got $actual"
    exit 1
  fi
}

try 0 0
try 42 42
try 12 1+13-2
try 13 1-13+25

echo OK
