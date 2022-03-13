$projectname=$args[0]

echo "Building App"
cd ./examples/$projectname
cargo build --release --target wasm32-unknown-unknown
cd ./target/wasm32-unknown-unknown/release/
cp $projectname".wasm" a.wasm
echo "Converting Binary to C Header"
wsl xxd -i a.wasm > app.wasm16.h
echo "Converting UTF-16LE to UTF-8"
wsl iconv -f UTF-16LE -t UTF-8 ./app.wasm16.h -o ../../../../../source/app.wasm.h 
cd ../../../../..
echo "Building 3DS Project"
make