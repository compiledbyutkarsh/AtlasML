# 🚀 AtlasML

A distributed machine learning job orchestration platform built in Rust.

AtlasML provides a Redis-backed task queue, worker execution pipeline, and orchestration APIs for managing ML workloads. The project explores core distributed systems concepts including task scheduling, worker coordination, queue-based execution, and scalable infrastructure design.

---

## ✨ Features

* ⚡ High-performance Rust backend
* 🔄 Redis-backed distributed task queue
* 🧠 ML job orchestration API
* 👷 Independent worker execution system
* 📦 JSON-based job serialization
* 🔗 Decoupled producer-consumer architecture
* 🚀 Horizontal scaling through multiple workers
* 🛠 Built with modern Rust ecosystem

---

## 🏗 Architecture

```text
                ┌─────────────────┐
                │     Client      │
                └────────┬────────┘
                         │
                         ▼
                ┌─────────────────┐
                │  Orchestrator   │
                │    (Axum API)   │
                └────────┬────────┘
                         │
                         ▼
                ┌─────────────────┐
                │      Redis      │
                │   Job Queue     │
                └────────┬────────┘
                         │
            ┌────────────┴────────────┐
            ▼                         ▼
     ┌─────────────┐          ┌─────────────┐
     │  Worker A   │          │  Worker B   │
     └─────────────┘          └─────────────┘
```

---

## 🛠 Tech Stack

### Backend

* Rust
* Axum
* Tokio

### Queue & Messaging

* Redis

### Serialization

* Serde
* Serde JSON

### Infrastructure

* Distributed Worker Model
* Queue-Based Execution
* Producer-Consumer Architecture

---

## 🚀 Getting Started

### Prerequisites

* Rust
* Cargo
* Redis

### Start Redis

```bash
brew services start redis
```

Verify:

```bash
redis-cli ping
```

Expected:

```text
PONG
```

---

### Run Orchestrator

```bash
cd services/orchestrator
cargo run
```

---

### Run Worker

```bash
cd services/worker
cargo run
```

---

## 🧪 Submit a Job

```bash
curl -X POST http://127.0.0.1:8080/jobs \
-H "Content-Type: application/json" \
-d '{"name":"bert-training"}'
```

---

## 📋 Example Output

Worker:

```text
🚀 worker started
⚙️ processing: bert-training
✅ completed: f7484c02-82ca-4e7c-8087-4fdcbfd68548
```

---

## 🎯 Project Goals

AtlasML is being developed as a learning-focused distributed systems project covering:

* Distributed task scheduling
* Queue-based execution systems
* Worker orchestration
* Infrastructure engineering
* Scalable backend architecture
* ML platform fundamentals

---

## 🗺 Roadmap

### Current

* [x] Redis-backed job queue
* [x] Distributed worker execution
* [x] Orchestration API
* [x] Job serialization pipeline

### Planned

* [ ] Worker registration
* [ ] Worker heartbeats
* [ ] Retry engine
* [ ] Dead-letter queue
* [ ] Job priorities
* [ ] Metrics endpoint
* [ ] Multi-worker coordination
* [ ] Docker Compose deployment
* [ ] Integration testing
* [ ] Scheduler service

---

## 📚 Inspiration

AtlasML draws inspiration from modern distributed compute and orchestration systems such as:

* Ray
* Celery
* Apache Airflow
* Kubernetes Jobs

### ⭐ If you find this project interesting, consider starring the repository.
