#!/bin/bash
npm install
watchexec --exts js --clear --restart 'npm test'