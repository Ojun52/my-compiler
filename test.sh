#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  cargo run -- -- "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42
assert 10 "3+32-25"
assert 8 "(1+2)*3-1"
assert 1 "-(1+2)*3+10"

echo OK
