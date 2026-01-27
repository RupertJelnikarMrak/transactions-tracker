# Solana blockchain state tracker

## Architecture Overview

Helius Webhooks, unlike their LazerStream service doesn't offer any history or
backlog guarantee. As such, I've decided to implement one of my own, thus
splitting the process into multiple layers.

### 1. Ingestion/Listener (Axum)

A lightweight HTTP server

It's only job is to acknowledge the HTTP request immediately and push the raw
JSON payload into NATS. Not much that can crash or memory leak to ensure an
almost 100% uptime as well as a really quick startup time to prevent Helius'
retry protocol to time out.

### 2. Message Broker (NATS JetStream)

Serves as a safety buffer. If the database goes offline or some other issue
occurs, events pile up in the on-disk buffer rather than being lost. When the
issue is resolved they are automatically pulled out, processed and pushed to
the database.

### 3. Processing/Interpreter

A Rust binary that consume messages from NATS. It parses Helius'
"Enhanced Transaction" arrays, filters out irrelevant information and commits
the data to the Postgres database.

### 4. Database (Postgre + pgSQL Triggers)

Some data, like the balances of specific wallets, needs to be derived from the
event data that is being acquired in real time. While the Interpreter could be
made to handle this, sometimes data will need to be pushed manually, or trough
a different script. If one of those time not all the derivatives are updated
correctly, it would lead to huge issues, especially in tracking down where it
was missed. Defining triggers within the DB to fire a custom script to derive
the necessary data automatically solves this issue.

pgSQL updates can lead to deadlocks, but for 99.9% of cases a
simple "try again after delay" in the interpreter solves the issue completely.
So unless somehow hundreds of transactions to or from a specific wallet happen
at the exact same millisecond it should be irrelevant.

## Current Progress and notes

All of the layers are functional, but Interpreter and Database currently only
have implementations for a simple transfer of the token and a trigger to update
affected wallets or add them if they do not yet exist in which case a previous
balance of 0 $SEAS is assumed.

For deployment we need to populate the DB with either all past transactions,
or at least the current balances of wallets holding the token.

At the moment it's perfectly fine to simply store all transactions, however
in later stages a large number of transactions will slow down indexes searching
and become a significant cost in server resources. So something like
TimescaleDB will absoutely need to be implemented and defined to cull or
compress old irrelevant data.

## Hosting

I've made the listener and interpreter to be as lightweight as possible, running
them on the same server as the database should not pose any issue.

The idea is to deploy the listener + interpreter binaries and NATS with Docker,
while running Postgres natively on the server for less headaches.

As I've said previously, my personal recommendation is Hetzner hosting, after
double checking they have servers available in USA as well apparently. I don't
think anyone else offers the same price to performance. I've never had any
issues or noticed downtimes with them and I've been renting a server from them
for more than a year.
