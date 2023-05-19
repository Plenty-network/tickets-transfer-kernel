build:
	cargo build --target wasm32-unknown-unknown --release
	wasm-strip ../target/wasm32-unknown-unknown/release/tickets_tz_kernel.wasm
	
	smart-rollup-installer get-reveal-installer \
	--upgrade-to ../target/wasm32-unknown-unknown/release/tickets_tz_kernel.wasm \
	--output ./installer_output/installer.hex \
	--preimages-dir ./installer_output/preimages

prepare:
	mkdir -p rollup_data/wasm_2_0_0
	cp -r ./installer_output/preimages/* ./rollup_data/wasm_2_0_0
	
debug-kernel:
	octez-smart-rollup-wasm-debugger ../target/wasm32-unknown-unknown/release/kernel.wasm --inputs ./debug/inputs.json