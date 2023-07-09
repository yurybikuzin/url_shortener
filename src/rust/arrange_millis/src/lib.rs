// ============================================================================
// ============================================================================

pub fn get(millis: u128) -> String {
    let secs = millis / 1000;
    let mins = secs / 60;
    let hours = mins / 60;
    let mins = mins % 60;
    let secs = secs % 60;
    let millis = millis % 1000;
    if hours != 0 {
        format!("{hours}:{mins:0>#2}:{secs:0>#2}.{millis:0>#3}")
    } else if mins != 0 {
        format!("{mins}:{secs:0>#2}.{millis:0>#3}")
    } else if secs != 0 {
        format!("{secs}.{millis:0>#3}s")
    } else {
        format!("{millis} ms")
    }
}

// ============================================================================
// ============================================================================
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        pretty_env_logger::init();

        assert_eq!(get(0), "0 ms");
        assert_eq!(get(999), "999 ms");
        assert_eq!(get(1000), "1.000s");
        assert_eq!(get(59023), "59.023s");
        assert_eq!(get(60000), "1:00.000");
        assert_eq!(get(60000 * 59), "59:00.000");
        assert_eq!(get(60000 * 60), "1:00:00.000");
        assert_eq!(get(60000 * 60 * 25), "25:00:00.000");
    }
}
