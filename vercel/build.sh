#!/bin/bash
PATH=$PATH:/vercel/.cargo/bin

cd app && npm ci && trunk build --release