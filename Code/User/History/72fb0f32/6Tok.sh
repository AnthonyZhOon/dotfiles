#!/bin/bash
./http-server --hostname 192.168.1.110 --port 8008 > >(tee -a stdout.log) 2> >(tee -a stderr.log >&2)