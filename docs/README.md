# 🦀 Conquer Rust: 실습 튜토리얼 & 진행 현황판

이 문서는 Rust를 밑바닥부터 고급 레벨까지 정복해 나가는 과정에서 **"언제든 학습을 멈추고, 다시 시작할 때 바로 이어서 진행할 수 있도록"** 현재 진행 상태와 각 단계별 상세 튜토리얼을 체계적으로 기록하는 대시보드입니다.

---

## 🚦 현재 학습 진행 상태 (Status Dashboard)

> **현재 진행 중:** `Milestone 2: 구조체, CLI 파싱 & 에러 처리 (Result, clap)`  
> **현재 실습 단계:** `Step 1: 모듈 분리 (src/lib.rs) & KvStore 구조체와 impl 구현` 진행 중  
> **최종 업데이트:** 2026-11-15

| 마일스톤 | 주제 | 상세 가이드 문서 | 진행 상태 |
| :---: | :--- | :--- | :---: |
| **M0** | Tooling & Dev Setup (Docker, Cargo, Make) | [ROADMAP.md](../ROADMAP.md#milestone-0-환경-설정--rust-핵심-도구-체계화) | ✅ **완료** |
| **M1** | 기초 문법 & 소유권 입문 (In-Memory KVS v1) | [docs/milestone_01_in_memory_kvs.md](./milestone_01_in_memory_kvs.md) | ✅ **완료** |
| **M2** | 구조체, CLI 파싱 & 에러 처리 (`Result`, `clap`) | [docs/milestone_02_struct_and_errors.md](./milestone_02_struct_and_errors.md) | 🔄 **진행 중 (Step 1)** |
| **M3** | 파일 영속성 엔진 (Bitcask Append-only 로그) | `docs/milestone_03_persistence_engine.md` | ⏳ 대기 |
| **M4** | Trait & Generics로 스토리지 엔진 추상화 | `docs/milestone_04_traits_and_generics.md` | ⏳ 대기 |
| **M5** | 동시성 (Concurrency)과 스레드 안전성 (`Arc`, `RwLock`) | `docs/milestone_05_concurrency.md` | ⏳ 대기 |
| **M6** | 비동기 네트워크 서버 & Redis RESP 프로토콜 (`tokio`) | `docs/milestone_06_async_network_server.md` | ⏳ 대기 |
| **M7** | 제로 카피 최적화 & 성능 벤치마크 (`bytes`, `criterion`) | `docs/milestone_07_optimization_and_benchmarks.md` | ⏳ 대기 |

---

## 📚 심화 학습 노트 (Study Notes)

학습 중 깊이 있게 탐구한 Rust의 핵심 원리와 최적화 패턴을 기록합니다.

- **[2026-11-15]** [clone() 없이 소유권을 가져오는 방법과 Zero-Copy 패턴](./study/2026-11-15_ownership_move_vs_clone.md)

---

## 📌 중간에 멈추고 새 세션에서 바로 재개하는 방법 (Resume Guide)

새로운 세션이나 터미널 창을 열었을 때, 아래 템플릿을 복사하여 어시스턴트에게 전달하면 직전 상태를 그대로 복원하여 학습을 이어갈 수 있습니다.

```markdown
나는 Python, Go, TypeScript 경험이 있는 개발자이며, Rust 마스터가 목표인 학습자입니다.
현재 `/Users/yongwoon/Desktop/yongwoon/source/active_projects/conquer-rust`에서 학습 중입니다.

[현재 상태]
- 전체 진행 현황은 `docs/README.md`를 참고해 주세요.
- 현재 진행 중인 마일스톤: Milestone 1 (In-Memory KVS v1)
- 현재 진행 중인 단계: docs/milestone_01_in_memory_kvs.md의 Step 1 (진행 중인 체크박스 확인)

[요청]
`docs/`의 현재 진도에 맞춰 다음 설명과 실습 단계를 진행해 주세요.
```

---

## 🛠️ 개발 환경 명령어 퀵 레퍼런스

모든 작업은 Docker 컨테이너(`conquer_rust_dev`) 내부에서 수행되도록 Makefile로 자동화되어 있습니다.

```bash
# 1. 컨테이너 백그라운드 기동
make up

# 2. 문법 및 타입 체크 (가장 빠름 - Go의 컴파일 속도와 유사)
make check

# 3. 코드 실행
make run

# 4. 인자(CLI arguments)를 전달하여 실행할 때
docker compose exec app cargo run -- set key value

# 5. Rust 공식 린터(Clippy) 정밀 검사
make clippy

# 6. 코드 자동 포맷팅 (rustfmt)
make fmt

# 7. 단위/통합 테스트 실행
make test
```
