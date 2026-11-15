# 📖 Milestone 1: 기초 문법 & 소유권 입문 (In-Memory KVS v1)

> **목표:** Rust의 가장 큰 장벽인 **소유권(Ownership)**, **빌림(Borrowing)**, **`String` vs `&str`**, **`Option<T>`과 패턴 매칭**을 이해하고, 터미널에서 동작하는 인메모리 Key-Value CLI v1을 작성합니다.

---

## 📋 진행 체크리스트 (Progress Tracker)

- [x] **Step 1:** CLI 인자 수집 (`std::env::args`) & 소유권(Ownership) 및 `String` vs `&str` _(완료)_
- [x] **Step 2:** 명령어 분기 처리 & `Option<T>` 및 `match` 패턴 매칭 _(완료)_
- [x] **Step 3:** 인메모리 저장소 연동 (`HashMap`) & 빌림(Borrowing, `&` vs `&mut`) _(완료)_
- [x] **Step 4:** 종합 기능 테스트 (`set`, `get`, `rm`) 및 `make clippy`, `make test` 검증 _(완료)_

> 🎉 **Milestone 1 완료!** (완료일: 2026-11-15)  
> 소유권, 빌림, `Option<T>`, `match` 패턴 매칭의 기초를 완벽히 소화하고 인메모리 KVS 프로토타입을 완성했습니다.  
> 다음 단계는 [docs/milestone_02_struct_and_errors.md](./milestone_02_struct_and_errors.md)로 이어집니다.

---

## 🎯 Step 1: CLI 인자 수집 (`std::env::args`) & 소유권 기초

### 1.1 Python, Go, TypeScript vs Rust 비교

CLI 프로그램의 첫걸음은 터미널에서 입력받은 인자(Arguments)를 배열로 읽어오는 것입니다.

| 언어                  | 명령줄 인자 수집 방식                                 | 특징 및 메모리 모델                                                                                  |
| :-------------------- | :---------------------------------------------------- | :--------------------------------------------------------------------------------------------------- |
| **Python**            | `import sys; args = sys.argv`                         | `sys.argv`는 문자열 리스트(`list[str]`), 모든 요소는 GC가 관리하는 힙 객체                           |
| **Go**                | `import "os"; args := os.Args`                        | `os.Args`는 `[]string`, 슬라이스 헤더(포인터+길이)를 복사하여 전달                                   |
| **TypeScript (Node)** | `const args = process.argv`                           | `string[]`, V8 엔진의 힙 가비지 컬렉터가 객체 수명 관리                                              |
| **Rust**              | `let args: Vec<String> = std::env::args().collect();` | **`args()`는 이터레이터, `collect()`를 통해 새로운 힙 배열(`Vec<String>`)을 생성하여 소유권을 획득** |

### 1.2 핵심 개념: `String` vs `&str`

- **`String` (소유권을 가진 힙 문자열):**
  - 가변(Mutable)하게 크기를 조절할 수 있습니다.
  - 자신이 위치한 메모리의 진짜 주인(Owner)입니다.
  - 스택에는 `[포인터, 용량(capacity), 길이(len)]` 총 24바이트(64비트 기준) 메타데이터만 저장되고, 실제 글자들은 힙에 저장됩니다.
- **`&str` (문자열 슬라이스, String Slice):**
  - 다른 어딘가(바이너리 데이터, 힙의 String, 스택)에 있는 UTF-8 문자열을 **"잠깐 빌려서 가리키는 읽기 전용 뷰"**입니다.
  - 스택에는 `[포인터, 길이(len)]` 16바이트만 저장됩니다.
  - 소유권이 없으므로 직접 해제할 수 없습니다.

> 💡 **왜 `env::args()`는 `&str`이 아니라 `String`을 반환할까요?**
> 운영체제(OS)로부터 넘겨받은 커맨드라인 문자열을 우리 프로그램의 메모리 공간에 안전하게 복사해 소유해야, 원본 OS 버퍼가 바뀌거나 사라져도 독립적으로 안전하게 다룰 수 있기 때문입니다.

### 1.3 실습 코드: `src/main.rs` 수정

[src/main.rs](../src/main.rs) 파일을 열어 아래와 같이 작성합니다:

```rust
use std::env;

fn main() {
    // env::args()는 이터레이터(Iterator)를 반환합니다.
    // .collect()로 수집하여 Vec<String>(동적 배열)의 소유권을 얻습니다.
    let args: Vec<String> = env::args().collect();

    // 디버그 출력 포맷: {:?}
    println!("전달받은 전체 인자: {:?}", args);
}
```

### 1.4 검증 및 실행

터미널에서 아래 명령을 실행하여 확인합니다:

```bash
# 기본 실행 (인자 0번째는 실행 바이너리 경로)
make run

# 커스텀 인자 전달 테스트
docker compose exec app cargo run -- set user_name "alice"
```

**예상 출력:**

```text
전달받은 전체 인자: ["target/debug/conquer-rust", "set", "user_name", "alice"]
```

---

## 🎯 Step 2: 명령어 분기 처리 & `Option<T>` 및 `match` 패턴 매칭

### 2.1 Python, Go, TypeScript vs Rust 비교

인자가 덜 들어왔거나 유효하지 않은 명령어일 때의 방어 코드를 작성합니다.

| 언어 | 부재(Absence) 표현 | 처리 방식의 위험성/특징 |
| :--- | :--- | :--- |
| **Python** | `None` | `if x is None:` 깜빡하면 런타임에 `AttributeError` 폭탄 |
| **Go** | `nil`, `ok` 관용구 | `val, ok := m[key]`, `nil` 포인터 접근 시 런타임 `panic: runtime error` |
| **TypeScript** | `null` \| `undefined` | Optional Chaining(`?.`) 지원하지만 런타임 `TypeError: Cannot read properties...` 발생 가능 |
| **Rust** | **`Option<T>` (`Some(T)` or `None`)** | **`null`/`nil` 키워드 자체가 언어에 존재하지 않음.** 모든 부재 처리를 컴파일 타임에 강제함! |

### 2.2 핵심 개념: `Option<T>`과 `match`

Rust는 널 포인터의 위험을 원천 차단하기 위해 표준 라이브러리의 `Option<T>` 열거형(Enum)을 사용합니다:

```rust
enum Option<T> {
    Some(T), // 값이 존재하는 경우
    None,    // 값이 없는 경우
}
```

배열의 n번째 인덱스에 안전하게 접근하려면 `args.get(1)`을 사용합니다:

- `args[1]`로 접근하면 범위를 벗어났을 때 즉시 패닉(`panic!`)이 납니다. (Go의 `panic: index out of range`와 동일)
- `args.get(1)`은 `Option<&String>`을 반환하므로, 컴파일러가 값이 없을 때(`None`)의 처리를 반드시 작성하도록 유도합니다.

### 2.3 실습 코드 예시

```rust
let command = match args.get(1) {
    Some(cmd) => cmd.as_str(), // &String -> &str로 변환 (역참조 강제 변환)
    None => {
        eprintln!("사용법: kvs <set|get|rm> [KEY] [VALUE]");
        return;
    }
};

match command {
    "set" => println!("SET 명령 실행"),
    "get" => println!("GET 명령 실행"),
    "rm"  => println!("RM 명령 실행"),
    _     => eprintln!("알 수 없는 명령: {}", command),
}
```

---

## 🎯 Step 3: 인메모리 저장소 연동 (`HashMap`) & 빌림(Borrowing)

### 3.1 핵심 개념: 가변성(`mut`)과 불변 빌림(`&`) vs 가변 빌림(`&mut`)

Rust의 모든 변수는 **기본적으로 불변(Immutable)**입니다.
값을 수정하려면 명시적으로 `mut` 키워드를 붙여야 합니다.

```rust
use std::collections::HashMap;

let mut store: HashMap<String, String> = HashMap::new();
```

#### 빌림의 대원칙 (Borrowing Rules - 컴파일 타임 Read-Write Lock):

1. **불변 참조(`&T`)는 동시에 여러 개 존재할 수 있다.** (동시 읽기 허용)
2. **가변 참조(`&mut T`)는 특정 시점에 오직 단 하나만 존재할 수 있다.** (쓰기 배타성)
3. **가변 참조와 불변 참조는 동시에 공존할 수 없다.** (읽는 도중에 쓰는 행위 원천 차단 $\to$ Data Race 방지)

### 3.2 `HashMap` 메서드와 소유권의 움직임

- **`insert(key, value)`**: Key와 Value의 소유권을 `HashMap` 내부로 **이동(Move)**시킵니다.
  ```rust
  store.insert(key, value); // key와 value의 소유권이 map으로 넘어감
  ```
- **`get(&key)`**: Key를 소각하지 않고 **빌려서(`&str` or `&String`)** 찾은 뒤, 저장된 값의 불변 참조(`Option<&String>`)를 반환합니다.
  ```rust
  if let Some(val) = store.get(&key) {
      println!("Found: {}", val);
  }
  ```
- **`remove(&key)`**: Key를 빌려서 찾은 뒤, 삭제된 값을 소유권(`Option<String>`) 형태로 꺼내어 반환합니다.

---

## 🎯 Step 4: 종합 기능 테스트 및 검증

### 4.1 최종 완성 코드 구조

CLI 명령어로 `set`, `get`, `rm`을 입력받아 결과를 출력하는 메인 루틴 완성.

### 4.2 검증 체크리스트

- [ ] `make check`로 타입 및 문법 에러 0건 확인
- [ ] `make clippy`로 Rust 관용구(Idiomatic code) 충족 여부 확인
- [ ] `make fmt`로 코드 스타일 정렬
- [ ] 터미널 테스트:
  - `docker compose exec app cargo run -- set name ferris`
  - `docker compose exec app cargo run -- get name`
  - `docker compose exec app cargo run -- rm name`
  - `docker compose exec app cargo run -- unknown` (에러 메시지 확인)

---

## 📌 다음 단계 예고 (Milestone 2)

Milestone 1에서는 단일 명령 실행 후 프로세스가 종료되므로 메모리 데이터가 휘발됩니다.
**Milestone 2**에서는 이를 재사용 가능한 구조체(`struct KvStore`)와 정식 CLI 파서(`clap`), 그리고 `Result<T, E>` 에러 처리 체계로 리팩토링합니다.
