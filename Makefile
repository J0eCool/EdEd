run: modules/out/game.wasm
	cargo run

modules/out/hello.wasm: modules/hello.rs
	mkdir -p modules/out
	rustc --target wasm32-unknown-unknown $< -o $@

modules/out/%.wasm: modules/%.cpp
	mkdir -p modules/out
	echo "this'n" $*
	python ../it-tools/src/cpp_itl_generator.py $< --cpp-out modules/out/$*.cpp --itl-out modules/out/$*.itl
	emcc modules/out/$*.cpp -o $@ -O1 -s ERROR_ON_UNDEFINED_SYMBOLS=0 -Imodules/out -Imodules -std=c++11
	wasm-decompile $@ -o modules/out/$*.wade
