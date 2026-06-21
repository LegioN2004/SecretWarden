build:
cargo build --workspace --release

install:
cp target/release/swd ~/Library/Application\ Support/secretwarden/bin/

start:
launchctl bootstrap gui/$$(id -u) ~/Library/LaunchAgents/com.secretwarden.daemon.plist

stop:
launchctl bootout gui/$$(id -u) ~/Library/LaunchAgents/com.secretwarden.daemon.plist

restart:
$(MAKE) stop
$(MAKE) start

logs:
tail -f ~/Library/Application\ Support/secretwarden/logs/daemon.log
