# 01Edu Docker Test

A utility to run 01Edu platform tests locally on your machine.

```
      ⬤⬤⬤     ⬤  
     ⬤   ⬤     ⬤  
     ⬤   ⬤     ⬤  
     ⬤   ⬤     ⬤  
      ⬤⬤⬤     ⬤  

    -- 01Edu Docker Test --
```

## Overview

This utility allows you to test your exercises locally using the same test environment as the 01Edu platform. It uses Docker to create a consistent testing environment regardless of your local setup.

## Prerequisites

- Docker installed and running on your machine
- Rust and Cargo installed on your machine
- Your exercise files ready for testing

## Installation

1. Clone this repository or download the utility
2. Navigate to the directory containing the utility code
3. Install using Cargo:
   ```
   cargo install --path .
   ```
4. The executable will be available in your PATH after installation

## Usage

```
piscine-test --exercise <EXERCISE_NAME> --language <LANGUAGE>
```

Or using short options:

```
piscine-test -e <EXERCISE_NAME> -l <LANGUAGE>
```

### Supported Languages

Currently, the utility supports the following languages:

- Java
- Dart

### Testing Java Exercises

For Java exercises:

1. Make sure you have a `student` directory in your current working directory
2. Copy your Java exercise directory into the `student` directory
3. Run the command:
   ```
   piscine-test -e <EXERCISE_NAME> -l java
   ```

The utility will mount your local `student` directory to `/app/student` in the Docker container.

### Testing Dart Exercises

For Dart exercises:

1. Make sure your Dart file is named exactly like the exercise (e.g., `intro.dart` for the "intro" exercise)
2. Run the command from the directory containing your Dart file:
   ```
   piscine-test -e <EXERCISE_NAME> -l dart
   ```

The utility will mount your local Dart file to the appropriate location in the Docker container.

## Features

- Automatic Docker image pulling if the required image doesn't exist locally
- Progress indication during image download
- Clear success/failure messages

## Troubleshooting

If you encounter issues:

- Make sure Docker is running
- Verify that your exercise files are in the correct location
- Ensure your exercise files have the correct name format
- Check that you have sufficient permissions to run Docker commands

## License

This utility is provided as-is under the terms specified by 01Edu.