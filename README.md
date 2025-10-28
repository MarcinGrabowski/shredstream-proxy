# Jito Shredstream Proxy

ShredStream provides the lowest latency to shreds from leaders on Solana. 

See more at https://docs.jito.wtf/lowlatencytxnfeed/


export PROTOC=/usr/bin/protoc
export PROTOBUF_SRC_BUILD_JOBS=1
export ROCKSDB_BUILD_JOBS=1








# sprawdź obecną wersję (pewnie 3.12.x)
protoc --version

# zainstaluj nowy protoc (np. 27.x) – binarka dla Linux x86_64
sudo apt-get update && sudo apt-get install -y unzip
PROTOC_VER=27.1
curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VER}/protoc-${PROTOC_VER}-linux-x86_64.zip
sudo unzip -o protoc-${PROTOC_VER}-linux-x86_64.zip -d /usr/local
protoc --version   # powinno pokazać 27.x