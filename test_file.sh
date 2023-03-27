#!/bin/sh

export lol=`printf %10s\\n | tr " " "="`

cd target/debug || exit
find target -executable -type f >file\ 1 && echo ok\ a

echo "$lol-ahahahha"
