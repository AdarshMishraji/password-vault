pub fn clr() {
    let _ = std::process::Command::new("cls")
        .status()
        .or_else(|_| std::process::Command::new("clear").status())
        .unwrap()
        .success();
}
