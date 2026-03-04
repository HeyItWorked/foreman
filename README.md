# foreman

A minimal async task queue inspired by Celery — built from scratch with
Tokio and Axum.

## Goals

- Learn `tokio::sync` primitives: channels, semaphores
- Build a concurrent worker pool with bounded parallelism
- Handle job retries with exponential backoff
- Ship a real HTTP API with `axum`

## Running

```bash
cargo run
```

The server listens on `http://localhost:3000`.

## API

### Submit a job

```bash
curl -X POST http://localhost:3000/jobs \
  -H "content-type: application/json" \
  -d '{"kind": "echo", "data": {"msg": "hello"}}'
# → {"id": "<uuid>"}
```

### Check status

```bash
curl http://localhost:3000/jobs/<id>
```

### List all jobs

```bash
curl http://localhost:3000/jobs
curl "http://localhost:3000/jobs?status=completed"
```

### Cancel a queued job

```bash
curl -X DELETE http://localhost:3000/jobs/<id>
```

## Built-in job kinds

| kind    | data fields     | what it does                          |
|---------|-----------------|---------------------------------------|
| `echo`  | any             | returns the payload unchanged         |
| `sleep` | `ms` (number)   | sleeps N milliseconds, returns `{slept_ms}` |
| `fail`  | any             | always errors (for testing retry)     |

## Status

> Work in progress — not production ready.
