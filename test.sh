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
try 14 1+13
try 16 1+13+2
try 12 1+13-2
try 13 1-13+25
try 41 ' 12 + 34 - 5 '
try 5 ' 1 * 2 + 3 '
try 7 ' 1 + 2 * 3 '
try 14 ' 1*2 + 3*4 '
try 62 ' 1*2 + 3*4*5 '
try 3 ' 1*2 - 3*4*5 + 6*7 + 9+10'

try 0 ' 1/2'
try 2 ' 1/2 + 3*4/5'
try 7 ' 1/2 + 3*4/5 + 6*7/8'
try 6 ' 1/2 + 3*4/5 + 6*7/8 + 9 - 10'

echo OK
