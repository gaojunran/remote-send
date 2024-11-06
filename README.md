# remote-send

A Rust CLI for temporarily exchanging files among all your devices.

Based on S3 storage, which needs to be configured by yourself.

## Development Status

This project is still in development.

Supported features:
- Upload a file to S3 storage.(`rs send` command)
- Download a file from S3 storage.(`rs recv` command)
- List all the files in S3 storage.(`rs list` command)

Unsupported features:
- Status progress bar while transferring.
- Upload/Download multiple files at once, or a directory.
- Tauri GUI.
- ...

## Learn More about this project

This project is, meanwhile, a good learning material for those 
who begins to work with Rust and other modern skills.

The whole project is divided into 3 main parts: 

- lib: The library that contains the core functionalities of the project.
- cli: The command-line interface that interacts with the user and uses the library. Support all the platforms.
- tauri: The frontend of the project for those who want to use it as a desktop app. Support all the platforms including mobile.