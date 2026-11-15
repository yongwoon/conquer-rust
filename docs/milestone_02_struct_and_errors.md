# 📖 Milestone 2: 구조체, 선언적 CLI(clap) & Result 기반 에러 처리

> **목표:** Milestone 1의 절차적 `main.rs` 코드를 비즈니스 로직 라이브러리(`src/lib.rs`)와 CLI 바이너리(`src/bin/kvs.rs`)로 분리하고, Rust의 객체 모델인 **구조체(`struct`) & `impl`**, **단위 테스트**, **`clap` 선언적 CLI**, **`Result<T, E>` 기반 에러 처리**를 체계적으로 마스터합니다.

---

## 📋 진행 체크리스트 (Progress Tracker)

- [ ] **Step 1:** 모듈 아키텍처 분리 (`src/lib.rs`) & `KvStore` 구조체(`struct`)와 `impl` 구현 *(현재 진행 중)*
- [ ] **Step 2:** `KvStore` 단위 테스트(Unit Test) 작성 및 검증 (`cargo test`)
- [ ] **Step 3:** 선언적 CLI 크레이트 `clap` 도입 및 바이너리(`src/bin/kvs.rs`) 분리
- [ ] **Step 4:** `Result<T, E>`, `?` (Try) 연산자 및 `thiserror`를 활용한 커스텀 에러 처리

---

## 🧠 핵심 개념 및 언어별 비교

### 1. 객체 모델: Class(Python/TS) vs Struct & Receiver(Go) vs Struct & Impl(Rust)

Rust에는 `class`나 `inheritance`(상속)가 없습니다. 데이터(상태)와 메서드(행위)를 명확히 분리합니다.

| 언어 | 데이터 정의 | 메서드 정의 | 상태 가변성(`self`/`this`) |
| :--- | :--- | :--- | :--- |
| **Python** | `class KvStore:` | `def get(self, key):` | `self`를 통해 런타임에 아무 속성이나 수정 가능 |
| **Go** | `type KvStore struct { ... }` | `func (s *KvStore) Get(key string)` | 포인터 리시버(`*s`)로 수정, 동시 접근 시 Data Race 위험 |
| **TypeScript** | `class KvStore { ... }` | `get(key: string): string` | `this` 바인딩 이슈, 런타임 가변성 제약 없음 |
| **Rust** | **`struct KvStore { ... }`** | **`impl KvStore { ... }`** | **`&self`(읽기 전용 빌림) vs `&mut self`(배타적 쓰기 빌림)**을 컴파일러가 강제! |

```rust
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    // 생성자 (Go의 NewKvStore() 패턴)
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    // 읽기 전용 메서드: &self (불변 빌림)
    pub fn get(&self, key: &str) -> Option<String> { ... }

    // 상태 수정 메서드: &mut self (가변 빌림, 배타적 접근 보장)
    pub fn set(&mut self, key: String, value: String) { ... }
}
```

---

### 2. 에러 처리 모델: Exception(Python/TS) vs Multiple Return(Go) vs `Result<T, E>`(Rust)

| 언어 | 에러 표현 방식 | 전파 및 처리 방식 | 문제점/특징 |
| :--- | :--- | :--- | :--- |
| **Python** | `raise Exception` | `try ... except` | 어디서 예외가 터질지 함수 시그니처만 보고는 알 수 없음 |
| **Go** | `(val, error)` 반환 | `if err != nil { return err }` | 보일러플레이트 반복, 에러 무시(`val, _ := ...`) 가능 |
| **TypeScript** | `throw new Error()` | `try ... catch` | `catch (e: unknown)` 타입 안전성 결여 |
| **Rust** | **`Result<T, E>` (`Ok(T)` \| `Err(E)`)** | **`?` (Try 연산자)로 간결한 전파** | **에러를 무시하면 컴파일 경고/에러 발생, 완벽한 타입 안전성** |

---

## 🎯 Step 1: 모듈 아키텍처 분리 & `KvStore` 구조체 구현

### 1.1 프로젝트 레이아웃 전환
Rust 표준 관례에 따라 라이브러리와 실행 바이너리를 분리합니다:
```text
conquer-rust/
├── Cargo.toml
├── src/
│   ├── lib.rs          # 📦 비즈니스 로직 코어 (KvStore 구조체, 엔진 인터페이스)
│   └── bin/
│       └── kvs.rs      # 🚀 CLI 인터페이스 바이너리 (clap 커맨드 파싱)
```

### 1.2 `KvStore` 인터페이스 설계
- `KvStore::new() -> KvStore`: 새 인스턴스 생성
- `set(&mut self, key: String, value: String)`: 키-값 삽입
- `get(&self, key: &str) -> Option<String>`: 키 조회
- `remove(&mut self, key: &str) -> Option<String>`: 키 삭제

---

## 🎯 Step 2: 단위 테스트(Unit Tests) 작성 및 검증

Rust는 테스트 프레임워크가 표준 내장되어 있어 별도의 Jest나 PyTest 설치가 필요 없습니다:
```bash
cargo test
```
`src/lib.rs` 하단에 `#[cfg(test)]` 모듈을 추가하여 `set_get`, `remove_key`, `get_non_existent` 등의 테스트를 통과시킵니다.

---

## 🎯 Step 3: 선언적 CLI 파서 `clap` 도입

`clap` 크레이트(v4, `derive` 기능)를 `Cargo.toml`에 추가하여, 수동 인자 파싱 대신 선언적이고 프로페셔널한 CLI 헬프(`--help`) 및 자동 검증을 구성합니다.

---

## 🎯 Step 4: `Result<T, E>`와 `thiserror` 에러 처리

키 삭제 실패(`KeyNotFound`), I/O 에러 등 향후 영속성 엔진(Milestone 3)으로 확장할 때 필요한 커스텀 에러 열거형을 정의하고 `?` 연산자로 우아하게 에러를 전파합니다.
