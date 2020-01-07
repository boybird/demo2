cd blog_front_end
export  GENERATE_SOURCEMAP=false
rm -rf build
npm run build --nomaps
cd ..
cargo build --release