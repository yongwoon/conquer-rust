# [2026-11-15] Rust 심화 학습: clone() 없이 소유권을 가져오는 방법과 Zero-Copy 패턴

> **작성일:** 2026-11-15  
> **관련 마일스톤:** Milestone 1 (Step 3: 인메모리 저장소 연동)  
> **핵심 키워드:** `Ownership`, `Move`, `Borrowing`, `clone()`, `Iterator::next()`, `Zero-Copy`

---

## 1. 문제의 배경: 왜 `clone()`을 써야만 했을까?

Step 3에서 `HashMap`에 데이터를 넣을 때 우리는 다음과 같은 코드를 작성했습니다:

```rust
let args: Vec<String> = env::args().collect();

let key = match args.get(2) {
    Some(k) => k.clone(), // 👈 여기서 왜 clone()을 썼을까?
    None => return,
};

store.insert(key, value);
```

### 왜 그냥 `k`를 넣으면 컴파일 에러가 날까?
1. **`args.get(2)`의 반환 타입:** `Option<&String>` (빌려온 불변 참조)
   - `args` 배열이 요소들의 진짜 소유자(Owner)입니다.
   - 만약 `args.get(2)`가 소유권(`String`)을 줘버리면(Move), `args` 배열의 2번 인덱스는 빈 구멍(Hole)이 되어 메모리 무결성이 깨집니다.
   - 따라서 컴파일러는 원본 배열을 해치지 않도록 **"빌려주는 것(`&String`)"**만 허용합니다.
2. **`store.insert(K, V)`의 요구 조건:** `String` (소유권)
   - `HashMap`은 데이터를 내부 버퍼에 영구적으로 보관해야 하므로, 빌려온 뷰(`&String`)가 아닌 **완전한 소유권을 가진 `String`**을 요구합니다.
3. **충돌과 해결:**
   - 빌려온 참조(`&String`)로는 남에게 소유권을 넘겨줄 수 없습니다.
   - 그래서 힙 메모리에 새로운 공간을 할당하고 내용을 그대로 복제하여 독자적인 소유권을 얻는 `k.clone()`을 사용한 것입니다.

---

## 2. `clone()`의 대가 (비용)

Python, Go, TypeScript에서는 런타임 가비지 컬렉터(GC)가 객체 참조 카운팅이나 힙 할당을 알아서 처리하기 때문에 개발자가 복사 비용을 간과하기 쉽습니다.

* Python: `key = args[2]` (포인터만 복사, 참조 카운트 +1)
* Go: `key := args[2]` (16바이트 문자열 헤더만 복사, 같은 바이트 배열 참조)
* TypeScript: `const key = args[2]` (포인터 참조 전달)
* **Rust의 `k.clone()`:** **실제 힙(Heap) 메모리를 새로 할당하고, UTF-8 바이트들을 바이트 단위로 복제(Deep Copy)**합니다.
  - 문자열 길이가 길거나 고빈도로 호출되는 루프 내부라면 상당한 CPU 및 메모리 오버헤드가 발생합니다.

---

## 3. `clone()` 없이 소유권을 가져오는 2가지 대안

어차피 `args`는 한 번 쓰고 버릴 데이터라면, 불필요한 힙 복제 없이 **소유권을 직접 이전(Move)**시킬 수 있습니다.

### 대안 1: `args.remove(index)` 사용하기

`Vec::remove(index)`는 해당 인덱스의 요소를 배열에서 완전히 떼어내며 **소유권(`String`)을 반환**합니다.

```rust
// 주의: 인덱스가 존재하지 않으면 패닉이 발생하므로 사전 검증 필요
if args.len() > 3 {
    // 2번 인덱스를 떼어내어 소유권을 key로 Move (clone 없음!)
    let key: String = args.remove(2);
    // 2번이 빠졌으므로 기존 3번 인덱스가 2번으로 당겨짐
    let value: String = args.remove(2);

    store.insert(key, value);
}
```

* **장점:** 힙 메모리 재할당 없이 기존 문자열의 소유권만 이전(Move).
* **단점:**
  - 인덱스가 범위를 벗어나면 패닉 발생 (`bounds check` 필수).
  - 요소를 꺼낸 뒤 뒤쪽에 있던 요소들을 한 칸씩 앞으로 당겨야 하므로 $O(N)$의 메모리 이동 비용 발생.

---

### 대안 2: 이터레이터(Iterator) 자체를 소비하기 (★ 가장 Rust다운 Idiomatic Zero-Copy 방식)

Rust 표준 라이브러리의 `env::args()`는 `Iterator` 트레이트를 구현하고 있습니다.  
굳이 `.collect::<Vec<String>>()`으로 중간 배열을 만들지 않고, **이터레이터에서 요소를 하나씩 꺼내면 소유권(`Option<String>`)째로 바로 나옵니다.**

```rust
use std::env;
use std::collections::HashMap;

fn main() {
    let mut args = env::args();

    // 0번째: 실행 파일 이름 (버림)
    args.next();

    // 1번째: 서브커맨드 (Option<String> 형태)
    let command = match args.next() {
        Some(cmd) => cmd,
        None => {
            eprintln!("사용법: kvs <set|get|rm> [KEY] [VALUE]");
            return;
        }
    };

    let mut store = HashMap::new();

    match command.as_str() {
        "set" => {
            // next()가 Option<String>을 반환하므로 clone() 없이 소유권 직행!
            let key = match args.next() {
                Some(k) => k,
                None => return,
            };
            let value = match args.next() {
                Some(v) => v,
                None => return,
            };

            // clone() 없이 소유권 Move! (Zero-Copy)
            store.insert(key, value);
            println!("OK");
        }
        "get" => {
            let key = match args.next() {
                Some(k) => k,
                None => return,
            };
            if let Some(v) = store.get(&key) {
                println!("{}", v);
            }
        }
        _ => {}
    }
}
```

* **동작 원리:**
  - `Iterator::next()`는 컬렉션의 요소를 빌려주는(`&T`) 것이 아니라, **값을 완전히 소진(Consume)하여 소유권(`Option<T>`)을 넘겨줍니다.**
  - 따라서 불필요한 배열(`Vec`) 할당도 없고, 문자열 딥카피(`clone()`)도 전혀 없습니다.
* **성능:** 극한의 효율성과 메모리 안전성을 동시에 달성하는 전형적인 Rust 패턴입니다.

---

## 4. 요약 정리

| 방식 | 소유권 전달 방식 | 힙 메모리 복제 | 추가 비용 | 추천 상황 |
| :--- | :--- | :---: | :--- | :--- |
| **`args.get(i).clone()`** | 불변 참조 빌림 후 복제 | **발생 (O(M))** | 힙 할당 비용 | 원본 배열을 유지한 채 값을 여러 번 써야 할 때 |
| **`args.remove(i)`** | 배열에서 떼어내어 Move | **없음** | 요소 쉬프트 비용 ($O(N)$) | 이미 `Vec`이 있고 특정 값만 빼내고 싶을 때 |
| **`args_iter.next()`** | 이터레이터에서 직접 Move | **없음 (Zero-Copy)** | **비용 0** | **CLI 인자처럼 순차적으로 읽고 버리는 파이프라인 (권장)** |
