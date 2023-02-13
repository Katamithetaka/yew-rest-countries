
mkdir dist
cd dist
mkdir public-url
cd ..

trunk build --release --public-url /rest-countries --dist dist/public-url/rest-countries
trunk build --release --public-url / --dist dist/public-url/root