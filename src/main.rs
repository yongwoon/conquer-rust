use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match args.get(1) {
        Some(cmd) => cmd.as_str(),
        None => {
            eprintln!("사용법: kvs <set|get|rm> [KEY] [VALUE]");
            return;
        }
    };

    // 가변(mutable) HashMap 생성
    let mut store: HashMap<String, String> = HashMap::new();

    // 초기 테스트용 기본 데이터 삽입
    store.insert("name".to_string(), "Ferris".to_string());
    store.insert("lang".to_string(), "Rust".to_string());

    match command {
        "set" => {
            let key = match args.get(2) {
                Some(k) => k.clone(),
                None => {
                    eprintln!("오류: KEY가 필요합니다. 사용법: kvs set <KEY> <VALUE>");
                    return;
                }
            };
            let value = match args.get(3) {
                Some(v) => v.clone(),
                None => {
                    eprintln!("오류: VALUE가 필요합니다. 사용법: kvs set <KEY> <VALUE>");
                    return;
                }
            };

            store.insert(key, value);
            println!("OK (저장 성공)");
        }
        "get" => {
            let key = match args.get(2) {
                Some(k) => k,
                None => {
                    eprintln!("오류: KEY가 필요합니다. 사용법: kvs get <KEY>");
                    return;
                }
            };

            // &key로 빌려주고, Option<&String>으로 불변 참조를 받음
            match store.get(key) {
                Some(val) => println!("{}", val),
                None => println!("Key not found"),
            }
        }
        "rm" => {
            let key = match args.get(2) {
                Some(k) => k,
                None => {
                    eprintln!("오류: KEY가 필요합니다. 사용법: kvs rm <KEY>");
                    return;
                }
            };

            match store.remove(key) {
                Some(_) => println!("OK (삭제 성공)"),
                None => println!("Key not found"),
            }
        }
        _ => eprintln!("알 수 없는 명령: {}", command),
    }
}
