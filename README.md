# Rust Calculator

A powerful command-line calculator written in Rust that evaluates mathematical expressions with proper operator precedence and parentheses support.

## Features

- ✅ **Basic Arithmetic**: Addition (+), Subtraction (-), Multiplication (*), Division (/)
- ✅ **Operator Precedence**: Follows standard mathematical order of operations
- ✅ **Parentheses Support**: Full support for nested parentheses grouping
- ✅ **Decimal Numbers**: Supports floating-point calculations
- ✅ **Error Handling**: Comprehensive error detection and reporting
- ✅ **Whitespace Tolerant**: Handles spaces, tabs, and newlines gracefully

## Quick Start

### Prerequisites
- Rust 1.70 or later installed on your system
- Cargo (comes with Rust)

### Installation & Running

```bash
# Clone or navigate to the project directory
cd calculator

# Run the calculator
cargo run

# Run tests
cargo test

# Build optimized binary
cargo build --release
```

## Usage Examples

The calculator evaluates expressions and prints results:

```
Calculating: 2*2 + 48/4
Result: 16

3 + 4 * 2 = 11
(3 + 4) * 2 = 14
10 / 2 - 3 = 2
2.5 * 4 + 1.5 = 11.5
((2 + 3) * 4) / 5 = 4
```

### Supported Operations

| Operation | Symbol | Example | Result |
|-----------|--------|---------|--------|
| Addition | `+` | `5 + 3` | `8` |
| Subtraction | `-` | `10 - 4` | `6` |
| Multiplication | `*` | `7 * 6` | `42` |
| Division | `/` | `15 / 3` | `5` |
| Parentheses | `()` | `(2 + 3) * 4` | `20` |

### Expression Examples

```rust
// Basic arithmetic
2 + 3           // = 5
10 - 4          // = 6
3 * 4           // = 12
15 / 3          // = 5

// Operator precedence
2 + 3 * 4       // = 14 (not 20)
10 - 6 / 2      // = 7 (not 2)

// Parentheses override precedence
(2 + 3) * 4     // = 20
2 * (3 + 4)     // = 14

// Nested parentheses
((2 + 3) * 4) / 5           // = 4
((2 + 3) * (4 + 1))         // = 25

// Decimal numbers
2.5 + 3.7       // = 6.2
10.5 / 2.1      // = 5
3.14 * 2        // = 6.28

// Complex expressions
1 + 2 * 3 + 4           // = 11
(1 + 2) * (3 + 4)       // = 21
10 + 5 * 2 - 3 / 3      // = 19
```

## Architecture

The calculator uses a three-phase approach:

### 1. **Parsing** (`parse` function)
- Converts input string into tokens
- Handles multi-digit numbers and decimals
- Validates parentheses matching
- Detects invalid characters

### 2. **Infix to Postfix Conversion** (`to_postfix` function)
- Implements the Shunting Yard algorithm
- Converts infix notation (2 + 3) to postfix notation (2 3 +)
- Handles operator precedence and associativity

### 3. **Evaluation** (`evaluate` function)
- Evaluates postfix expressions using a stack
- Performs the actual arithmetic operations
- Returns the final result

## Error Handling

The calculator provides detailed error messages for various scenarios:

| Error Type | Description | Example |
|------------|-------------|---------|
| `BadToken` | Invalid character in expression | `2 + @` → `BadToken('@')` |
| `MismatchedParens` | Unbalanced parentheses | `(2 + 3` → `MismatchedParens` |
| `DivisionByZero` | Division by zero detected | `5 / 0` → `DivisionByZero` |
| `InvalidExpression` | Malformed expression | Empty input → `InvalidExpression` |

## Code Structure

```
src/
├── main.rs                 # Main calculator implementation
│   ├── Operator enum       # Mathematical operators (+, -, *, /)
│   ├── Token enum          # Expression tokens (numbers, operators, brackets)
│   ├── Error enum          # Error types
│   ├── Calculator struct   # Main calculator logic
│   └── tests module        # Comprehensive test suite
```

### Key Components

- **`Operator`**: Enum representing mathematical operations with precedence logic
- **`Token`**: Enum for parsed expression elements (numbers, operators, brackets)
- **`Calculator`**: Main struct containing parsing and evaluation logic
- **`Error`**: Comprehensive error handling for various failure cases

## Testing

The project includes 18 comprehensive tests covering:

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test precedence      # Operator precedence tests
cargo test parentheses     # Parentheses handling tests
cargo test error           # Error handling tests

# Run with verbose output
cargo test -- --nocapture
```

### Test Coverage
- ✅ Basic arithmetic operations
- ✅ Operator precedence rules
- ✅ Parentheses and nesting
- ✅ Decimal number support
- ✅ Whitespace handling
- ✅ Error conditions
- ✅ Complex expressions
- ✅ Edge cases

## API Reference

### Main Functions

```rust
// Parse expression into tokens
pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error>

// Convert infix to postfix notation
pub fn to_postfix(tokens: Vec<Token>) -> Vec<Token>

// Evaluate postfix expression
pub fn evaluate(tokens: Vec<Token>) -> Result<f64, Error>

// Complete calculation (parse + convert + evaluate)
pub fn calculate<T: AsRef<str>>(expr: T) -> Result<f64, Error>
```

### Usage in Code

```rust
use calculator::Calculator;

// Simple calculation
let result = Calculator::calculate("2 + 3 * 4")?;
println!("Result: {}", result); // Result: 14

// Step-by-step processing
let tokens = Calculator::parse("(2 + 3) * 4")?;
let postfix = Calculator::to_postfix(tokens);
let result = Calculator::evaluate(postfix)?;
```

## Implementation Details

### Operator Precedence
- **Level 2**: `*` (multiply), `/` (divide)
- **Level 1**: `+` (add), `-` (subtract)

### Algorithm: Shunting Yard
The calculator uses Dijkstra's Shunting Yard algorithm to convert infix expressions to postfix notation, which allows for efficient evaluation while respecting operator precedence and parentheses.

### Memory Safety
- Written in safe Rust with no unsafe blocks
- Comprehensive error handling prevents panics
- Stack-based evaluation ensures memory efficiency

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Submit a pull request

## License

This project is open source and available under the MIT License.

## Future Enhancements

Potential improvements for future versions:
- [ ] Support for negative numbers (e.g., `-5 + 3`)
- [ ] Exponentiation operator (`^` or `**`)
- [ ] Mathematical functions (sin, cos, sqrt, etc.)
- [ ] Variable support
- [ ] Interactive REPL mode
- [ ] Scientific notation support 