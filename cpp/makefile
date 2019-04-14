buildCpp:
	docker run --rm -t -i --volume "$(PWD)":/src trzeci/emscripten emcc -O3 -s ALLOW_MEMORY_GROWTH=1 -s EXPORTED_FUNCTIONS=['_pixelmatch'] -o pixelmatch.js -s pixelmatch.cpp
