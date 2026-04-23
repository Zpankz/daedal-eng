```markdown
# daedal-eng Development Patterns

> Auto-generated skill from repository analysis

## Overview
This skill teaches the core development patterns and conventions used in the `daedal-eng` Rust codebase. You'll learn about file naming, import/export styles, commit message habits, and how to write and organize tests. While no formal workflows were detected, this guide provides best practices and suggested commands for common development tasks.

## Coding Conventions

### File Naming
- Use **camelCase** for file names.
  - Example: `myModule.rs`, `dataProcessor.rs`

### Import Style
- Use **relative imports** within the codebase.
  - Example:
    ```rust
    mod utils;
    use crate::utils::helperFunction;
    ```

### Export Style
- Use **named exports** for modules and functions.
  - Example:
    ```rust
    pub fn process_data() { /* ... */ }
    ```

### Commit Messages
- Freeform style, no enforced prefixes.
- Average commit message length: ~30 characters.
  - Example: `fix parser bug in dataProcessor`

## Workflows

### Adding a New Module
**Trigger:** When you need to add a new feature or logical component.
**Command:** `/add-module`

1. Create a new file using camelCase, e.g., `newFeature.rs`.
2. Implement your module logic.
3. Use relative imports to use code from other modules.
4. Export public functions or structs with `pub`.
5. Add or update tests in a corresponding `*.test.*` file.

### Writing a Test
**Trigger:** When you need to verify functionality.
**Command:** `/write-test`

1. Create a test file matching the pattern `*.test.*`, e.g., `dataProcessor.test.rs`.
2. Write test functions using Rust's built-in test framework.
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_process_data() {
           assert_eq!(process_data(), expected_result);
       }
   }
   ```
3. Run tests with `cargo test`.

### Committing Changes
**Trigger:** When ready to save your work.
**Command:** `/commit-changes`

1. Write a concise, freeform commit message (~30 chars).
2. Example: `implement new data parser`

## Testing Patterns

- **Test Framework:** Not explicitly detected; use Rust's built-in test framework.
- **File Pattern:** Place tests in files matching `*.test.*`, e.g., `moduleName.test.rs`.
- **Test Example:**
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_functionality() {
          // Test logic here
      }
  }
  ```

## Commands
| Command           | Purpose                                 |
|-------------------|-----------------------------------------|
| /add-module       | Scaffold and add a new module           |
| /write-test       | Create and write a new test file        |
| /commit-changes   | Commit staged changes with a message    |
```
