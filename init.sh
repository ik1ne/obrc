pushd $(dirname "$0")/setup
python create_measurements.py 1_000_000_000
go build
time ./1brc-go > actual.txt
popd
