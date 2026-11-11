#[derive(Debug)]
enum Status {
    Ready,
    Warning(u8),
    Offline,
}

fn describe(status: Status) -> &'static str {
    match status {
        Status::Ready => "ready",
        Status::Warning(level) if level >= 3 => "critical",
        Status::Warning(_) => "warning",
        Status::Offline => "offline",
    }
}

fn main() {
    let current = Status::Warning(4);
    println!("{}", describe(current));
}
