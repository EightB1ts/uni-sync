#!/bin/bash

# Check for Rust Function
check_rust() {
	echo '🦀 Checking for Rust'
	if ! command -v rustc &>/dev/null; then
		echo '⛔ Could not locate Rust Compiler ⛔'
		echo 'Try running:'
		echo 'curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh'
		exit
	else
		echo '✅ Its Rusty!'
	fi
}

build_app() {

	if [ -f "uni-sync" ]; then
		rm uni-sync
	fi

	echo '⚒️  Building Uni-Sync'
	cargo build --release

	if [ ! -f "target/release/uni-sync" ]; then
		echo '⛔ Build Failed ⛔'
		exit
	fi

	mv target/release/uni-sync .
	rm -rf ./target

}

install_app() {
	cat >'uni-sync.service' <<SERVICE
[Unit]
Description=Uni-Sync service
[Service]
ExecStart=/usr/local/bin/uni-sync
[Install]
WantedBy=multi-user.target
SERVICE
	sudo mv -f uni-sync.service /usr/lib/systemd/system
	sudo mv -f uni-sync /usr/local/bin
	sudo cp -n uni-sync.json /usr/local/bin
	sudo chown $USER /usr/local/bin/uni-sync.json
	sudo restorecon /usr/lib/systemd/system/uni-sync.service
	sudo restorecon /usr/local/bin/uni-sync
	sudo restorecon /usr/local/bin/uni-sync.json
	sudo systemctl daemon-reload
	sudo systemctl enable uni-sync
	sudo systemctl restart uni-sync
}

check_rust
build_app

read -p "Would you like to install as Service? [N/y]: " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
	install_app
fi
