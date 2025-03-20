# Unit Conversion API

A comprehensive Rust-based microservice for unit conversion, built with Actix Web. This API provides a flexible and extensible system for converting between various units across multiple measurement categories.

## Features

- Convert between units in the same category
- Compare measurements to determine relationships (equal, less than, greater than)
- Support for 11 measurement categories with numerous units in each
- Extensible architecture for adding new unit types
- RESTful API with JSON request/response format
- Configurable runtime settings via environment variables
- Health check endpoint for monitoring

## Supported Unit Categories

The API currently supports the following measurement categories:

1. **Length** - meters, kilometers, miles, feet, inches, etc.
2. **Mass** - kilograms, grams, pounds, ounces, tons, etc.
3. **Volume** - liters, gallons, cubic meters, cubic feet, etc.
4. **Temperature** - Celsius, Fahrenheit, Kelvin
5. **Area** - square meters, square feet, acres, hectares, etc.
6. **Time** - seconds, minutes, hours, days, etc.
7. **Speed** - meters per second, kilometers per hour, miles per hour, etc.
8. **Pressure** - pascal, bar, psi, atmosphere, etc.
9. **Energy** - joules, calories, kilowatt-hours, BTU, etc.
10. **Power** - watts, kilowatts, horsepower, etc.

## Prerequisites

- Rust (latest stable version)
- Cargo

## Installation and Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/unit-conversion-api.git
   cd unit-conversion-api
   ```

2. Set up environment variables (optional):
   Create a `.env` file in the root directory with the following variables:
   ```
   HOST=127.0.0.1
   PORT=8080
   LOG_LEVEL=info
   RUN_MODE=development
   ```

3. Build and run the service:
   ```bash
   cargo run
   ```

The API will be available at http://127.0.0.1:8080 by default.

## API Documentation

### Base Endpoints

- `GET /` - Root endpoint, returns a welcome message
- `GET /api/health` - Health check endpoint, returns service status

### Unit Category Endpoints

- `GET /api/categories` - Lists all available unit categories
  
  **Response Example:**
  ```json
  {
    "categories": [
      "length", "mass", "volume", "temperature", "area",
      "time", "speed", "pressure", "energy", "power"
    ]
  }
  ```

- `GET /api/categories/{category}/units` - Lists all units for a specific category
  
  **Response Example (for length):**
  ```json
  {
    "category": "length",
    "units": [
      {"name": "meter", "symbol": "m"},
      {"name": "kilometer", "symbol": "km"},
      {"name": "centimeter", "symbol": "cm"},
      {"name": "millimeter", "symbol": "mm"},
      {"name": "inch", "symbol": "in"},
      {"name": "foot", "symbol": "ft"},
      {"name": "yard", "symbol": "yd"},
      {"name": "mile", "symbol": "mi"}
    ]
  }
  ```

### Conversion Endpoints

- `POST /api/convert` - Converts a value from one unit to another
  
  **Request:**
  ```json
  {
    "value": 100,
    "from_category": "length",
    "from_unit": "cm",
    "to_unit": "m"
  }
  ```
  
  **Response:**
  ```json
  {
    "from_value": 100,
    "from_unit": "cm",
    "to_value": 1,
    "to_unit": "m",
    "category": "length"
  }
  ```

- `POST /api/compare` - Compares two measurements
  
  **Request:**
  ```json
  {
    "value1": 1,
    "unit1": "m",
    "value2": 100,
    "unit2": "cm",
    "category": "length"
  }
  ```
  
  **Response:**
  ```json
  {
    "result": 0,
    "relation": "equal"
  }
  ```
  
  *Note: Result values: -1 (less than), 0 (equal), 1 (greater than)*

## Configuration

The service can be configured via environment variables:

| Variable   | Description                      | Default        |
|------------|----------------------------------|----------------|
| HOST       | The host to bind to              | 127.0.0.1      |
| PORT       | The port to bind to              | 8080           |
| LOG_LEVEL  | The log level                    | info           |
| RUN_MODE   | The running mode                 | development    |

## Development

To run the service in development mode with auto-reload:

```bash
cargo watch -x run
```

To run tests:

```bash
cargo test
```

## Architecture

The unit conversion system is based on a flexible, extensible design:

- `UnitCategory` - A trait representing categories of units
- `Unit` - A trait representing specific units within a category
- `Measurement` - A generic struct representing a value with a specific unit

This design allows for easy extension with new unit types and categories.

### Adding New Units

To add new unit types:

1. Implement the `UnitCategory` trait for your category
2. Implement the `Unit` trait for each specific unit
3. Register the new units in the registry system

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request 