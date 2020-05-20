#!/bin/sh
cd ./bindings
rm go.sum
go get -u
make static_external
