Sure! Let’s break down the second part about **controlling how tests are run** in Rust in the simplest way possible. 🚀

---

### **1. Running Tests in Parallel**
- By default, Rust runs tests **in parallel** (at the same time) to save time.
- **Problem**: If tests share resources (like files or environment variables), they might interfere with each other.
- **Solution**: Run tests **one at a time** by setting the number of threads to 1.

#### **Example**
```bash
cargo test -- --test-threads=1
```
- This runs tests one after another, avoiding conflicts.

---

### **2. Showing Output for Passing Tests**
- By default, Rust **hides output** (like `println!`) from passing tests to keep the output clean.
- If you want to see the output for **all tests** (even passing ones), use the `--show-output` flag.

#### **Example**
```bash
cargo test -- --show-output
```
- Now, you’ll see all `println!` output, even for passing tests.

---

### **3. Running a Subset of Tests**
Sometimes, you don’t want to run all tests. You can run specific tests by:
- **Name**: Run a single test by its name.
- **Filter**: Run tests whose names contain a specific word.

#### **Example: Run a Single Test**
```bash
cargo test test_name
```
- Replace `test_name` with the name of the test you want to run.

#### **Example: Run Tests with a Common Word**
```bash
cargo test add
```
- This runs all tests with `add` in their names (e.g., `add_two_and_two`, `add_three_and_two`).

---

### **4. Ignoring Tests**
- Some tests are slow or only needed in specific cases. You can **ignore** them using the `#[ignore]` attribute.
- Ignored tests won’t run by default, but you can run them explicitly.

#### **Example: Ignore a Test**
```rust
#[test]
#[ignore]
fn expensive_test() {
    // This test is ignored by default.
}
```

#### **Run Ignored Tests**
```bash
cargo test -- --ignored
```
- This runs only the ignored tests.

---

### **5. Running All Tests (Including Ignored)**
If you want to run **all tests**, including ignored ones:
```bash
cargo test -- --include-ignored
```

---

### **Key Takeaways**
1. **Parallel Tests**: Use `--test-threads=1` to run tests one at a time.
2. **Show Output**: Use `--show-output` to see `println!` for passing tests.
3. **Run Specific Tests**:
   - By name: `cargo test test_name`.
   - By filter: `cargo test add` (runs all tests with `add` in their names).
4. **Ignore Tests**: Use `#[ignore]` to skip slow tests.
5. **Run Ignored Tests**: Use `cargo test -- --ignored`.

---

### **Example: Full Workflow**
#### **Test Code**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn add_two_and_two() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(3 + 2, 5);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // Simulate a slow test.
        std::thread::sleep(std::time::Duration::from_secs(2));
        assert_eq!(1 + 1, 2);
    }
}
```

#### **Commands**
1. Run all tests:
   ```bash
   cargo test
   ```
2. Run only `add_two_and_two`:
   ```bash
   cargo test add_two_and_two
   ```
3. Run tests with `add` in their names:
   ```bash
   cargo test add
   ```
4. Run ignored tests:
   ```bash
   cargo test -- --ignored
   ```
5. Show output for all tests:
   ```bash
   cargo test -- --show-output
   ```

---

This is everything you need to control how tests are run in Rust! Let me know if you have more questions. 😊
