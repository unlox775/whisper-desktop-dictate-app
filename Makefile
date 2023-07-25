.PHONY: release

release:
	rm -Rf target/release/WhisperDesktopDictate.app ./WhisperDesktopDictate.app
	mkdir -p target/release/WhisperDesktopDictate.app/Contents/MacOS
	mkdir -p target/release/WhisperDesktopDictate.app/Contents/Resources
	cp release/macos/Info.plist target/release/WhisperDesktopDictate.app/Contents/
	bin/generate-iconset
	cp target/release/WhisperDesktopDictate.icns target/release/WhisperDesktopDictate.app/Contents/Resources/
	cp -rp assets/* target/release/WhisperDesktopDictate.app/Contents/Resources/
	cargo build --release
	cp target/release/transcribe-rust target/release/WhisperDesktopDictate.app/Contents/MacOS/WhisperDesktopDictate
	ln -sn target/release/WhisperDesktopDictate.app
