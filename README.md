# Voice Typing Application

This is a cross-platform voice typing application, allowing users to dictate text using their voice instead of typing on a keyboard. It is built with **Tauri**, **TypeScript**, **Vite**, and **Rust**, with **Whisper** (a state-of-the-art speech recognition model) integrated for accurate transcription. The application currently supports Windows and is still under active development.

## Features

- **Voice-to-Text**: Dictate text directly into any text field or editor, replacing manual typing.
- **Whisper Integration**: Uses OpenAI's Whisper model for accurate voice recognition.
- **Cross-Platform**: Built with Tauri, meaning it can be packaged for other platforms in the future (currently Windows).
- **Fast & Lightweight**: Built with TypeScript, Vite for fast frontend development, and Rust for a highly efficient backend.

## Installation

### Prerequisites

To build and run this application locally, you need:

- [Rust](https://www.rust-lang.org) (for the backend)
- [Tauri CLI](https://tauri.app)
- [bun](https://bun.sh) (for managing frontend dependencies)

### Get Started

Before running the application, you need to download the Whisper model. Use the provided script to do so:

1. Download the Whisper model by running the following command:

    ```bash
    ./scripts/download-ggml.sh
    ```

2. Install the dependencies using **bun**:

    ```bash
    bun install
    ```

3. To run the application in development mode, use the following command:

    ```bash
    bun run tauri dev
    ```

This will start the app in development mode, and you can begin using the voice-to-text functionality.

## Usage

- **Start Dictating**: Once the app is running, click the microphone button to start dictating.
- **Stop Dictating**: Click the stop button to end your speech-to-text session.
- The text will appear in the active text field, replacing the need for manual typing.

## Current Limitations

- **Platform**: Currently, the application is only available for **Windows**.
- **In Development**: The app is still under development, and some features may be subject to change.
- **Accuracy**: While Whisper provides high-quality transcription, it's not perfect and might require improvements.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Feel free to fork the repository and submit pull requests. If you find any issues or have suggestions, please open an issue.

## Acknowledgments

- **Whisper**: Speech recognition model by OpenAI that powers the voice-to-text feature in this app.
- **Tauri**: A framework for building small, secure, and cross-platform desktop apps with web technologies.
- **Vite**: A fast build tool that ensures quick development feedback.
- **Rust**: The backend language, ensuring high performance and reliabi
