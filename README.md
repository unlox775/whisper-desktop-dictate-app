# Whisper Desktop Dictate

Whisper Desktop Dictate is a prototype desktop application built with Rust and the OpenAI Whisper API. When launched, the application displays a microphone icon and begins listening for audio input. Once you finish speaking, simply click on the microphone icon. The application will then transition to a "transcribing" state, during which it leverages the power of the Whisper API to transcribe the audio input. As soon as the transcription process is complete, the application window closes, and the transcribed text is copied to your clipboard for easy pasting elsewhere.

## Features

- Instant transcription: Click the microphone icon when you are done recording to quickly transcribe your spoken words.
- Easy to use: Once the transcription is done, the transcribed text is automatically copied to your clipboard for immediate use.
- Icon transitions: The application uses intuitive icons to communicate its current state (listening, transcribing).

## Current Status

This application is currently in prototype stage. To use it, you need to launch the program using `cargo run` in your terminal. This will prompt the system to request microphone access.

Please note that although a macOS app bundle can be created using `make release`, the bundled app currently does not request access to the microphone due to macOS-level restrictions. Work is ongoing to integrate the necessary macOS frameworks to enable this functionality.

## Installation

To install and use this application, follow these steps:

1. Clone this repository.
2. Install Rust and Dependencies:
   - `brew install rust`
   - `brew install gtk+4`
3. Install the required dependencies: SoX and the OpenAI Whisper API.
   - Install SOX with Homebrew using `brew install sox`.
   - Install the Whisper API by following the installation steps in the [Whisper repository](https://github.com/openai/whisper) 

## Usage

1. Run `cargo run`, which will open up a window, and immediately start recording audio.
   - It may ask at this point if you wan to grant Terminal access to your microphone
2. Click on the microphone icon when you are done recording
3. It shows a transcription while it is running Whisper, speech-to-text transcription
4. When it is done, the window will close
5. The transcribed text will automatically be added to your clipboard.  Paste it into any window to see the transcription

## Notes
The Whisper API is an Automatic Speech Recognition system developed by OpenAI. It uses powerful pre-trained models that are downloaded to your machine, allowing for local transcription without the need for a network connection.

Please be aware that these models can be memory-intensive. While this application is configured to use the "small" model to balance transcription quality with resource usage, the transcription process can still take a considerable amount of time, particularly on older machines.
