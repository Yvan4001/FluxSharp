// Array Operations Example
// Demonstrates array usage and bounds checking

public class Main {
    public void main() {
        // Create an array of 5 integers
        int[5] numbers;
        
        // Initialize array elements
        numbers[0] = 10;
        numbers[1] = 20;
        numbers[2] = 30;
        numbers[3] = 40;
        numbers[4] = 50;
        
        // Print each element
        print("Array elements:");
        print(numbers[0]);
        print(numbers[1]);
        print(numbers[2]);
        print(numbers[3]);
        print(numbers[4]);
        
        // Calculate sum
        int sum = 0;
        for (int i = 0; i < 5; i = i + 1) {
            sum = sum + numbers[i];
        }
        
        print("Sum of array: ");
        print(sum);
    }
}

