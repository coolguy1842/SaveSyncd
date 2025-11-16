#!/usr/bin/env sh

cd docs
redocly build-docs ./savesyncd.yaml --output documentation.html
cd ..