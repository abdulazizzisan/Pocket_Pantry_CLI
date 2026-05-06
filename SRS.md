# Software Requirements Specification: Pocket Pantry CLI

## 1. Purpose

Pocket Pantry CLI is a small command-line Rust learning project for practicing the concepts covered in chapters 1 through 6 of *The Rust Programming Language*.

The project should be intentionally small, terminal-based, and implemented with the Rust standard library only. Its main goal is not production polish, but deliberate use of the following Rust concepts:

- Cargo project structure
- Variables, mutability, constants, scalar and compound data types
- Functions and expressions
- Comments and readable code organization
- Control flow with `if`, `loop`, `while`, and `for`
- Ownership, borrowing, references, and slices
- Structs and methods
- Enums and `match`
- `Option<T>`
- Basic module organization

## 2. Project Overview

Pocket Pantry CLI lets a user manage a small list of pantry items from the terminal during one program run. The user can add items, view all items, search items by name, update quantities, mark items as low stock, remove items, and view a summary.

Data does not need to persist after the program exits. This keeps the scope focused on chapters 1 through 6 and avoids file I/O, error handling patterns, traits, generics, or collections-heavy design beyond what is useful for learning.

## 3. Intended User

The intended user is a beginner Rust learner who has completed chapters 1 through 6 of the official Rust Book and wants to build a short, practical project that reinforces those concepts.

## 4. Product Scope

### In Scope

- A terminal menu interface
- In-memory pantry item management
- Struct-based item modeling
- Enum-based command and category modeling
- Pattern matching for user commands
- Functions that demonstrate ownership, borrowing, mutable references, and string slices
- Simple module separation
- Clear terminal output

### Out of Scope

- Saving or loading data from files
- External crates
- Databases
- Networking
- GUI or web interface
- Advanced error handling with `Result<T, E>`
- Lifetimes beyond ordinary borrowing
- Traits, generics, iterators as a main design focus, or smart pointers

## 5. Learning Objectives

By completing this project, the learner should be able to:

1. Create and run a Rust project with Cargo.
2. Define custom structs with named fields.
3. Implement methods using `impl`.
4. Define enums with variants that may hold data.
5. Use `match` to branch on enum values.
6. Use `Option<T>` to represent missing search results.
7. Pass values by ownership, immutable reference, and mutable reference.
8. Use `String` and `&str` appropriately.
9. Organize code into multiple modules.
10. Use loops and conditionals to build an interactive command-line flow.

## 6. Functional Requirements

### FR-1: Start Application

When the program starts, it shall display:

- Application name
- A short menu of available commands
- A prompt asking the user to choose an action

The application shall continue running until the user chooses the quit command.

### FR-2: Add Pantry Item

The user shall be able to add a pantry item.

Each item shall include:

- Name
- Quantity
- Category
- Stock status

The program shall ask the user for the item name, quantity, and category.

The program shall assign the stock status automatically:

- `InStock` if quantity is greater than the low-stock threshold
- `LowStock` if quantity is greater than zero and less than or equal to the low-stock threshold
- `OutOfStock` if quantity is zero

The low-stock threshold shall be defined as a constant.

Rust concepts practiced:

- `String`
- Numeric types
- Constants
- Struct construction
- Function parameters
- Ownership when creating a new item

### FR-3: View All Items

The user shall be able to view all pantry items.

For each item, the program shall display:

- Item number
- Name
- Quantity
- Category
- Stock status

If there are no items, the program shall display a friendly empty-state message.

Rust concepts practiced:

- Borrowing a collection immutably
- `for` loop
- Struct field access
- Formatting output

### FR-4: Search Item By Name

The user shall be able to search for an item by name.

The search shall:

- Ask the user for a search term
- Match items by exact name or case-insensitive comparison
- Return the matching item if found
- Display a not-found message if no match exists

The search function shall return `Option<&PantryItem>` or a similar `Option`-based design.

Rust concepts practiced:

- String slices
- Immutable references
- `Option<T>`
- `match`
- Function return values

### FR-5: Update Item Quantity

The user shall be able to update the quantity of an existing item.

The program shall:

- Ask for the item name
- Search for the item
- Ask for the new quantity
- Update the item if found
- Recalculate the stock status
- Display a not-found message if no matching item exists

Rust concepts practiced:

- Mutable references
- Struct methods
- `if` expressions
- `match`
- Borrowing rules

### FR-6: Remove Item

The user shall be able to remove an item by name.

The program shall:

- Ask for the item name
- Remove the matching item if found
- Display the removed item name
- Display a not-found message if no matching item exists

Rust concepts practiced:

- Ownership transfer when removing a value
- `Option<T>`
- `match`
- Vector indexing or position lookup

### FR-7: View Summary

The user shall be able to view a pantry summary.

The summary shall include:

- Total number of items
- Number of in-stock items
- Number of low-stock items
- Number of out-of-stock items

Rust concepts practiced:

- Counters with mutable variables
- `for` loop
- `match` on enum variants
- Borrowing items immutably

### FR-8: Quit Application

The user shall be able to quit the application from the main menu.

The program shall display a goodbye message before exiting.

Rust concepts practiced:

- `loop`
- `break`
- Enum-based commands
- `match`

## 7. Data Requirements

### 7.1 Pantry Item

The application shall define a `PantryItem` struct.

Suggested fields:

```rust
struct PantryItem {
    name: String,
    quantity: u32,
    category: Category,
    status: StockStatus,
}
```

### 7.2 Category

The application shall define a `Category` enum.

Suggested variants:

```rust
enum Category {
    Grain,
    Protein,
    Vegetable,
    Fruit,
    Snack,
    Other(String),
}
```

The `Other(String)` variant gives practice with enum variants that store data.

### 7.3 Stock Status

The application shall define a `StockStatus` enum.

Suggested variants:

```rust
enum StockStatus {
    InStock,
    LowStock,
    OutOfStock,
}
```

### 7.4 Command

The application shall define a `Command` enum for menu choices.

Suggested variants:

```rust
enum Command {
    Add,
    View,
    Search,
    Update,
    Remove,
    Summary,
    Quit,
    Invalid,
}
```

User input should be converted into `Command`, then handled with `match`.

## 8. Suggested Module Structure

The project should use a small module layout:

```text
src/
  main.rs
  pantry.rs
  menu.rs
```

Suggested responsibilities:

- `main.rs`: program loop and high-level command handling
- `pantry.rs`: structs, enums, and pantry item behavior
- `menu.rs`: menu display and input parsing helpers

This module structure is intentionally modest so the project stays aligned with chapter 6.

## 9. User Interface Requirements

The application shall use a text-based menu similar to:

```text
Pocket Pantry

1. Add item
2. View all items
3. Search item
4. Update quantity
5. Remove item
6. View summary
7. Quit

Choose an option:
```

The application shall accept numeric menu choices.

Invalid input shall not crash the program. The application shall display a simple invalid-choice message and return to the menu.

## 10. Non-Functional Requirements

### NFR-1: Simplicity

The implementation should prefer clear beginner-friendly Rust over clever or highly abstract code.

### NFR-2: Standard Library Only

The implementation should not use external crates.

### NFR-3: Readability

Code should use meaningful names and small functions.

### NFR-4: Chapter Alignment

The implementation should avoid concepts that are introduced after chapter 6 unless absolutely necessary.

### NFR-5: Short Build Time

The project should compile quickly with `cargo run`.

## 11. Suggested Function List

The following functions are recommended but not required exactly:

```rust
fn main()
fn print_menu()
fn read_line() -> String
fn parse_command(input: &str) -> Command
fn add_item(items: &mut Vec<PantryItem>)
fn view_items(items: &[PantryItem])
fn find_item<'a>(items: &'a [PantryItem], name: &str) -> Option<&'a PantryItem>
fn find_item_mut<'a>(items: &'a mut [PantryItem], name: &str) -> Option<&'a mut PantryItem>
fn update_quantity(items: &mut Vec<PantryItem>)
fn remove_item(items: &mut Vec<PantryItem>)
fn print_summary(items: &[PantryItem])
```

Note: explicit lifetime annotations may not be necessary depending on the final function signatures. If the compiler can infer them, prefer the simpler version.

## 12. Acceptance Criteria

The project is complete when:

1. `cargo run` starts the application successfully.
2. The main menu appears and repeats until quit.
3. A user can add at least three items.
4. A user can view all added items.
5. A user can search for an existing item and see its details.
6. Searching for a missing item displays a not-found message.
7. A user can update an item quantity.
8. Updating quantity recalculates stock status correctly.
9. A user can remove an item.
10. The summary counts all stock statuses correctly.
11. Invalid menu choices do not crash the application.
12. The code uses at least one struct, three enums, and one `match` expression.
13. The code is split into at least two modules.

## 13. Manual Test Scenarios

### Scenario 1: Add and View Items

1. Start the application.
2. Choose `Add item`.
3. Add `Rice` with quantity `5` and category `Grain`.
4. Add `Beans` with quantity `1` and category `Protein`.
5. Choose `View all items`.
6. Verify both items are displayed.

### Scenario 2: Low Stock Status

1. Add an item with quantity `1`.
2. View all items.
3. Verify the item is marked as `LowStock`.

### Scenario 3: Out Of Stock Status

1. Add an item with quantity `0`.
2. View all items.
3. Verify the item is marked as `OutOfStock`.

### Scenario 4: Search Existing Item

1. Add `Rice`.
2. Choose `Search item`.
3. Search for `Rice`.
4. Verify item details are displayed.

### Scenario 5: Search Missing Item

1. Choose `Search item`.
2. Search for `Coffee`.
3. Verify a not-found message is displayed.

### Scenario 6: Update Quantity

1. Add `Beans` with quantity `1`.
2. Update `Beans` to quantity `6`.
3. View all items.
4. Verify `Beans` now has quantity `6` and status `InStock`.

### Scenario 7: Remove Item

1. Add `Rice`.
2. Remove `Rice`.
3. View all items.
4. Verify `Rice` is no longer displayed.

### Scenario 8: Summary

1. Add one in-stock item.
2. Add one low-stock item.
3. Add one out-of-stock item.
4. Choose `View summary`.
5. Verify the summary counts are correct.

## 14. Suggested Development Milestones

### Milestone 1: Cargo Setup

- Create the Cargo project.
- Print the application name and menu.
- Implement the main loop.

### Milestone 2: Data Modeling

- Add `PantryItem`.
- Add `Category`.
- Add `StockStatus`.
- Implement a constructor-style function or method for creating items.

### Milestone 3: Commands

- Add the `Command` enum.
- Convert menu input into `Command`.
- Use `match` in the main loop.

### Milestone 4: Core Features

- Implement add item.
- Implement view items.
- Implement search item.

### Milestone 5: Mutation Features

- Implement update quantity.
- Implement remove item.
- Recalculate stock status after changes.

### Milestone 6: Summary and Cleanup

- Implement summary.
- Clean up function names and module boundaries.
- Run manual tests.

## 15. Stretch Goals

These are optional and should be attempted only after the core project works:

1. Allow partial name search instead of exact search.
2. Prevent duplicate item names.
3. Add a `describe` method to each enum for display text.
4. Let the user list only low-stock items.
5. Sort items alphabetically before displaying them.
6. Add comments explaining where ownership and borrowing are being practiced.

## 16. Recommended Constraint For Learning

After each feature, pause and answer these questions in comments or a learning journal:

1. Which values did this feature own?
2. Which functions borrowed data immutably?
3. Which functions borrowed data mutably?
4. Where did `match` make the code clearer?
5. Where did `Option<T>` help model the absence of data?

