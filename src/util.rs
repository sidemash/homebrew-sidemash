use  std::collections::hash_map::Keys;

pub(crate) fn join<T>(vec: Keys<'_, String, T>, start:&str, sep:&str, end:&str) -> String {
    let mut result = String::new();
    let mut i = 0;
    result.push_str(start);
    for s in vec {
        if i != 0 { result.push_str(sep) }
        result.push_str(s);
        i = i+1;
    }
    result.push_str(end);
    result
}