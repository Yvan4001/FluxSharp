# FluxSharp Control Flow

If statements, loops, and flow control.

## If Statements

### Basic If

```flux
if (condition) {
    // Execute if condition is true
}
```

### If-Else

```flux
if (x > 10) {
    PrintLine("Greater");
} else {
    PrintLine("Not greater");
}
```

### If-Else If

```flux
if (score >= 90) {
    PrintLine("A");
} else if (score >= 80) {
    PrintLine("B");
} else if (score >= 70) {
    PrintLine("C");
} else {
    PrintLine("F");
}
```

### Nested If

```flux
if (age >= 18) {
    if (has_license) {
        PrintLine("Can drive");
    } else {
        PrintLine("Need license");
    }
}
```

## Conditions

### Simple Comparison

```flux
if (x > 5) { }
if (x < 10) { }
if (x == 5) { }
if (x != 5) { }
if (x >= 5) { }
if (x <= 5) { }
```

### Logical Operators

```flux
if (x > 5 && y < 10) {       // AND
    // Both conditions true
}

if (x > 5 || y < 10) {       // OR
    // At least one true
}

if (!(x > 5)) {              // NOT
    // Condition false
}
```

### Complex Conditions

```flux
if (age >= 18 && has_license && !is_suspended) {
    PrintLine("Can operate");
}

if (status == 1 || status == 2 || status == 3) {
    PrintLine("Valid status");
}
```

## While Loops

### Basic While

```flux
while (condition) {
    // Repeat while true
}
```

### Counter Loop

```flux
int i = 0;
while (i < 10) {
    PrintInt(i);
    i++;  // or i += 1
}
```

### Break Statement

```flux
int i = 0;
while (true) {
    if (i >= 10) {
        break;      // Exit loop
    }
    PrintInt(i);
    i++;
}
```

### Continue Statement

```flux
int i = 0;
while (i < 10) {
    i++;
    if (i % 2 == 0) {
        continue;   // Skip to next iteration
    }
    PrintInt(i);
}
```

## For Loops

### Basic For Loop

```flux
for (int i = 0; i < 10; i++) {
    PrintInt(i);
}
```

### For with Different Operators

```flux
for (int i = 10; i > 0; i--) {
    // Countdown
    PrintInt(i);
}

for (int i = 0; i < 100; i += 5) {
    // Step by 5
    PrintInt(i);
}
```

### For with Complex Condition

```flux
for (int i = 0; i < length && found == false; i++) {
    // More complex condition
}
```

### Nested For Loops

```flux
for (int i = 0; i < 3; i++) {
    for (int j = 0; j < 3; j++) {
        PrintInt(i);
        PrintInt(j);
    }
}
```

### For with Break

```flux
for (int i = 0; i < 100; i++) {
    if (i == 50) {
        break;      // Exit loop
    }
    PrintInt(i);
}
```

### For with Continue

```flux
for (int i = 0; i < 10; i++) {
    if (i % 2 == 0) {
        continue;   // Skip even numbers
    }
    PrintInt(i);
}
```

## Return Statement

### Return from Function

```flux
public int GetValue() {
    return 42;
}

public void DoWork() {
    return;         // Exit without value
}
```

### Early Return

```flux
public int Validate(int value) {
    if (value < 0) {
        return -1;      // Early exit
    }
    
    if (value > 100) {
        return 100;     // Cap at 100
    }
    
    return value;
}
```

## Break and Continue

### Break

```flux
for (int i = 0; i < 100; i++) {
    if (found) {
        break;      // Exit loop
    }
}

while (true) {
    if (done) {
        break;      // Exit infinite loop
    }
}
```

### Continue

```flux
for (int i = 0; i < 10; i++) {
    if (i == 5) {
        continue;   // Skip iteration 5
    }
    ProcessItem(i);
}

while (processing) {
    if (IsInvalid()) {
        continue;   // Skip to next iteration
    }
    DoWork();
}
```

## Control Flow Examples

### Simple Validation

```flux
public int ValidateAge(int age) {
    if (age < 0) {
        return -1;
    }
    if (age > 150) {
        return -1;
    }
    return age;
}
```

### Count to Target

```flux
int count = 0;
while (count < 100) {
    count++;
    if (count % 10 == 0) {
        PrintInt(count);
    }
}
```

### Matrix Processing

```flux
for (int i = 0; i < 10; i++) {
    for (int j = 0; j < 10; j++) {
        if (matrix[i][j] == 0) {
            continue;   // Skip zeros
        }
        ProcessValue(matrix[i][j]);
    }
}
```

### Search Loop

```flux
int target = 42;
int found_index = -1;

for (int i = 0; i < 100; i++) {
    if (array[i] == target) {
        found_index = i;
        break;
    }
}
```

### Conditional Logic

```flux
if (hour >= 6 && hour < 12) {
    PrintLine("Morning");
} else if (hour >= 12 && hour < 18) {
    PrintLine("Afternoon");
} else if (hour >= 18 && hour < 22) {
    PrintLine("Evening");
} else {
    PrintLine("Night");
}
```

## Condition Types

From grammar:

```
condition = {
    "(" ~ condition ~ ")"                      // Parentheses
    | operand ~ comparison_op ~ operand ...    // Comparison
    | operand ~ logical_op ~ condition         // Logical
    | operand                                  // Simple operand
}
```

Supported:
- Comparisons: `<`, `>`, `<=`, `>=`, `==`, `!=`
- Logical: `&&`, `||`
- Parentheses for grouping

---

Next: Read [OPERATORS.md](OPERATORS.md)

