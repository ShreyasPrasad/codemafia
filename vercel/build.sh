#!/bin/bash
PATH=$PATH:/vercel/.cargo/bin

cd ../app && trunk build --release