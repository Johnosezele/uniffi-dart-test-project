# UniFFI Dart Custom Type Test Project

This project serves as a testbed and example for using custom types with [UniFFI](https://github.com/mozilla/uniffi-rs) and the [uniffi-dart](https://github.com/Johnosezele/uniffi-dart) plugin, specifically targeting UniFFI version 0.29+.

It demonstrates how to define a custom type in Rust that wraps a built-in UniFFI type (in this case, `MyUuid` wrapping `string`) and how to call functions using this custom type from Dart.

## Project Structure

- `src/api.udl`: The UniFFI Definition Language file that defines the FFI interface, including the custom type `MyUuid` and the function `process_uuid`.
- `src/lib.rs`: The Rust library implementing the `MyUuid` custom type and the `process_uuid` function.
- `build.rs`: The Rust build script responsible for running UniFFI's code generation (scaffolding).
- `Cargo.toml`: The Rust project's manifest file, defining dependencies, including `uniffi` and a local path dependency for `uniffi-dart` (if you're testing local changes to the plugin).
- `test_custom_type.dart`: A Dart script to test the generated FFI bindings, ensuring that the custom type can be passed to and from Rust correctly.
- `.gitignore`: Specifies intentionally untracked files that Git should ignore (e.g., build artifacts in `target/`).

## Prerequisites

- Rust toolchain (latest stable recommended)
- Dart SDK (latest stable recommended)
- UniFFI CLI (if you need to regenerate scaffolding manually, though `build.rs` handles this during `cargo build`)

## Building the Project

1.  **Clone the repository (if you haven't already):**
    ```bash
    # If you've pushed this to GitHub
    # git clone https://github.com/your-username/uniffi-dart-test-project.git
    # cd uniffi-dart-test-project
    ```

2.  **Build the Rust library and generate FFI bindings:**
    Navigate to the project root directory (`/Users/mac/Dev/uniffi-dart-test-project`) and run:
    ```bash
    cargo build
    ```
    This command will compile the Rust code and trigger the `build.rs` script, which in turn uses `uniffi-dart` to generate the Dart FFI bindings. The generated Dart file will typically be located in a path like `target/debug/build/uniffi_dart_test_lib-xxxxxxxxxxxxxxxx/out/uniffi_uniffi_dart_test_lib.dart`.

## Running the Dart Test

After successfully building the project, you can run the Dart test script to verify the FFI integration:

1.  **Ensure the import path in `test_custom_type.dart` is correct.**
    The script currently uses:
    ```dart
    import 'target/debug/build/uniffi_dart_test_lib-4a0124c54bb5a90f/out/uniffi_uniffi_dart_test_lib.dart' as uniffi;
    ```
    The hash (`4a0124c54bb5a90f`) in the path might change between builds. You may need to update this path to match the actual output directory created by `cargo build`.

2.  **Run the test script:**
    From the project root directory, execute:
    ```bash
    dart run test_custom_type.dart
    ```
    If everything is set up correctly, you should see output indicating the original and processed UUIDs, followed by "Test passed successfully!".

## Purpose

This project was created to:

- Validate the implementation of custom type support in the `uniffi-dart` plugin for UniFFI 0.29.
- Provide a clear example of how to define and use custom types with UniFFI and Dart.
- Serve as a minimal reproducible example for debugging issues related to custom types in `uniffi-dart`.
