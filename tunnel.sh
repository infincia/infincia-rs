#!/bin/sh

ssh -L 9000:127.0.0.1:5432 -f -N root@infincia.com
