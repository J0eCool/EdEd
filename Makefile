WASM_FILES=\
modules/out/canvas.wasm \
modules/out/input.wasm \
modules/out/notes.wasm \
modules/out/texture.wasm \

OPT=-O1

default: $(WASM_FILES)

modules/out/hello.wasm: modules/hello.rs
	mkdir -p modules/out
	rustc --target wasm32-unknown-unknown $< -o $@

modules/out/%.wasm: modules/%.cpp
	mkdir -p modules/out
	python ../it-tools/src/cpp_itl_generator.py $< --cpp modules/out/$*.cpp --itl modules/out/$*.itl --wasm $@
	emcc modules/out/$*.cpp -o $@ $(OPT) -s ERROR_ON_UNDEFINED_SYMBOLS=0 -Imodules/out -Imodules -std=c++11
	wasm-decompile $@ -o modules/out/$*.wade

build: src/*.rs
	cargo build

run: $(WASM_FILES)
	cargo run
.PHONY: run
