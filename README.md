# HTML Parser and Browser Engine

A simple HTML parser and browser engine written in Rust. This project parses HTML documents, builds a DOM tree, and creates a layout tree for rendering.

## Features

- HTML parsing with support for basic elements and text nodes
- DOM tree construction
- Layout tree generation for visual representation
- Simple rendering of the layout tree in the console

## Project Structure

- `src/dom.rs`: DOM node and element data structures
- `src/html.rs`: HTML parsing functionality
- `src/layout.rs`: Layout tree generation and rendering
- `src/main.rs`: Main application entry point
- `tests/`: Unit tests for HTML parsing and layout

## Usage

1. Clone the repository:
   ```
   git clone https://github.com/Electroww/html-parser.git
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the project:
   ```
   cargo run
   ```

   This will parse the included `src/index.html` file and display the DOM and layout trees in the console.

## Testing

Run the tests with:
```
cargo test
```

## License

This project is open source and available under the MIT License.
