# FluxSharp Standard Library and Math Functions

Built-in functions for common operations.

## Math Functions

Mathematical functions for numerical operations.

### Absolute Value (abs)

**Purpose**: Returns the absolute value (magnitude) of a number, removing the sign.

```flux
int result = abs(-42);      // 42
int positive = abs(10);     // 10
```

### Power (pow)

**Purpose**: Raises a number to a power. Returns base raised to the exponent.

```flux
int squared = pow(5, 2);    // 25 (5²)
int cubed = pow(2, 3);      // 8 (2³)
double result = pow(2.5, 2); // 6.25
```

### Square Root (sqrt)

**Purpose**: Returns the square root of a number.

```flux
double result = sqrt(16.0);  // 4.0
double root = sqrt(2.0);     // 1.414...
```

### Floor (floor)

**Purpose**: Returns the largest integer less than or equal to the number.

```flux
double value = 3.7;
int floored = floor(value);  // 3

double negative = -2.3;
int neg_floor = floor(negative); // -3
```

### Ceiling (ceil)

**Purpose**: Returns the smallest integer greater than or equal to the number.

```flux
double value = 3.2;
int ceiled = ceil(value);    // 4

double negative = -2.7;
int neg_ceil = ceil(negative); // -2
```

### Rounding (round)

**Purpose**: Rounds a number to the nearest integer.

```flux
double value = 3.5;
int rounded = round(value);  // 4

double smaller = 3.2;
int rounded2 = round(smaller); // 3
```

### Minimum (min)

**Purpose**: Returns the smaller of two numbers.

```flux
int smallest = min(5, 3);    // 3
int negative = min(-10, -5); // -10
double smaller = min(2.5, 2.3); // 2.3
```

### Maximum (max)

**Purpose**: Returns the larger of two numbers.

```flux
int largest = max(5, 3);     // 5
int bigger = max(-10, -5);   // -5
double larger = max(2.5, 2.3); // 2.5
```

### Absolute Difference (abs_diff)

**Purpose**: Returns the absolute difference between two numbers.

```flux
int diff = abs_diff(5, 2);   // 3
int reverse = abs_diff(2, 5); // 3 (order doesn't matter)
```

### Modulo (mod or %)

**Purpose**: Returns the remainder after division.

```flux
int remainder = 17 % 5;      // 2
int even_check = 10 % 2;     // 0 (divisible)
```

## String Functions

String manipulation functions.

### Length

**Purpose**: Returns the number of characters in a string.

```flux
int len = "hello".length();  // 5
int empty_len = "".length(); // 0
```

### Concatenation (+)

**Purpose**: Combines two strings together.

```flux
string greeting = "Hello" + " " + "World"; // "Hello World"
string message = "Count: " + "42";
```

### Substring

**Purpose**: Extracts a portion of a string from start to end position.

```flux
string text = "Hello World";
string sub = substring(text, 0, 5);  // "Hello"
string world = substring(text, 6, 11); // "World"
```

### Character At

**Purpose**: Returns the character at a specific position in a string.

```flux
string text = "Hello";
char ch = char_at(text, 0);  // 'H'
char last = char_at(text, 4); // 'o'
```

### To Uppercase

**Purpose**: Converts all characters to uppercase.

```flux
string upper = "hello".to_upper(); // "HELLO"
string mixed = "HeLLo".to_upper(); // "HELLO"
```

### To Lowercase

**Purpose**: Converts all characters to lowercase.

```flux
string lower = "HELLO".to_lower(); // "hello"
string mixed = "HeLLo".to_lower(); // "hello"
```

### String Contains

**Purpose**: Checks if a string contains another string.

```flux
bool has_world = contains("Hello World", "World");  // true
bool has_test = contains("Hello World", "test");    // false
```

### String Index Of

**Purpose**: Finds the position of a substring in a string (or -1 if not found).

```flux
int pos = index_of("Hello World", "World"); // 6
int not_found = index_of("Hello World", "xyz"); // -1
```

## Array Functions

Array manipulation functions.

### Array Length

**Purpose**: Returns the size of an array (number of elements).

```flux
int[10] arr;
int size = length(arr);     // 10

byte[256] buffer;
int buf_size = length(buffer); // 256
```

### Array Clear

**Purpose**: Sets all elements of an array to zero/default.

```flux
int[100] data;
clear(data);               // All elements become 0

string[10] names;
clear(names);              // All elements become empty string
```

### Array Copy

**Purpose**: Copies elements from one array to another.

```flux
int[10] source;
int[10] destination;

// Initialize source
for (int i = 0; i < 10; i++) {
    source[i] = i * 10;
}

// Copy to destination
copy(source, destination);
```

### Array Fill

**Purpose**: Fills an array with a specific value.

```flux
int[100] numbers;
fill(numbers, 42);         // All elements become 42

string[10] names;
fill(names, "empty");      // All elements become "empty"
```

## Type Conversion Functions

### Int to String

**Purpose**: Converts an integer to its string representation.

```flux
int number = 42;
string text = to_string(number); // "42"
string negative = to_string(-100); // "-100"
```

### String to Int

**Purpose**: Converts a string to an integer value.

```flux
string text = "42";
int value = to_int(text);  // 42

string negative = "-100";
int neg_value = to_int(negative); // -100
```

### Float to Int

**Purpose**: Converts a floating-point number to an integer (truncates decimal).

```flux
double value = 3.7;
int truncated = to_int(value); // 3

double pi = 3.14159;
int pi_int = to_int(pi);   // 3
```

### Int to Float

**Purpose**: Converts an integer to a floating-point number.

```flux
int number = 42;
double decimal = to_double(number); // 42.0

int five = 5;
float five_float = to_float(five);  // 5.0
```

## Memory and System Functions

### Memory Allocation (malloc)

**Purpose**: Allocates a block of memory of a given size.

```flux
byte[1024] buffer = malloc(1024);
```

### Memory Free (free)

**Purpose**: Frees allocated memory.

```flux
free(buffer);
```

### Sleep

**Purpose**: Pauses program execution for a specified number of milliseconds.

```flux
sleep(1000);  // Sleep for 1 second
sleep(500);   // Sleep for half a second
```

## I/O Functions

### Print

**Purpose**: Outputs text to the console/standard output.

```flux
print("Hello");
print("Count: ");
```

### Print Line

**Purpose**: Outputs text followed by a newline.

```flux
print_line("Hello World");  // Adds newline after
print_line("Line 2");
```

### Print Integer

**Purpose**: Outputs an integer value to the console.

```flux
int value = 42;
print_int(value);          // Prints "42"
```

### Print Float

**Purpose**: Outputs a floating-point number to the console.

```flux
double pi = 3.14159;
print_float(pi);           // Prints "3.14159"
```

### Read Line

**Purpose**: Reads a line of text from standard input.

```flux
string input = read_line(); // Waits for user input
```

### Read Integer

**Purpose**: Reads an integer from standard input.

```flux
int value = read_int();    // Reads integer from user
```

---

**Tip**: These built-in functions are essential for writing practical programs. Use them to manipulate data, perform calculations, and interact with users.

