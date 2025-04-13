example-minimum:
	mkdir -p output
	cargo run --example minimum -- svg/aday.svg output/out.png
example-svg2pdf:
	mkdir -p output
	cargo run --example svg2pdf -- svg/aday.svg output/out.pdf
example: example-minimum example-svg2pdf
	@: nothing
generate: generate-wasm generate-frontend generate-public
	@: nothing
generate-wasm:
	cargo install wasm-pack
	@: 昔は1.81でないと動かなかった
	@: rustup default 1.81
	wasm-pack build . -d ./frontend/output
	@: rustup default stable
generate-frontend:
	cd frontend && npm install
generate-public:
	mkdir -p frontend/public/output
	cp -rf svg frontend/public/output/
run:
	cd frontend && npm run dev
create-next-app:
	npx create-next-app@latest frontend --no-tailwind --no-turbopack --yes