#!/usr/bin/env bash

set -e

tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
