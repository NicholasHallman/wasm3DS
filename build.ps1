echo "Building App"
cd ./app
cargo build --target wasm32-unknown-unknown
echo "Converting Binary to C Header"
wsl xxd -i ./target/wasm32-unknown-unknown/debug/app.wasm > ../source/app.wasm16.h
cd ..
echo "Converting UTF-16LE to UTF-8"
wsl iconv -f UTF-16LE -t UTF-8 ./source/app.wasm16.h -o ./source/app.wasm.h 
echo "Building 3DS Project"
make