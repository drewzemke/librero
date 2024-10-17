fn main() {
    let child = std::process::Command::new("pnpm")
        .arg("-C")
        .arg("../e2e")
        .arg("run")
        .arg("test")
        .spawn()
        .expect("Failed to spawn Playwright test process");

    let output = child
        .wait_with_output()
        .expect("Failed to get Playwright test output");

    assert!(output.status.success(), "Playwright tests failed");
}
