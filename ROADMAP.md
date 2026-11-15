# Conquer Rust: Master Roadmap (Beginner to Advanced)

이 로드맵은 Rust의 핵심 철학을 초급부터 고급까지 **Key-Value 스토리지 엔진(kvs)**을 만들며 단계별로 익히도록 설계된 체계적인 실습 가이드입니다.

---

## 🗺️ 전체 로드맵 개요

| 단계 | 마일스톤 | 핵심 주제 | 주요 완성 산출물 |
|:---:|:---|:---|:---|
| **0** | **Tooling & Setup** | Cargo, Clippy, rustfmt, Rust Analyzer | 개발 환경 셋업 & Cargo 워크플로우 숙지 |
| **1** | **Beginner** | 소유권, 빌림, 슬라이스, 기본 CLI | In-Memory Key-Value v1 (`std::env`) |
| **2** | **Structure & Errors** | 구조체, 열거형, `Result`, `clap` | 모듈 분리 (`lib.rs` / `main.rs`) & `clap` 도입 |
| **3** | **Persistence** | 파일 I/O, Serde, Bitcask 아키텍처 | 파일 기반 영속성 엔진 (Append-only 로그) |
| **4** | **Traits & Generics** | 트레이트 추상화, 동적/정적 디스패치 | 플러그형 스토리지 인터페이스 (`KvsEngine`) |
| **5** | **Concurrency** | `Arc`, `RwLock`, `Send`/`Sync`, 스레드 풀 | 멀티스레드 안전한 동시성 스토리지 |
| **6** | **Async & Networking** | Tokio 비동기, TCP 소켓, RESP 프로토콜 | Redis 호환 네트워크 서버 & 클라이언트 |
| **7** | **Advanced & Perf** | Zero-copy(`bytes`), 수명(Lifetime), 벤치마크 | Criterion 벤치마크 & 제로 카피 최적화 |

---

## 📌 상세 마일스톤

### Milestone 0: 환경 설정 & Rust 핵심 도구 체계화
- **목표:** 개발 루프(Code $\to$ Lint $\to$ Test $\to$ Format) 자동화
- **학습 내용:**
  - `cargo init --bin`: 새 프로젝트 구조 이해 (`Cargo.toml`, `src/main.rs`)
  - `cargo check`: 컴파일 없이 빠르게 타입/문법 검사 (Go의 빌드 속도 체감)
  - `cargo clippy`: Rust 시니어 개발자 같은 린터의 조언 활용법
  - `cargo fmt`: 공식 스타일 가이드 포맷팅
- **완료 조건:** `cargo check`, `cargo clippy`, `cargo test`가 모두 오류 없이 통과하는 기본 환경 구축

---

### Milestone 1: 기초 문법 & 소유권 입문 (In-Memory KVS v1)
- **목표:** Rust의 가장 큰 장벽인 **소유권(Ownership)과 빌림(Borrowing)** 체화
- **구현 기능:**
  - `cargo run -- set <KEY> <VALUE>`
  - `cargo run -- get <KEY>`
  - `cargo run -- rm <KEY>`
- **학습 개념:**
  - `String` (소유하는 힙 문자열) vs `&str` (빌려온 문자열 슬라이스)
  - 소유권의 이동(Move)과 복사(Copy)의 차이
  - `std::collections::HashMap` 기본 사용법
  - `Option<T>`과 `match`, `if let` 패턴 매칭
- **Go/Python/TS 비교 힌트:**
  - Go/Python처럼 참조를 아무 데나 복사해서 넘기면 컴파일 에러가 납니다. "누가 이 데이터의 주인인가?"를 항상 생각해야 합니다.
- **완료 조건:** 터미널에서 `set`, `get`, `rm` 명령이 메모리 상에서 정상 동작하는 CLI

---

### Milestone 2: 구조체, 커맨드라인 파싱 & 견고한 에러 처리
- **목표:** 비즈니스 로직과 CLI 인터페이스의 분리, 그리고 `Result<T, E>` 기반 에러 처리
- **구현 기능:**
  - `src/lib.rs`에 `KvStore` 구조체 구현 (`new`, `set`, `get`, `remove`)
  - `src/bin/kvs.rs`에 `clap` 기반의 선언적 서브커맨드 CLI 구성
  - 실패 시 패닉(`panic!`) 대신 `Result` 반환
- **학습 개념:**
  - `struct` 정의와 `impl` 블록 (`&self`, `&mut self`, 연관 함수 `KvStore::new()`)
  - 가변 참조(`&mut`)의 유일성 규칙 (동시에 하나의 가변 참조만 허용)
  - `Result<T, E>`와 `?` (Try) 연산자로 에러 전파
  - `thiserror` 크레이트로 커스텀 에러 열거형(`KvsError`) 정의
- **완료 조건:** `KvStore` 단위 테스트(Unit Test) 작성 및 통과

---

### Milestone 3: 디스크 영속성 엔진 (Bitcask 로그 모델)
- **목표:** 프로세스가 재시작되어도 데이터가 유지되는 Append-only 로그 스토리지 엔진 구현
- **구현 기능:**
  - 디스크 로그 파일(`*.log`)에 쓰기 연산(Command)을 순차 추가
  - 메모리에는 `Key -> (파일 오프셋, 길이)`를 기록하는 인덱스(`KeyDir`) 유지
  - 재시작 시 로그 파일을 한 번 읽어 메모리 인덱스 복구(Crash Recovery)
  - (확장) 오래된 로그를 정리하는 컴팩션(Compaction)
- **학습 개념:**
  - 파일 I/O: `std::fs::File`, `std::fs::OpenOptions`, `BufReader`, `BufWriter`
  - `std::io::Seek` 트레이트를 활용한 특정 오프셋 점프 읽기
  - `serde`와 `serde_json` (또는 `bincode`)을 이용한 구조체 직렬화/역직렬화
- **완료 조건:** 프로세스를 껐다 켜도 이전에 `set`한 데이터가 `get`으로 정확히 복구됨

---

### Milestone 4: Trait(트레이트)와 Generics로 엔진 추상화
- **목표:** 객체지향의 인터페이스를 뛰어넘는 Rust의 **Trait 시스템** 마스터
- **구현 기능:**
  - `KvsEngine` 트레이트 정의:
    ```rust
    pub trait KvsEngine {
        fn set(&mut self, key: String, value: String) -> Result<()>;
        fn get(&self, key: String) -> Result<Option<String>>;
        fn remove(&mut self, key: String) -> Result<()>;
    }
    ```
  - 자체 제작 `KvStore`와 외부 크레이트 `sled`(임베디드 DB) 모두 `KvsEngine` 구현
  - CLI 플래그(`--engine kvs` 또는 `--engine sled`)로 런타임/컴파일타임 교체 지원
- **학습 개념:**
  - Trait 정의 및 구현 (`impl KvsEngine for KvStore`)
  - 정적 디스패치 (`fn run<E: KvsEngine>(engine: E)`) vs 동적 디스패치 (`Box<dyn KvsEngine>`)
  - Go의 암시적 인터페이스 vs Rust의 명시적 Trait의 장단점
- **완료 조건:** 동일한 테스트 슈트가 `KvStore`와 `SledEngine` 양쪽에서 모두 100% 통과

---

### Milestone 5: 동시성(Concurrency)과 스레드 안전성
- **목표:** 컴파일러가 보장하는 데이터 레이스 프리(Data-race-free) 동시성 체화
- **구현 기능:**
  - 여러 스레드에서 동시에 `get` 요청을 읽고, 순차적으로 `set`을 수행하는 엔진
- **학습 개념:**
  - `Send`와 `Sync` 트레이트의 원리와 의미
  - `Arc<T>` (Atomic Reference Counting)로 여러 스레드 간 소유권 공유
  - `Mutex<T>` (상호 배제)와 `RwLock<T>` (다중 읽기/단일 쓰기)
  - Go의 뮤텍스(`mu.Lock()`) vs Rust의 뮤텍스(`let guard = lock.write().unwrap()`) 차이 체감
  - 스레드 풀(Thread Pool) 라이브러리(`rayon` or 커스텀 스레드 풀)
- **완료 조건:** 멀티스레드 동시 읽기/쓰기 스트레스 테스트 통과

---

### Milestone 6: 비동기 네트워크 서버 & Redis 호환 프로토콜
- **목표:** 비동기 I/O 런타임 `tokio`와 네트워크 프로토콜 파싱 마스터
- **구현 기능:**
  - `kvs-server`와 `kvs-client` 바이너리 분리
  - TCP 소켓 리스닝 및 클라이언트 요청 비동기 처리
  - Redis 표준 RESP 프로토콜 파서 구현 $\to$ 실제 `redis-cli`로 접근 가능
- **학습 개념:**
  - 비동기 프로그래밍 기초: `async fn`, `await`, `Future`, `tokio::spawn`
  - 비동기 스트림 읽기/쓰기: `tokio::io::AsyncReadExt`, `AsyncWriteExt`
  - 상태 머신과 버퍼 파싱 기법
- **완료 조건:** 터미널에서 공식 `redis-cli -p 4000`으로 접속하여 `SET`, `GET` 정상 동작

---

### Milestone 7: 고급 최적화 & 제로 카피(Zero-Copy)
- **목표:** 불필요한 메모리 할당 제거 및 C/C++ 수준의 극한 성능 달성
- **구현 기능:**
  - 프로토콜 파싱 시 문자열 복사(`String::clone`)를 없애고 `bytes::Bytes` 슬라이스로 처리
  - `criterion` 크레이트로 읽기/쓰기 Throughput(RPS) 및 레이턴시 벤치마크 측정
- **학습 개념:**
  - 명시적 수명(Lifetimes, `'a`)을 이용한 무복사(Zero-copy) 참조 유지
  - 메모리 프로파일링 및 릴리즈 빌드 최적화 (`LTO = true`, `codegen-units = 1`)
- **완료 조건:** 벤치마크 리포트 생성 및 제로 카피 적용 전후의 메모리 할당/성능 비교 분석
