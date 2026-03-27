# FluxSharp Arrays

Arrays and fixed-size collections.

## Array Declaration

### Basic Array Declaration

```flux
int[100] numbers;           // Array of 100 integers
string[256] names;          // Array of 256 strings
double[1000] values;        // Array of 1000 doubles
```

### Array with Expression Size

```flux
byte[80 * 25] screen;       // 2000 bytes (80 * 25)
int[512] buffer;            // 512 integers
byte[4 * 1024] page;        // 4096 bytes (4 * 1024)
```

### Array of Arrays (Multi-dimensional)

```flux
int[10][20] matrix;         // 2D array: 10x20
byte[100][100] grid;        // 2D grid
int[5][4][3] cube;          // 3D array
```

## Array Initialization

### Declaration and Assignment

```flux
int[10] numbers;
numbers[0] = 10;
numbers[1] = 20;
numbers[9] = 100;
```

### Multi-dimensional Initialization

```flux
int[3][3] matrix;
matrix[0][0] = 1;
matrix[0][1] = 2;
matrix[1][0] = 3;
matrix[2][2] = 9;
```

## Array Access

### Single Index Access

```flux
int[10] arr;
arr[0] = 42;            // Set element
int value = arr[0];     // Get element
```

### Using Variables as Index

```flux
int[100] data;
int index = 5;
data[index] = 100;      // Access with variable
int val = data[index];
```

### Using Expressions as Index

```flux
int[10] arr;
int i = 0;
arr[i + 1] = 10;        // Access with expression
arr[i * 2] = 20;
```

### Multi-dimensional Access

```flux
int[10][20] matrix;
matrix[0][0] = 1;
matrix[5][10] = 100;
int value = matrix[3][7];
```

## Array in Loops

### For Loop with Array

```flux
int[10] numbers;
numbers[0] = 10;
numbers[1] = 20;
numbers[2] = 30;

for (int i = 0; i < 10; i++) {
    numbers[i] = i * 10;
}
```

### While Loop with Array

```flux
byte[256] buffer;
int i = 0;
while (i < 256) {
    buffer[i] = 0;
    i++;
}
```

### Processing Array Elements

```flux
int[100] values;
// Initialize...

int sum = 0;
for (int i = 0; i < 100; i++) {
    sum += values[i];
}
```

## Array Types

### Primitive Arrays

```flux
int[100] integers;
float[50] decimals;
double[200] precise;
string[16] texts;
bool[8] flags;
byte[256] bytes;
```

### Struct Arrays

```flux
public struct Point {
    int x;
    int y;
}

Point[100] points;
points[0].x = 10;
points[0].y = 20;
```

### Class Arrays

```flux
public class Person {
    public string name;
    public int age;
}

Person[50] people;
people[0].name = "Alice";
people[0].age = 30;
```

## Multi-dimensional Arrays

### 2D Arrays

```flux
int[10][10] grid;
grid[0][0] = 1;
grid[5][5] = 100;
int val = grid[3][7];
```

### 3D Arrays

```flux
byte[16][16][16] volume;
volume[0][0][0] = 255;
byte b = volume[8][8][8];
```

### Array of Arrays

```flux
int[4][3] rows;         // 4 arrays of 3 elements
for (int i = 0; i < 4; i++) {
    for (int j = 0; j < 3; j++) {
        rows[i][j] = i * j;
    }
}
```

## Array Size Expressions

From grammar:

```
size_expr = { int_literal ~ (("*" | "+") ~ int_literal)* }
```

Valid size expressions:

```flux
int[100] simple;
int[80 * 25] calculated;        // 2000
int[512] block;
int[256 + 256] combined;        // 512
byte[4 * 1024] page;            // 4096
int[16 + 32 + 64] mixed;        // 112
```

## Array Assignment

### Assignment Statement

```flux
int[10] arr;
arr[0] = 42;
arr[5] = 100;
arr[9] = 1;
```

### Array Access Assignment

From grammar:

```
array_access = { ident ~ "[" ~ (int_literal | ident) ~ "]" ~ "=" ~ expr ~ ";" }
```

Examples:

```flux
int[100] data;
data[0] = 10;           // Literal index
data[i] = 20;           // Variable index
data[i + 1] = 30;       // Expression not fully supported by grammar
```

## Common Array Patterns

### Initialize Array

```flux
int[10] values;
for (int i = 0; i < 10; i++) {
    values[i] = i * 10;
}
```

### Copy Array Elements

```flux
int[10] source;
int[10] dest;

for (int i = 0; i < 10; i++) {
    dest[i] = source[i];
}
```

### Find Maximum

```flux
int[100] numbers;
// Initialize...

int max = numbers[0];
for (int i = 1; i < 100; i++) {
    if (numbers[i] > max) {
        max = numbers[i];
    }
}
```

### Calculate Sum

```flux
int[100] values;
// Initialize...

int sum = 0;
for (int i = 0; i < 100; i++) {
    sum += values[i];
}
```

### Matrix Operations

```flux
int[10][10] matrix;

// Initialize
for (int i = 0; i < 10; i++) {
    for (int j = 0; j < 10; j++) {
        matrix[i][j] = i + j;
    }
}

// Process
for (int i = 0; i < 10; i++) {
    for (int j = 0; j < 10; j++) {
        ProcessValue(matrix[i][j]);
    }
}
```

## Array Memory

Arrays are:
- Fixed size (determined at compile time)
- Zero-indexed (0 to size-1)
- Contiguous in memory
- Bounds checked at runtime

```flux
byte[80 * 25] vga;      // 2000 bytes contiguous
int[100] data;          // 800 bytes (8 per int)
string[10] strs;        // 10 string pointers
```

---

Next: Read [ASYNC_AWAIT.md](ASYNC_AWAIT.md)

