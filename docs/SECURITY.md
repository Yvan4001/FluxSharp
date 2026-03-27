# Security Features & Best Practices

## 🔒 Built-in Security Protections

FluxSharp includes 8 security features enabled by default to protect your programs.

---

## File Size Protection

**Limit:** 50 MB maximum

**What it does:** Prevents loading extremely large files that could consume memory.

**Example:**
```bash
# ❌ This will be rejected
dd if=/dev/zero of=huge.fsh bs=1M count=100
fluxc compile huge.fsh
# Error: File too large: 104857600 bytes > 52428800 bytes limit
```

**Why:** Protects against denial-of-service attacks and accidental memory exhaustion.

---

## Statement Limit

**Limit:** 10,000 statements per block

**What it does:** Prevents infinite loops and massive code blocks.

**Example:**
```flux
void main() {
    int i = 0;
    // If this loop goes over 10,000 iterations worth of code...
    while (i < 20000) {  // ❌ Too many statements
        serial_print(i);
        i = i + 1;
    }
}
```

**Why:** Prevents accidental or intentional infinite loops that freeze the compiler.

---

## Operator Chain Limit

**Limit:** 1,000 operators per expression

**What it does:** Prevents excessively complex expressions that could cause issues.

**Example:**
```flux
void main() {
    // ❌ 5000 operators in one line (not real, but example)
    int x = 1 + 1 + 1 + 1 + ... (5000 times);
    // Error: Expression has too many operators: 5000 > 1000 limit
}
```

**Why:** Prevents expression bombs and compilation complexity.

---

## ASM Output Limit

**Limit:** 100 MB maximum

**What it does:** Prevents generated assembly code from becoming too large.

**Example:**
```bash
# A program that generates 200 MB of assembly would be rejected
# Error: Generated ASM output too large: 209715200 bytes > 104857600 bytes limit
```

**Why:** Prevents memory exhaustion during compilation.

---

## Execution Timeout

**Limit:** 30 seconds maximum execution time

**What it does:** Stops programs that run too long.

**Example:**
```flux
void main() {
    // If this takes longer than 30 seconds...
    long_running_operation();
    // ⚠️  Warning: Process took longer than 30 seconds
}
```

**Why:** Prevents fork bombs and runaway processes.

---

## Symlink Protection

**Limit:** Symlinks are blocked entirely

**What it does:** Prevents reading through symbolic links (which could be security risks).

**Example:**
```bash
# ❌ This will be rejected
ln -sf /etc/passwd password.fsh
fluxc compile password.fsh
# Error: Symlinks are not allowed: "password.fsh"
```

**Why:** Prevents symlink attacks that could trick the compiler into reading sensitive files.

---

## Path Traversal Protection

**Limit:** Cannot use `../` to escape directory

**What it does:** Prevents writing files outside intended directories.

**Example:**
```bash
# ❌ This will be rejected
fluxc compile test.fsh -o ../../etc/passwd
# Error: Path traversal detected: "../../etc/passwd" contains '..'
```

**Why:** Prevents accidental or malicious overwriting of system files.

---

## Input Validation

**What it does:** Ensures file paths are safe and legitimate.

**Checks:**
- ✅ File exists and is readable
- ✅ File is regular file (not device, pipe, etc.)
- ✅ File doesn't exceed size limits
- ✅ Path doesn't contain traversal attempts

---

## 🛡️ Security Best Practices

### 1. Validate Input Parameters

```flux
void divide(int a, int b) {
    // ✅ Good: Check before dividing
    if (b == 0) {
        serial_print("Error: Cannot divide by zero");
        return;
    }
    int result = a / b;
    serial_print(result);
}

// ❌ Bad: No validation
void unsafe_divide(int a, int b) {
    int result = a / b;  // Crashes if b=0
    serial_print(result);
}
```

### 2. Use Meaningful Variable Names

```flux
// ✅ Good: Clear what each variable does
int user_age = 25;
string account_status = "active";
bool is_verified = true;

// ❌ Bad: Unclear purpose
int x = 25;
string y = "active";
bool z = true;
```

### 3. Keep Functions Small and Focused

```flux
// ✅ Good: Each function does one thing
void initialize_user(string name) { }
void validate_age(int age) { }
void save_user_data() { }

// ❌ Bad: Everything in one function
void do_everything() {
    // 500 lines of code mixing all concerns
}
```

### 4. Check Assumptions

```flux
// ✅ Good: Verify conditions
void calculate_discount(int count, double price_per_item) {
    if (count <= 0) {
        serial_print("Error: Invalid count");
        return;
    }
    if (price_per_item < 0) {
        serial_print("Error: Invalid price");
        return;
    }
    // Safe to calculate
}

// ❌ Bad: Assume inputs are valid
void unsafe_calculate(int count, double price) {
    // What if count is negative?
    // What if price is negative?
    double total = count * price;
    serial_print(total);
}
```

### 5. Document Complex Logic

```flux
class Algorithm {
    // Calculate Fibonacci number at position n
    // Uses iterative approach for O(n) performance
    // Parameters:
    //   n: Position in sequence (0-indexed)
    // Returns:
    //   Fibonacci number at position n
    public void fib(int n) {
        // Implementation
    }
}
```

### 6. Use Constants for Magic Numbers

```flux
// ✅ Good: Constants are meaningful
int MAX_RETRIES = 3;
int TIMEOUT_SECONDS = 30;
double TAX_RATE = 0.08;

void retry_operation() {
    int attempts = 0;
    while (attempts < MAX_RETRIES) {
        // Try operation
        attempts = attempts + 1;
    }
}

// ❌ Bad: Magic numbers are confusing
void unsafe_retry() {
    int attempts = 0;
    while (attempts < 3) {  // Why 3? What does this mean?
        // Try operation
        attempts = attempts + 1;
    }
}
```

---

## 📊 Security Features Summary

| Feature | Limit | Protects Against |
|---------|-------|------------------|
| File size | 50 MB | Memory exhaustion, DoS |
| Statements | 10,000 | Infinite loops, freezing |
| Operators | 1,000 | Expression bombs |
| ASM output | 100 MB | Compilation overflow |
| Timeout | 30 seconds | Runaway processes, fork bombs |
| Symlinks | Blocked | Symlink attacks |
| Path traversal | Blocked | Directory escape, file overwrite |

---

## ✅ Security Score

**FluxSharp Security:** 8/10

**What's Protected:**
- ✅ File size validation
- ✅ Infinite loop prevention
- ✅ Path traversal blocking
- ✅ Symlink blocking
- ✅ Execution timeout
- ✅ Memory limits

**What Requires External Setup:**
- ⏳ Complete sandbox (requires OS-level seccomp/pledge)
- ⏳ Resource quotas (requires cgroups/ulimit)
- ⏳ Dependency signature verification

---

## 🔧 Compiler Security Options

All security features are **enabled by default** and cannot be disabled.

This is intentional - safety is not optional in FluxSharp.

---

## 🚀 Future Security Improvements

- ✅ Sandbox support with seccomp/pledge
- ✅ Memory usage quotas via cgroups
- ✅ CPU usage limits
- ✅ File descriptor limits
- ✅ Audit logging of all operations

---

## Summary

FluxSharp protects you with multiple layers of security by default. Follow best practices and your programs will be safe, fast, and reliable.

**Remember:** Security is a feature, not an afterthought!

---

See **LANGUAGE_GUIDE.md** for more information on using FluxSharp safely.

