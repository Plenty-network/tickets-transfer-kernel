build:
	cargo build --target wasm32-unknown-unknown --release
	wasm-strip ./target/wasm32-unknown-unknown/release/tickets_tz_kernel.wasm
	
	smart-rollup-installer get-reveal-installer \
	--upgrade-to ./target/wasm32-unknown-unknown/release/tickets_tz_kernel.wasm \
	--output ./installer_output/installer.hex \
	--preimages-dir ./installer_output/preimages

# prepare:
	

debug-kernel:
	octez-smart-rollup-wasm-debugger ./target/wasm32-unknown-unknown/release/tickets_tz_kernel.wasm --inputs ./debug/inputs.json